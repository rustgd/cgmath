use core::cmp::{Eq, Ord};
use core::f64::consts::pi;
use num::cast::*;
use vec::Vec3;

pub trait Angle<T>: Add<self,self>
                  , Sub<self,self>
                  , Mul<T,self>
                  , Div<T,self>
                  , Modulo<T,self>
                  , Neg<self>
                  , Eq, Ord {
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

pub impl<T:Copy Eq> Radians<T>: Eq {
    #[inline(always)] pure fn eq(other: &Radians<T>) -> bool { *self == **other }
    #[inline(always)] pure fn ne(other: &Radians<T>) -> bool { *self != **other }
}

pub impl<T:Copy Ord> Radians<T>: Ord {
    #[inline(always)] pure fn lt(other: &Radians<T>) -> bool { *self <  **other }
    #[inline(always)] pure fn le(other: &Radians<T>) -> bool { *self <= **other }
    #[inline(always)] pure fn ge(other: &Radians<T>) -> bool { *self >= **other }
    #[inline(always)] pure fn gt(other: &Radians<T>) -> bool { *self >  **other }
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

pub impl<T:Copy Eq> Degrees<T>: Eq {
    #[inline(always)] pure fn eq(other: &Degrees<T>) -> bool { *self == **other }
    #[inline(always)] pure fn ne(other: &Degrees<T>) -> bool { *self != **other }
}

pub impl<T:Copy Ord> Degrees<T>: Ord {
    #[inline(always)] pure fn lt(other: &Degrees<T>) -> bool { *self <  **other }
    #[inline(always)] pure fn le(other: &Degrees<T>) -> bool { *self <= **other }
    #[inline(always)] pure fn ge(other: &Degrees<T>) -> bool { *self >= **other }
    #[inline(always)] pure fn gt(other: &Degrees<T>) -> bool { *self >  **other }
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