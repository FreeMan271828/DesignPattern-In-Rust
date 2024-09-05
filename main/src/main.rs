use Observer;
use Observer::{ObserverTrait, SubjectTrait};

fn main(){
    let mut subject = Observer::Subject::new();
    subject.attach(Observer1::new());
    subject.attach(Observer2::new());
    subject.notify();
}

struct Observer1 {}
struct Observer2 {}
impl ObserverTrait for Observer1{
    fn new() -> Self{
        Observer1{}
    }
    fn update() {
        println!("Observer1")
    }
}
impl ObserverTrait for Observer2{
    fn new() -> Self{ Observer2{}}
    fn update() {
        println!("Observer2")
    }
}
