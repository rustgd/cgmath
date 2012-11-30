use core::cmp::{Eq, Ord};
use core::f64::consts::pi;

use funs::triganomic::{cos, sin};
use mat::{Mat3, Mat4};
use num::cast::{NumCast, cast};
use quat::Quat;
use vec::Vec3;

/**
 * The base trait for anglular units
 */
pub trait Angle<T>: Add<self,self>
                  , Sub<self,self>
                  , Mul<T,self>
                  , Div<T,self>
                  , Modulo<T,self>
                  , Neg<self>
                  , Eq, Ord {
    static pure fn full_turn() -> self;
    static pure fn half_turn() -> self;
    static pure fn quadrant()  -> self;
    static pure fn sextant()   -> self;
    static pure fn octant()    -> self;
    
    pure fn to_radians(&self) -> Radians<T>;
    pure fn to_degrees(&self) -> Degrees<T>;
    pure fn wrap(&self) -> self;
    pure fn opposite(&self) -> self;
}





pub enum Radians<T> = T;

pub impl<T:Copy Num NumCast> Radians<T>: Angle<T> {
    #[inline(always)] static pure fn full_turn()    -> Radians<T> { Radians(move cast(2.0 * pi)) }  // TODO: calculate absolute values
    #[inline(always)] static pure fn half_turn()    -> Radians<T> { Radians(move cast(pi))       }
    #[inline(always)] static pure fn quadrant()     -> Radians<T> { Radians(move cast(pi / 2.0)) }
    #[inline(always)] static pure fn sextant()      -> Radians<T> { Radians(move cast(pi / 3.0)) }
    #[inline(always)] static pure fn octant()       -> Radians<T> { Radians(move cast(pi / 4.0)) }
    
    #[inline(always)] pure fn to_radians(&self) -> Radians<T> { *self }
    #[inline(always)] pure fn to_degrees(&self) -> Degrees<T> { Degrees(**self * cast(180.0 / pi)) }
    
    #[inline(always)]
    pure fn wrap(&self) -> Radians<T> {
        (*self) % cast(2.0 * pi)   // TODO: keep in the domain of 0 to two_pi
    }
    
    #[inline(always)]
    pure fn opposite(&self) -> Radians<T> {
        (self + Angle::half_turn()).wrap()     // TODO: test!
    }
}
    
pub impl<T:Copy Num> Radians<T>: Add<Radians<T>, Radians<T>> {
    #[inline(always)]
    pure fn add(rhs: &Radians<T>) -> Radians<T> {
        Radians(*self + **rhs)
    }
}
    
pub impl<T:Copy Num> Radians<T>: Sub<Radians<T>, Radians<T>> {
    #[inline(always)]
    pure fn sub(&self, rhs: &Radians<T>) -> Radians<T> {
        Radians(**self - **rhs)
    }
}
    
pub impl<T:Copy Num> Radians<T>: Mul<T, Radians<T>> {
    #[inline(always)]
    pure fn mul(&self, rhs: &T) -> Radians<T> {
        Radians(**self * *rhs)
    }
}
    
pub impl<T:Copy Num> Radians<T>: Div<T, Radians<T>> {
    #[inline(always)]
    pure fn div(&self, rhs: &T) -> Radians<T> {
        Radians(**self / *rhs)
    }
}
    
pub impl<T:Copy Num> Radians<T>: Modulo<T, Radians<T>> {
    #[inline(always)]
    pure fn modulo(&self, rhs: &T) -> Radians<T> {
        Radians(**self % *rhs)
    }
}
    
pub impl<T:Copy Num> Radians<T>: Neg<Radians<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Radians<T> {
        Radians(-**self)
    }
}

pub impl<T:Copy Eq> Radians<T>: Eq {
    #[inline(always)] pure fn eq(&self, other: &Radians<T>) -> bool { **self == **other }
    #[inline(always)] pure fn ne(&self, other: &Radians<T>) -> bool { **self != **other }
}

pub impl<T:Copy Ord> Radians<T>: Ord {
    #[inline(always)] pure fn lt(&self, other: &Radians<T>) -> bool { **self <  **other }
    #[inline(always)] pure fn le(&self, other: &Radians<T>) -> bool { **self <= **other }
    #[inline(always)] pure fn ge(&self, other: &Radians<T>) -> bool { **self >= **other }
    #[inline(always)] pure fn gt(&self, other: &Radians<T>) -> bool { **self >  **other }
}

pub impl<T> Radians<T>: ToStr {
    pure fn to_str() -> ~str { fmt!("%? rad", *self) }
}





pub enum Degrees<T> = T;

