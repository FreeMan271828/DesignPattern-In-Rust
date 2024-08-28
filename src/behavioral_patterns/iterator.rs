pub trait Aggregate<T> {
    fn new() -> Self;
    fn add(&mut self, data: T);
    fn del(&mut self, index: usize) -> Option<&mut Self>;
    fn count(&self) -> usize;
}

pub trait Iterator<T> {
    fn reset(&mut self);
    fn print(&mut self) -> Option<&T>;
    fn next(&mut self);
    fn has_next(&self) -> bool;
}

pub struct List<T> {
    array: Vec<T>,
    index: usize,
}

impl<T> Aggregate<T> for List<T> {
     fn new() -> Self {
        List {
            array: Vec::new(),
            index: 0,
        }
    }

    fn add(&mut self, data: T) {
        self.array.push(data);
    }

    fn del(&mut self, index: usize) -> Option<&mut Self> {
        if index < self.count() {
            self.array.remove(index);
            Some(self)
        } else {
            None
        }
    }

    fn count(&self) -> usize {
        self.array.len()
    }
}

impl<T> Iterator<T> for List<T> {
    fn reset(&mut self) {
        self.index = 0;
    }
    fn print(&mut self) -> Option<&T> {
        if self.index>=self.count() {
            None
        }
        else {
            self.array.get(self.index)
        }
    }

    fn next(&mut self) {
        if self.has_next() {
            self.index += 1;
        }
    }

    fn has_next(&self) -> bool {
        self.index < self.count()
    }
}