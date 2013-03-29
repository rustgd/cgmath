use core::cast::transmute;
use core::cmp::{Eq, Ord};
use core::sys::size_of;
use core::vec::raw::buf_as_slice;

use std::cmp::{FuzzyEq, FUZZY_EPSILON};
use numeric::*;
use numeric::number::Number;
use numeric::number::Number::{zero,one};

use vec::{
    Vector,
    Vector4,
    MutableVector,
    NumericVector,
    NumericVector4,
    MutableNumericVector,
    ToHomogeneous,
    EuclideanVector,
    MutableEuclideanVector,
    EquableVector,
    OrdinalVector,
    BooleanVector,
};

/**
 * A 4-dimensional vector
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
 * * `w` - the fourth component of the vector
 */
#[deriving(Eq)]
pub struct Vec4<T> { x: T, y: T, z: T, w: T }

impl<T:Copy + Eq> Vector<T> for Vec4<T> {
    #[inline(always)]
    fn from_value(value: T) -> Vec4<T> {
        Vector4::new(value, value, value, value)
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { transmute(self) }
    }
}

impl<T> Vector4<T> for Vec4<T> {
    #[inline(always)]
    fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 { x: x, y: y, z: z, w: w }
    }
}

impl<T:Copy + Eq> Index<uint, T> for Vec4<T> {
    #[inline(always)]
    fn index(&self, i: &uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 4) |slice| { slice[*i] } }
    }
}

impl<T:Copy> MutableVector<T> for Vec4<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &'self mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => fail!(fmt!("index out of bounds: expected an index from 0 to 3, but found %u", i))
        }
    }

    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        *self.index_mut(a) <-> *self.index_mut(b);
    }
}

impl<T:Copy + Number + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> NumericVector<T> for Vec4<T> {
    #[inline(always)]
    fn identity() -> Vec4<T> {
        Vector4::new(one::<T>(), one::<T>(), one::<T>(), one::<T>())
    }

    #[inline(always)]
    fn zero() -> Vec4<T> {
        Vector4::new(zero::<T>(), zero::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self[0] == zero() &&
        self[1] == zero() &&
        self[2] == zero() &&
        self[3] == zero()
    }

    #[inline(always)]
    fn mul_t(&self, value: T) -> Vec4<T> {
        Vector4::new(self[0] * value,
                     self[1] * value,
                     self[2] * value,
                     self[3] * value)
    }

    #[inline(always)]
    fn div_t(&self, value: T) -> Vec4<T> {
        Vector4::new(self[0] / value,
                     self[1] / value,
                     self[2] / value,
                     self[3] / value)
    }

    #[inline(always)]
    fn add_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vector4::new(self[0] + other[0],
                     self[1] + other[1],
                     self[2] + other[2],
                     self[3] + other[3])
    }

    #[inline(always)]
    fn sub_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vector4::new(self[0] - other[0],
                     self[1] - other[1],
                     self[2] - other[2],
                     self[3] - other[3])
    }

    #[inline(always)]
    fn mul_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vector4::new(self[0] * other[0],
                     self[1] * other[1],
                     self[2] * other[2],
                     self[3] * other[3])
    }

    #[inline(always)]
    fn div_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vector4::new(self[0] / other[0],
                     self[1] / other[1],
                     self[2] / other[2],
                     self[3] / other[3])
    }

    #[inline(always)]
    fn dot(&self, other: &Vec4<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2] +
        self[3] * other[3]
    }
}

impl<T:Copy + Number + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Neg<Vec4<T>> for Vec4<T> {
    #[inline(always)]
    fn neg(&self) -> Vec4<T> {
        Vector4::new(-self[0], -self[1], -self[2], -self[3])
    }
}

