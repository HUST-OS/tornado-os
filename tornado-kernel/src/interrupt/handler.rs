use riscv::register::{stvec, sstatus::Sstatus, sepc, scause::{self, Trap, Exception}};
use crate::println;

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
pub struct TrapFrame {
    x: [usize; 32],
    sstatus: Sstatus,
    sepc: usize,
}

#[export_name = "rust_supervisor_timer"]
pub extern "C" fn supervisor_timer(trap_frame: &mut TrapFrame) -> *mut TrapFrame {
    // panic!("Supervisor timer: {:08x}", sepc::read());
    timer::tick();
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
            panic!("Exception! {:?}, sepc: {:#08x}", e, sepc::read()),
        Trap::Interrupt(_) => unreachable!("SBI or CPU design fault")
    }
}

fn breakpoint(trap_frame: &mut TrapFrame) -> *mut TrapFrame {
    println!("Breakpoint at {:#08x}", trap_frame.sepc);
    trap_frame.sepc = trap_frame.sepc.wrapping_add(2);
    trap_frame
}
