use core::cast::transmute;
use core::cmp::{Eq, Ord};
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::util::swap;
use core::vec::raw::buf_as_slice;

use std::cmp::{FuzzyEq, FUZZY_EPSILON};
use numeric::*;
use numeric::number::Number;
use numeric::number::Number::{zero,one};

use vec::{
    Vec3,
    Vector,
    Vector2,
    Vector3,
    MutableVector,
    NumericVector,
    NumericVector2,
    MutableNumericVector,
    ToHomogeneous,
    EuclideanVector,
    MutableEuclideanVector,
    EquableVector,
    OrdinalVector,
    BooleanVector,
};

/**
 * A 2-dimensional vector
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
 */
 #[deriving_eq]
pub struct Vec2<T> { x: T, y: T }

impl<T:Copy + Eq> Vector<T> for Vec2<T> {
    #[inline(always)]
    static pure fn from_value(value: T) -> Vec2<T> {
        Vector2::new(value, value)
    }

    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Vec2<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

impl<T> Vector2<T> for Vec2<T> {
    #[inline(always)]
    static pure fn new(x: T, y: T ) -> Vec2<T> {
        Vec2 { x: x, y: y }
    }
}

impl<T:Copy + Eq> Index<uint, T> for Vec2<T> {
    #[inline(always)]
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 2) |slice| { slice[i] } }
    }
}

impl<T:Copy> MutableVector<T> for Vec2<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &self/mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => fail!(fmt!("index out of bounds: expected an index from 0 to 1, but found %u", i))
        }
    }

    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        swap(self.index_mut(a),
             self.index_mut(b));
    }
}

impl<T:Copy + Number + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> NumericVector<T> for Vec2<T> {
    #[inline(always)]
    static pure fn identity() -> Vec2<T> {
        Vector2::new(one::<T>(), one::<T>())
    }

    #[inline(always)]
    static pure fn zero() -> Vec2<T> {
        Vector2::new(zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    pure fn is_zero(&self) -> bool {
        self[0] == zero() &&
        self[1] == zero()
    }

    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Vec2<T> {
        Vector2::new(self[0] * value,
                     self[1] * value)
    }

    #[inline(always)]
    pure fn div_t(&self, value: T) -> Vec2<T> {
        Vector2::new(self[0] / value,
                     self[1] / value)
    }

    #[inline(always)]
    pure fn add_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vector2::new(self[0] + other[0],
                     self[1] + other[1])
    }

    #[inline(always)]
    pure fn sub_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vector2::new(self[0] - other[0],
                     self[1] - other[1])
    }

    #[inline(always)]
    pure fn mul_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vector2::new(self[0] * other[0],
                     self[1] * other[1])
    }

    #[inline(always)]
    pure fn div_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vector2::new(self[0] / other[0],
                     self[1] / other[1])
    }

    #[inline(always)]
    pure fn dot(&self, other: &Vec2<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1]
    }
}

impl<T:Copy + Number + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Neg<Vec2<T>> for Vec2<T> {
    #[inline(always)]
    pure fn neg(&self) -> Vec2<T> {
        Vector2::new(-self[0], -self[1])
    }
}

impl<T:Copy + Number + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> NumericVector2<T> for Vec2<T> {
    #[inline(always)]
    static pure fn unit_x() -> Vec2<T> {
        Vector2::new(one::<T>(), zero::<T>())
    }

    #[inline(always)]
    static pure fn unit_y() -> Vec2<T> {
        Vector2::new(zero::<T>(), one::<T>())
    }

    #[inline(always)]
    pure fn perp_dot(&self, other: &Vec2<T>) ->T {
        (self[0] * other[1]) - (self[1] * other[0])
    }
}

impl<T:Copy + Number + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> MutableNumericVector<&self/T> for Vec2<T> {
    #[inline(always)]
    fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index_mut(0);
        *self.index_mut(1) = -*self.index_mut(1);
    }

    #[inline(always)]
    fn mul_self_t(&mut self, value: &T) {
        *self.index_mut(0) *= (*value);
        *self.index_mut(1) *= (*value);
    }

    #[inline(always)]
    fn div_self_t(&mut self, value: &T) {
        *self.index_mut(0) /= (*value);
        *self.index_mut(1) /= (*value);
    }

    #[inline(always)]
    fn add_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) += other[0];
        *self.index_mut(1) += other[1];
    }

    #[inline(always)]
    fn sub_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) -= other[0];
        *self.index_mut(1) -= other[1];
    }

    #[inline(always)]
    fn mul_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) *= other[0];
        *self.index_mut(1) *= other[1];
    }

    #[inline(always)]
    fn div_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) /= other[0];
        *self.index_mut(1) /= other[1];
    }
}

impl<T:Copy + Number> ToHomogeneous<Vec3<T>> for Vec2<T> {
    #[inline(always)]
    pure fn to_homogeneous(&self) -> Vec3<T> {
        Vector3::new(self.x, self.y, zero())
    }
}

impl<T:Copy + Float + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> EuclideanVector<T> for Vec2<T> {
    #[inline(always)]
    pure fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline(always)]
    pure fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline(always)]
    pure fn distance2(&self, other: &Vec2<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline(always)]
    pure fn distance(&self, other: &Vec2<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline(always)]
    pure fn angle(&self, other: &Vec2<T>) -> T {
        atan2(self.perp_dot(other), self.dot(other))
    }

    #[inline(always)]
    pure fn normalize(&self) -> Vec2<T> {
        self.mul_t(one::<T>()/self.length())
    }

    #[inline(always)]
    pure fn normalize_to(&self, length: T) -> Vec2<T> {
        self.mul_t(length / self.length())
    }

    #[inline(always)]
    pure fn lerp(&self, other: &Vec2<T>, amount: T) -> Vec2<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }
}

