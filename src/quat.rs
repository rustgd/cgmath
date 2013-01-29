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
use numeric::*;
use numeric::number::Number;
use numeric::number::Number::{zero,one};

use mat::{Mat3, Mat4};

use vec::{
    Vec3,
    vec3,
    dvec3,
};

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
#[deriving_eq]
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
        Quat::from_sv(w, Vec3::new(xi, yj, zk))
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
        Quat { s: s, v: v }
    }
    
    /**
     * # Return value
     *
     * The multiplicative identity, ie: `q = 1 + 0i + 0j + 0i`
     */
    #[inline(always)]
    static pure fn identity() -> Quat<T> {
        Quat::new(one(), zero(), zero(), zero())
    }
    
    /**
     * # Return value
     *
     * The additive identity, ie: `q = 0 + 0i + 0j + 0i`
     */
    #[inline(always)]
    static pure fn zero() -> Quat<T> {
        Quat::new(zero(), zero(), zero(), zero())
    }
    
    /**
     * # Return value
     *
     * The result of multiplying the quaternion a scalar
     */
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Quat<T> {
        Quat::new(self[0] * value,
                  self[1] * value,
                  self[2] * value,
                  self[3] * value)
    }
    
    /**
     * # Return value
     *
     * The result of dividing the quaternion a scalar
     */
    #[inline(always)]
    pure fn div_t(&self, value: T) -> Quat<T> {
        Quat::new(self[0] / value,
                  self[1] / value,
                  self[2] / value,
                  self[3] / value)
    }
    
    /**
     * # Return value
     *
     * The result of multiplying the quaternion by a vector
     */
    #[inline(always)]
    pure fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T>  {
        let tmp = self.v.cross(vec).add_v(&vec.mul_t(self.s));
        self.v.cross(&tmp).mul_t(Number::from(2)).add_v(vec)
    }
    
    /**
     * # Return value
     *
     * The sum of this quaternion and `other` 
     */
    #[inline(always)]
    pure fn add_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(self[0] + other[0],
                  self[1] + other[1],
                  self[2] + other[2],
                  self[3] + other[3])
    }
    
    /**
     * # Return value
     *
     * The sum of this quaternion and `other` 
     */
    #[inline(always)]
    pure fn sub_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(self[0] - other[0],
                  self[1] - other[1],
                  self[2] - other[2],
                  self[3] - other[3])
    }
    
    /**
     * # Return value
     *
     * The the result of multipliplying the quaternion by `other`
     */
    #[inline(always)]
    pure fn mul_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(self.s * other.s   - self.v.x * other.v.x - self.v.y * other.v.y - self.v.z * other.v.z,
                  self.s * other.v.x + self.v.x * other.s   + self.v.y * other.v.z - self.v.z * other.v.y, 
                  self.s * other.v.y + self.v.y * other.s   + self.v.z * other.v.x - self.v.x * other.v.z, 
                  self.s * other.v.z + self.v.z * other.s   + self.v.x * other.v.y - self.v.y * other.v.x) 
    }
    
    /**
     * # Return value
     *
     * The dot product of the quaternion and `other`
     */
    #[inline(always)]
    pure fn dot(&self, other: &Quat<T>) -> T {
        self.s * other.s + self.v.dot(&other.v)
    }
    
    /**
     * # Return value
     *
     * The conjugate of the quaternion
     */
    #[inline(always)]
    pure fn conjugate(&self) -> Quat<T> {
        Quat::from_sv(self.s, -self.v)
    }
    
    /**
     * # Return value
     *
     * The multiplicative inverse of the quaternion
     */
    #[inline(always)]
    pure fn inverse(&self) -> Quat<T> {
        self.conjugate().div_t(self.magnitude2())
    }
    
    /**
     * # Return value
     *
     * The squared magnitude of the quaternion. This is useful for
     * magnitude comparisons where the exact magnitude does not need to be
     * calculated.
     */
    #[inline(always)]
    pure fn magnitude2(&self) -> T {
        self.s * self.s + self.v.length2()
    }
    
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
    #[inline(always)]
    pure fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }
    
    /**
     * # Return value
     *
     * The normalized quaternion
     */
    #[inline(always)]
    pure fn normalize(&self) -> Quat<T> {
        self.mul_t(one::<T>()/self.magnitude())
    }
    
    /**
     * Normalised linear interpolation
     *
     * # Return value
     *
     * The intoperlated quaternion
     */
    #[inline(always)]
    pure fn nlerp(&self, other: &Quat<T>, amount: T) -> Quat<T> {
        self.mul_t(one::<T>() - amount).add_q(&other.mul_t(amount)).normalize()
    }
    
    /**
     * Spherical Linear Intoperlation
     *
     * Perform a spherical linear interpolation between the quaternion and
     * `other`. Both quaternions should be normalized first.
     *
     * # Return value
     *
     * The intoperlated quaternion
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
            let robust_dot = dot.clamp(-one::<T>(), one());     // stay within the domain of acos()
            
            let theta_0 = acos(robust_dot);                     // the angle between the quaternions
            let theta = theta_0 * amount;                       // the fraction of theta specified by `amount`
            
            let q = other.sub_q(&self.mul_t(robust_dot))
                         .normalize();
            
            return self.mul_t(cos(theta)).add_q(&q.mul_t(sin(theta)));
        }
    }
    
    /**
     * # Return value
     *
     * A pointer to the first component of the quaternion
     */
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Quat<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
    
    // TODO: Move to Rotation implementation. See: https://github.com/mozilla/rust/issues/4306
    #[inline(always)]
    static pure fn from_angle_x(radians: T) -> Quat<T> {
        let _2 = Number::from(2);
        Quat::new(cos(radians / _2), sin(radians), zero(), zero())
    }
    
    // TODO: Move to Rotation implementation. See: https://github.com/mozilla/rust/issues/4306
    #[inline(always)]
    static pure fn from_angle_y(radians: T) -> Quat<T> {
        let _2 = Number::from(2);
        Quat::new(cos(radians / _2), zero(), sin(radians), zero())
    }
    
    // TODO: Move to Rotation implementation. See: https://github.com/mozilla/rust/issues/4306
    #[inline(always)]
    static pure fn from_angle_z(radians: T) -> Quat<T> {
        let _2 = Number::from(2);
        Quat::new(cos(radians / _2), zero(), zero(), sin(radians))
    }
    
    // TODO: Move to Rotation implementation. See: https://github.com/mozilla/rust/issues/4306
    #[inline(always)]
    static pure fn from_angle_xyz(radians_x: T, radians_y: T, radians_z: T) -> Quat<T> {
        // http://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles#Conversion
        let _2 = Number::from(2);
        let xdiv2 = radians_x / _2;
        let ydiv2 = radians_y / _2;
        let zdiv2 = radians_z / _2;
        Quat::new(cos(zdiv2) * cos(xdiv2) * cos(ydiv2) + sin(zdiv2) * sin(xdiv2) * sin(ydiv2),
                  sin(zdiv2) * cos(xdiv2) * cos(ydiv2) - cos(zdiv2) * sin(xdiv2) * sin(ydiv2),
                  cos(zdiv2) * sin(xdiv2) * cos(ydiv2) + sin(zdiv2) * cos(xdiv2) * sin(ydiv2),
                  cos(zdiv2) * cos(xdiv2) * sin(ydiv2) - sin(zdiv2) * sin(xdiv2) * cos(ydiv2))
    }
    
    // TODO: Move to Rotation implementation. See: https://github.com/mozilla/rust/issues/4306
    #[inline(always)]
    static pure fn from_angle_axis(radians: T, axis: &Vec3<T>) -> Quat<T> {
        let half = radians / Number::from(2);
        Quat::from_sv(cos(half), axis.mul_t(sin(half)))
    }
    
    // TODO: Move to Rotation implementation. See: https://github.com/mozilla/rust/issues/4306
    #[inline(always)]
    static pure fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Quat<T> {
        Mat3::from_axes(x, y, z).to_quat()
    }
    
    pure fn get_angle_axis(&self) -> (T, Vec3<T>) {
        fail(~"Not yet implemented.")
    }
    
    // TODO: Move to Rotation implementation. See: https://github.com/mozilla/rust/issues/4306
    #[inline(always)]
    static pure fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> Quat<T> {
        Mat3::look_at(dir, up).to_quat()
    }
    
    // TODO: Move to Rotation implementation. See: https://github.com/mozilla/rust/issues/4306
    #[inline(always)]
    pure fn concat(&self, other: &Quat<T>) -> Quat<T> { self.mul_q(other) }
    
    // TODO: Move to Rotation implementation. See: https://github.com/mozilla/rust/issues/4306
    #[inline(always)]
    pure fn rotate_vec(&self, vec: &Vec3<T>) -> Vec3<T> { self.mul_v(vec) }
    
    /**
     * Convert the quaternion to a 3 x 3 rotation matrix
     */
    // TODO: Move to Rotation implementation. See: https://github.com/mozilla/rust/issues/4306
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
        
        let _1: T = one();
        
        Mat3::new(_1 - yy2 - zz2,      xy2 + sz2,      xz2 - sy2,
                       xy2 - sz2, _1 - xx2 - zz2,      yz2 + sx2,
                       xz2 + sy2,      yz2 - sx2, _1 - xx2 - yy2)
    }
    
    // TODO: Move to Rotation implementation. See: https://github.com/mozilla/rust/issues/4306
    #[inline(always)]
    pure fn to_quat(&self) -> Quat<T> { *self }
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

