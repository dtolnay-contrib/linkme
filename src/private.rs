pub trait Slice {
    type Element;
}

impl<T> Slice for [T] {
    type Element = T;
}

pub enum Void {}

pub fn value<T>() -> T {
    panic!()
}
