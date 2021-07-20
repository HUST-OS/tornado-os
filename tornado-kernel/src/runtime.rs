use riscv::register::{
    sstatus::{self, Sstatus, SPP},
    scause::{self, Scause, Trap, Exception},
    stvec::{self, TrapMode}, stval,
    satp::Satp,
};
use core::{
    pin::Pin,
    ops::{Generator, GeneratorState},
};
use crate::mm;

pub fn init(trampoline_va_start: mm::VirtAddr) {
    extern "C" { fn strampoline(); }
    let trampoline_pa_start = strampoline as usize;
    let trap_entry_fn_pa = trampoline_trap_entry as usize;
    let trap_entry_fn_va = trap_entry_fn_pa - trampoline_pa_start + trampoline_va_start.0;
    let mut addr = trap_entry_fn_va;
    if addr & 0x2 != 0 {
        addr += 0x2; // 必须对齐到4个字节
    }
    unsafe { stvec::write(addr, TrapMode::Direct) };
}

#[repr(C)]
pub struct Runtime { 
    task_satp: Satp,
    trampoline_resume: fn(*mut ResumeContext, Satp),
    current_stack: mm::VirtAddr,
    context_addr: mm::VirtAddr,
}

impl Runtime {
    pub fn new(trampoline_va_start: mm::VirtAddr, context_addr: mm::VirtAddr) -> Self {
        Runtime {
            task_satp: unsafe { core::mem::MaybeUninit::zeroed().assume_init() },
            current_stack: unsafe { core::mem::MaybeUninit::zeroed().assume_init() },
            trampoline_resume: {
                extern "C" { fn strampoline(); }
                let trampoline_pa_start = strampoline as usize;
                let resume_fn_pa = trampoline_resume as usize;
                let resume_fn_va = resume_fn_pa - trampoline_pa_start + trampoline_va_start.0;
                // println!("pa start = {:x?}, pa = {:x?}, va = {:x?}",trampoline_pa_start, resume_fn_pa, resume_fn_va);
                unsafe { core::mem::transmute(resume_fn_va) }
            },
            context_addr,
        }
    }

    unsafe fn reset(&mut self) {
        self.context_mut().sstatus = sstatus::read();
        self.context_mut().kernel_stack = 0x233333666666; // 将会被resume函数覆盖
    }

    // 在处理异常的时候，使用context_mut得到运行时当前用户的上下文，可以改变上下文的内容
    pub unsafe fn context_mut(&mut self) -> &mut ResumeContext {
        &mut *(self.context_addr.0 as *mut ResumeContext)
    }

    pub unsafe fn prepare_user_app(&mut self, new_stack: mm::VirtAddr, new_sepc: usize, new_satp: Satp, privilege: SPP) {
        sstatus::set_spp(privilege);
        self.reset();
        self.context_mut().sp = new_stack.0;
        self.context_mut().sepc = new_sepc;
        self.task_satp = new_satp;
    }

    pub fn execute_until_trap(self: Pin<&mut Self>) -> KernelTrap {
        let (scause, stval) = match self.resume(()) {
            GeneratorState::Yielded(v) => v,
            _ => unreachable!()
        };
        match scause.cause() {
            Trap::Exception(Exception::UserEnvCall) => KernelTrap::Syscall(),
            Trap::Exception(Exception::LoadFault) => KernelTrap::LoadAccessFault(stval),
            Trap::Exception(Exception::StoreFault) => KernelTrap::StoreAccessFault(stval),
            Trap::Exception(Exception::IllegalInstruction) => KernelTrap::IllegalInstruction(stval),
            e => panic!("unhandled exception: {:?}! stval: {:#x?}", e, stval),
            // e => panic!("unhandled exception: {:?}! stval: {:#x?}, ctx: {:#x?}", e, stval, unsafe { self.context_mut() })
        }
    }
}

impl Generator for Runtime {
    type Yield = (Scause, usize);
    type Return = ();
    fn resume(mut self: Pin<&mut Self>, _arg: ()) -> GeneratorState<Self::Yield, Self::Return> {
        (self.trampoline_resume)(
            unsafe { self.context_mut() } as *mut _,
            self.task_satp
        );
        let stval = stval::read();
        GeneratorState::Yielded((scause::read(), stval))
    }
}

#[derive(Debug)]
#[repr(C)]
pub enum KernelTrap {
    Syscall(),
    LoadAccessFault(usize),
    StoreAccessFault(usize),
    IllegalInstruction(usize),
}

// 应当放到跳板数据页上，用户和内核
#[derive(Debug)]
#[repr(C)]
pub struct ResumeContext {
    pub ra: usize, // 0
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize, // 30
    pub sstatus: Sstatus, // 31
    pub sepc: usize, // 32
    pub kernel_stack: usize, // 33
    pub kernel_satp: Satp, // 34
}

/*
跳板页设计：
1. 共享的代码跳板页 - 整个系统里有一个
2. 数据跳板页 - 每个处理核一个，暂时中转目前内核和用户的上下文
trampoline段：保存共享的代码
分配的其它页，每个核一个：保存数据跳板页
*/

