#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"
            : "volatile");
    }
    ret
}
#[allow(unused)]
const SBI_SET_TIMER: usize = 0;
#[allow(unused)]
const SBI_CONSOLE_PUTCHAR: usize = 1;
#[allow(unused)]
const SBI_CONSOLE_GETCHAR: usize = 2;
#[allow(unused)]
const SBI_CLEAR_IPI: usize = 3;
#[allow(unused)]
const SBI_SEND_IPI: usize = 4;
#[allow(unused)]
const SBI_REMOTE_FENCE_I: usize = 5;
#[allow(unused)]
const SBI_REMOTE_SFENCE_VMA: usize = 6;
#[allow(unused)]
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
#[allow(unused)]
const SBI_SHUTDOWN: usize = 8;

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

#[allow(unused)]
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

#[allow(unused)]
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    unreachable!()
}

#[allow(unused)]
pub fn set_timer(time: usize) {
    sbi_call(SBI_SET_TIMER, time, 0, 0);
}
