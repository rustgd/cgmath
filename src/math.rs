import cmp::Ord;

//
//  Abs
//
trait Abs {
    pure fn abs() -> self;
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
//  Min
//
#[inline(always)]
pure fn min<T:Copy Ord>(&&a:T, &&b:T) -> T {
    if a < b { a }
    else     { b }
}

//
//  Max
//
#[inline(always)]
pure fn max<T:Copy Ord>(&&a:T, &&b:T) -> T {
    if a > b { a }
    else     { b }
}

//
//  Sqrt
//
trait Sqrt {
    pure fn sqrt() -> self;
}

// impl int: Sqrt {
//     #[inline(always)]
//     pure fn sqrt() -> int {
//         sqrt(self)
//     }
// }

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