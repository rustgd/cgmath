use ncast::*;
use ntrait::Float;
use vector::{Vec3, Vec2, Vec4};

pub enum Radians<ThetaT: Float> = ThetaT;

pub impl<ThetaT:Num NumCast Float> Radians<ThetaT> {
    #[inline(always)]
    pure fn to_degrees() -> Degrees<ThetaT> {
        Degrees(*self * cast(180f64 / f64::consts::pi))
    }
}

pub enum Degrees<ThetaT: Float> = ThetaT;

pub impl<ThetaT:Num NumCast Float> Degrees<ThetaT> {
    #[inline(always)]
    pure fn to_radians() -> Radians<ThetaT> {
        Radians(*self * cast(f64::consts::pi / 180f64))
    }
}

/**
 * Angle and Trigonometry Functions
 */
pub trait Trig<Result> {
    pure fn sin()  -> Result;
    pure fn cos()  -> Result;
    pure fn tan()  -> Result;
    pure fn asin() -> Result;
    pure fn acos() -> Result;
    pure fn atan() -> Result;
    pure fn sinh() -> Result;
    pure fn cosh() -> Result;
    pure fn tanh() -> Result;
}

pub impl
<
    ThetaT:Copy NumCast Float,
    Result:NumCast
>
Radians<ThetaT>: Trig<Result> {
    #[inline(always)] pure fn sin()  -> Result { cast(f64::sin (cast(*self))) }
    #[inline(always)] pure fn cos()  -> Result { cast(f64::cos (cast(*self))) }
    #[inline(always)] pure fn tan()  -> Result { cast(f64::tan (cast(*self))) }
    #[inline(always)] pure fn asin() -> Result { cast(f64::asin(cast(*self))) }
    #[inline(always)] pure fn acos() -> Result { cast(f64::acos(cast(*self))) }
    #[inline(always)] pure fn atan() -> Result { cast(f64::atan(cast(*self))) }
    #[inline(always)] pure fn sinh() -> Result { cast(f64::sinh(cast(*self))) }
    #[inline(always)] pure fn cosh() -> Result { cast(f64::cosh(cast(*self))) }
    #[inline(always)] pure fn tanh() -> Result { cast(f64::tanh(cast(*self))) }
}

#[inline(always)] pub pure fn sin<T:Trig <Result>, Result:NumCast>(angle: &T) -> Result { angle.sin() }
#[inline(always)] pub pure fn cos<T:Trig <Result>, Result:NumCast>(angle: &T) -> Result { angle.cos() }
#[inline(always)] pub pure fn tan<T:Trig <Result>, Result:NumCast>(angle: &T) -> Result { angle.tan() }
#[inline(always)] pub pure fn asin<T:Trig<Result>, Result:NumCast>(x: &T)     -> Result { x.asin() }
#[inline(always)] pub pure fn acos<T:Trig<Result>, Result:NumCast>(x: &T)     -> Result { x.acos() }
#[inline(always)] pub pure fn atan<T:Trig<Result>, Result:NumCast>(x: &T)     -> Result { x.atan() }
#[inline(always)] pub pure fn sinh<T:Trig<Result>, Result:NumCast>(x: &T)     -> Result { x.sinh() }
#[inline(always)] pub pure fn cosh<T:Trig<Result>, Result:NumCast>(x: &T)     -> Result { x.cosh() }
#[inline(always)] pub pure fn tanh<T:Trig<Result>, Result:NumCast>(x: &T)     -> Result { x.tanh() }

pub impl
<
    ThetaT:Copy Trig<T>,
    T:NumCast
>
Vec2<ThetaT>: Trig<Vec2<T>> {
    #[inline(always)]
    pure fn sin() -> Vec2<T> {
        Vec2::new(sin(&self[0]),
                  sin(&self[1]))
    }
    
    #[inline(always)]
    pure fn cos() -> Vec2<T> {
        Vec2::new(cos(&self[0]),
                  cos(&self[1]))
    }
    
    #[inline(always)]
    pure fn tan() -> Vec2<T> {
        Vec2::new(tan(&self[0]),
                  tan(&self[1]))
    }
    
    #[inline(always)]
    pure fn asin() -> Vec2<T> {
        Vec2::new(asin(&self[0]),
                  asin(&self[1]))
    }
    
    #[inline(always)]
    pure fn acos() -> Vec2<T> {
        Vec2::new(acos(&self[0]),
                  acos(&self[1]))
    }
    
    #[inline(always)]
    pure fn atan() -> Vec2<T> {
        Vec2::new(atan(&self[0]),
                  atan(&self[1]))
    }
    
    #[inline(always)]
    pure fn sinh() -> Vec2<T> {
        Vec2::new(sinh(&self[0]),
                  sinh(&self[1]))
    }
    
    #[inline(always)]
    pure fn cosh() -> Vec2<T> {
        Vec2::new(cosh(&self[0]),
                  cosh(&self[1]))
    }
    
    #[inline(always)]
    pure fn tanh() -> Vec2<T> {
        Vec2::new(tanh(&self[0]),
                  tanh(&self[1]))
    }
}

