use core::cmp::Eq;

pub trait Dimensional<T>: Index<uint, T> {
    static pure fn dim() -> uint;       // dim and size_of are pretty useless at the moment due to
    static pure fn size_of() -> uint;   // how Rust's static methods are currently implemented
    pure fn to_ptr() -> *T;
}