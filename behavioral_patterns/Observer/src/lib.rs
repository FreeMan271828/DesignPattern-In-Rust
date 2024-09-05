/// 定义Subject的结构体, 因为Subject采用Vec存储观察者, 抽象等次-1
pub struct Subject{
    _observer: Vec<Box<dyn ObserverTrait>>
}
impl SubjectTrait for Subject{
    fn new() -> Self{
        Subject{ _observer: Vec::new()}
    }
    fn attach(&mut self, observer: &dyn ObserverTrait) {
        self._observer.push(*observer.clone()); // 注意这里需要克隆observer
    }

    fn detach(&mut self, observer: &Box<dyn ObserverTrait>) {
        self._observer.retain(|obs| !obs.equals(observer));
    }

    fn notify(&self) {
        self._observer.iter().for_each(|obs|{
            obs.update();
        })
    }
}

/// 观察者特征
pub trait ObserverTrait {
    fn update(&self);
}

/// 目标特征
pub trait SubjectTrait {
    fn new()->Self;
    fn attach(&mut self, observer: &dyn ObserverTrait);
    fn detach(&mut self, observer: &Box<dyn ObserverTrait>);
    fn notify(&self);
}
