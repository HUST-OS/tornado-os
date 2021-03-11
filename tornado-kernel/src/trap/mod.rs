mod handler;
mod timer;

pub use handler::TrapFrame;
pub use timer::set_next_timeout;

/// 初始化中断相关的子模块
/// 
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}