impl<T:Copy + Number> NumericVector4<T> for Vec4<T> {
    #[inline(always)]
    fn unit_x() -> Vec4<T> {
        Vector4::new(one::<T>(), zero::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_y() -> Vec4<T> {
        Vector4::new(zero::<T>(), one::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_z() -> Vec4<T> {
        Vector4::new(zero::<T>(), zero::<T>(), one::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_w() -> Vec4<T> {
        Vector4::new(zero::<T>(), zero::<T>(), zero::<T>(), one::<T>())
    }
}

impl<'self,T:Copy + Number + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> MutableNumericVector<T> for Vec4<T> {
    #[inline(always)]
    fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index_mut(0);
        *self.index_mut(1) = -*self.index_mut(1);
        *self.index_mut(2) = -*self.index_mut(2);
        *self.index_mut(3) = -*self.index_mut(3);
    }

    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        *self.index_mut(0) *= value;
        *self.index_mut(1) *= value;
        *self.index_mut(2) *= value;
        *self.index_mut(3) *= value;
    }

    #[inline(always)]
    fn div_self_t(&mut self, value: T) {
        *self.index_mut(0) /= value;
        *self.index_mut(1) /= value;
        *self.index_mut(2) /= value;
        *self.index_mut(3) /= value;
    }

    #[inline(always)]
    fn add_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) += other[0];
        *self.index_mut(1) += other[1];
        *self.index_mut(2) += other[2];
        *self.index_mut(3) += other[3];
    }

    #[inline(always)]
    fn sub_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) -= other[0];
        *self.index_mut(1) -= other[1];
        *self.index_mut(2) -= other[2];
        *self.index_mut(3) -= other[3];
    }

    #[inline(always)]
    fn mul_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) *= other[0];
        *self.index_mut(1) *= other[1];
        *self.index_mut(2) *= other[2];
        *self.index_mut(3) *= other[3];
    }

    #[inline(always)]
    fn div_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) /= other[0];
        *self.index_mut(1) /= other[1];
        *self.index_mut(2) /= other[2];
        *self.index_mut(3) /= other[3];
    }
}

impl<T:Copy + Float + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> EuclideanVector<T> for Vec4<T> {
    #[inline(always)]
    fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline(always)]
    fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline(always)]
    fn distance2(&self, other: &Vec4<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline(always)]
    fn distance(&self, other: &Vec4<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline(always)]
    fn angle(&self, other: &Vec4<T>) -> T {
        acos(self.dot(other) / (self.length() * other.length()))
    }

    #[inline(always)]
    fn normalize(&self) -> Vec4<T> {
        self.mul_t(one::<T>()/self.length())
    }

    #[inline(always)]
    fn normalize_to(&self, length: T) -> Vec4<T> {
        self.mul_t(length / self.length())
    }

    #[inline(always)]
    fn lerp(&self, other: &Vec4<T>, amount: T) -> Vec4<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }
}

impl<T: Copy + Float + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> MutableEuclideanVector<T> for Vec4<T> {
    #[inline(always)]
    fn normalize_self(&mut self) {
        let n = one::<T>() / self.length();
        self.mul_self_t(n);
    }

    #[inline(always)]
    fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    fn lerp_self(&mut self, other: &Vec4<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Copy + Float + FuzzyEq<T>> FuzzyEq<T> for Vec4<T> {
    #[inline(always)]
    fn fuzzy_eq(&self, other: &Vec4<T>) -> bool {
        self.fuzzy_eq_eps(other, &Number::from(FUZZY_EPSILON))
    }

    #[inline(always)]
    fn fuzzy_eq_eps(&self, other: &Vec4<T>, epsilon: &T) -> bool {
        self[0].fuzzy_eq_eps(&other[0], epsilon) &&
        self[1].fuzzy_eq_eps(&other[1], epsilon) &&
        self[2].fuzzy_eq_eps(&other[2], epsilon) &&
        self[3].fuzzy_eq_eps(&other[3], epsilon)
    }
}

impl<T:Copy + Ord + Eq> OrdinalVector<T, Vec4<bool>> for Vec4<T> {
    #[inline(always)]
    fn less_than(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] < other[0],
                     self[1] < other[1],
                     self[2] < other[2],
                     self[3] < other[3])
    }

    #[inline(always)]
    fn less_than_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] <= other[0],
                     self[1] <= other[1],
                     self[2] <= other[2],
                     self[3] <= other[3])
    }

    #[inline(always)]
    fn greater_than(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] > other[0],
                     self[1] > other[1],
                     self[2] > other[2],
                     self[3] > other[3])
    }

    #[inline(always)]
    fn greater_than_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] >= other[0],
                     self[1] >= other[1],
                     self[2] >= other[2],
                     self[3] >= other[3])
    }
}

impl<T:Copy + Eq> EquableVector<T, Vec4<bool>> for Vec4<T> {
    #[inline(always)]
    fn equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] == other[0],
                     self[1] == other[1],
                     self[2] == other[2],
                     self[3] == other[3])
    }

    #[inline(always)]
    fn not_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] != other[0],
                     self[1] != other[1],
                     self[2] != other[2],
                     self[3] != other[3])
    }
}

impl BooleanVector for Vec4<bool> {
    #[inline(always)]
    fn any(&self) -> bool {
        self[0] || self[1] || self[2] || self[3]
    }

    #[inline(always)]
    fn all(&self) -> bool {
        self[0] && self[1] && self[2] && self[3]
    }

    #[inline(always)]
    fn not(&self) -> Vec4<bool> {
        Vector4::new(!self[0], !self[1], !self[2], !self[3])
    }
}

// GLSL-style type aliases, corresponding to Section 4.1.5 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

