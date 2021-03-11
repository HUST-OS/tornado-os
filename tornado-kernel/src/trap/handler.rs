use riscv::register::{stvec, sstatus::{self, SPP, Sstatus}, sepc, scause::{self, Trap, Exception}, stval};
use crate::{hart::KernelHartInfo, println};
use crate::task::current_task;

use super::timer;

#[cfg(not(test))]
global_asm!(include_str!("./interrupt.asm"));

pub fn init() {
    extern "C" {
        fn trap_vector_table();
    }
    unsafe {
        stvec::write(trap_vector_table as usize, stvec::TrapMode::Vectored);
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TrapFrame {
    x: [usize; 32],
    sstatus: Sstatus,
    sepc: usize,
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
            sepc: pc
        };
        // 设置栈顶
        ans.x[2] = stack_top;
        // 设置线程指针
        ans.x[4] = tp;
        ans
    }
}

use core::fmt;
impl fmt::Display for TrapFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.x;
        writeln!(f, "")?;
        writeln!(f, "x0: {:016x}, ra: {:016x}, sp: {:016x}, gp: {:016x}", 0, x[1], x[2], x[3])?;
        writeln!(f, "tp: {:016x}, t0: {:016x}, t1: {:016x}, t2: {:016x}", x[4], x[5], x[6], x[7])?;
        writeln!(f, "s0: {:016x}, s1: {:016x}, a0: {:016x}, a1: {:016x}", x[8], x[9], x[10], x[11])?;
        writeln!(f, "a2: {:016x}, a3: {:016x}, a4: {:016x}, a5: {:016x}", x[12], x[13], x[14], x[15])?;
        writeln!(f, "a6: {:016x}, a7: {:016x}, s2: {:016x}, s3: {:016x}", x[16], x[17], x[18], x[19])?;
        writeln!(f, "s4: {:016x}, s5: {:016x}, s6: {:016x}, s7: {:016x}", x[20], x[21], x[22], x[23])?;
        writeln!(f, "s8: {:016x}, s9: {:016x}, s10:{:016x}, s11:{:016x}", x[24], x[25], x[26], x[27])?;
        writeln!(f, "t3: {:016x}, t4: {:016x}, t5: {:016x}, t6: {:016x}", x[28], x[29], x[30], x[31])
    }
}

#[export_name = "rust_supervisor_timer"]
pub extern "C" fn supervisor_timer(trap_frame: &mut TrapFrame) -> *mut TrapFrame {
    // panic!("Supervisor timer: {:08x}", sepc::read());
    timer::tick(); // 设置下一个时钟中断时间
    // 保存当前任务的上下文
    if let Some(handle) = current_task() {
        KernelHartInfo::save_task_context(handle, trap_frame);
        println!("Current task: {:x?}", handle);
    }

    trap_frame
}

#[export_name = "rust_supervisor_software"]
pub fn supervisor_software() {
    panic!("Supervisor software: {:08x}", sepc::read());
}

#[export_name = "rust_supervisor_external"]
pub fn supervisor_external() {
    panic!("Supervisor external: {:08x}", sepc::read());
}

#[export_name = "rust_trap_exception"]
pub extern "C" fn trap_exception(trap_frame: &mut TrapFrame) -> *mut TrapFrame {
    match scause::read().cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(trap_frame),
        Trap::Exception(e) => 
            panic!("Exception! {:?}, sepc: {:#08x}, stval: {:#08x}, trap_frame: {}", e, sepc::read(), stval::read(), trap_frame),
        Trap::Interrupt(_) => unreachable!("SBI or CPU design fault")
    }
}

fn breakpoint(trap_frame: &mut TrapFrame) -> *mut TrapFrame {
    println!("Breakpoint at {:#08x}", trap_frame.sepc);
    trap_frame.sepc = trap_frame.sepc.wrapping_add(2);
    trap_frame
}