use super::timer;
use crate::{syscall::{syscall as do_syscall, SyscallResult}, task::kernel_should_switch};
use crate::task::TaskResult;
use crate::hart;
use core::fmt;
use riscv::register::{
    scause::{self, Exception, Trap},
    sepc,
    sstatus::{self, Sstatus, SPP},
    stval, stvec,
};

macro_rules! save_non_switch {
    () => {
        "addi   sp, sp, -16 * {REGBYTES}
        SAVE   ra, 0
        SAVE   t0, 1
        SAVE   t1, 2
        SAVE   t2, 3
        SAVE   t3, 4
        SAVE   t4, 5
        SAVE   t5, 6
        SAVE   t6, 7
        SAVE   a0, 8
        SAVE   a1, 9
        SAVE   a2, 10
        SAVE   a3, 11
        SAVE   a4, 12
        SAVE   a5, 13
        SAVE   a6, 14
        SAVE   a7, 15"
    };
}

macro_rules! restore_non_switch {
    () => {
        "LOAD    ra, 0
        LOAD    t0, 1
        LOAD    t1, 2
        LOAD    t2, 3
        LOAD    t3, 4
        LOAD    t4, 5
        LOAD    t5, 6
        LOAD    t6, 7
        LOAD    a0, 8
        LOAD    a1, 9
        LOAD    a2, 10
        LOAD    a3, 11
        LOAD    a4, 12
        LOAD    a5, 13
        LOAD    a6, 14
        LOAD    a7, 15
        addi    sp, sp, 16 * {REGBYTES}"
    };
}

macro_rules! save_switch {
    () => {
        "addi	sp, sp, -34 * {REGBYTES}
        SAVE	x1, 1
        addi	x1, sp, 34 * {REGBYTES}
        SAVE	x1, 2
        SAVE	x3, 3
        SAVE	x4, 4
        SAVE	x5, 5
        SAVE	x6, 6
        SAVE	x7, 7
        SAVE	x8, 8
        SAVE	x9, 9
        SAVE	x10, 10
        SAVE	x11, 11
        SAVE	x12, 12
        SAVE	x13, 13
        SAVE	x14, 14
        SAVE	x15, 15
        SAVE	x16, 16
        SAVE	x17, 17
        SAVE	x18, 18
        SAVE	x19, 19
        SAVE	x20, 20
        SAVE	x21, 21
        SAVE	x22, 22
        SAVE	x23, 23
        SAVE	x24, 24
        SAVE	x25, 25
        SAVE	x26, 26
        SAVE	x27, 27
        SAVE	x28, 28
        SAVE	x29, 29
        SAVE	x30, 30
        SAVE	x31, 31
        csrr 	t0, sstatus
        csrr 	t1, sepc
        SAVE	t0, 32
        SAVE	t1, 33"
    };
}

macro_rules! restore_switch {
    () => {
        "
        LOAD	t0, 32
        LOAD	t1, 33
        csrw 	sstatus, t0
        csrw	sepc, t1
        LOAD	x1, 1
        LOAD	x3, 3
        LOAD	x4, 4
        LOAD	x5, 5
        LOAD	x6, 6
        LOAD	x7, 7
        LOAD	x8, 8
        LOAD	x9, 9
        LOAD	x10, 10
        LOAD	x11, 11
        LOAD	x12, 12
        LOAD	x13, 13
        LOAD	x14, 14
        LOAD	x15, 15
        LOAD	x16, 16
        LOAD	x17, 17
        LOAD	x18, 18
        LOAD	x19, 19
        LOAD	x20, 20
        LOAD	x21, 21
        LOAD	x22, 22
        LOAD	x23, 23
        LOAD	x24, 24
        LOAD	x25, 25
        LOAD	x26, 26
        LOAD	x27, 27
        LOAD	x28, 28
        LOAD	x29, 29
        LOAD	x30, 30
        LOAD	x31, 31
        LOAD	sp, 2"
    };
}