pub type vec4  = Vec4<f32>;     // a four-component single-precision floating-point vector
pub type dvec4 = Vec4<f64>;     // a four-component double-precision floating-point vector
pub type bvec4 = Vec4<bool>;    // a four-component Boolean vector
pub type ivec4 = Vec4<i32>;     // a four-component signed integer vector
pub type uvec4 = Vec4<u32>;     // a four-component unsigned integer vector

// Static method wrappers for GLSL-style types

pub impl vec4 {
    #[inline(always)] fn new(x: f32, y: f32, z: f32, w: f32) -> vec4 { Vector4::new(x, y, z, w) }
    #[inline(always)] fn from_value(v: f32) -> vec4 { Vector::from_value(v) }
    #[inline(always)] fn identity() -> vec4 { NumericVector::identity() }
    #[inline(always)] fn zero() -> vec4 { NumericVector::zero() }

    #[inline(always)] fn unit_x() -> vec4 { NumericVector4::unit_x() }
    #[inline(always)] fn unit_y() -> vec4 { NumericVector4::unit_y() }
    #[inline(always)] fn unit_z() -> vec4 { NumericVector4::unit_z() }
    #[inline(always)] fn unit_w() -> vec4 { NumericVector4::unit_w() }

    #[inline(always)] fn dim() -> uint { 4 }
    #[inline(always)] fn size_of() -> uint { size_of::<vec4>() }
}

pub impl dvec4 {
    #[inline(always)] fn new(x: f64, y: f64, z: f64, w: f64) -> dvec4 { Vector4::new(x, y, z, w) }
    #[inline(always)] fn from_value(v: f64) -> dvec4 { Vector::from_value(v) }
    #[inline(always)] fn identity() -> dvec4 { NumericVector::identity() }
    #[inline(always)] fn zero() -> dvec4 { NumericVector::zero() }

    #[inline(always)] fn unit_x() -> dvec4 { NumericVector4::unit_x() }
    #[inline(always)] fn unit_y() -> dvec4 { NumericVector4::unit_y() }
    #[inline(always)] fn unit_z() -> dvec4 { NumericVector4::unit_z() }
    #[inline(always)] fn unit_w() -> dvec4 { NumericVector4::unit_w() }

    #[inline(always)] fn dim() -> uint { 4 }
    #[inline(always)] fn size_of() -> uint { size_of::<dvec4>() }
}


pub impl bvec4 {
    #[inline(always)] fn new(x: bool, y: bool, z: bool, w: bool) -> bvec4 { Vector4::new(x, y, z, w) }
    #[inline(always)] fn from_value(v: bool) -> bvec4 { Vector::from_value(v) }

    #[inline(always)] fn dim() -> uint { 4 }
    #[inline(always)] fn size_of() -> uint { size_of::<bvec4>() }
}

pub impl ivec4 {
    #[inline(always)] fn new(x: i32, y: i32, z: i32, w: i32) -> ivec4 { Vector4::new(x, y, z, w) }
    #[inline(always)] fn from_value(v: i32) -> ivec4 { Vector::from_value(v) }
    #[inline(always)] fn identity() -> ivec4 { NumericVector::identity() }
    #[inline(always)] fn zero() -> ivec4 { NumericVector::zero() }

    #[inline(always)] fn unit_x() -> ivec4 { NumericVector4::unit_x() }
    #[inline(always)] fn unit_y() -> ivec4 { NumericVector4::unit_y() }
    #[inline(always)] fn unit_z() -> ivec4 { NumericVector4::unit_z() }
    #[inline(always)] fn unit_w() -> ivec4 { NumericVector4::unit_w() }

    #[inline(always)] fn dim() -> uint { 4 }
    #[inline(always)] fn size_of() -> uint { size_of::<ivec4>() }
}

pub impl uvec4 {
    #[inline(always)] fn new(x: u32, y: u32, z: u32, w: u32) -> uvec4 { Vector4::new(x, y, z, w) }
    #[inline(always)] fn from_value(v: u32) -> uvec4 { Vector::from_value(v) }
    #[inline(always)] fn identity() -> uvec4 { NumericVector::identity() }
    #[inline(always)] fn zero() -> uvec4 { NumericVector::zero() }

    #[inline(always)] fn unit_x() -> uvec4 { NumericVector4::unit_x() }
    #[inline(always)] fn unit_y() -> uvec4 { NumericVector4::unit_y() }
    #[inline(always)] fn unit_z() -> uvec4 { NumericVector4::unit_z() }
    #[inline(always)] fn unit_w() -> uvec4 { NumericVector4::unit_w() }

    #[inline(always)] fn dim() -> uint { 4 }
    #[inline(always)] fn size_of() -> uint { size_of::<uvec4>() }
}
