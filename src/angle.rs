use num::cast::*;
use vec::Vec3;

pub enum Angle<T> {
    degrees(T),
    radians(T),
}

pub impl<T:Copy Num NumCast> Angle<T> {
    pure fn degrees() -> T {
        match self {
            degrees(theta) => theta,
            radians(theta) => theta * cast(180f64 / f64::consts::pi)
        }
    }
    
    pure fn radians() -> T {
        match self {
            degrees(theta) => theta * cast(f64::consts::pi / 180f64),
            radians(theta) => theta
        }
    }
}

pub impl<T:Copy Num> Angle<T>: Add<T,Angle<T>> {
    #[inline(always)]
    pure fn add(rhs: &T) -> Angle<T> {
        match self {
            degrees(theta) => degrees(theta + *rhs),
            radians(theta) => radians(theta + *rhs)
        }
    }
}

pub impl<T:Copy Num> Angle<T>: Sub<T,Angle<T>> {
    #[inline(always)]
    pure fn sub(rhs: &T) -> Angle<T> {
        match self {
            degrees(theta) => degrees(theta - *rhs),
            radians(theta) => radians(theta - *rhs)
        }
    }
}

pub impl<T:Copy Num> Angle<T>: Mul<T,Angle<T>> {
    #[inline(always)]
    pure fn mul(rhs: &T) -> Angle<T> {
        match self {
            degrees(theta) => degrees(theta * *rhs),
            radians(theta) => radians(theta * *rhs)
        }
    }
}

pub impl<T:Copy Num> Angle<T>: Div<T,Angle<T>> {
    #[inline(always)]
    pure fn div(rhs: &T) -> Angle<T> {
        match self {
            degrees(theta) => degrees(theta / *rhs),
            radians(theta) => radians(theta / *rhs)
        }
    }
}

pub impl<T:Copy Num> Angle<T>: Modulo<T,Angle<T>> {
    #[inline(always)]
    pure fn modulo(rhs: &T) -> Angle<T> {
        match self {
            degrees(theta) => degrees(theta % *rhs),
            radians(theta) => radians(theta % *rhs)
        }
    }
}

pub impl<T:Copy Num> Angle<T>: Neg<Angle<T>> {
    #[inline(always)]
    pure fn neg() -> Angle<T> {
        match self {
            degrees(theta) => degrees(-theta),
            radians(theta) => radians(-theta)
        }
    }
}

pub struct AxisRotation<T> {
    axis: Vec3<T>,
    theta: Angle<T>,
}

pub struct Euler<T> {
    x: T,   // pitch
    y: T,   // yaw
    z: T,   // roll
}