impl TrapFrame {
    // 新建任务时，构建它的上下文
    pub fn new_task_context(is_user: bool, pc: usize, tp: usize, stack_top: usize) -> TrapFrame {
        // 设置sstatus的特权级
        if is_user {
            unsafe { sstatus::set_spp(SPP::User) };
        } else {
            unsafe { sstatus::set_spp(SPP::Supervisor) };
        }
        // sret到用户线程后，开启中断
        unsafe { sstatus::set_spie() };
        let sstatus = sstatus::read();
        let mut ans = TrapFrame {
            x: [0; 32],
            sstatus,
            sepc: pc,
        };
        // 设置栈顶
        ans.x[2] = stack_top;
        // 设置线程指针
        ans.x[4] = tp;
        ans
    }
}

#[naked]
#[link_section = ".text"]
pub unsafe extern "C" fn trap_vector() {
    asm!("
	.option push 
	.option norvc
	j	{trap_exception}
	j	{supervisor_software}
	j	{interrupt_reserved}
	j	{interrupt_reserved}
	j	{interrupt_reserved}
	j	{supervisor_timer}
	j	{interrupt_reserved}
	j	{interrupt_reserved}
	j	{interrupt_reserved}
	j	{supervisor_external}
	j	{interrupt_reserved}
	j	{interrupt_reserved}
	j	{interrupt_reserved}
	j	{interrupt_reserved}
	j	{interrupt_reserved}
	j	{interrupt_reserved}
	.option pop
    ",
    trap_exception = sym trap_exception,
    supervisor_software = sym supervisor_software,
    supervisor_timer = sym supervisor_timer,
    supervisor_external = sym supervisor_external,
    interrupt_reserved = sym interrupt_reserved,
    options(noreturn))
}

#[naked]
#[link_section = ".text"]
pub unsafe extern "C" fn interrupt_reserved() -> ! {
    // 死循环
    asm!("1: j 1b", options(noreturn))
}

pub fn init() {
    unsafe {
        stvec::write(trap_vector as usize, stvec::TrapMode::Vectored);
    }
}

#[naked]
#[link_section = ".text"]
pub unsafe extern "C" fn supervisor_timer() {
    asm!(
        define_load_store!(),
        save_non_switch!(),
        "mv     a0, sp",
        "call   {supervisor_timer}",
        restore_non_switch!(),
        "sret",
        REGBYTES = const core::mem::size_of::<usize>(),
        supervisor_timer = sym rust_supervisor_timer,
        options(noreturn)
    )
}

pub extern "C" fn rust_supervisor_timer(trap_frame: &mut TrapFrame) -> *mut TrapFrame {
    // panic!("Supervisor timer: {:08x}", sepc::read());
    // 设置下一个时钟中断时间
    timer::set_next_timeout();
    // 找到下一个应当切换到的地址空间
      // 当前的asid用于should_yield函数
    let next_task = unsafe {
        (&*crate::SHARED_PAYLOAD).peek_task(kernel_should_switch)
    };
    // 根据地址空间，找到相应的satp寄存器和用户信息，如SwapContext
    match next_task {
        TaskResult::ShouldYield(user_task_id, user_asid_val) => {
            println!("[kernel] 时钟中断，任务编号{}，空间{}", user_task_id, user_asid_val);
            let user_asid = unsafe {core::mem::transmute(user_asid_val as u16) };
            // 切换到对应的地址空间
            let user_satp = hart::KernelHartInfo::get_satp(user_asid)
                .expect("fetch satp register from next asid");
            // super::switch_to_user(context, user_satp.inner())
            todo!()
        },
        TaskResult::Task(_task_id, _task_repr) => unimplemented!("should not be a task here"),
        TaskResult::NoWakeTask => unsafe { riscv::asm::wfi() },
        TaskResult::Finished => crate::sbi::shutdown()
    }
    trap_frame
}

