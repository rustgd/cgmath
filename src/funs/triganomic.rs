/**
 * Angle and Trigonometry Functions
 *
 * This module corresponds to Section 8.1 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */
use angle::Radians;
use num::cast::cast;
use num::kinds::Number;
use vec::{Vec3, Vec2, Vec4};

/**
 * Triganomic functions
 *
 * http://en.wikipedia.org/wiki/Trigonometric_functions
 */
priv trait Trig<T> {
    pure fn sin(&self) -> T;
    pure fn cos(&self) -> T;
    pure fn tan(&self) -> T;
}

#[inline(always)] pub pure fn sin<T:Trig<R>, R>(theta: &T) -> R { theta.sin() }
#[inline(always)] pub pure fn cos<T:Trig<R>, R>(theta: &T) -> R { theta.cos() }
#[inline(always)] pub pure fn tan<T:Trig<R>, R>(theta: &T) -> R { theta.tan() }

priv impl<T:Copy Number> Radians<T>: Trig<T> {
    #[inline(always)] pure fn sin(&self) -> T { cast(f64::sin(cast(**self))) }
    #[inline(always)] pure fn cos(&self) -> T { cast(f64::cos(cast(**self))) }
    #[inline(always)] pure fn tan(&self) -> T { cast(f64::tan(cast(**self))) }
}

pub impl<T:Copy Number> Vec2<Radians<T>>: Trig<Vec2<T>>  {
    #[inline(always)]
    pure fn sin(&self) -> Vec2<T> {
        Vec2::new(sin(&self[0]),
                  sin(&self[1]))
    }
    
    #[inline(always)]
    pure fn cos(&self) -> Vec2<T> {
        Vec2::new(cos(&self[0]),
                  cos(&self[1]))
    }
    
    #[inline(always)]
    pure fn tan(&self) -> Vec2<T> {
        Vec2::new(tan(&self[0]),
                  tan(&self[1]))
    }
}

pub impl<T:Copy Number> Vec3<Radians<T>>: Trig<Vec3<T>>  {
    #[inline(always)]
    pure fn sin(&self) -> Vec3<T> {
        Vec3::new(sin(&self[0]),
                  sin(&self[1]),
                  sin(&self[2]))
    }
    
    #[inline(always)]
    pure fn cos(&self) -> Vec3<T> {
        Vec3::new(cos(&self[0]),
                  cos(&self[1]),
                  cos(&self[2]))
    }
    
    #[inline(always)]
    pure fn tan(&self) -> Vec3<T> {
        Vec3::new(tan(&self[0]),
                  tan(&self[1]),
                  tan(&self[2]))
    }
}

pub impl<T:Copy Number> Vec4<Radians<T>>: Trig<Vec4<T>>  {
    #[inline(always)]
    pure fn sin(&self) -> Vec4<T> {
        Vec4::new(sin(&self[0]),
                  sin(&self[1]),
                  sin(&self[2]),
                  sin(&self[3]))
    }
    
    #[inline(always)]
    pure fn cos(&self) -> Vec4<T> {
        Vec4::new(cos(&self[0]),
                  cos(&self[1]),
                  cos(&self[2]),
                  cos(&self[3]))
    }
    
    #[inline(always)]
    pure fn tan(&self) -> Vec4<T> {
        Vec4::new(tan(&self[0]),
                  tan(&self[1]),
                  tan(&self[2]),
                  tan(&self[3]))
    }
}

/**
 * Inverse triganomic functions
 *
 * http://en.wikipedia.org/wiki/Inverse_trigonometric_functions
 */
pub trait InvTrig {
    pure fn asin(&self) -> Radians<self>;
    pure fn acos(&self) -> Radians<self>;
    pure fn atan(&self) -> Radians<self>;
}

#[inline(always)] pub pure fn asin<T:InvTrig>(x: &T) -> Radians<T> { x.asin() }
#[inline(always)] pub pure fn acos<T:InvTrig>(x: &T) -> Radians<T> { x.acos() }
#[inline(always)] pub pure fn atan<T:InvTrig>(x: &T) -> Radians<T> { x.atan() }

pub impl f32: InvTrig {
    #[inline(always)] pure fn asin(&self) -> Radians<f32> { Radians(f32::asin(*self)) }
    #[inline(always)] pure fn acos(&self) -> Radians<f32> { Radians(f32::acos(*self)) }
    #[inline(always)] pure fn atan(&self) -> Radians<f32> { Radians(f32::atan(*self)) }
}

pub impl f64: InvTrig {
    #[inline(always)] pure fn asin(&self) -> Radians<f64> { Radians(f64::asin(*self)) }
    #[inline(always)] pure fn acos(&self) -> Radians<f64> { Radians(f64::acos(*self)) }
    #[inline(always)] pure fn atan(&self) -> Radians<f64> { Radians(f64::atan(*self)) }
}

pub impl float: InvTrig {
    #[inline(always)] pure fn asin(&self) -> Radians<float> { Radians(f64::asin(*self as f64) as float) }
    #[inline(always)] pure fn acos(&self) -> Radians<float> { Radians(f64::acos(*self as f64) as float) }
    #[inline(always)] pure fn atan(&self) -> Radians<float> { Radians(f64::atan(*self as f64) as float) }
}

// TODO: figure out how to merge with InvTrig
pub trait InvTrigV<T> {
    pure fn asin(&self) -> T;
    pure fn acos(&self) -> T;
    pure fn atan(&self) -> T;
}

pub impl<T:Copy Number InvTrig> Vec2<T>: InvTrigV<Vec2<Radians<T>>>  {
    #[inline(always)]
    pure fn asin(&self) -> Vec2<Radians<T>> {
        Vec2::new(asin(&self[0]),
                  asin(&self[1]))
    }
    
