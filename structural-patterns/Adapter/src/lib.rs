pub mod my_mod;

use std::fmt;
use crate::my_mod::nb_logger::NbLogger;

// 定义一个日志工厂的 trait
pub trait LogFactory {
    fn debug(&self, tag: &str, message: &str);
}

// 实现牛逼的 Logger
pub struct NbLoggerImp;
impl NbLogger for NbLoggerImp {
    fn d(&self, _priority: i32, message: &str, _obj: &[&dyn fmt::Display]) {
        println!("牛逼logger记录:{}", message);
    }
}

/// 日志适配器，用于适配不同的日志系统
pub struct LogAdapter {
    // 要求实现了NbLogger的结构
    // 将NbLogger适配LogFactory
    nb_logger: Box<dyn NbLogger>,
}

impl LogAdapter {
    // 创建一个新的 LogAdapter 实例
    pub fn new(nb_logger: Box<dyn NbLogger>) -> Self {
        LogAdapter { nb_logger }
    }
}

impl LogFactory for LogAdapter {
    fn debug(&self, _tag: &str, message: &str) {
        self.nb_logger.d(1, message, &[]);
    }
}
