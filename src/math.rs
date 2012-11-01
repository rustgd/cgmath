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

impl float: MinMax {
    pure fn min(other: &float) -> float {
        if self < *other { self   }
        else             { *other }
    }
    
    pure fn max(other: &float) -> float {
        if self > *other { self   }
        else             { *other }
    }
    
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

pub impl<T: NumCast> T: Sqrt {
    #[inline(always)]
    pure fn sqrt() -> T {
        f64::sqrt(self.cast()).cast()
    }
}