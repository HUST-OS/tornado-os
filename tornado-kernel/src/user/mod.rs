//! 用户态程序相关
mod load;
mod space;
mod trap;

pub use trap::{enter_user, prepare_user};

pub const USER_APPS: [&'static str; 5] = ["user_task.bin", "alloc-test.bin", "yield-task0.bin", "yield-task1.bin", "database.bin"];
