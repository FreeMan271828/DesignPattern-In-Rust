pub mod nb_logger{
    // 定义一个关联函数特质，用于创建实例
    pub trait Factory {
        fn new() -> Self;
    }

    // 定义一个日志记录器特质
    pub trait NbLogger {
        fn d(&self, tag: &str, message: &str);
    }

    // 实现日志记录器的类型
    pub struct NbLoggerImp {}

    // 实现日志记录器特质
    impl NbLogger for NbLoggerImp {
        fn d(&self, tag: &str, message: &str) {
            println!("{}: message is {}", tag, message);
        }
    }

    // 实现工厂特质，用于创建实例
    impl Factory for NbLoggerImp {
        fn new() -> Self {
            NbLoggerImp {}
        }
    }
}