impl<T:Copy + Float + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> MutableEuclideanVector<&self/T> for Vec2<T> {
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

    fn lerp_self(&mut self, other: &Vec2<T>, amount: &T) {
        self.add_self_v(&other.sub_v(&*self).mul_t(*amount));
    }
}

impl<T:Copy + Float + FuzzyEq<T>> FuzzyEq<T> for Vec2<T> {
    #[inline(always)]
    pure fn fuzzy_eq(&self, other: &Vec2<T>) -> bool {
        self.fuzzy_eq_eps(other, &Number::from(FUZZY_EPSILON))
    }

    #[inline(always)]
    pure fn fuzzy_eq_eps(&self, other: &Vec2<T>, epsilon: &T) -> bool {
        self[0].fuzzy_eq_eps(&other[0], epsilon) &&
        self[1].fuzzy_eq_eps(&other[1], epsilon)
    }
}

impl<T:Copy + Ord + Eq> OrdinalVector<T, Vec2<bool>> for Vec2<T> {
    #[inline(always)]
    pure fn less_than(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vector2::new(self[0] < other[0],
                     self[1] < other[1])
    }

    #[inline(always)]
    pure fn less_than_equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vector2::new(self[0] <= other[0],
                     self[1] <= other[1])
    }

    #[inline(always)]
    pure fn greater_than(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vector2::new(self[0] > other[0],
                     self[1] > other[1])
    }

    #[inline(always)]
    pure fn greater_than_equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vector2::new(self[0] >= other[0],
                     self[1] >= other[1])
    }
}

impl<T:Copy + Eq> EquableVector<T, Vec2<bool>> for Vec2<T> {
    #[inline(always)]
    pure fn equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vector2::new(self[0] == other[0],
                     self[1] == other[1])
    }

    #[inline(always)]
    pure fn not_equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vector2::new(self[0] != other[0],
                     self[1] != other[1])
    }
}

impl BooleanVector for Vec2<bool> {
    #[inline(always)]
    pure fn any(&self) -> bool {
        self[0] || self[1]
    }

    #[inline(always)]
    pure fn all(&self) -> bool {
        self[0] && self[1]
    }

    #[inline(always)]
    pure fn not(&self) -> Vec2<bool> {
        Vector2::new(!self[0], !self[1])
    }
}

// GLSL-style type aliases, corresponding to Section 4.1.5 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

pub type vec2  = Vec2<f32>;     // a two-component single-precision floating-point vector
pub type dvec2 = Vec2<f64>;     // a two-component double-precision floating-point vector
pub type bvec2 = Vec2<bool>;    // a two-component Boolean vector
pub type ivec2 = Vec2<i32>;     // a two-component signed integer vector
pub type uvec2 = Vec2<u32>;     // a two-component unsigned integer vector

// Static method wrappers for GLSL-style types

impl vec2 {
    #[inline(always)] static pure fn new(x: f32, y: f32) -> vec2 { Vector2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: f32) -> vec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> vec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> vec2 { NumericVector::zero() }

    #[inline(always)] static pure fn unit_x() -> vec2 { NumericVector2::unit_x() }
    #[inline(always)] static pure fn unit_y() -> vec2 { NumericVector2::unit_y() }

    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<vec2>() }
}

impl dvec2 {
    #[inline(always)] static pure fn new(x: f64, y: f64) -> dvec2 { Vector2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec2 { NumericVector::zero() }

    #[inline(always)] static pure fn unit_x() -> dvec2 { NumericVector2::unit_x() }
    #[inline(always)] static pure fn unit_y() -> dvec2 { NumericVector2::unit_y() }

    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dvec2>() }
}

impl bvec2 {
    #[inline(always)] static pure fn new(x: bool, y: bool) -> bvec2 { Vector2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec2 { Vector::from_value(v) }

    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<bvec2>() }
}

impl ivec2 {
    #[inline(always)] static pure fn new(x: i32, y: i32) -> ivec2 { Vector2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> ivec2 { NumericVector::zero() }

    #[inline(always)] static pure fn unit_x() -> ivec2 { NumericVector2::unit_x() }
    #[inline(always)] static pure fn unit_y() -> ivec2 { NumericVector2::unit_y() }

    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<ivec2>() }
}

impl uvec2 {
    #[inline(always)] static pure fn new(x: u32, y: u32) -> uvec2 { Vector2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> uvec2 { NumericVector::zero() }

    #[inline(always)] static pure fn unit_x() -> uvec2 { NumericVector2::unit_x() }
    #[inline(always)] static pure fn unit_y() -> uvec2 { NumericVector2::unit_y() }

    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<uvec2>() }
}

// Type aliases named in a more 'Rustic' style

pub type vec2i   = Vec2<int>;
pub type vec2i8  = Vec2<i8>;
pub type vec2i16 = Vec2<i16>;
pub type vec2i32 = Vec2<i32>;
pub type vec2i64 = Vec2<i64>;
pub type vec2f   = Vec2<float>;
pub type vec2f32 = Vec2<f32>;
pub type vec2f64 = Vec2<f64>;
pub type vec2b   = Vec2<bool>;
