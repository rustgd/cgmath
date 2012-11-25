use core::cast::transmute;
use core::cmp::{Eq, Ord};
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;

use dim::Dimensional;
use funs::exp::*;
use funs::extent::*;
use funs::trig::*;
use mat::{Mat3, Mat4};
use num::cast::*;
use num::default_eq::DefaultEq;
use vec::Vec3;


///
/// The base quaternion trait
///
pub trait Quaternion<T>: Dimensional<T>, Eq, DefaultEq, Neg<self> {
    static pure fn identity() -> self;
    static pure fn zero() -> self;
    
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
    
    pure fn to_mat3() -> Mat3<T>;
    pure fn to_mat4() -> Mat4<T>;
}

pub trait ToQuat<T> {
    pure fn to_Quat() -> Quat<T>;
}






pub struct Quat<T> { s: T, v: Vec3<T> }

pub impl<T> Quat<T> {
    #[inline(always)]
    static pure fn new(s: T, vx: T, vy: T, vz: T) -> Quat<T> {
        Quat::from_sv(move s, move Vec3::new(move vx, move vy, move vz))
    }
    
    #[inline(always)]
    static pure fn from_sv(s: T, v: Vec3<T>) -> Quat<T> {
        Quat { s: move s, v: move v }
    }
}

pub impl<T:Copy Num NumCast Exp Clamp Ord InvTrig> Quat<T>: Quaternion<T> {
    #[inline(always)]
    static pure fn identity() -> Quat<T> {
        Quat::new(NumCast::one(),
                  NumCast::one(),
                  NumCast::one(),
                  NumCast::one())
    }
    
    #[inline(always)]
    static pure fn zero() -> Quat<T> {
        Quat::new(NumCast::zero(),
                  NumCast::zero(),
                  NumCast::zero(),
                  NumCast::zero())
    }
    
    #[inline(always)]
    static pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    static pure fn size_of() -> uint { size_of::<Quat<T>>() }
    
    #[inline(always)]
    pure fn to_ptr() -> *T {
        to_unsafe_ptr(&self[0])
    }
    
    #[inline(always)]
    pure fn neg() -> Quat<T> {
        Quat::from_sv(-self.s, -self.v)
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
        let tmp = self.v.cross(vec).add_v(&vec.mul_t(self.s));
        self.v.cross(&tmp).mul_t(cast(2)).add_v(vec)
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
        Quat::new(self.s * other.s   - self.v.x * other.v.x - self.v.y * other.v.y - self.v.z * other.v.z,
                  self.s * other.v.x + self.v.x * other.s   + self.v.y * other.v.z - self.v.z * other.v.y, 
                  self.s * other.v.y + self.v.y * other.s   + self.v.z * other.v.x - self.v.x * other.v.z, 
                  self.s * other.v.z + self.v.z * other.s   + self.v.x * other.v.y - self.v.y * other.v.x) 
    }
    
    #[inline(always)]
    pure fn dot(other: &Quat<T>) -> T {
        self.s * other.s + self.v.dot(&other.v)
    }
    
    #[inline(always)]
    pure fn conjugate() -> Quat<T> {
        Quat::from_sv(self.s, -self.v)
    }
    
    #[inline(always)]
    pure fn inverse() -> Quat<T> {
        self.conjugate().div_t(self.length2())
    }
    
    #[inline(always)]
    pure fn length2() -> T {
        self.s * self.s + self.v.length2()
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
    pub pure fn from_axis_angle(axis: Vec3<T>, theta: Angle<T>) -> Quat<T> {
        let half = theta / cast(2);
        Quat::from_sv(cos(&half), axis.mul_t(sin(&half)))
    }
    
    #[inline(always)]
    pure fn to_mat3() -> Mat3<T> {
        let x2 = self.v.x + self.v.x;
        let y2 = self.v.y + self.v.y;
        let z2 = self.v.z + self.v.z;
        
        let xx2 = x2 * self.v.x;
        let xy2 = x2 * self.v.y;
        let xz2 = x2 * self.v.z;
        
        let yy2 = y2 * self.v.y;
        let yz2 = y2 * self.v.z;
        let zz2 = z2 * self.v.z;
        
        let sy2 = y2 * self.s;
        let sz2 = z2 * self.s;
        let sx2 = x2 * self.s;
        
        let _1: T = cast(1);
        
        Mat3::new(_1 - yy2 - zz2,      xy2 - sz2,      xz2 + sy2,
                       xy2 + sz2, _1 - xx2 - zz2,      yz2 - sx2,
                       xz2 - sy2,      yz2 + sx2, _1 - xx2 - yy2)
    }
    
    #[inline(always)]
    pure fn to_mat4() -> Mat4<T> {
        self.to_mat3().to_mat4()
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