pub impl<T:Copy Float> Quat<T>: Neg<Quat<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Quat<T> {
        Quat::new(-self[0], -self[1], -self[2], -self[3])
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

// GLSL-style type aliases for quaternions. These are not present in the GLSL
// specification, but they roughly follow the same nomenclature.

pub type quat  = Quat<f32>;             /// a single-precision floating-point quaternion
pub type dquat = Quat<f64>;             /// a double-precision floating-point quaternion

// Static method wrappers for GLSL-style types

pub impl quat {
    #[inline(always)] static pure fn new(w: f32, xi: f32, yj: f32, zk: f32) -> quat { Quat::new(w, xi, yj, zk) }
    #[inline(always)] static pure fn from_sv(s: f32, v: vec3) -> quat { Quat::from_sv(s, v) }
    #[inline(always)] static pure fn identity() -> quat { Quat::identity() }
    #[inline(always)] static pure fn zero() -> quat { Quat::zero() }
    
    #[inline(always)] static pure fn from_angle_x(radians: f32) -> quat { Quat::from_angle_x(radians) }
    #[inline(always)] static pure fn from_angle_y(radians: f32) -> quat { Quat::from_angle_y(radians) }
    #[inline(always)] static pure fn from_angle_z(radians: f32) -> quat { Quat::from_angle_z(radians) }
    #[inline(always)] static pure fn from_angle_xyz(radians_x: f32, radians_y: f32, radians_z: f32)
        -> quat { Quat::from_angle_xyz(radians_x, radians_y, radians_z) }
    #[inline(always)] static pure fn from_angle_axis(radians: f32, axis: &vec3) -> quat { Quat::from_angle_axis(radians, axis) }
    #[inline(always)] static pure fn from_axes(x: vec3, y: vec3, z: vec3) -> quat { Quat::from_axes(x, y, z) }
    #[inline(always)] static pure fn look_at(dir: &vec3, up: &vec3) -> quat { Quat::look_at(dir, up) }
}

pub impl dquat {
    #[inline(always)] static pure fn new(w: f64, xi: f64, yj: f64, zk: f64) -> dquat { Quat::new(w, xi, yj, zk) }
    #[inline(always)] static pure fn from_sv(s: f64, v: dvec3) -> dquat { Quat::from_sv(s, v) }
    #[inline(always)] static pure fn identity() -> dquat { Quat::identity() }
    #[inline(always)] static pure fn zero() -> dquat { Quat::zero() }
    
    #[inline(always)] static pure fn from_angle_x(radians: f64) -> dquat { Quat::from_angle_x(radians) }
    #[inline(always)] static pure fn from_angle_y(radians: f64) -> dquat { Quat::from_angle_y(radians) }
    #[inline(always)] static pure fn from_angle_z(radians: f64) -> dquat { Quat::from_angle_z(radians) }
    #[inline(always)] static pure fn from_angle_xyz(radians_x: f64, radians_y: f64, radians_z: f64)
        -> dquat { Quat::from_angle_xyz(radians_x, radians_y, radians_z) }
    #[inline(always)] static pure fn from_angle_axis(radians: f64, axis: &dvec3) -> dquat { Quat::from_angle_axis(radians, axis) }
    #[inline(always)] static pure fn from_axes(x: dvec3, y: dvec3, z: dvec3) -> dquat { Quat::from_axes(x, y, z) }
    #[inline(always)] static pure fn look_at(dir: &dvec3, up: &dvec3) -> dquat { Quat::look_at(dir, up) }
}