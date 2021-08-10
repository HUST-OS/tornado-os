//! 从用户过来的系统调用在这里处理
use super::{syscall, SyscallResult};
use crate::{
    async_rt::{self, ext_intr_off, ext_intr_on, TaskState},
    hart::KernelHartInfo,
    memory::{self, Satp},
    memory::{AddressSpaceId, VirtualAddress, VirtualPageNumber, KERNEL_MAP_OFFSET},
    plic, task,
    trap::{self, SwapContext},
    virtio::VIRTIO_BLOCK,
    SHAREDPAYLOAD_BASE,
};
use riscv::register::{
    scause::{self, Interrupt, Trap},
    sepc, sie, stval, stvec,
};

const BLOCK_SIZE: usize = 512;
pub static mut WAKE_NUM: usize = 1;

/// 中断/异常/系统调用处理函数，用户态发生中断/异常/系统调用会陷入到这里
pub extern "C" fn user_trap_handler() {
    // 从[`KernelHartInfo`]中获取用户地址空间的 [`Satp`] 结构
    let user_satp = KernelHartInfo::prev_satp().expect("get prev user satp");
    // 从[`KernelHartInfo`]中获取用户地址空间编号
    let asid = KernelHartInfo::get_prev_asid();
    let swap_cx = unsafe { get_swap_cx(&user_satp, asid) };
    // 从SwapContext中获取用户传给内核的参数
    let mut param = [0usize; 6];
    for (idx, x) in swap_cx.x[9..15].iter().enumerate() {
        param[idx] = *x;
    }
    let a6 = swap_cx.x[15];
    let a7 = swap_cx.x[16];
    match scause::read().cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // 时钟中断，还未实现
            println!("s mode timer!");
            unimplemented!() // todo
        }
        Trap::Exception(scause::Exception::Breakpoint) => {
            // 用户目前通过断点异常通知内核发生了错误，这时候直接退出
            println!("user mode panic!");
            crate::sbi::shutdown();
        }
        Trap::Exception(scause::Exception::UserEnvCall) => {
            // 用户系统调用
            match syscall(param, user_satp.inner(), a6, a7) {
                SyscallResult::Procceed { code, extra } => {
                    swap_cx.x[9] = code;
                    swap_cx.x[10] = extra;
                    swap_cx.epc = swap_cx.epc.wrapping_add(4);
                    trap::switch_to_user(swap_cx, user_satp.inner(), asid)
                }
                SyscallResult::Retry => {
                    // 不跳过指令，继续运行
                    trap::switch_to_user(swap_cx, user_satp.inner(), asid)
                }
                SyscallResult::NextASID { asid, satp } => {
                    // 需要切换地址空间
                    // 跳过`do_yield`指令
                    swap_cx.epc = swap_cx.epc.wrapping_add(4);
                    // 需要转到目标地址空间去运行
                    println!("[syscall] yield: {}", asid);
                    let next_swap_contex = unsafe { get_swap_cx(&satp, asid) };
                    trap::switch_to_user(next_swap_contex, satp.inner(), asid)
                }
                SyscallResult::KernelTask => {
                    // 需要运行内核任务
                    // 跳过 `do_yield` 指令
                    swap_cx.epc = swap_cx.epc.wrapping_add(4);
                    println!("[syscall] yield kernel");
                    let shared_payload =
                        unsafe { async_rt::SharedPayload::load(crate::SHAREDPAYLOAD_BASE) };
                    trap::init();
                    async_rt::run_until_idle(
                        || unsafe { shared_payload.peek_task(async_rt::kernel_should_switch) },
                        |task_repr| unsafe { shared_payload.delete_task(task_repr) },
                        |task_repr, new_state| unsafe {
                            shared_payload.set_task_state(task_repr, new_state)
                        },
                    );
                    crate::end()
                }
                SyscallResult::IOTask {
                    // 需要注册IO任务
                    block_id,
                    buf_ptr,
                    write,
                } => {
                    let wake_task_repr = unsafe { next_task_repr() };
                    let process = KernelHartInfo::current_process().expect("get kernel process");
                    unsafe {
                        let shared_payload = async_rt::SharedPayload::load(SHAREDPAYLOAD_BASE);
                        let task = if write {
                            task::new_kernel(
                                write_block_task(
                                    block_id,
                                    buf_ptr,
                                    user_satp.inner(),
                                    wake_task_repr,
                                ),
                                process,
                                shared_payload.shared_scheduler,
                                shared_payload.shared_set_task_state,
                            )
                        } else {
                            task::new_kernel(
                                read_block_task(
                                    block_id,
                                    buf_ptr,
                                    user_satp.inner(),
                                    wake_task_repr,
                                ),
                                process,
                                shared_payload.shared_scheduler,
                                shared_payload.shared_set_task_state,
                            )
                        };
                        let task_repr = task.task_repr();
                        println!("[syscall] new kernel task: {:x}", task_repr);
                        ext_intr_off();
                        shared_payload.add_task(0, AddressSpaceId::from_raw(0), task_repr);
                        ext_intr_on();
                    }
                    // 运行下一条指令
                    swap_cx.epc = swap_cx.epc.wrapping_add(4);
                    trap::switch_to_user(swap_cx, user_satp.inner(), asid)
                }
                SyscallResult::Check => {
                    // 内核检查
                    // 如果有未唤醒的块设备读写任务，将其唤醒
                    unsafe {
                        if WAKE_NUM > 1 {
                            VIRTIO_BLOCK.0.wake_ops.notify(WAKE_NUM);
                            WAKE_NUM = 1;
                        }
                    }
                    // 运行下一条指令
                    swap_cx.epc = swap_cx.epc.wrapping_add(4);
                    trap::switch_to_user(swap_cx, user_satp.inner(), asid)
                }
                SyscallResult::Terminate(exit_code) => {
                    // 用户执行器认为可以退出系统了
                    println!("User exit!");
                    crate::sbi::shutdown();
                }
            }
        }
        Trap::Interrupt(Interrupt::SupervisorExternal) => {
            // 用户态被外部中断打断
            // 用户态被外部中断打断
            //
            // todo: 这里有个问题，就是最后一次外部中断的时候程序运行在共享调度器里面的时候
            // 后面的任务不会被唤醒，执行器一直轮询找不到`醒着的`任务
            //
            // 目前的想法是用户执行器需要一个自检机制，当一直轮询找不到任务的次数达到某个阈值的时候
            // 通过一个系统调用陷入内核检查是否有没有唤醒的块设备读写任务，将其唤醒
            unsafe {
                let irq = plic::plic_claim();
                if irq == 1 {
                    // virtio 外部中断
                    let _intr_ret = VIRTIO_BLOCK.handle_interrupt().unwrap();
                    let sepc = sepc::read();
                    if sepc < SHAREDPAYLOAD_BASE {
                        println!("extr intr in user");
                        // 运行在用户程序中
                        // 唤醒一定数量的任务
                        VIRTIO_BLOCK.0.wake_ops.notify(WAKE_NUM);
                        WAKE_NUM = 1;
                    } else {
                        // 运行在共享调度器中
                        // 这里不唤醒，增加计数，下次外部中断来的时候一起唤醒
                        println!("extr intr in shared");
                        WAKE_NUM += 1;
                    }
                    plic::plic_complete(irq);
                    // 不跳过指令回到用户态运行
                    trap::switch_to_user(swap_cx, user_satp.inner(), asid)
                } else {
                    panic!("unknown S mode external interrupt! irq: {}", irq);
                }
            }
        }
        _ => todo!(
            "scause: {:?}, sepc: {:#x}, stval: {:#x}, {:x?}",
            scause::read().cause(),
            sepc::read(),
            stval::read(),
            swap_cx
        ),
    }
}

