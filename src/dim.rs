use core::cmp::Eq;

pub trait Dimensional<T>: Index<uint, T> {
    static pure fn dim() -> uint;
    pure fn to_ptr() -> *T;
}