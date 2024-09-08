use Adapter;
use Adapter::LogFactory;

fn main(){
    let box_nb_logger = Box::new(Adapter::NbLoggerImp);
    let log = Adapter::LogAdapter::new(box_nb_logger);
    log.debug("error","内存溢出");
}