/// 获取[`SwapContext`]的可变引用
///
/// 给定 satp 寄存器，获取 [`SwapContext`] 的裸指针
///
/// todo: 需要根据地址空间编号来得到 [`SwapContext`]
pub unsafe fn get_swap_cx<'cx>(satp: &'cx Satp, asid: usize) -> &'cx mut SwapContext {
    let swap_cx_va = VirtualAddress(memory::swap_contex_va(asid));
    let swap_cx_vpn = VirtualPageNumber::floor(swap_cx_va);
    let swap_cx_ppn = satp.translate(swap_cx_vpn).unwrap();
    // 将物理页号转换成裸指针
    (swap_cx_ppn
        .start_address()
        .0
        .wrapping_add(KERNEL_MAP_OFFSET) as *mut SwapContext)
        .as_mut()
        .unwrap()
}

#[allow(missing_docs)]
async fn read_block_task(block_id: usize, buf_ptr: usize, user_satp: usize, wake_task_repr: usize) {
    let buf = unsafe { super::get_user_buf_mut(user_satp, buf_ptr, BLOCK_SIZE) };
    VIRTIO_BLOCK.read_block(block_id, buf).await;
    unsafe {
        let shared_payload = async_rt::SharedPayload::load(SHAREDPAYLOAD_BASE);
        ext_intr_off();
        shared_payload.set_task_state(wake_task_repr, TaskState::Ready);
        ext_intr_on();
    }
}

#[allow(missing_docs)]
async fn write_block_task(
    block_id: usize,
    buf_ptr: usize,
    user_satp: usize,
    wake_task_repr: usize,
) {
    let buf = unsafe { super::get_user_buf_mut(user_satp, buf_ptr, BLOCK_SIZE) };
    VIRTIO_BLOCK.write_block(block_id, buf).await;
    unsafe {
        let shared_payload = async_rt::SharedPayload::load(SHAREDPAYLOAD_BASE);
        ext_intr_off();
        shared_payload.set_task_state(wake_task_repr, TaskState::Ready);
        ext_intr_on();
    }
}

/// 从共享调度器中拿出下一个任务的指针，不弹出
///
/// note: 这个函数需要保证调用时共享调度器 `peek_task` 的返回值是任务指针
/// 一般只用于 `enroll_read` 或 `enroll_write` 系统调用
unsafe fn next_task_repr() -> usize {
    let shared_payload = unsafe { async_rt::SharedPayload::load(SHAREDPAYLOAD_BASE) };
    ext_intr_off();
    let next_task = shared_payload.peek_task(should_switch);
    ext_intr_on();
    match next_task {
        task::TaskResult::Task(task_repr) => task_repr,
        _ => unreachable!(),
    }
}

extern "C" fn should_switch(asid: AddressSpaceId) -> bool {
    asid.into_inner() != KernelHartInfo::get_prev_asid()
}

/// 读取当前 PC 值
#[allow(unused)]
#[inline]
fn read_pc() -> usize {
    let pc: usize;
    unsafe {
        asm!("auipc {}, 0", out(reg) pc, options(nomem, nostack));
    }
    pc
}
