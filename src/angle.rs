use core::f64::consts::pi;
use num::cast::*;
use vec::Vec3;

pub trait Angle<T>: Add<self,self>
                  , Sub<self,self>
                  , Mul<T,self>
                  , Div<T,self>
                  , Modulo<T,self>
                  , Neg<self> {
    pure fn to_radians() -> Radians<T>;
    pure fn to_degrees() -> Degrees<T>;
}

pub enum Radians<T> = T;

pub impl<T:Copy Num NumCast> Radians<T>: Angle<T> {
    #[inline(always)] pure fn to_radians() -> Radians<T> { self }
    #[inline(always)] pure fn to_degrees() -> Degrees<T> { Degrees(*self * cast(180.0 / pi)) }
    
    #[inline(always)] pure fn add(rhs: &Radians<T>) -> Radians<T> { Radians(*self + **rhs) }
    #[inline(always)] pure fn sub(rhs: &Radians<T>) -> Radians<T> { Radians(*self - **rhs) }
    #[inline(always)] pure fn mul(rhs: &T)          -> Radians<T> { Radians(*self * *rhs) }
    #[inline(always)] pure fn div(rhs: &T)          -> Radians<T> { Radians(*self / *rhs) }
    #[inline(always)] pure fn modulo(rhs: &T)       -> Radians<T> { Radians(*self % *rhs) }
    #[inline(always)] pure fn neg()                 -> Radians<T> { Radians(-*self) }
}

pub enum Degrees<T> = T;

pub impl<T:Copy Num NumCast> Degrees<T>: Angle<T> {
    #[inline(always)] pure fn to_radians() -> Radians<T> { Radians(*self * cast(pi / 180.0)) }
    #[inline(always)] pure fn to_degrees() -> Degrees<T> { self }
    
    #[inline(always)] pure fn add(rhs: &Degrees<T>) -> Degrees<T> { Degrees(*self + **rhs) }
    #[inline(always)] pure fn sub(rhs: &Degrees<T>) -> Degrees<T> { Degrees(*self - **rhs) }
    #[inline(always)] pure fn mul(rhs: &T)          -> Degrees<T> { Degrees(*self * *rhs) }
    #[inline(always)] pure fn div(rhs: &T)          -> Degrees<T> { Degrees(*self / *rhs) }
    #[inline(always)] pure fn modulo(rhs: &T)       -> Degrees<T> { Degrees(*self % *rhs) }
    #[inline(always)] pure fn neg()                 -> Degrees<T> { Degrees(-*self) }
}

pub struct AxialRotation<T> {
    axis: Vec3<T>,
    theta: Radians<T>,
}

pub struct Euler<T> {
    x: Radians<T>,   // pitch
    y: Radians<T>,   // yaw
    z: Radians<T>,   // roll
}