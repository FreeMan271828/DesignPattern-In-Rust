use crate::behavioral_patterns::iterator::{Iterator, List, ListIterator};

mod behavioral_patterns;

fn main() {
    let mut v: ListIterator<i32> = behavioral_patterns::iterator::AbstractList::new();
    v.add(1); v.add(2);
    v.add(3); v.add(4);
    print_list(v);
}
fn print_list<T: std::fmt::Display>(mut list: ListIterator<T>){
    let mut index = 0;
    list.first();
    while !list.is_done() {
        println!("Index:{}, Data:{}", index, list.current_item().unwrap());
        index+=1;
        list.next();
    }
}