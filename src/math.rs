use cmp::Ord;
use num::{Num, from_int};

use num_util::*;

// TODO: move to a more appropriate module
pub trait ToPtr<T> {
    pure fn to_ptr() -> *T;
}

pub trait ExactEq {
    pure fn exact_eq(other: &self) -> bool;
}

trait MinMax {
    pure fn min(other: &self) -> self;
    pure fn max(other: &self) -> self;
}

#[inline(always)] pure fn min<T:MinMax>(a: &T, b: &T) -> T { a.min(b) }
#[inline(always)] pure fn max<T:MinMax>(a: &T, b: &T) -> T { a.max(b) }

impl u8: MinMax {
    #[inline(always)] pure fn min(other: &u8) -> u8 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &u8) -> u8 { if self > *other { self } else { *other } }
}

impl u16: MinMax {
    #[inline(always)] pure fn min(other: &u16) -> u16 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &u16) -> u16 { if self > *other { self } else { *other } }
}

impl u32: MinMax {
    #[inline(always)] pure fn min(other: &u32) -> u32 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &u32) -> u32 { if self > *other { self } else { *other } }
}

impl u64: MinMax {
    #[inline(always)] pure fn min(other: &u64) -> u64 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &u64) -> u64 { if self > *other { self } else { *other } }
}

impl uint: MinMax {
    #[inline(always)] pure fn min(other: &uint) -> uint { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &uint) -> uint { if self > *other { self } else { *other } }
}

impl i8: MinMax {
    #[inline(always)] pure fn min(other: &i8) -> i8 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &i8) -> i8 { if self > *other { self } else { *other } }
}

impl i16: MinMax {
    #[inline(always)] pure fn min(other: &i16) -> i16 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &i16) -> i16 { if self > *other { self } else { *other } }
}

impl i32: MinMax {
    #[inline(always)] pure fn min(other: &i32) -> i32 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &i32) -> i32 { if self > *other { self } else { *other } }
}

impl i64: MinMax {
    #[inline(always)] pure fn min(other: &i64) -> i64 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &i64) -> i64 { if self > *other { self } else { *other } }
}

impl int: MinMax {
    #[inline(always)] pure fn min(other: &int) -> int { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &int) -> int { if self > *other { self } else { *other } }
}

impl f32: MinMax {
    #[inline(always)] pure fn min(other: &f32) -> f32 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &f32) -> f32 { if self > *other { self } else { *other } }
}

impl f64: MinMax {
    #[inline(always)] pure fn min(other: &f64) -> f64 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &f64) -> f64 { if self > *other { self } else { *other } }
}

impl float: MinMax {
    #[inline(always)] pure fn min(other: &float) -> float { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &float) -> float { if self > *other { self } else { *other } }
}


trait Abs {
    pure fn abs() -> self;
}

#[inline(always)]
pure fn abs<T: Abs>(x: &T) -> T {
    x.abs()
}

impl i8: Abs {
    #[inline(always)]
    pure fn abs() -> i8 {
        if self >= 0 { self }
        else         {-self }
    }
}

impl i16: Abs {
    #[inline(always)]
    pure fn abs() -> i16 {
        if self >= 0 { self }
        else         {-self }
    }
}

impl i32: Abs {
    #[inline(always)]
    pure fn abs() -> i32 {
        if self >= 0 { self }
        else         {-self }
    }
}

impl i64: Abs {
    #[inline(always)]
    pure fn abs() -> i64 {
        if self >= 0 { self }
        else         {-self }
    }
}

impl int: Abs {
    #[inline(always)]
    pure fn abs() -> int {
        if self >= 0 { self }
        else         {-self }
    }
}

impl f32: Abs {
    #[inline(always)]
    pure fn abs() -> f32 {
        if self >= 0f32 { self }
        else            {-self }
    }
}

impl f64: Abs {
    #[inline(always)]
    pure fn abs() -> f64 {
        if self >= 0f64 { self }
        else            {-self }
    }
}

impl float: Abs {
    #[inline(always)]
    pure fn abs() -> float {
        if self >= 0f { self }
        else          {-self }
    }
}


trait Sqrt {
    pure fn sqrt() -> self;
}

#[inline(always)]
pure fn sqrt<T: Sqrt>(x: T) -> T {
    x.sqrt()
}

pub impl<T: NumCast> T: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> T {
        f64::sqrt(self.cast()).cast()
    }
}