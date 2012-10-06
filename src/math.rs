use cmp::Ord;

//
//  Abs
//
trait Abs {
    pure fn abs() -> self;
}

impl int: Abs {
    #[inline]
    pure fn abs() -> int {
        if self >= 0 { self }
        else         {-self }
    }
}

impl float: Abs {
    #[inline]
    pure fn abs() -> float {
        if self >= 0f { self }
        else          {-self }
    }
}

impl f32: Abs {
    #[inline]
    pure fn abs() -> f32 {
        if self >= 0f32 { self }
        else            {-self }
    }
}

impl f64: Abs {
    #[inline]
    pure fn abs() -> f64 {
        if self >= 0f64 { self }
        else            {-self }
    }
}

//
//  Min
//
#[inline]
pure fn min<T:Copy Ord>(a: &T, b: &T) -> T {
    if a < b { *a }
    else     { *b }
}

//
//  Max
//
#[inline]
pure fn max<T:Copy Ord>(a: &T, b: &T) -> T {
    if a > b { *a }
    else     { *b }
}

//
//  Sqrt
//
trait Sqrt {
    pure fn sqrt() -> self;
}

// impl int: Sqrt {
//     #[inline]
//     pure fn sqrt() -> int {
//         sqrt(self)
//     }
// }

impl float: Sqrt {
    #[inline]
    pure fn sqrt() -> float {
        float::sqrt(self)
    }
}

impl f32: Sqrt {
    #[inline]
    pure fn sqrt() -> f32 {
        f32::sqrt(self)
    }
}

impl f64: Sqrt {
    #[inline]
    pure fn sqrt() -> f64 {
        f64::sqrt(self)
    }
}