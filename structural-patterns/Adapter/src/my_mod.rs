pub mod nb_logger {

    use std::fmt;

    // 定义一个牛逼的 Logger trait
    pub trait NbLogger {
        fn d(&self, priority: i32, message: &str, obj: &[&dyn fmt::Display]);
    }
}