    #[inline(always)]
    pure fn acos(&self) -> Vec2<Radians<T>> {
        Vec2::new(acos(&self[0]),
                  acos(&self[1]))
    }
    
    #[inline(always)]
    pure fn atan(&self) -> Vec2<Radians<T>> {
        Vec2::new(atan(&self[0]),
                  atan(&self[1]))
    }
}

pub impl<T:Copy Number InvTrig> Vec3<T>: InvTrigV<Vec3<Radians<T>>>  {
    #[inline(always)]
    pure fn asin(&self) -> Vec3<Radians<T>> {
        Vec3::new(asin(&self[0]),
                  asin(&self[1]),
                  asin(&self[2]))
    }
    
    #[inline(always)]
    pure fn acos(&self) -> Vec3<Radians<T>> {
        Vec3::new(acos(&self[0]),
                  acos(&self[1]),
                  acos(&self[2]))
    }
    
    #[inline(always)]
    pure fn atan(&self) -> Vec3<Radians<T>> {
        Vec3::new(atan(&self[0]),
                  atan(&self[1]),
                  atan(&self[2]))
    }
}

pub impl<T:Copy Number InvTrig> Vec4<T>: InvTrigV<Vec4<Radians<T>>>  {
    #[inline(always)]
    pure fn asin(&self) -> Vec4<Radians<T>> {
        Vec4::new(asin(&self[0]),
                  asin(&self[1]),
                  asin(&self[2]),
                  asin(&self[3]))
    }
    
    #[inline(always)]
    pure fn acos(&self) -> Vec4<Radians<T>> {
        Vec4::new(acos(&self[0]),
                  acos(&self[1]),
                  acos(&self[2]),
                  acos(&self[3]))
    }
    
    #[inline(always)]
    pure fn atan(&self) -> Vec4<Radians<T>> {
        Vec4::new(atan(&self[0]),
                  atan(&self[1]),
                  atan(&self[2]),
                  atan(&self[3]))
    }
}



/**
 * Hyperbolic functions
 *
 * http://en.wikipedia.org/wiki/Hyperbolic_function
 */
pub trait Hyp {
    pure fn sinh(&self) -> self;
    pure fn cosh(&self) -> self;
    pure fn tanh(&self) -> self;
    // pure fn asinh() -> self;
    // pure fn acosh() -> self;
    // pure fn atanh() -> self;
}

#[inline(always)] pub pure fn sinh<T:Hyp>(x: &T) -> T { x.sinh() }
#[inline(always)] pub pure fn cosh<T:Hyp>(x: &T) -> T { x.cosh() }
#[inline(always)] pub pure fn tanh<T:Hyp>(x: &T) -> T { x.tanh() }

pub impl f32: Hyp {
    #[inline(always)] pure fn sinh(&self) -> f32 { f32::sinh(*self) }
    #[inline(always)] pure fn cosh(&self) -> f32 { f32::cosh(*self) }
    #[inline(always)] pure fn tanh(&self) -> f32 { f32::tanh(*self) }
}

pub impl f64: Hyp {
    #[inline(always)] pure fn sinh(&self) -> f64 { f64::sinh(*self) }
    #[inline(always)] pure fn cosh(&self) -> f64 { f64::cosh(*self) }
    #[inline(always)] pure fn tanh(&self) -> f64 { f64::tanh(*self) }
}

pub impl float: Hyp {
    #[inline(always)] pure fn sinh(&self) -> float { f64::sinh(*self as f64) as float }
    #[inline(always)] pure fn cosh(&self) -> float { f64::cosh(*self as f64) as float }
    #[inline(always)] pure fn tanh(&self) -> float { f64::tanh(*self as f64) as float }
}

pub impl <T:Copy Hyp> Vec2<T>: Hyp {
    #[inline(always)]
    pure fn sinh(&self) -> Vec2<T> {
        Vec2::new(sinh(&self[0]),
                  sinh(&self[1]))
    }
    
    #[inline(always)]
    pure fn cosh(&self) -> Vec2<T> {
        Vec2::new(cosh(&self[0]),
                  cosh(&self[1]))
    }
    
    #[inline(always)]
    pure fn tanh(&self) -> Vec2<T> {
        Vec2::new(tanh(&self[0]),
                  tanh(&self[1]))
    }
}

pub impl <T:Copy Hyp> Vec3<T>: Hyp {
    #[inline(always)]
    pure fn sinh(&self) -> Vec3<T> {
        Vec3::new(sinh(&self[0]),
                  sinh(&self[1]),
                  sinh(&self[2]))
    }
    
    #[inline(always)]
    pure fn cosh(&self) -> Vec3<T> {
        Vec3::new(cosh(&self[0]),
                  cosh(&self[1]),
                  cosh(&self[2]))
    }
    
    #[inline(always)]
    pure fn tanh(&self) -> Vec3<T> {
        Vec3::new(tanh(&self[0]),
                  tanh(&self[1]),
                  tanh(&self[2]))
    }
}

pub impl <T:Copy Hyp> Vec4<T>: Hyp  {
    #[inline(always)]
    pure fn sinh(&self) -> Vec4<T> {
        Vec4::new(sinh(&self[0]),
                  sinh(&self[1]),
                  sinh(&self[2]),
                  sinh(&self[3]))
    }
    
    #[inline(always)]
    pure fn cosh(&self) -> Vec4<T> {
        Vec4::new(cosh(&self[0]),
                  cosh(&self[1]),
                  cosh(&self[2]),
                  cosh(&self[3]))
    }
    
    #[inline(always)]
    pure fn tanh(&self) -> Vec4<T> {
        Vec4::new(tanh(&self[0]),
                  tanh(&self[1]),
                  tanh(&self[2]),
                  tanh(&self[3]))
    }
}