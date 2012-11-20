use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::{addr_of, to_unsafe_ptr};
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;

use dim::Dimensional;
use funs::exp::*;
use funs::trig::*;
use funs::common::*;
use mat::{Mat3, Mat4};
use num::cast::*;
use num::default_eq::DefaultEq;
use vec::Vec3;


//
//  Quaternion
//
pub trait Quaternion<T>: Dimensional<T>, Eq, DefaultEq, Neg<self> {
    pure fn mul_t(value: T) -> self;
    pure fn div_t(value: T) -> self;
    
    pure fn mul_v(vec: &Vec3<T>) -> Vec3<T>;
    
    pure fn add_q(other: &self) -> self;
    pure fn sub_q(other: &self) -> self;
    pure fn mul_q(other: &self) -> self;
    
    pure fn dot(other: &self) -> T;
    
    pure fn conjugate() -> self;
    pure fn inverse() -> self;
    pure fn length2() -> T;
    pure fn length() -> T;
    pure fn normalize() -> self;
    
    pure fn nlerp(other: &self, amount: T) -> self;
    pure fn slerp(other: &self, amount: T) -> self;
    
    pure fn to_Mat3() -> Mat3<T>;
    pure fn to_Mat4() -> Mat4<T>;
}

pub trait ToQuat<T> {
    pure fn to_Quat() -> Quat<T>;
}






pub struct Quat<T> { w: T, x: T, y: T, z: T }

pub mod Quat {
    #[inline(always)]
    pub pure fn new<T>(w: T, x: T, y: T, z: T) -> Quat<T> {
        Quat { w: move w, x: move x, y: move y, z: move z }
    }
    
    #[inline(always)]
    pub pure fn from_sv<T:Copy>(s: T, v: Vec3<T>) -> Quat<T> {
        Quat::new(s, v.x, v.y, v.z)
    }

    #[inline(always)]
    pub pure fn from_axis_angle<T:Copy Num NumCast Trig AngleConv>(axis: Vec3<T>, theta: T) -> Quat<T> {
        let half = radians(&theta) / cast(2);
        from_sv(cos(&half), axis.mul_t(sin(&half)))
    }
    
    #[inline(always)]
    pub pure fn zero<T:Copy NumCast>() -> Quat<T> {
        let _0 = cast(0);
        Quat::new(_0, _0, _0, _0)
    }
    
    #[inline(always)]
    pub pure fn identity<T:Copy NumCast>() -> Quat<T> {
        let _0 = cast(0);
        Quat::new(cast(1), _0, _0, _0)
    }
}

