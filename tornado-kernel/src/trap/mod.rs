mod handler;
mod timer;

pub use handler::TrapFrame;

/// 初始化中断相关的子模块
/// 
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}

#[cfg(target_pointer_width = "64")]
macro_rules! define_load_store {
    () => {
        ".altmacro
        .macro SAVE reg, offset
            sd  \\reg, \\offset*8(sp)
        .endm
        .macro SAVE_N n
            SAVE  x\\n, \\n
        .endm
        .macro LOAD reg, offset
            ld  \\reg, \\offset*8(sp)
        .endm
        .macro LOAD_N n
            LOAD  x\\n, \\n
        .endm"
    };
}

// 这个函数里不包含写satp的过程，需要别的函数先写satp和刷新页表
#[naked]
#[link_section = ".text"]
unsafe extern "C" fn supervisor_restore(_target_frame: *mut TrapFrame) -> ! {
    asm!(define_load_store!(), "
        mv      sp, a0

        LOAD    t0, 32
        LOAD    t1, 33
        csrw    sstatus, t0
        csrw    sepc, t1

        LOAD    x1, 1
        .set    n, 3
        .rept   29
            LOAD_N  %n
            .set    n, n + 1
        .endr

        LOAD	sp, 2
        sret
    ", options(noreturn))
}
