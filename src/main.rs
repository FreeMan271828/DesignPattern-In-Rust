use crate::behavioral_patterns::iterator::{Aggregate, Iterator, List};

mod behavioral_patterns;

fn print<T>(list: &mut List<T>)
where T:std::fmt::Display{
    while list.has_next() {
        println!("data: {}", list.print().unwrap());
        list.next();
    }
    list.reset()
}

fn main() {
    let mut vec = List::new();
    vec.add(1);
    vec.add(2);
    println!("{}", vec.count());
    vec.add(3);
    vec.del(0);
    print(&mut vec);
}
