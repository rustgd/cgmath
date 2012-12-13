/**
 * > Every morning in the early part of October 1843, on my coming down to
 *   breakfast, your brother William Edward and yourself used to ask me: "Well,
 *   Papa, can you multiply triples?" Whereto I was always obliged to reply,
 *   with a sad shake of the head, "No, I can only add and subtract them."
 *
 *   Sir William Hamilton
 */

use core::cast::transmute;
use core::cmp::{Eq, Ord};
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;

use angle::Angle;
use funs::common::*;
use funs::exponential::*;
use funs::triganomic::*;
use mat::{Mat3, Mat4};
use num::types::{Float, Number};
use vec::Vec3;


/**
 * The base quaternion trait
 *
 * # Type parameters
 *
 * * `T` - The type of the components. Should be a floating point type.
 * * `V3` - The 3-dimensional vector type that will containin the imaginary
 *          components of the quaternion.
 */
pub trait Quaternion<T,V3>: Index<uint, T> Eq Neg<self> {
    /**
     * # Return value
     *
     * The multiplicative identity, ie: `q = 1 + 0i + 0j + 0i`
     */
    static pure fn identity() -> self;
    
    /**
     * # Return value
     *
     * The additive identity, ie: `q = 0 + 0i + 0j + 0i`
     */
    static pure fn zero() -> self;
    
    /**
     * # Return value
     *
     * The result of multiplying the quaternion a scalar
     */
    pure fn mul_t(&self, value: T) -> self;
    
    /**
     * # Return value
     *
     * The result of dividing the quaternion a scalar
     */
    pure fn div_t(&self, value: T) -> self;
    
    /**
     * # Return value
     *
     * The result of multiplying the quaternion by a vector
     */
    pure fn mul_v(&self, vec: &V3) -> V3;
    