pub impl<T:Copy Num NumCast Trig Exp Extent Ord FuzzyEq> Quat<T>: Quaternion<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    pure fn to_ptr() -> *T {
        addr_of(&self[0])
    }
    
    #[inline(always)]
    pure fn neg() -> Quat<T> {
        Quat::new(-self[0], -self[1], -self[2], -self[3])
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Quat<T> {
        Quat::new(self[0] * value,
                  self[1] * value,
                  self[2] * value,
                  self[3] * value)
    }
    
    #[inline(always)]
    pure fn div_t(value: T) -> Quat<T> {
        Quat::new(self[0] / value,
                  self[1] / value,
                  self[2] / value,
                  self[3] / value)
    }

    #[inline(always)]
    pure fn mul_v(vec: &Vec3<T>) -> Vec3<T>  {
        let base = Vec3{ x:self.x, y:self.y, z:self.z };
        let tmp = base.cross(vec).add_v(&vec.mul_t(self.w));
        base.cross(&tmp).mul_t(cast(2)).add_v(vec)
    }
    
    #[inline(always)]
    pure fn add_q(other: &Quat<T>) -> Quat<T> {
        Quat::new(self[0] + other[0],
                  self[1] + other[1],
                  self[2] + other[2],
                  self[3] + other[3])
    }
    
    #[inline(always)]
    pure fn sub_q(other: &Quat<T>) -> Quat<T> {
        Quat::new(self[0] - other[0],
                  self[1] - other[1],
                  self[2] - other[2],
                  self[3] - other[3])
    }
    
    #[inline(always)]
    pure fn mul_q(other: &Quat<T>) -> Quat<T> {
        Quat::new(self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
                  self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y, 
                  self.w * other.y + self.y * other.w + self.z * other.x - self.x * other.z, 
                  self.w * other.z + self.z * other.w + self.x * other.y - self.y * other.x) 
    }
    
    #[inline(always)]
    pure fn dot(other: &Quat<T>) -> T {
        self.w * other.w +
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }
    
    #[inline(always)]
    pure fn conjugate() -> Quat<T> {
        Quat::new(self.w, -self.x, -self.y, -self.z)
    }
    
    #[inline(always)]
    pure fn inverse() -> Quat<T> {
        let mut n: T = cast(1);
        n /= self.length2();
        self.conjugate().mul_t(n)
    }
    
    #[inline(always)]
    pure fn length2() -> T {
        self.w * self.w +
        self.x * self.x +
        self.y * self.y +
        self.z * self.z
    }
    
    #[inline(always)]
    pure fn length() -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn normalize() -> Quat<T> {
        let mut n: T = cast(1);
        n /= self.length();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn nlerp(other: &Quat<T>, amount: T) -> Quat<T> {
        let _1: T = cast(1);
        self.mul_t(_1 - amount).add_q(&other.mul_t(amount)).normalize()
    }
    
    /**
     * Spherical Linear Intoperlation
     *
     * Both quaternions should be normalized first, or else strange things will
     * will happen...
     *
     * Note: The `acos` used in `slerp` is an expensive operation, so unless your
     * quarternions a far away from each other it's generally more advisable to
     * use nlerp when you know your rotations are going to be small.
     *
     * See *[Understanding Slerp, Then Not Using It]
     * (http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/)*
     * for more information. The [Arcsynthesis OpenGL tutorial]
     * (http://www.arcsynthesis.org/gltut/Positioning/Tut08%20Interpolation.html)
     * also provides a good explanation.
     */
    #[inline(always)]
    pure fn slerp(other: &Quat<T>, amount: T) -> Quat<T> {
        let dot: T = cast(self.dot(other));
        
        // if quaternions are close together use `nlerp`
        let dot_threshold = cast(0.9995);
        if dot > dot_threshold { return self.nlerp(other, amount) }
        
        let robust_dot = dot.clamp(&-cast(1), &cast(1));    // stay within the domain of acos()
        let theta_0 = acos(&robust_dot);                    // the angle between the quaternions
        let theta = theta_0 * amount;                       // the fraction of theta specified by `amount`
        
        let q = other.sub_q(&self.mul_t(robust_dot))
                     .normalize();
        
        self.mul_t(cos(&theta)).add_q(&q.mul_t(sin(&theta)))
    }
    
    #[inline(always)]
    pure fn to_Mat3() -> Mat3<T> {
        let x2 = self.x + self.x;
        let y2 = self.y + self.y;
        let z2 = self.z + self.z;
        
        let xx2 = x2 * self.x;
        let xy2 = x2 * self.y;
        let xz2 = x2 * self.z;
        
        let yy2 = y2 * self.y;
        let yz2 = y2 * self.z;
        let zz2 = z2 * self.z;
        
        let wy2 = y2 * self.w;
        let wz2 = z2 * self.w;
        let wx2 = x2 * self.w;
        
        let _1: T = cast(1);
        
        Mat3::new(_1 - yy2 - zz2,      xy2 - wz2,      xz2 + wy2,
                       xy2 + wz2, _1 - xx2 - zz2,      yz2 - wx2,
                       xz2 - wy2,      yz2 + wx2, _1 - xx2 - yy2)
    }
    
    #[inline(always)]
    pure fn to_Mat4() -> Mat4<T> {
        self.to_Mat3().to_Mat4()
    }
}

pub impl<T:Copy> Quat<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Quat<T>, *T>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy DefaultEq> Quat<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Quat<T>) -> bool {
        self.default_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Quat<T>) -> bool {
        !(self == *other)
    }
}

pub impl<T:Copy FuzzyEq> Quat<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Quat<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
}

pub impl<T:Copy DefaultEq> Quat<T>: DefaultEq {
    #[inline(always)]
    pure fn default_eq(other: &Quat<T>) -> bool {
        self[0].default_eq(&other[0]) &&
        self[1].default_eq(&other[1]) &&
        self[2].default_eq(&other[2]) &&
        self[3].default_eq(&other[3])
    }
}