pub fn supervisor_software() {
    panic!("Supervisor software: {:08x}", sepc::read());
}

pub fn supervisor_external() {
    panic!("Supervisor external: {:08x}", sepc::read());
}

#[naked]
#[link_section = ".text"]
pub unsafe extern "C" fn trap_exception() {
    asm!(
        save_switch!(),
        "mv     a0, sp",
        "call   {trap_exception}",
        restore_switch!(),
        "sret",
        REGBYTES = const core::mem::size_of::<usize>(),
        trap_exception = sym rust_trap_exception,
        options(noreturn)
    )
}

pub extern "C" fn rust_trap_exception(trap_frame: &mut TrapFrame) -> *mut TrapFrame {
    match scause::read().cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(trap_frame),
        Trap::Exception(Exception::UserEnvCall) => syscall(trap_frame),
        Trap::Exception(e) => panic!(
            "Exception! {:?}, sepc: {:#08x}, stval: {:#08x}, trap_frame: {}",
            e,
            sepc::read(),
            stval::read(),
            trap_frame
        ),
        Trap::Interrupt(_) => unreachable!("SBI or CPU design fault"),
    }
}

fn breakpoint(trap_frame: &mut TrapFrame) -> *mut TrapFrame {
    println!("Breakpoint at {:#08x}", trap_frame.sepc);
    trap_frame.sepc = trap_frame.sepc.wrapping_add(2);
    trap_frame
}

fn syscall(trap_frame: &mut TrapFrame) -> *mut TrapFrame {
    println!(
        "Syscall at {:#08x}; note that user_satp /*todo*/",
        trap_frame.sepc
    );
    let param = [trap_frame.x[10], trap_frame.x[11], 0, 0, 0, 0]; // a0, a1
    match do_syscall(param, 0 /* todo */, trap_frame.x[16], trap_frame.x[17]) {
        // a6, a7
        SyscallResult::Procceed { code, extra } => {
            trap_frame.x[10] = code; // a0
            trap_frame.x[11] = extra; // a1
            trap_frame.sepc = trap_frame.sepc.wrapping_add(4); // skip `ecall` instruction
            trap_frame
        }
        SyscallResult::Retry => trap_frame, // don't skip
        _ => unimplemented!(),
    }
}

/// 强制陷入内核时需要保存的上下文
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TrapFrame {
    x: [usize; 32],
    sstatus: Sstatus,
    sepc: usize,
}

impl fmt::Display for TrapFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.x;
        writeln!(f, "")?;
        writeln!(
            f,
            "x0: {:016x}, ra: {:016x}, sp: {:016x}, gp: {:016x}",
            0, x[1], x[2], x[3]
        )?;
        writeln!(
            f,
            "tp: {:016x}, t0: {:016x}, t1: {:016x}, t2: {:016x}",
            x[4], x[5], x[6], x[7]
        )?;
        writeln!(
            f,
            "s0: {:016x}, s1: {:016x}, a0: {:016x}, a1: {:016x}",
            x[8], x[9], x[10], x[11]
        )?;
        writeln!(
            f,
            "a2: {:016x}, a3: {:016x}, a4: {:016x}, a5: {:016x}",
            x[12], x[13], x[14], x[15]
        )?;
        writeln!(
            f,
            "a6: {:016x}, a7: {:016x}, s2: {:016x}, s3: {:016x}",
            x[16], x[17], x[18], x[19]
        )?;
        writeln!(
            f,
            "s4: {:016x}, s5: {:016x}, s6: {:016x}, s7: {:016x}",
            x[20], x[21], x[22], x[23]
        )?;
        writeln!(
            f,
            "s8: {:016x}, s9: {:016x}, s10:{:016x}, s11:{:016x}",
            x[24], x[25], x[26], x[27]
        )?;
        writeln!(
            f,
            "t3: {:016x}, t4: {:016x}, t5: {:016x}, t6: {:016x}",
            x[28], x[29], x[30], x[31]
        )
    }
}
