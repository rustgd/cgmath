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

pub struct AxisRotation<T> {
    axis: Vec3<T>,
    theta: Angle<T>,
}

pub struct Euler<T> {
    x: T,   // pitch
    y: T,   // yaw
    z: T,   // roll
}