// 函数作用：
// 1. 先保存寄存器
// 2. 再切换地址空间
// a0 = 生成器上下文
// sp = 内核栈
// sscratch = 用户的a0值
// 注意：_ctx必须也映射到跳板页里面
#[naked]
#[link_section = ".trampoline"] 
unsafe extern "C" fn trampoline_resume(_ctx: *mut ResumeContext, _user_satp: usize) {
    asm!(
        // a0 = 生成器上下文, a1 = 用户的地址空间配置, sp = 内核栈
        "addi   sp, sp, -15*8",
        "sd     ra, 0*8(sp)
        sd      gp, 1*8(sp)
        sd      tp, 2*8(sp)
        sd      s0, 3*8(sp)
        sd      s1, 4*8(sp)
        sd      s2, 5*8(sp)
        sd      s3, 6*8(sp)
        sd      s4, 7*8(sp)
        sd      s5, 8*8(sp)
        sd      s6, 9*8(sp)
        sd      s7, 10*8(sp)
        sd      s8, 11*8(sp)
        sd      s9, 12*8(sp)
        sd      s10, 13*8(sp)
        sd      s11, 14*8(sp)", // 保存子函数寄存器，到内核栈
        "csrrw  a1, satp, a1", // 写用户的地址空间配置到satp，读内核的satp到a1
        "sfence.vma", // 立即切换地址空间
        // a0 = 生成器上下文, a1 = 内核的地址空间配置, sp = 内核栈
        "sd     sp, 33*8(a0)", // 保存内核栈位置
        "mv     sp, a0", 
        // a1 = 内核的地址空间配置, sp = 生成器上下文
        "sd     a1, 34*8(sp)", // 保存内核的地址空间配置
        "ld     t0, 31*8(sp)
        ld      t1, 32*8(sp)
        csrw    sstatus, t0
        csrw    sepc, t1
        ld      ra, 0*8(sp)
        ld      gp, 2*8(sp)
        ld      tp, 3*8(sp)
        ld      t0, 4*8(sp)
        ld      t1, 5*8(sp)
        ld      t2, 6*8(sp)
        ld      s0, 7*8(sp)
        ld      s1, 8*8(sp)
        ld      a0, 9*8(sp)
        ld      a1, 10*8(sp)
        ld      a2, 11*8(sp)
        ld      a3, 12*8(sp)
        ld      a4, 13*8(sp)
        ld      a5, 14*8(sp)
        ld      a6, 15*8(sp)
        ld      a7, 16*8(sp)
        ld      s2, 17*8(sp)
        ld      s3, 18*8(sp)
        ld      s4, 19*8(sp)
        ld      s5, 20*8(sp)
        ld      s6, 21*8(sp)
        ld      s7, 22*8(sp)
        ld      s8, 23*8(sp)
        ld      s9, 24*8(sp)
        ld     s10, 25*8(sp)
        ld     s11, 26*8(sp)
        ld      t3, 27*8(sp)
        ld      t4, 28*8(sp)
        ld      t5, 29*8(sp)
        ld      t6, 30*8(sp)", // 加载生成器上下文寄存器，除了a0
        // sp = 生成器上下文
        "csrw   sscratch, sp",
        "ld     sp, 1*8(sp)", // 加载用户栈
        // sp = 用户栈, sscratch = 生成器上下文
        "sret", // set priv, j sepc
        options(noreturn)
    )
}

#[naked]
#[link_section = ".trampoline"]
unsafe extern "C" fn trampoline_trap_entry() {
    asm!(
        ".p2align 2", // 对齐到4字节
        // sp = 用户栈, sscratch = 生成器上下文
        "csrrw  sp, sscratch, sp", 
        // sp = 生成器上下文, sscratch = 用户栈
        "sd     ra, 0*8(sp)
        sd      gp, 2*8(sp)
        sd      tp, 3*8(sp)
        sd      t0, 4*8(sp)
        sd      t1, 5*8(sp)
        sd      t2, 6*8(sp)
        sd      s0, 7*8(sp)
        sd      s1, 8*8(sp)
        sd      a0, 9*8(sp)
        sd      a1, 10*8(sp)
        sd      a2, 11*8(sp)
        sd      a3, 12*8(sp)
        sd      a4, 13*8(sp)
        sd      a5, 14*8(sp)
        sd      a6, 15*8(sp)
        sd      a7, 16*8(sp)
        sd      s2, 17*8(sp)
        sd      s3, 18*8(sp)
        sd      s4, 19*8(sp)
        sd      s5, 20*8(sp)
        sd      s6, 21*8(sp)
        sd      s7, 22*8(sp)
        sd      s8, 23*8(sp)
        sd      s9, 24*8(sp)
        sd     s10, 25*8(sp)
        sd     s11, 26*8(sp)
        sd      t3, 27*8(sp)
        sd      t4, 28*8(sp)
        sd      t5, 29*8(sp)
        sd      t6, 30*8(sp)",
        "csrr   t0, sstatus
        sd      t0, 31*8(sp)",
        "csrr   t1, sepc
        sd      t1, 32*8(sp)",
        // sp = 生成器上下文, sscratch = 用户栈
        "csrrw  t2, sscratch, sp", 
        // sp = 生成器上下文, sscratch = 生成器上下文, t2 = 用户栈
        "sd     t2, 1*8(sp)", // 保存用户栈
        "ld     t3, 34*8(sp)", // t3 = 内核的地址空间配置
        "csrw   satp, t3", // 写内核的地址空间配置；用户的地址空间配置将丢弃
        "sfence.vma", // 立即切换地址空间
        "ld     sp, 33*8(sp)", 
        // sp = 内核栈
        "ld     ra, 0*8(sp)
        ld      gp, 1*8(sp)
        ld      tp, 2*8(sp)
        ld      s0, 3*8(sp)
        ld      s1, 4*8(sp)
        ld      s2, 5*8(sp)
        ld      s3, 6*8(sp)
        ld      s4, 7*8(sp)
        ld      s5, 8*8(sp)
        ld      s6, 9*8(sp)
        ld      s7, 10*8(sp)
        ld      s8, 11*8(sp)
        ld      s9, 12*8(sp)
        ld      s10, 13*8(sp)
        ld      s11, 14*8(sp)
        addi    sp, sp, 15*8", // sp = 内核栈
        "jr     ra", // ret指令
        options(noreturn)
    )
}
