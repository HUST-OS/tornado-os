#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(test)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;
use tornado_user::{read_timer, reset_timer};

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    const ROUNDS: [usize; 5] = [50000, 100000, 150000, 200000, 250000];
    println!("[swap-speed] start profiling!");
    // 正式测试
    for &rounds in ROUNDS.iter() {
        for test_id in 0..=5 {
            reset_timer();
            for _ in 0..rounds {
                unsafe { no_switch() }; // 没有进入上下文
                work(); // 开始运行
                unsafe { no_restore() }; // 没有上下文恢复
            }
            let time1 = read_timer();
            if test_id != 0 { // 第0次视为热身
                 // println!("[swap-speed] no swap ctx, rounds: {}, timer: {}", rounds, time1);
            }
            reset_timer();
            for _ in 0..rounds {
                unsafe { switch_by_task() };
                work(); // 开始运行
                unsafe { restore_by_task() };
            }
            let time2 = read_timer();
            if test_id != 0 {
                // println!("[swap-speed] swap by task, rounds: {}, timer: {}", rounds, time2);
            }
            reset_timer();
            for _ in 0..rounds {
                unsafe { switch_by_thread() };
                work(); // 开始运行
                unsafe { restore_by_thread() };
            }
            let time3 = read_timer();
            if test_id != 0 {
                // println!("[swap-speed] swap by thread, rounds: {}, timer: {}", rounds, time3);
            }
            if test_id != 0 {
                // (time2 - time1) / 5000 => ms
                // (time2 - time1) / 5000 / rounds => ms/round
                // (time2 - time1) / 5000 / rounds * 1000000000 => ns/it
                // (time2 - time1) * (1000000 / rounds) => ns/it
                let a1 = (time2 - time1) * (1_000_000 / rounds);
                let a2 = (time3 - time1) * (1_000_000 / rounds);
                println!(
                    "[swap-speed] rounds: {}, task: {} ns/iter, thread: {} ns/iter",
                    rounds, a1, a2
                );
            }
        }
    }
    0
}

// 计算密集型负载
#[inline(always)]
fn work() {
    for i in 0..1000 {
        let a = core::hint::black_box(i);
        let b = core::hint::black_box(1);
        let c = a + b;
        core::hint::black_box(c);
    }
}

#[inline(always)]
unsafe fn no_switch() {
    // 空函数。
}

#[inline(always)]
unsafe fn no_restore() {
    // 空函数。
}

#[inline(always)]
unsafe fn switch_by_thread() {
    let mut stack = [0usize; 35];
    let mut tmp = 0_usize;
    asm!(
    "sd     {tmp}, 33*8({stack})",
    "mv     {tmp}, {tmp}",
    "sd     {tmp}, 34*8({stack})",
    "ld     {tmp}, 31*8({stack})
    ld      {tmp}, 32*8({stack})
    addi   {tmp}, {tmp}, 1
    addi   {tmp}, {tmp}, 1
    ld      {tmp}, 0*8({stack})
    ld      {tmp}, 2*8({stack})
    ld      {tmp}, 3*8({stack})
    ld      {tmp}, 4*8({stack})
    ld      {tmp}, 5*8({stack})
    ld      {tmp}, 6*8({stack})
    ld      {tmp}, 7*8({stack})
    ld      {tmp}, 8*8({stack})
    ld      {tmp}, 9*8({stack})
    ld      {tmp}, 10*8({stack})
    ld      {tmp}, 11*8({stack})
    ld      {tmp}, 12*8({stack})
    ld      {tmp}, 13*8({stack})
    ld      {tmp}, 14*8({stack})
    ld      {tmp}, 15*8({stack})
    ld      {tmp}, 16*8({stack})
    ld      {tmp}, 17*8({stack})
    ld      {tmp}, 18*8({stack})
    ld      {tmp}, 19*8({stack})
    ld      {tmp}, 20*8({stack})
    ld      {tmp}, 21*8({stack})
    ld      {tmp}, 22*8({stack})
    ld      {tmp}, 23*8({stack})
    ld      {tmp}, 24*8({stack})
    ld      {tmp}, 25*8({stack})
    ld      {tmp}, 26*8({stack})
    ld      {tmp}, 27*8({stack})
    ld      {tmp}, 28*8({stack})
    ld      {tmp}, 29*8({stack})
    ld      {tmp}, 30*8({stack})",
    "addi   {tmp}, {tmp}, 1",
    "ld     {tmp}, 1*8({stack})", // 加载用户栈
    stack = in(reg) stack.as_mut_ptr(),
    tmp = inlateout(reg) tmp
    );
    core::hint::black_box(tmp);
}

