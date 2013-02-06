use core::cast::transmute;
use core::cmp::{Eq, Ord};
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::util::swap;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;
use numeric::*;
use numeric::number::Number::{zero,one};

use vec::{
    Vec4,
    Vector,
    Vector3,
    Vector4,
    MutableVector,
    NumericVector,
    NumericVector3,
    MutableNumericVector,
    MutableNumericVector3,
    ToHomogeneous,
    EuclideanVector,
    MutableEuclideanVector,
    EquableVector,
    OrdinalVector,
    BooleanVector,
};

/**
 * A 3-dimensional vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 *
 * # Fields
 *
 * * `x` - the first component of the vector
 * * `y` - the second component of the vector
 * * `z` - the third component of the vector
 */
#[deriving_eq]
pub struct Vec3<T> { x: T, y: T, z: T }

pub impl<T:Copy Eq> Vec3<T>: Vector<T> {
    #[inline(always)]
    static pure fn from_value(value: T) -> Vec3<T> {
        Vector3::new(value, value, value)
    }
    
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Vec3<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T> Vec3<T>: Vector3<T> {
    #[inline(always)]
    static pure fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x: x, y: y, z: z }
    }
}

pub impl<T:Copy Eq> Vec3<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 3) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> Vec3<T>: MutableVector<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &self/mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => fail!(fmt!("index out of bounds: expected an index from 0 to 2, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        swap(self.index_mut(a),
             self.index_mut(b));
    }
}

pub impl<T:Copy Number> Vec3<T>: NumericVector<T> {
    #[inline(always)]
    static pure fn identity() -> Vec3<T> {
        Vector3::new(one::<T>(), one::<T>(), one::<T>())
    }
    
    #[inline(always)]
    static pure fn zero() -> Vec3<T> {
        Vector3::new(zero::<T>(), zero::<T>(), zero::<T>())
    }
    
    #[inline(always)]
    pure fn is_zero(&self) -> bool {
        self[0] == zero() &&
        self[1] == zero() &&
        self[2] == zero()
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Vec3<T> {
        Vector3::new(self[0] * value,
                     self[1] * value,
                     self[2] * value)
    }
    
    #[inline(always)]
    pure fn div_t(&self, value: T) -> Vec3<T> {
        Vector3::new(self[0] / value,
                     self[1] / value,
                     self[2] / value)
    }
    
    #[inline(always)]
    pure fn add_v(&self, other: &Vec3<T>) -> Vec3<T>{
        Vector3::new(self[0] + other[0],
                     self[1] + other[1],
                     self[2] + other[2])
    }
    
    #[inline(always)]
    pure fn sub_v(&self, other: &Vec3<T>) -> Vec3<T>{
        Vector3::new(self[0] - other[0],
                     self[1] - other[1],
                     self[2] - other[2])
    }
    
    #[inline(always)]
    pure fn mul_v(&self, other: &Vec3<T>) -> Vec3<T>{
        Vector3::new(self[0] * other[0],
                     self[1] * other[1],
                     self[2] * other[2])
    }
    
    #[inline(always)]
    pure fn div_v(&self, other: &Vec3<T>) -> Vec3<T>{
        Vector3::new(self[0] / other[0],
                     self[1] / other[1],
                     self[2] / other[2])
    }
    
    #[inline(always)]
    pure fn dot(&self, other: &Vec3<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2]
    }
}

pub impl<T:Copy Number> Vec3<T>: Neg<Vec3<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Vec3<T> {
        Vector3::new(-self[0], -self[1], -self[2])
    }
}

pub impl<T:Copy Number> Vec3<T>: NumericVector3<T> {
    #[inline(always)]
    static pure fn unit_x() -> Vec3<T> {
        Vector3::new(one::<T>(), zero::<T>(), zero::<T>())
    }
    
    #[inline(always)]
    static pure fn unit_y() -> Vec3<T> {
        Vector3::new(zero::<T>(), one::<T>(), zero::<T>())
    }
    
    #[inline(always)]
    static pure fn unit_z() -> Vec3<T> {
        Vector3::new(zero::<T>(), zero::<T>(), one::<T>())
    }
    
