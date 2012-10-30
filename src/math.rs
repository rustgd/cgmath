use cmp::Ord;
use num::{Num, from_int};

// TODO: move to a more appropriate module
pub trait ToPtr<T> {
    pure fn to_ptr() -> *T;
}

pub trait ExactEq {
    pure fn exact_eq(other: &self) -> bool;
}

//
//  Min
//
#[inline(always)]
pure fn min<T:Copy Ord>(a: &T, b: &T) -> T {
    if a < b { *a }
    else     { *b }
}

//
//  Max
//
#[inline(always)]
pure fn max<T:Copy Ord>(a: &T, b: &T) -> T {
    if a > b { *a }
    else     { *b }
}

// #[inline(always)]
// pure fn abs<T:Copy Num Ord>(x: &T) -> T {
//     if x >= &from_int(0) { *x } else {-x }
// }

//
//  Abs
//
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

impl float: Abs {
    #[inline(always)]
    pure fn abs() -> float {
        if self >= 0f { self }
        else          {-self }
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

//
//  Sqrt
//
trait Sqrt {
    pure fn sqrt() -> self;
}

#[inline(always)]
pure fn sqrt<T: Sqrt>(x: T) -> T {
    x.sqrt()
}

impl u8: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> u8 {
        (self as float).sqrt() as u8
    }
}

impl u16: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> u16 {
        (self as float).sqrt() as u16
    }
}

impl u32: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> u32 {
        (self as float).sqrt() as u32
    }
}

impl u64: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> u64 {
        (self as float).sqrt() as u64
    }
}

impl uint: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> uint {
        (self as float).sqrt() as uint
    }
}

impl i8: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> i8 {
        (self as float).sqrt() as i8
    }
}

impl i16: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> i16 {
        (self as float).sqrt() as i16
    }
}

impl i32: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> i32 {
        (self as float).sqrt() as i32
    }
}

impl i64: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> i64 {
        (self as float).sqrt() as i64
    }
}

impl int: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> int {
        (self as float).sqrt() as int
    }
}

impl float: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> float {
        float::sqrt(self)
    }
}

impl f32: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> f32 {
        f32::sqrt(self)
    }
}

impl f64: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> f64 {
        f64::sqrt(self)
    }
}