pub impl<T:Copy Num NumCast> Degrees<T>: Angle<T> {
    #[inline(always)] static pure fn full_turn()    -> Degrees<T> { Degrees(move cast(360.0)) }
    #[inline(always)] static pure fn half_turn()    -> Degrees<T> { Degrees(move cast(180.0)) }
    #[inline(always)] static pure fn quadrant()     -> Degrees<T> { Degrees(move cast(90.0))  }
    #[inline(always)] static pure fn sextant()      -> Degrees<T> { Degrees(move cast(60.0))  }
    #[inline(always)] static pure fn octant()       -> Degrees<T> { Degrees(move cast(45.0))  }
    
    #[inline(always)] pure fn to_radians(&self) -> Radians<T> { Radians(**self * cast(pi / 180.0)) }
    #[inline(always)] pure fn to_degrees(&self) -> Degrees<T> { *self }
    
    #[inline(always)]
    pure fn wrap(&self) -> Degrees<T> {
        (*self) % cast(360)   // TODO: keep in the domain of 0 to 360
    }
    
    #[inline(always)]
    pure fn opposite(&self) -> Degrees<T> {
        (self + Angle::half_turn()).wrap()      // TODO: test!
    }
}

pub impl<T:Copy Num> Degrees<T>: Add<Degrees<T>, Degrees<T>> {
    #[inline(always)]
    pure fn add(rhs: &Degrees<T>) -> Degrees<T> {
        Degrees(*self + **rhs)
    }
}
    
pub impl<T:Copy Num> Degrees<T>: Sub<Degrees<T>, Degrees<T>> {
    #[inline(always)]
    pure fn sub(&self, rhs: &Degrees<T>) -> Degrees<T> {
        Degrees(**self - **rhs)
    }
}
    
pub impl<T:Copy Num> Degrees<T>: Mul<T, Degrees<T>> {
    #[inline(always)]
    pure fn mul(&self, rhs: &T) -> Degrees<T> {
        Degrees(**self * *rhs)
    }
}
    
pub impl<T:Copy Num> Degrees<T>: Div<T, Degrees<T>> {
    #[inline(always)]
    pure fn div(&self, rhs: &T) -> Degrees<T> {
        Degrees(**self / *rhs)
    }
}
    
pub impl<T:Copy Num> Degrees<T>: Modulo<T, Degrees<T>> {
    #[inline(always)]
    pure fn modulo(&self, rhs: &T) -> Degrees<T> {
        Degrees(**self % *rhs)
    }
}
    
pub impl<T:Copy Num> Degrees<T>: Neg<Degrees<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Degrees<T> {
        Degrees(-**self)
    }
}

pub impl<T:Copy Eq> Degrees<T>: Eq {
    #[inline(always)] pure fn eq(&self, other: &Degrees<T>) -> bool { **self == **other }
    #[inline(always)] pure fn ne(&self, other: &Degrees<T>) -> bool { **self != **other }
}

pub impl<T:Copy Ord> Degrees<T>: Ord {
    #[inline(always)] pure fn lt(&self, other: &Degrees<T>) -> bool { **self <  **other }
    #[inline(always)] pure fn le(&self, other: &Degrees<T>) -> bool { **self <= **other }
    #[inline(always)] pure fn ge(&self, other: &Degrees<T>) -> bool { **self >= **other }
    #[inline(always)] pure fn gt(&self, other: &Degrees<T>) -> bool { **self >  **other }
}

pub impl<T> Degrees<T>: ToStr {
    pure fn to_str() -> ~str { fmt!("%?\xB0", *self) }
}





/**
 * An angular rotation around an arbitary axis
 */
pub struct Rotation<T> {
    theta: Radians<T>,
    axis: Vec3<T>,
}

pub impl<T:Copy Num NumCast> Rotation<T> {
    #[inline(always)]
    static pure fn new(theta: Radians<T>, axis: Vec3<T>) -> Rotation<T> {
        Rotation { theta: move theta, axis: move axis }
    }
    
    #[inline(always)]
    pure fn to_mat3() -> Mat3<T> {
        let c:  T = cos(&self.theta);
        let s:  T = sin(&self.theta);
        let _0: T = cast(0);
        let _1: T = cast(1);
        let t:  T = _1 - c;
        
        let x = self.axis.x;
        let y = self.axis.y;
        let z = self.axis.z;
        
        Mat3::new(t * x * x + c,       t * x * y + s * z,   t * x * z - s * y,
                  t * x * y - s * z,   t * y * y + c,       t * y * z + s * x,
                  t * x * z - s - y,   t * y * z - s * x,   t * z * z + c)
    }
    
    #[inline(always)]
    pure fn to_mat4() -> Mat4<T> {
        self.to_mat3().to_mat4()
    }
    
    #[inline(always)]
    pure fn to_quat() -> Quat<T> {
        let half = self.theta / cast(2);
        Quat::from_sv(cos(&half), self.axis.mul_t(sin(&half)))
    }
}





pub struct Euler<T> {
    x: Radians<T>,   // pitch
    y: Radians<T>,   // yaw
    z: Radians<T>,   // roll
}