    /**
     * # Return value
     *
     * The sum of this quaternion and `other` 
     */
    pure fn add_q(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The sum of this quaternion and `other` 
     */
    pure fn sub_q(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The the result of multipliplying the quaternion by `other`
     */
    pure fn mul_q(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The dot product of the quaternion and `other`
     */
    pure fn dot(&self, other: &self) -> T;
    
    /**
     * # Return value
     *
     * The conjugate of the quaternion
     */
    pure fn conjugate(&self) -> self;
    
    /**
     * # Return value
     *
     * The multiplicative inverse of the quaternion
     */
    pure fn inverse(&self) -> self;
    
    /**
     * # Return value
     *
     * The squared magnitude of the quaternion. This is useful for
     * magnitude comparisons where the exact magnitude does not need to be
     * calculated.
     */
    pure fn magnitude2(&self) -> T;
    
    /**
     * # Return value
     *
     * The magnitude of the quaternion
     *
     * # Performance notes
     *
     * For instances where the exact magnitude of the quaternion does not need
     * to be known, for example for quaternion-quaternion magnitude comparisons,
     * it is advisable to use the `magnitude2` method instead.
     */
    pure fn magnitude(&self) -> T;
    
    /**
     * # Return value
     *
     * The normalized quaternion
     */
    pure fn normalize(&self) -> self;
    
    /**
     * Normalised linear interpolation
     *
     * # Return value
     *
     * The intoperlated quaternion
     */
    pure fn nlerp(&self, other: &self, amount: T) -> self;
    
    /**
     * Perform a spherical linear interpolation between the quaternion and
     * `other`.
     *
     * # Return value
     *
     * The intoperlated quaternion
     *
     * # Performance notes
     *
     * This is more accurate than `nlerp` but is also more
     * computationally intensive.
     */ 
    pure fn slerp(&self, other: &self, amount: T) -> self;
    
    /**
     * Convert the quaternion to a 3 x 3 rotation matrix
     */
    pure fn to_mat3(&self) -> Mat3<T>;
    
    /**
     * Convert the quaternion to a 4 x 4 transformation matrix
     */
    pure fn to_mat4(&self) -> Mat4<T>;
    
    /**
     * # Return value
     *
     * A pointer to the first component of the quaternion
     */
    pure fn to_ptr(&self) -> *T;
}

pub trait ToQuat<T> {
    /**
     * Convert `self` to a quaternion
     */
    pure fn to_Quat() -> Quat<T>;
}





/**
 * A quaternion in scalar/vector form
 *
 * # Type parameters
 *
 * * `T` - The type of the components. Should be a floating point type.
 *
 * # Fields
 *
 * * `s` - the scalar component
 * * `v` - a vector containing the three imaginary components
 */
pub struct Quat<T> { s: T, v: Vec3<T> }

pub impl<T:Copy Float> Quat<T> {
    /**
     * Construct the quaternion from one scalar component and three
     * imaginary components
     *
     * # Arguments
     *
     * * `w`  - the scalar component
     * * `xi` - the fist imaginary component
     * * `yj` - the second imaginary component
     * * `zk` - the third imaginary component
     */
    #[inline(always)]
    static pure fn new(w: T, xi: T, yj: T, zk: T) -> Quat<T> {
        Quat::from_sv(move w, move Vec3::new(move xi, move yj, move zk))
    }
    
    /**
     * Construct the quaternion from a scalar and a vector
     *
     * # Arguments
     *
     * * `s` - the scalar component
     * * `v` - a vector containing the three imaginary components
     */
    #[inline(always)]
    static pure fn from_sv(s: T, v: Vec3<T>) -> Quat<T> {
        Quat { s: move s, v: move v }
    }
    
    #[inline(always)]
    static pure fn from_axis_angle<A:Angle<T>>(axis: &Vec3<T>, theta: A) -> Quat<T> {
        let half = theta.to_radians() / Number::from(2);
        Quat::from_sv(cos(&half), axis.mul_t(sin(&half)))
    }
}

pub impl<T:Copy> Quat<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Quat<T>, *T>(
                to_unsafe_ptr(self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy Float Exp Extent InvTrig> Quat<T>: Quaternion<T, Vec3<T>> {
    #[inline(always)]
    static pure fn identity() -> Quat<T> {
        Quat::new(Number::from(1),
                  Number::from(0),
                  Number::from(0),
                  Number::from(0))
    }
    
    #[inline(always)]
    static pure fn zero() -> Quat<T> {
        Quat::new(Number::from(0),
                  Number::from(0),
                  Number::from(0),
                  Number::from(0))
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Quat<T> {
        Quat::new(self[0] * value,
                  self[1] * value,
                  self[2] * value,
                  self[3] * value)
    }
    
    #[inline(always)]
    pure fn div_t(&self, value: T) -> Quat<T> {
        Quat::new(self[0] / value,
                  self[1] / value,
                  self[2] / value,
                  self[3] / value)
    }

    #[inline(always)]
    pure fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T>  {
        let tmp = self.v.cross(vec).add_v(&vec.mul_t(self.s));
        self.v.cross(&tmp).mul_t(Number::from(2)).add_v(vec)
    }
    
    #[inline(always)]
    pure fn add_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(self[0] + other[0],
                  self[1] + other[1],
                  self[2] + other[2],
                  self[3] + other[3])
    }
    
    #[inline(always)]
    pure fn sub_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(self[0] - other[0],
                  self[1] - other[1],
                  self[2] - other[2],
                  self[3] - other[3])
    }
    
    #[inline(always)]
    pure fn mul_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(self.s * other.s   - self.v.x * other.v.x - self.v.y * other.v.y - self.v.z * other.v.z,
                  self.s * other.v.x + self.v.x * other.s   + self.v.y * other.v.z - self.v.z * other.v.y, 
                  self.s * other.v.y + self.v.y * other.s   + self.v.z * other.v.x - self.v.x * other.v.z, 
                  self.s * other.v.z + self.v.z * other.s   + self.v.x * other.v.y - self.v.y * other.v.x) 
    }
    
    #[inline(always)]
    pure fn dot(&self, other: &Quat<T>) -> T {
        self.s * other.s + self.v.dot(&other.v)
    }
    
    #[inline(always)]
    pure fn conjugate(&self) -> Quat<T> {
        Quat::from_sv(self.s, -self.v)
    }
    
    #[inline(always)]
    pure fn inverse(&self) -> Quat<T> {
        self.conjugate().div_t(self.magnitude2())
    }
    
    #[inline(always)]
    pure fn magnitude2(&self) -> T {
        self.s * self.s + self.v.length2()
    }
    
    #[inline(always)]
    pure fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }
    
    #[inline(always)]
    pure fn normalize(&self) -> Quat<T> {
        let mut n: T = Number::from(1);
        n /= self.magnitude();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn nlerp(&self, other: &Quat<T>, amount: T) -> Quat<T> {
        let _1: T = Number::from(1);
        self.mul_t(_1 - amount).add_q(&other.mul_t(amount)).normalize()
    }
    
    /**
     * Spherical Linear Intoperlation
     *
     * Both quaternions should be normalized first, or else strange things will
     * will happen...
     *
     * # Performance notes
     *
     * The `acos` operation used in `slerp` is an expensive operation, so unless
     * your quarternions a far away from each other it's generally more advisable
     * to use `nlerp` when you know your rotations are going to be small.
     *
     * - [Understanding Slerp, Then Not Using It]
     *   (http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/)
     * - [Arcsynthesis OpenGL tutorial]
     *   (http://www.arcsynthesis.org/gltut/Positioning/Tut08%20Interpolation.html)
     */
    #[inline(always)]
    pure fn slerp(&self, other: &Quat<T>, amount: T) -> Quat<T> {
        let dot = self.dot(other);
        
        let dot_threshold = Number::from(0.9995);
        
        if dot > dot_threshold {
            return self.nlerp(other, amount);                   // if quaternions are close together use `nlerp`
        } else {
            let robust_dot = dot.clamp(&-Number::from(1),
                                       &Number::from(1));       // stay within the domain of acos()
            
            let theta_0 = acos(&robust_dot);                    // the angle between the quaternions
            let theta = theta_0 * amount;                       // the fraction of theta specified by `amount`
            
            let q = other.sub_q(&self.mul_t(robust_dot))
                         .normalize();
            
            return self.mul_t(cos(&theta)).add_q(&q.mul_t(sin(&theta)));
        }
    }
    
    #[inline(always)]
    pure fn to_mat3(&self) -> Mat3<T> {
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
        
        let _1: T = Number::from(1);
        
        Mat3::new(_1 - yy2 - zz2,      xy2 + sz2,      xz2 - sy2,
                       xy2 - sz2, _1 - xx2 - zz2,      yz2 + sx2,
                       xz2 + sy2,      yz2 - sx2, _1 - xx2 - yy2)
    }
    
    #[inline(always)]
    pure fn to_mat4(&self) -> Mat4<T> {
        self.to_mat3().to_mat4()
    }
    
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Quat<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T:Copy Float> Quat<T>: Neg<Quat<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Quat<T> {
        Quat::new(-self[0], -self[1], -self[2], -self[3])
    }
}

pub impl<T:Copy Eq> Quat<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Quat<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2] &&
        self[3] == other[3]
    }
    
    #[inline(always)]
    pure fn ne(&self, other: &Quat<T>) -> bool {
        !(self == other)
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
