//! 用户态程序相关
mod load;
mod space;
mod trap;

pub use trap::first_enter_user;

pub const USER_APPS: [&'static str; 2] = ["user_task.bin", "alloc-test.bin"];
