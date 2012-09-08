import cmp::Ord;

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

#[inline(always)]
pure fn min<T:copy Ord>(&&a:T, &&b:T) -> T {
    if a < b { a }
    else     { b }
}

#[inline(always)]
pure fn max<T:copy Ord>(&&a:T, &&b:T) -> T {
    if a > b { a }
    else     { b }
}