    #[inline(always)]
    pure fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vector3::new((self[1] * other[2]) - (self[2] * other[1]),
                     (self[2] * other[0]) - (self[0] * other[2]),
                     (self[0] * other[1]) - (self[1] * other[0]))
    }
}

pub impl<T:Copy Number> Vec3<T>: MutableNumericVector<&self/T> {
    #[inline(always)]
    fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index_mut(0);
        *self.index_mut(1) = -*self.index_mut(1);
        *self.index_mut(2) = -*self.index_mut(2);
    }
    
    #[inline(always)]
    fn mul_self_t(&mut self, value: &T) {
        *self.index_mut(0) *= (*value);
        *self.index_mut(1) *= (*value);
        *self.index_mut(2) *= (*value);
    }
    
    #[inline(always)]
    fn div_self_t(&mut self, value: &T) {
        *self.index_mut(0) /= (*value);
        *self.index_mut(1) /= (*value);
        *self.index_mut(2) /= (*value);
    }
    
    #[inline(always)]
    fn add_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) += other[0];
        *self.index_mut(1) += other[1];
        *self.index_mut(2) += other[2];
    }
    
    #[inline(always)]
    fn sub_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) -= other[0];
        *self.index_mut(1) -= other[1];
        *self.index_mut(2) -= other[2];
    }
    
    #[inline(always)]
    fn mul_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) *= other[0];
        *self.index_mut(1) *= other[1];
        *self.index_mut(2) *= other[2];
    }
    
    #[inline(always)]
    fn div_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) /= other[0];
        *self.index_mut(1) /= other[1];
        *self.index_mut(2) /= other[2];
    }
}

pub impl<T:Copy Number> Vec3<T>: MutableNumericVector3<&self/T> {
    #[inline(always)]
    fn cross_self(&mut self, other: &Vec3<T>) {
        *self = self.cross(other);
    }
}

pub impl<T:Copy Number> Vec3<T>: ToHomogeneous<Vec4<T>> {
    #[inline(always)]
    pure fn to_homogeneous(&self) -> Vec4<T> {
        Vector4::new(self.x, self.y, self.z, zero())
    }
}

pub impl<T:Copy Float> Vec3<T>: EuclideanVector<T> {
    #[inline(always)]
    pure fn length2(&self) -> T {
        self.dot(self)
    }
    
    #[inline(always)]
    pure fn length(&self) -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn distance2(&self, other: &Vec3<T>) -> T {
        other.sub_v(self).length2()
    }
    
    #[inline(always)]
    pure fn distance(&self, other: &Vec3<T>) -> T {
        other.distance2(self).sqrt()
    }
    
    #[inline(always)]
    pure fn angle(&self, other: &Vec3<T>) -> T {
        atan2(self.cross(other).length(), self.dot(other))
    }
    
    #[inline(always)]
    pure fn normalize(&self) -> Vec3<T> {
        self.mul_t(one::<T>()/self.length())
    }
    
    #[inline(always)]
    pure fn normalize_to(&self, length: T) -> Vec3<T> {
        self.mul_t(length / self.length())
    }
    
    #[inline(always)]
    pure fn lerp(&self, other: &Vec3<T>, amount: T) -> Vec3<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }
}

pub impl<T:Copy Float> Vec3<T>: MutableEuclideanVector<&self/T> {
    #[inline(always)]
    fn normalize_self(&mut self) {
        let n = one::<T>() / self.length();
        self.mul_self_t(&n);
    }
    
    #[inline(always)]
    fn normalize_self_to(&mut self, length: &T) {
        let n = length / self.length();
        self.mul_self_t(&n);
    }
    
    fn lerp_self(&mut self, other: &Vec3<T>, amount: &T) {
        self.add_self_v(&other.sub_v(&*self).mul_t(*amount));
    }
}

pub impl<T:Copy FuzzyEq Eq> Vec3<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Vec3<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2])
    }
}