#[inline(always)]
unsafe fn restore_by_thread() {
    let mut stack = [0usize; 35];
    let mut tmp = 0_usize;
    asm!(
        "addi   {tmp}, {tmp}, 1",
        "sd     ra, 0*8({stack})
        sd      gp, 2*8({stack})
        sd      tp, 3*8({stack})
        sd      t0, 4*8({stack})
        sd      t1, 5*8({stack})
        sd      t2, 6*8({stack})
        sd      s0, 7*8({stack})
        sd      s1, 8*8({stack})
        sd      a0, 9*8({stack})
        sd      a1, 10*8({stack})
        sd      a2, 11*8({stack})
        sd      a3, 12*8({stack})
        sd      a4, 13*8({stack})
        sd      a5, 14*8({stack})
        sd      a6, 15*8({stack})
        sd      a7, 16*8({stack})
        sd      s2, 17*8({stack})
        sd      s3, 18*8({stack})
        sd      s4, 19*8({stack})
        sd      s5, 20*8({stack})
        sd      s6, 21*8({stack})
        sd      s7, 22*8({stack})
        sd      s8, 23*8({stack})
        sd      s9, 24*8({stack})
        sd     s10, 25*8({stack})
        sd     s11, 26*8({stack})
        sd      t3, 27*8({stack})
        sd      t4, 28*8({stack})
        sd      t5, 29*8({stack})
        sd      t6, 30*8({stack})",
        "addi   {tmp}, {tmp}, 1
        sd      t0, 31*8({stack})",
        "addi   {tmp}, {tmp}, 1
        sd      t1, 32*8({stack})",
        "addi   {tmp}, {tmp}, 1",
        "addi   {tmp}, {tmp}, 1", // ret指令
        stack = in(reg) stack.as_mut_ptr(),
        tmp = inlateout(reg) tmp
    );
    core::hint::black_box(tmp);
}

#[inline(always)]
unsafe fn switch_by_task() {
    let mut stack = [0usize; 16];
    let mut tmp = 0_usize;
    asm!(
    "sd     ra, 0*8({stack})
    sd      gp, 1*8({stack})
    sd      tp, 2*8({stack})
    sd      s0, 3*8({stack})
    sd      s1, 4*8({stack})
    sd      s2, 5*8({stack})
    sd      s3, 6*8({stack})
    sd      s4, 7*8({stack})
    sd      s5, 8*8({stack})
    sd      s6, 9*8({stack})
    sd      s7, 10*8({stack})
    sd      s8, 11*8({stack})
    sd      s9, 12*8({stack})
    sd      s10, 13*8({stack})
    sd      s11, 14*8({stack})",
    "addi   {tmp}, {tmp}, 1", // 一次读写csr的开销
    stack = in(reg) stack.as_mut_ptr(),
    tmp = inlateout(reg) tmp
    );
    core::hint::black_box(tmp);
}

#[inline(always)]
unsafe fn restore_by_task() {
    let mut stack = [0usize; 16];
    let mut tmp = 0_usize;
    asm!(
    "ld     {tmp}, 0*8({stack})
    ld      {tmp}, 1*8({stack})
    ld      {tmp}, 2*8({stack})
    ld      {tmp}, 3*8({stack})
    ld      {tmp}, 4*8({stack})
    ld      {tmp}, 5*8({stack})
    ld      {tmp}, 6*8({stack})
    ld      {tmp}, 7*8({stack})
    ld      {tmp}, 8*8({stack})
    ld      {tmp}, 9*8({stack})
    ld      {tmp}, 10*8({stack})
    ld      {tmp}, 11*8({stack})
    ld      {tmp}, 12*8({stack})
    ld      {tmp}, 13*8({stack})
    ld      {tmp}, 14*8({stack})
    addi    {tmp}, {tmp}, 15*8",
    "addi   {tmp}, {tmp}, 1", // 一次读写csr的开销
    stack = in(reg) stack.as_mut_ptr(),
    tmp = inlateout(reg) tmp
    );
    core::hint::black_box(tmp);
}
