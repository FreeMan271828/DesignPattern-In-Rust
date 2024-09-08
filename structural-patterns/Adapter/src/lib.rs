pub mod my_mod;
use crate::my_mod::nb_logger::NbLogger;

// 定义一个日志工厂的 trait
pub trait OldLogger {
    fn debug(&self, message: &str);
}

/// 日志适配器，用于适配不同的日志系统
pub struct LogAdapter {
    // 要求实现了NbLogger的结构
    // 将NbLogger适配LogFactory
    nb_logger: Box<dyn NbLogger>,
    tag: String,
}

impl LogAdapter {
    // 创建一个新的 LogAdapter 实例
    pub fn new(nb_logger: Box<dyn NbLogger>, tag: String) -> Self {
        LogAdapter { nb_logger, tag }
    }
}

/// 使用旧的方法(debug)实现新接口(d)的功能
impl OldLogger for LogAdapter{
    fn debug(&self, message: &str) {
        self.nb_logger.d( &*self.tag, message);
    }
}