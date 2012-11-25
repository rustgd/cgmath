use vec::{Vec2, Vec3, Vec4};

/// Should only be implemented on signed types.
pub trait Sign {
    pure fn abs() -> self;
    
    /// returns `-1` if the number is negative, `0` if the number is equal to 0,
    /// and `1` if the number is positive
    pure fn sign() -> self;
}

#[inline(always)] pub pure fn abs<T:Sign>(x: &T)  -> T { x.abs()  }
#[inline(always)] pub pure fn sign<T:Sign>(x: &T) -> T { x.sign() }

pub impl i8: Sign {
    #[inline(always)] pure fn abs()  -> i8 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i8 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl i16: Sign {
    #[inline(always)] pure fn abs()  -> i16 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i16 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl i32: Sign {
    #[inline(always)] pure fn abs()  -> i32 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i32 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl i64: Sign {
    #[inline(always)] pure fn abs()  -> i64 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i64 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl int: Sign {
    #[inline(always)] pure fn abs()  -> int { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> int { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl f32: Sign {
    #[inline(always)] pure fn abs()  -> f32 { if self >= 0f32 { self } else {-self } }
    #[inline(always)] pure fn sign() -> f32 { if self > 0f32 { 1f32 } else if self == 0f32 { 0f32 } else { -1f32 } }
}

pub impl f64: Sign {
    #[inline(always)] pure fn abs()  -> f64 { if self >= 0f64 { self } else {-self } }
    #[inline(always)] pure fn sign() -> f64 { if self > 0f64 { 1f64 } else if self == 0f64 { 0f64 } else { -1f64 } }
}

pub impl float: Sign {
    #[inline(always)] pure fn abs()  -> float { if self >= 0f { self } else {-self } }
    #[inline(always)] pure fn sign() -> float { if self > 0f { 1f } else if self == 0f { 0f } else { -1f } }
}



pub impl<T:Copy Sign> Vec2<T>: Sign {
    #[inline(always)]
    pure fn abs() -> Vec2<T> {
        Vec2::new(abs(&self[0]),
                  abs(&self[1]))
    }
    
    #[inline(always)]
    pure fn sign() -> Vec2<T> {
        Vec2::new(sign(&self[0]),
                  sign(&self[1]))
    }
}

pub impl<T:Copy Sign> Vec3<T>: Sign {
    #[inline(always)]
    pure fn abs() -> Vec3<T> {
        Vec3::new(abs(&self[0]),
                  abs(&self[1]),
                  abs(&self[2]))
    }
    
    #[inline(always)]
    pure fn sign() -> Vec3<T> {
        Vec3::new(sign(&self[0]),
                  sign(&self[1]),
                  sign(&self[2]))
    }
}

pub impl<T:Copy Sign> Vec4<T>: Sign {
    #[inline(always)]
    pure fn abs() -> Vec4<T> {
        Vec4::new(abs(&self[0]),
                  abs(&self[1]),
                  abs(&self[2]),
                  abs(&self[3]))
    }
    
    #[inline(always)]
    pure fn sign() -> Vec4<T> {
        Vec4::new(sign(&self[0]),
                  sign(&self[1]),
                  sign(&self[2]),
                  sign(&self[3]))
    }
}