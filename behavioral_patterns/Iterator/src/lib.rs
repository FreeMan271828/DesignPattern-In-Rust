pub mod iterator {
    pub trait Iterator{
        type Item;
        fn first(&mut self);
        fn next(&mut self);
        fn is_done(&self) ->bool;
        fn current_item(&self) -> Option<&Self::Item>;
    }

    /// 工厂模式, 是聚合对象的抽象
    pub trait AbstractList{
        type Item;
        fn new()->Self;
    }
    /// 具体的聚合对象, List是基于Vec的聚合对象
    pub trait List: AbstractList{
        fn new()->Self;
        fn add(&mut self, data:Self::Item);
        fn count(&self)->usize;
        fn get(&self,index:usize)->Option<&Self::Item>;
    }

    pub struct ListIterator<T>{
        _current:usize,
        pub _list:Vec<T>,
    }
    impl<T> AbstractList for ListIterator<T> {
        type Item = T;
        fn new() -> Self {
            ListIterator{
                _current:0,
                _list: Vec::new()
            }
        }
    }
    impl<T> List for ListIterator<T>{
        fn new() -> Self {
            ListIterator{
                _current: 0,
                _list: vec![],
            }
        }
        fn add(&mut self, data: Self::Item) {
            self._list.push(data);
        }
        fn count(&self) -> usize {
            self._list.len()
        }
        fn get(&self,index: usize) -> Option<&Self::Item> {
            if index >= self.count(){
                panic!("Index-Out-Of-Bound")
            }
            self._list.get(index)
        }
    }
    impl<T> Iterator for ListIterator<T>{
        type Item = T;
        fn first(&mut self) {
            self._current=0;
        }
        fn next(&mut self) {
            self._current+=1;
        }
        fn is_done(&self) -> bool {
            self._current >= self._list.len()
        }
        fn current_item(&self) -> Option<&Self::Item> {
            if self.is_done(){
                panic!("iterator-Out-Of-Bound");
            }
            self._list.get(self._current)
        }
    }
}