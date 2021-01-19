use riscv::register::{stvec, sepc, scause};

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

#[export_name = "supervisor_timer"]
pub fn supervisor_timer() {
    panic!("Supervisor timer: {:08x}", sepc::read());
}

#[export_name = "supervisor_software"]
pub fn supervisor_software() {
    panic!("Supervisor software: {:08x}", sepc::read());
}

#[export_name = "supervisor_external"]
pub fn supervisor_external() {
    panic!("Supervisor external: {:08x}", sepc::read());
}

#[export_name = "trap_exception"]
pub fn trap_exception() {
    panic!("Exception! {:?}, sepc: {:08x}", scause::read().cause(), sepc::read());
}
