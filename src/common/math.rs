use cmp::Ord;
use num::*;

// TODO: move to a more appropriate module
pub trait ToPtr<T> {
    pure fn to_ptr() -> *T;
}

pub trait ExactEq {
    pure fn exact_eq(other: &self) -> bool;
}