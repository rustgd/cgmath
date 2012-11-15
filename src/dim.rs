use core::cmp::Eq;

pub trait Dimensional<T>: Eq, Index<uint, T> {
    static pure fn dim() -> uint;
    pure fn to_ptr() -> *T;
}