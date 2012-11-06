use ncast::*;
use vector::*;

/**
 * Angle and Trigonometry Functions
 */
pub trait Trig {
    pure fn radians() -> self;
    pure fn degrees() -> self;
    pure fn sin() -> self;
    pure fn cos() -> self;
    pure fn tan() -> self;
    pure fn asin() -> self;
    pure fn acos() -> self;
    pure fn atan() -> self;
    pure fn sinh() -> self;
    pure fn cosh() -> self;
    pure fn tanh() -> self;
}

#[inline(always)] pub pure fn radians<T:Trig>(degrees: &T) -> T { degrees.radians() }
#[inline(always)] pub pure fn degrees<T:Trig>(radians: &T) -> T { radians.degrees() }
#[inline(always)] pub pure fn sin<T:Trig>(angle: &T) -> T { angle.sin() }
#[inline(always)] pub pure fn cos<T:Trig>(angle: &T) -> T { angle.cos() }
#[inline(always)] pub pure fn tan<T:Trig>(angle: &T) -> T { angle.tan() }
#[inline(always)] pub pure fn asin<T:Trig>(x: &T) -> T { x.asin() }
#[inline(always)] pub pure fn acos<T:Trig>(x: &T) -> T { x.acos() }
#[inline(always)] pub pure fn atan<T:Trig>(x: &T) -> T { x.atan() }
#[inline(always)] pub pure fn sinh<T:Trig>(x: &T) -> T { x.sinh() }
#[inline(always)] pub pure fn cosh<T:Trig>(x: &T) -> T { x.cosh() }
#[inline(always)] pub pure fn tanh<T:Trig>(x: &T) -> T { x.tanh() }

pub impl f32: Trig {
    #[inline(always)] pure fn radians() -> f32 { self * (f32::consts::pi / 180f32) }
    #[inline(always)] pure fn degrees() -> f32 { self * (180f32 / f32::consts::pi) }
    #[inline(always)] pure fn sin() -> f32 { f32::sin(self) }
    #[inline(always)] pure fn cos() -> f32 { f32::cos(self) }
    #[inline(always)] pure fn tan() -> f32 { f32::tan(self) }
    #[inline(always)] pure fn asin() -> f32 { f32::asin(self) }
    #[inline(always)] pure fn acos() -> f32 { f32::acos(self) }
    #[inline(always)] pure fn atan() -> f32 { f32::atan(self) }
    #[inline(always)] pure fn sinh() -> f32 { f32::sinh(self) }
    #[inline(always)] pure fn cosh() -> f32 { f32::cosh(self) }
    #[inline(always)] pure fn tanh() -> f32 { f32::tanh(self) }
}

pub impl f64: Trig {
    #[inline(always)] pure fn radians() -> f64 { self * (f64::consts::pi / 180f64) }
    #[inline(always)] pure fn degrees() -> f64 { self * (180f64 / f64::consts::pi) }
    #[inline(always)] pure fn sin() -> f64 { f64::sin(self) }
    #[inline(always)] pure fn cos() -> f64 { f64::cos(self) }
    #[inline(always)] pure fn tan() -> f64 { f64::tan(self) }
    #[inline(always)] pure fn asin() -> f64 { f64::asin(self) }
    #[inline(always)] pure fn acos() -> f64 { f64::acos(self) }
    #[inline(always)] pure fn atan() -> f64 { f64::atan(self) }
    #[inline(always)] pure fn sinh() -> f64 { f64::sinh(self) }
    #[inline(always)] pure fn cosh() -> f64 { f64::cosh(self) }
    #[inline(always)] pure fn tanh() -> f64 { f64::tanh(self) }
}

pub impl float: Trig {
    #[inline(always)] pure fn radians() -> float { self * (float::consts::pi / 180f) }
    #[inline(always)] pure fn degrees() -> float { self * (180f / float::consts::pi) }
    #[inline(always)] pure fn sin() -> float { cast(float::sin(cast(self))) }
    #[inline(always)] pure fn cos() -> float { cast(float::cos(cast(self))) }
    #[inline(always)] pure fn tan() -> float { cast(float::tan(cast(self))) }
    #[inline(always)] pure fn asin() -> float { cast(float::asin(cast(self))) }
    #[inline(always)] pure fn acos() -> float { cast(float::acos(cast(self))) }
    #[inline(always)] pure fn atan() -> float { cast(float::atan(cast(self))) }
    #[inline(always)] pure fn sinh() -> float { cast(float::sinh(cast(self))) }
    #[inline(always)] pure fn cosh() -> float { cast(float::cosh(cast(self))) }
    #[inline(always)] pure fn tanh() -> float { cast(float::tanh(cast(self))) }
}

