use Adapter;
use Adapter::OldLogger;
use Adapter::my_mod::nb_logger::{Factory, NbLogger, NbLoggerImp};

fn main(){
    let nb_logger: Box<dyn NbLogger> = Box::new(NbLoggerImp{});
    let warn_adapter = Adapter::LogAdapter::new(Box::new(NbLoggerImp::new()), "warn".to_string());
    warn_adapter.debug("有悬空指针");
}