pub impl<T:Copy Ord Eq> Vec3<T>: OrdinalVector<T, Vec3<bool>> {
    #[inline(always)]
    pure fn less_than(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vector3::new(self[0] < other[0],
                     self[1] < other[1],
                     self[2] < other[2])
    }
    
    #[inline(always)]
    pure fn less_than_equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vector3::new(self[0] <= other[0],
                     self[1] <= other[1],
                     self[2] <= other[2])
    }
    
    #[inline(always)]
    pure fn greater_than(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vector3::new(self[0] > other[0],
                     self[1] > other[1],
                     self[2] > other[2])
    }
    
    #[inline(always)]
    pure fn greater_than_equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vector3::new(self[0] >= other[0],
                     self[1] >= other[1],
                     self[2] >= other[2])
    }
}

pub impl<T:Copy Eq> Vec3<T>: EquableVector<T, Vec3<bool>> {
    #[inline(always)]
    pure fn equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vector3::new(self[0] == other[0],
                     self[1] == other[1],
                     self[2] == other[2])
    }
    
    #[inline(always)]
    pure fn not_equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vector3::new(self[0] != other[0],
                     self[1] != other[1],
                     self[2] != other[2])
    }
}

pub impl Vec3<bool>: BooleanVector {
    #[inline(always)]
    pure fn any(&self) -> bool {
        self[0] || self[1] || self[2]
    }
    
    #[inline(always)]
    pure fn all(&self) -> bool {
        self[0] && self[1] && self[2]
    }
    
    #[inline(always)]
    pure fn not(&self) -> Vec3<bool> { 
        Vector3::new(!self[0], !self[1], !self[2])
    }
}

// GLSL-style type aliases, corresponding to Section 4.1.5 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

pub type vec3  = Vec3<f32>;     // a three-component single-precision floating-point vector
pub type dvec3 = Vec3<f64>;     // a three-component double-precision floating-point vector
pub type bvec3 = Vec3<bool>;    // a three-component Boolean vector
pub type ivec3 = Vec3<i32>;     // a three-component signed integer vector
pub type uvec3 = Vec3<u32>;     // a three-component unsigned integer vector

// Static method wrappers for GLSL-style types

pub impl vec3 {
    #[inline(always)] static pure fn new(x: f32, y: f32, z: f32) -> vec3 { Vector3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: f32) -> vec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> vec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> vec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> vec3 { NumericVector3::unit_x() }
    #[inline(always)] static pure fn unit_y() -> vec3 { NumericVector3::unit_y() }
    #[inline(always)] static pure fn unit_z() -> vec3 { NumericVector3::unit_z() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<vec3>() }
}

pub impl dvec3 {
    #[inline(always)] static pure fn new(x: f64, y: f64, z: f64) -> dvec3 { Vector3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> dvec3 { NumericVector3::unit_x() }
    #[inline(always)] static pure fn unit_y() -> dvec3 { NumericVector3::unit_y() }
    #[inline(always)] static pure fn unit_z() -> dvec3 { NumericVector3::unit_z() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dvec3>() }
}

pub impl bvec3 {
    #[inline(always)] static pure fn new(x: bool, y: bool, z: bool) -> bvec3 { Vector3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec3 { Vector::from_value(v) }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<bvec3>() }
}

pub impl ivec3 {
    #[inline(always)] static pure fn new(x: i32, y: i32, z: i32) -> ivec3 { Vector3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> ivec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> ivec3 { NumericVector3::unit_x() }
    #[inline(always)] static pure fn unit_y() -> ivec3 { NumericVector3::unit_y() }
    #[inline(always)] static pure fn unit_z() -> ivec3 { NumericVector3::unit_z() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<ivec3>() }
}

pub impl uvec3 {
    #[inline(always)] static pure fn new(x: u32, y: u32, z: u32) -> uvec3 { Vector3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> uvec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> uvec3 { NumericVector3::unit_x() }
    #[inline(always)] static pure fn unit_y() -> uvec3 { NumericVector3::unit_y() }
    #[inline(always)] static pure fn unit_z() -> uvec3 { NumericVector3::unit_z() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<uvec3>() }
}