pub impl
<
    ThetaT:Copy Trig<T>,
    T:NumCast
>
Vec3<ThetaT>: Trig<Vec3<T>> {
    #[inline(always)]
    pure fn sin() -> Vec3<T> {
        Vec3::new(sin(&self[0]),
                  sin(&self[1]),
                  sin(&self[2]))
    }
    
    #[inline(always)]
    pure fn cos() -> Vec3<T> {
        Vec3::new(cos(&self[0]),
                  cos(&self[1]),
                  cos(&self[2]))
    }
    
    #[inline(always)]
    pure fn tan() -> Vec3<T> {
        Vec3::new(tan(&self[0]),
                  tan(&self[1]),
                  tan(&self[2]))
    }
    
    #[inline(always)]
    pure fn asin() -> Vec3<T> {
        Vec3::new(asin(&self[0]),
                  asin(&self[1]),
                  asin(&self[2]))
    }
    
    #[inline(always)]
    pure fn acos() -> Vec3<T> {
        Vec3::new(acos(&self[0]),
                  acos(&self[1]),
                  acos(&self[2]))
    }
    
    #[inline(always)]
    pure fn atan() -> Vec3<T> {
        Vec3::new(atan(&self[0]),
                  atan(&self[1]),
                  atan(&self[2]))
    }
    
    #[inline(always)]
    pure fn sinh() -> Vec3<T> {
        Vec3::new(sinh(&self[0]),
                  sinh(&self[1]),
                  sinh(&self[2]))
    }
    
    #[inline(always)]
    pure fn cosh() -> Vec3<T> {
        Vec3::new(cosh(&self[0]),
                  cosh(&self[1]),
                  cosh(&self[2]))
    }
    
    #[inline(always)]
    pure fn tanh() -> Vec3<T> {
        Vec3::new(tanh(&self[0]),
                  tanh(&self[1]),
                  tanh(&self[2]))
    }
}

pub impl
<
    ThetaT:Copy Trig<T>,
    T:NumCast
>
Vec4<ThetaT>: Trig<Vec4<T>>  {
    #[inline(always)]
    pure fn sin() -> Vec4<T> {
        Vec4::new(sin(&self[0]),
                  sin(&self[1]),
                  sin(&self[2]),
                  sin(&self[3]))
    }
    
    #[inline(always)]
    pure fn cos() -> Vec4<T> {
        Vec4::new(cos(&self[0]),
                  cos(&self[1]),
                  cos(&self[2]),
                  cos(&self[3]))
    }
    
    #[inline(always)]
    pure fn tan() -> Vec4<T> {
        Vec4::new(tan(&self[0]),
                  tan(&self[1]),
                  tan(&self[2]),
                  tan(&self[3]))
    }
    
    #[inline(always)]
    pure fn asin() -> Vec4<T> {
        Vec4::new(asin(&self[0]),
                  asin(&self[1]),
                  asin(&self[2]),
                  asin(&self[3]))
    }
    
    #[inline(always)]
    pure fn acos() -> Vec4<T> {
        Vec4::new(acos(&self[0]),
                  acos(&self[1]),
                  acos(&self[2]),
                  acos(&self[3]))
    }
    
    #[inline(always)]
    pure fn atan() -> Vec4<T> {
        Vec4::new(atan(&self[0]),
                  atan(&self[1]),
                  atan(&self[2]),
                  atan(&self[3]))
    }
    
    #[inline(always)]
    pure fn sinh() -> Vec4<T> {
        Vec4::new(sinh(&self[0]),
                  sinh(&self[1]),
                  sinh(&self[2]),
                  sinh(&self[3]))
    }
    
    #[inline(always)]
    pure fn cosh() -> Vec4<T> {
        Vec4::new(cosh(&self[0]),
                  cosh(&self[1]),
                  cosh(&self[2]),
                  cosh(&self[3]))
    }
    
    #[inline(always)]
    pure fn tanh() -> Vec4<T> {
        Vec4::new(tanh(&self[0]),
                  tanh(&self[1]),
                  tanh(&self[2]),
                  tanh(&self[3]))
    }
}