pub impl<T:Copy Trig> Vec2<T>: Trig {
    #[inline(always)] pure fn radians() -> Vec2<T> {
        Vec2::new(radians(&self[0]),
                  radians(&self[1]))
    }
    
    #[inline(always)] pure fn degrees() -> Vec2<T> {
        Vec2::new(degrees(&self[0]),
                  degrees(&self[1]))
    }
    
    #[inline(always)] pure fn sin() -> Vec2<T> {
        Vec2::new(sin(&self[0]),
                  sin(&self[1]))
    }
    
    #[inline(always)] pure fn cos() -> Vec2<T> {
        Vec2::new(cos(&self[0]),
                  cos(&self[1]))
    }
    
    #[inline(always)] pure fn tan() -> Vec2<T> {
        Vec2::new(tan(&self[0]),
                  tan(&self[1]))
    }
    
    #[inline(always)] pure fn asin() -> Vec2<T> {
        Vec2::new(asin(&self[0]),
                  asin(&self[1]))
    }
    
    #[inline(always)] pure fn acos() -> Vec2<T> {
        Vec2::new(acos(&self[0]),
                  acos(&self[1]))
    }
    
    #[inline(always)] pure fn atan() -> Vec2<T> {
        Vec2::new(atan(&self[0]),
                  atan(&self[1]))
    }
    
    #[inline(always)] pure fn sinh() -> Vec2<T> {
        Vec2::new(sinh(&self[0]),
                  sinh(&self[1]))
    }
    
    #[inline(always)] pure fn cosh() -> Vec2<T> {
        Vec2::new(cosh(&self[0]),
                  cosh(&self[1]))
    }
    
    #[inline(always)] pure fn tanh() -> Vec2<T> {
        Vec2::new(tanh(&self[0]),
                  tanh(&self[1]))
    }
}

pub impl<T:Copy Trig> Vec3<T>: Trig {
    #[inline(always)] pure fn radians() -> Vec3<T> {
        Vec3::new(radians(&self[0]),
                  radians(&self[1]),
                  radians(&self[2]))
    }
    
    #[inline(always)] pure fn degrees() -> Vec3<T> {
        Vec3::new(degrees(&self[0]),
                  degrees(&self[1]),
                  degrees(&self[2]))
    }
    
    #[inline(always)] pure fn sin() -> Vec3<T> {
        Vec3::new(sin(&self[0]),
                  sin(&self[1]),
                  sin(&self[2]))
    }
    
    #[inline(always)] pure fn cos() -> Vec3<T> {
        Vec3::new(cos(&self[0]),
                  cos(&self[1]),
                  cos(&self[2]))
    }
    
    #[inline(always)] pure fn tan() -> Vec3<T> {
        Vec3::new(tan(&self[0]),
                  tan(&self[1]),
                  tan(&self[2]))
    }
    
    #[inline(always)] pure fn asin() -> Vec3<T> {
        Vec3::new(asin(&self[0]),
                  asin(&self[1]),
                  asin(&self[2]))
    }
    
    #[inline(always)] pure fn acos() -> Vec3<T> {
        Vec3::new(acos(&self[0]),
                  acos(&self[1]),
                  acos(&self[2]))
    }
    
    #[inline(always)] pure fn atan() -> Vec3<T> {
        Vec3::new(atan(&self[0]),
                  atan(&self[1]),
                  atan(&self[2]))
    }
    
    #[inline(always)] pure fn sinh() -> Vec3<T> {
        Vec3::new(sinh(&self[0]),
                  sinh(&self[1]),
                  sinh(&self[2]))
    }
    
    #[inline(always)] pure fn cosh() -> Vec3<T> {
        Vec3::new(cosh(&self[0]),
                  cosh(&self[1]),
                  cosh(&self[2]))
    }
    
    #[inline(always)] pure fn tanh() -> Vec3<T> {
        Vec3::new(tanh(&self[0]),
                  tanh(&self[1]),
                  tanh(&self[2]))
    }
}

pub impl<T:Copy Trig> Vec4<T>: Trig {
    #[inline(always)] pure fn radians() -> Vec4<T> {
        Vec4::new(radians(&self[0]),
                  radians(&self[1]),
                  radians(&self[2]),
                  radians(&self[3]))
    }
    
    #[inline(always)] pure fn degrees() -> Vec4<T> {
        Vec4::new(degrees(&self[0]),
                  degrees(&self[1]),
                  degrees(&self[2]),
                  degrees(&self[3]))
    }
    
    #[inline(always)] pure fn sin() -> Vec4<T> {
        Vec4::new(sin(&self[0]),
                  sin(&self[1]),
                  sin(&self[2]),
                  sin(&self[3]))
    }
    
    #[inline(always)] pure fn cos() -> Vec4<T> {
        Vec4::new(cos(&self[0]),
                  cos(&self[1]),
                  cos(&self[2]),
                  cos(&self[3]))
    }
    
    #[inline(always)] pure fn tan() -> Vec4<T> {
        Vec4::new(tan(&self[0]),
                  tan(&self[1]),
                  tan(&self[2]),
                  tan(&self[3]))
    }
    
    #[inline(always)] pure fn asin() -> Vec4<T> {
        Vec4::new(asin(&self[0]),
                  asin(&self[1]),
                  asin(&self[2]),
                  asin(&self[3]))
    }
    
    #[inline(always)] pure fn acos() -> Vec4<T> {
        Vec4::new(acos(&self[0]),
                  acos(&self[1]),
                  acos(&self[2]),
                  acos(&self[3]))
    }
    
    #[inline(always)] pure fn atan() -> Vec4<T> {
        Vec4::new(atan(&self[0]),
                  atan(&self[1]),
                  atan(&self[2]),
                  atan(&self[3]))
    }
    
    #[inline(always)] pure fn sinh() -> Vec4<T> {
        Vec4::new(sinh(&self[0]),
                  sinh(&self[1]),
                  sinh(&self[2]),
                  sinh(&self[3]))
    }
    
    #[inline(always)] pure fn cosh() -> Vec4<T> {
        Vec4::new(cosh(&self[0]),
                  cosh(&self[1]),
                  cosh(&self[2]),
                  cosh(&self[3]))
    }
    
    #[inline(always)] pure fn tanh() -> Vec4<T> {
        Vec4::new(tanh(&self[0]),
                  tanh(&self[1]),
                  tanh(&self[2]),
                  tanh(&self[3]))
    }
}