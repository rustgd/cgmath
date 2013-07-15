// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Abstract vector types

use math::{Dimensioned, SwapComponents};

/// Generic vector trait
pub trait Vec<T,Slice>: Dimensioned<T,Slice>
                      + SwapComponents {}

/// Vectors with numeric components
pub trait NumVec<T,Slice>: Neg<T> {
    pub fn add_t(&self, value: T) -> Self;
    pub fn sub_t(&self, value: T) -> Self;
    pub fn mul_t(&self, value: T) -> Self;
    pub fn div_t(&self, value: T) -> Self;
    pub fn rem_t(&self, value: T) -> Self;

    pub fn add_v(&self, other: &Self) -> Self;
    pub fn sub_v(&self, other: &Self) -> Self;
    pub fn mul_v(&self, other: &Self) -> Self;
    pub fn div_v(&self, other: &Self) -> Self;
    pub fn rem_v(&self, other: &Self) -> Self;

    pub fn neg_self(&mut self);
    pub fn add_self_t(&mut self, value: T);
    pub fn sub_self_t(&mut self, value: T);
    pub fn mul_self_t(&mut self, value: T);
    pub fn div_self_t(&mut self, value: T);
    pub fn rem_self_t(&mut self, value: T);

    pub fn add_self_v(&mut self, other: &Self);
    pub fn sub_self_v(&mut self, other: &Self);
    pub fn mul_self_v(&mut self, other: &Self);
    pub fn div_self_v(&mut self, other: &Self);
    pub fn rem_self_v(&mut self, other: &Self);

    pub fn dot(&self, other: &Self) -> T;

    pub fn comp_add(&self) -> T;
    pub fn comp_mul(&self) -> T;
}

/// Vectors with floating point components
pub trait FloatVec<T,Slice>: NumVec<T,Slice> + ApproxEq<T> {
    pub fn magnitude2(&self) -> T;
    pub fn magnitude(&self) -> T;
    pub fn angle(&self, other: &Self) -> T;
    pub fn normalize(&self) -> Self;
    pub fn normalize_to(&self, magnitude: T) -> Self;
    pub fn lerp(&self, other: &Self, amount: T) -> Self;
    pub fn normalize_self(&mut self);
    pub fn normalize_self_to(&mut self, magnitude: T);
    pub fn lerp_self(&mut self, other: &Self, amount: T);
}

/// Vectors with orderable components
pub trait OrdVec<T,Slice,BV>: Vec<T,Slice> {
    pub fn lt_t(&self, value: T) -> BV;
    pub fn le_t(&self, value: T) -> BV;
    pub fn ge_t(&self, value: T) -> BV;
    pub fn gt_t(&self, value: T) -> BV;

    pub fn lt_v(&self, other: &Self) -> BV;
    pub fn le_v(&self, other: &Self) -> BV;
    pub fn ge_v(&self, other: &Self) -> BV;
    pub fn gt_v(&self, other: &Self) -> BV;

    pub fn min_t(&self, other: T) -> Self;
    pub fn max_t(&self, other: T) -> Self;

    pub fn min_v(&self, other: &Self) -> Self;
    pub fn max_v(&self, other: &Self) -> Self;

    pub fn comp_min(&self) -> T;
    pub fn comp_max(&self) -> T;
}

/// Vectors with components that can be tested for equality
pub trait EqVec<T,Slice,BV>: Eq {
    pub fn eq_t(&self, value: T) -> BV;
    pub fn ne_t(&self, value: T) -> BV;
    pub fn eq_v(&self, other: &Self) -> BV;
    pub fn ne_v(&self, other: &Self) -> BV;
}

/// Vectors with boolean components
pub trait BoolVec<Slice>: Vec<bool,Slice> + Not<Self> {
    pub fn any(&self) -> bool;
    pub fn all(&self) -> bool;
}

#[deriving(Clone, Eq)]
pub struct Vec2<T> { x: T, y: T }

// GLSL-style type aliases
pub type vec2  = Vec2<f32>;
pub type dvec2 = Vec2<f64>;
pub type bvec2 = Vec2<bool>;
pub type ivec2 = Vec2<i32>;
pub type uvec2 = Vec2<u32>;

// Rust-style type aliases
pub type Vec2f   = Vec2<float>;
pub type Vec2f32 = Vec2<f32>;
pub type Vec2f64 = Vec2<f64>;
pub type Vec2i   = Vec2<int>;
pub type Vec2i8  = Vec2<i8>;
pub type Vec2i16 = Vec2<i16>;
pub type Vec2i32 = Vec2<i32>;
pub type Vec2i64 = Vec2<i64>;
pub type Vec2u   = Vec2<uint>;
pub type Vec2u8  = Vec2<u8>;
pub type Vec2u16 = Vec2<u16>;
pub type Vec2u32 = Vec2<u32>;
pub type Vec2u64 = Vec2<u64>;
pub type Vec2b   = Vec2<bool>;

pub trait ToVec2<T> {
    pub fn to_vec2(&self) -> Vec2<T>;
}

pub trait AsVec2<T> {
    pub fn as_vec2<'a>(&'a self) -> &'a Vec2<T>;
    pub fn as_mut_vec2<'a>(&'a mut self) -> &'a mut Vec2<T>;
}

impl_dimensioned!(Vec2, T, 2)
impl_swap_components!(Vec2)
impl_approx!(Vec2 { x, y })

impl<T> Vec2<T> {
    /// Construct a new vector from the supplied components.
    #[inline]
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x: x, y: y }
    }
}

impl<T:Clone> Vec2<T> {
    /// Construct a new vector with each component set to `value`.
    #[inline]
    pub fn from_value(value: T) -> Vec2<T> {
        Vec2::new(value.clone(),
                  value.clone())
    }
}

impl<T:Clone + Num> ToVec3<T> for Vec2<T> {
    /// Converts the vector to a three-dimensional homogeneous vector:
    /// `[x, y] -> [x, y, 0]`
    pub fn to_vec3(&self) -> Vec3<T> {
        Vec3::new((*self).index(0).clone(),
                  (*self).index(1).clone(),
                  zero!(T))
    }
}

/// Constants for two-dimensional vectors.
impl<T:Num> Vec2<T> {
    /// Returns a two-dimensional vector with each component set to `1`.
    #[inline]
    pub fn identity() -> Vec2<T> {
        Vec2::new(one!(T), one!(T))
    }

    /// Returns a two-dimensional vector with each component set to `0`.
    #[inline]
    pub fn zero() -> Vec2<T> {
        Vec2::new(zero!(T), zero!(T))
    }

    /// Returns a zeroed two-dimensional vector with the `x` component set to `1`.
    #[inline]
    pub fn unit_x() -> Vec2<T> {
        Vec2::new(one!(T), zero!(T))
    }

    /// Returns a zeroed two-dimensional vector with the `y` component set to `1`.
    #[inline]
    pub fn unit_y() -> Vec2<T> {
        Vec2::new(zero!(T), one!(T))
    }
}

/// Numeric operations specific to two-dimensional vectors.
impl<T:Num> Vec2<T> {
    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(&self, other: &Vec2<T>) -> T {
        (*self.index(0) * *other.index(1)) -
        (*self.index(1) * *other.index(0))
    }
}

impl<T> Vec<T,[T,..2]> for Vec2<T> {}

impl<T:Num> NumVec<T,[T,..2]> for Vec2<T> {
    /// Returns a new vector with `value` added to each component.
    #[inline]
    pub fn add_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) + value,
                  *self.index(1) + value)
    }

    /// Returns a new vector with `value` subtracted from each component.
    #[inline]
    pub fn sub_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) - value,
                  *self.index(1) - value)
    }

    /// Returns the scalar multiplication of the vector with `value`.
    #[inline]
    pub fn mul_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) * value,
                  *self.index(1) * value)
    }

    /// Returns a new vector with each component divided by `value`.
    #[inline]
    pub fn div_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) / value,
                  *self.index(1) / value)
    }

    /// Returns the remainder of each component divided by `value`.
    #[inline]
    pub fn rem_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) % value,
                  *self.index(1) % value)
    }

    /// Returns the sum of the two vectors.
    #[inline]
    pub fn add_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1))
    }

    /// Ruturns the result of subtrating `other` from the vector.
    #[inline]
    pub fn sub_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1))
    }

    /// Returns the component-wise product of the vector and `other`.
    #[inline]
    pub fn mul_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) * *other.index(0),
                  *self.index(1) * *other.index(1))
    }

    /// Returns the component-wise quotient of the vectors.
    #[inline]
    pub fn div_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) / *other.index(0),
                  *self.index(1) / *other.index(1))
    }

    /// Returns the component-wise remainder of the vector divided by `other`.
    #[inline]
    pub fn rem_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) % *other.index(0),
                  *self.index(1) % *other.index(1))
    }

    /// Negates each component of the vector.
    #[inline]
    pub fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index(0);
        *self.index_mut(1) = -*self.index(1);
    }

    /// Adds `value` to each component of the vector.
    #[inline]
    pub fn add_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) + value;
        *self.index_mut(1) = *self.index(1) + value;
    }

    /// Subtracts `value` from each component of the vector.
    #[inline]
    pub fn sub_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) - value;
        *self.index_mut(1) = *self.index(1) - value;
    }

    #[inline]
    pub fn mul_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) * value;
        *self.index_mut(1) = *self.index(1) * value;
    }

    #[inline]
    pub fn div_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) / value;
        *self.index_mut(1) = *self.index(1) / value;
    }

    #[inline]
    pub fn rem_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) % value;
        *self.index_mut(1) = *self.index(1) % value;
    }

    #[inline]
    pub fn add_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) = *self.index(0) + *other.index(0);
        *self.index_mut(1) = *self.index(1) + *other.index(1);
    }

    #[inline]
    pub fn sub_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) = *self.index(0) - *other.index(0);
        *self.index_mut(1) = *self.index(1) - *other.index(1);
    }

    #[inline]
    pub fn mul_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) = *self.index(0) * *other.index(0);
        *self.index_mut(1) = *self.index(1) * *other.index(1);
    }

    #[inline]
    pub fn div_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) = *self.index(0) / *other.index(0);
        *self.index_mut(1) = *self.index(1) / *other.index(1);
    }

    #[inline]
    pub fn rem_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) = *self.index(0) % *other.index(0);
        *self.index_mut(1) = *self.index(1) % *other.index(1);
    }

    /// Returns the dot product of the vector and `other`.
    #[inline]
    pub fn dot(&self, other: &Vec2<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1)
    }

    /// Returns the sum of the vector's components.
    #[inline]
    pub fn comp_add(&self) -> T {
        *self.index(0) + *self.index(1)
    }

    /// Returns the product of the vector's components.
    #[inline]
    pub fn comp_mul(&self) -> T {
        *self.index(0) * *self.index(1)
    }
}

impl<T:Num> Neg<Vec2<T>> for Vec2<T> {
    /// Returns the vector with each component negated.
    #[inline]
    pub fn neg(&self) -> Vec2<T> {
        Vec2::new(-*self.index(0),
                  -*self.index(1))
    }
}

impl<T:Float> FloatVec<T,[T,..2]> for Vec2<T> {
    /// Returns the squared magnitude of the vector. This does not perform a
    /// square root operation like in the `magnitude` method and can therefore
    /// be more efficient for comparing the magnitudes of two vectors.
    #[inline]
    pub fn magnitude2(&self) -> T {
        self.dot(self)
    }

    /// Returns the magnitude (length) of the vector.
    #[inline]
    pub fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }

    /// Returns the angle between the vector and `other`.
    #[inline]
    pub fn angle(&self, other: &Vec2<T>) -> T {
        self.perp_dot(other).atan2(&self.dot(other))
    }

    /// Returns the result of normalizing the vector to a magnitude of `1`.
    #[inline]
    pub fn normalize(&self) -> Vec2<T> {
        self.normalize_to(one!(T))
    }

    /// Returns the result of normalizing the vector to `magnitude`.
    #[inline]
    pub fn normalize_to(&self, magnitude: T) -> Vec2<T> {
        self.mul_t(magnitude / self.magnitude())
    }

    /// Returns the result of linarly interpolating the magnitude of the vector
    /// to the magnitude of `other` by the specified amount.
    #[inline]
    pub fn lerp(&self, other: &Vec2<T>, amount: T) -> Vec2<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    /// Normalises the vector to a magnitude of `1`.
    #[inline]
    pub fn normalize_self(&mut self) {
        let rlen = self.magnitude().recip();
        self.mul_self_t(rlen);
    }

    /// Normalizes the vector to `magnitude`.
    #[inline]
    pub fn normalize_self_to(&mut self, magnitude: T) {
        let n = magnitude / self.magnitude();
        self.mul_self_t(n);
    }

    /// Linearly interpolates the magnitude of the vector to the magnitude of
    /// `other` by the specified amount.
    pub fn lerp_self(&mut self, other: &Vec2<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Orderable> OrdVec<T,[T,..2],Vec2<bool>> for Vec2<T> {
    #[inline]
    pub fn lt_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) < value,
                  *self.index(1) < value)
    }

    #[inline]
    pub fn le_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) <= value,
                  *self.index(1) <= value)
    }

    #[inline]
    pub fn ge_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) >= value,
                  *self.index(1) >= value)
    }

    #[inline]
    pub fn gt_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) > value,
                  *self.index(1) > value)
    }

    #[inline]
    pub fn lt_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) < *other.index(0),
                  *self.index(1) < *other.index(1))
    }

    #[inline]
    pub fn le_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) <= *other.index(0),
                  *self.index(1) <= *other.index(1))
    }

    #[inline]
    pub fn ge_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) >= *other.index(0),
                  *self.index(1) >= *other.index(1))
    }

    #[inline]
    pub fn gt_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) > *other.index(0),
                  *self.index(1) > *other.index(1))
    }

    #[inline]
    pub fn min_t(&self, value: T) -> Vec2<T> {
        Vec2::new(self.index(0).min(&value),
                  self.index(1).min(&value))
    }

    #[inline]
    pub fn max_t(&self, value: T) -> Vec2<T> {
        Vec2::new(self.index(0).max(&value),
                  self.index(1).max(&value))
    }

    #[inline]
    pub fn min_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self.index(0).min(other.index(0)),
                  self.index(1).min(other.index(1)))
    }

    #[inline]
    pub fn max_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self.index(0).max(other.index(0)),
                  self.index(1).max(other.index(1)))
    }

    /// Returns the smallest component of the vector.
    #[inline]
    pub fn comp_min(&self) -> T {
        self.index(0).min(self.index(1))
    }

    /// Returns the largest component of the vector.
    #[inline]
    pub fn comp_max(&self) -> T {
        self.index(0).max(self.index(1))
    }
}

impl<T:Eq> EqVec<T,[T,..2],Vec2<bool>> for Vec2<T> {
    #[inline]
    pub fn eq_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) == value,
                  *self.index(1) == value)
    }

    #[inline]
    pub fn ne_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) != value,
                  *self.index(1) != value)
    }

    #[inline]
    pub fn eq_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) == *other.index(0),
                  *self.index(1) == *other.index(1))
    }

    #[inline]
    pub fn ne_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) != *other.index(0),
                  *self.index(1) != *other.index(1))
    }
}

impl BoolVec<[bool,..2]> for Vec2<bool> {
    /// Returns `true` if any of the components of the vector are equal to
    /// `true`, otherwise `false`.
    #[inline]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1)
    }

    /// Returns `true` if _all_ of the components of the vector are equal to
    /// `true`, otherwise `false`.
    #[inline]
    pub fn all(&self) -> bool {
        *self.index(0) && *self.index(1)
    }
}

impl<T:Not<T>> Not<Vec2<T>> for Vec2<T> {
    pub fn not(&self) -> Vec2<T> {
        Vec2::new(!*self.index(0),
                  !*self.index(1))
    }
}

#[cfg(test)]
mod vec2_tests {
    use math::vec::*;

    static A: Vec2<float> = Vec2 { x: 1.0, y: 2.0 };
    static B: Vec2<float> = Vec2 { x: 3.0, y: 4.0 };
    static F1: float = 1.5;
    static F2: float = 0.5;

    #[test]
    fn test_swap() {
        let mut mut_a = A;

        mut_a.swap(0, 1);
        assert_eq!(*mut_a.index(0), *A.index(1));
        assert_eq!(*mut_a.index(1), *A.index(0));
    }

    #[test]
    fn test_num() {
        let mut mut_a = A;

        assert_eq!(-A, Vec2::new::<float>(-1.0, -2.0));
        assert_eq!(A.neg(), Vec2::new::<float>(-1.0, -2.0));

        assert_eq!(A.mul_t(F1), Vec2::new::<float>(1.5, 3.0));
        assert_eq!(A.div_t(F2), Vec2::new::<float>(2.0, 4.0));

        assert_eq!(A.add_v(&B), Vec2::new::<float>(4.0, 6.0));
        assert_eq!(A.sub_v(&B), Vec2::new::<float>(-2.0, -2.0));
        assert_eq!(A.mul_v(&B), Vec2::new::<float>(3.0, 8.0));
        assert_eq!(A.div_v(&B), Vec2::new::<float>(1.0/3.0, 2.0/4.0));

        mut_a.neg_self();
        assert_eq!(mut_a, -A);
        mut_a = A;

        mut_a.mul_self_t(F1);
        assert_eq!(mut_a, A.mul_t(F1));
        mut_a = A;

        mut_a.div_self_t(F2);
        assert_eq!(mut_a, A.div_t(F2));
        mut_a = A;

        mut_a.add_self_v(&B);
        assert_eq!(mut_a, A.add_v(&B));
        mut_a = A;

        mut_a.sub_self_v(&B);
        assert_eq!(mut_a, A.sub_v(&B));
        mut_a = A;

        mut_a.mul_self_v(&B);
        assert_eq!(mut_a, A.mul_v(&B));
        mut_a = A;

        mut_a.div_self_v(&B);
        assert_eq!(mut_a, A.div_v(&B));
    }

    #[test]
    fn test_comp_add() {
        assert_eq!(A.comp_add(), 3.0);
        assert_eq!(B.comp_add(), 7.0);
    }

    #[test]
    fn test_comp_mul() {
        assert_eq!(A.comp_mul(), 2.0);
        assert_eq!(B.comp_mul(), 12.0);
    }

    #[test]
    fn test_approx_eq() {
        assert!(!Vec2::new::<float>(0.000001, 0.000001).approx_eq(&Vec2::new::<float>(0.0, 0.0)));
        assert!(Vec2::new::<float>(0.0000001, 0.0000001).approx_eq(&Vec2::new::<float>(0.0, 0.0)));
    }

    #[test]
    fn test_magnitude() {
        let a = Vec2::new(5.0, 12.0); // (5, 12, 13) Pythagorean triple
        let b = Vec2::new(3.0, 4.0); // (3, 4, 5) Pythagorean triple

        assert_eq!(a.magnitude(), 13.0);
        assert_eq!(a.magnitude2(), 13.0 * 13.0);

        assert_eq!(b.magnitude(), 5.0);
        assert_eq!(b.magnitude2(), 5.0 * 5.0);
    }

    #[test]
    fn test_angle() {
        assert!(Vec2::new::<float>(1.0, 0.0).angle(&Vec2::new::<float>(0.0, 1.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec2::new::<float>(10.0, 0.0).angle(&Vec2::new::<float>(0.0, 5.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec2::new::<float>(-1.0, 0.0).angle(&Vec2::new::<float>(0.0, 1.0)).approx_eq(&-Real::frac_pi_2::<float>()));
    }

    #[test]
    fn test_normalize() {
        assert!(Vec2::new::<float>(3.0, 4.0).normalize().approx_eq(&Vec2::new::<float>(3.0/5.0, 4.0/5.0)));
        // TODO: test normalize_to, normalize_self, and normalize_self_to
    }

    #[test]
    fn test_lerp() {
        let c = Vec2::new::<float>(-2.0, -1.0);
        let d = Vec2::new::<float>( 1.0,  0.0);

        assert_eq!(c.lerp(&d, 0.75), Vec2::new::<float>(0.250, -0.250));

        let mut mut_c = c;
        mut_c.lerp_self(&d, 0.75);
        assert_eq!(mut_c, c.lerp(&d, 0.75));
    }

    #[test]
    fn test_comp_min() {
        assert_eq!(A.comp_min(), 1.0);
        assert_eq!(B.comp_min(), 3.0);
    }

    #[test]
    fn test_comp_max() {
        assert_eq!(A.comp_max(), 2.0);
        assert_eq!(B.comp_max(), 4.0);
    }

    #[test]
    fn test_boolean() {
        let tf = Vec2::new(true, false);
        let ff = Vec2::new(false, false);
        let tt = Vec2::new(true, true);

        assert_eq!(tf.any(), true);
        assert_eq!(tf.all(), false);
        assert_eq!(!tf, Vec2::new(false, true));

        assert_eq!(ff.any(), false);
        assert_eq!(ff.all(), false);
        assert_eq!(!ff, Vec2::new(true, true));

        assert_eq!(tt.any(), true);
        assert_eq!(tt.all(), true);
        assert_eq!(!tt, Vec2::new(false, false));
    }
}

#[deriving(Clone, Eq)]
pub struct Vec3<T> { x: T, y: T, z: T }

// GLSL-style type aliases
pub type vec3  = Vec3<f32>;
pub type dvec3 = Vec3<f64>;
pub type bvec3 = Vec3<bool>;
pub type ivec3 = Vec3<i32>;
pub type uvec3 = Vec3<u32>;

// Rust-style type aliases
pub type Vec3f   = Vec3<float>;
pub type Vec3f32 = Vec3<f32>;
pub type Vec3f64 = Vec3<f64>;
pub type Vec3i   = Vec3<int>;
pub type Vec3i8  = Vec3<i8>;
pub type Vec3i16 = Vec3<i16>;
pub type Vec3i32 = Vec3<i32>;
pub type Vec3i64 = Vec3<i64>;
pub type Vec3u   = Vec3<uint>;
pub type Vec3u8  = Vec3<u8>;
pub type Vec3u16 = Vec3<u16>;
pub type Vec3u32 = Vec3<u32>;
pub type Vec3u64 = Vec3<u64>;
pub type Vec3b   = Vec3<bool>;

pub trait ToVec3<T> {
    pub fn to_vec3(&self) -> Vec3<T>;
}

pub trait AsVec3<T> {
    pub fn as_vec3<'a>(&'a self) -> &'a Vec3<T>;
    pub fn as_mut_vec3<'a>(&'a mut self) -> &'a mut Vec3<T>;
}

impl_dimensioned!(Vec3, T, 3)
impl_swap_components!(Vec3)
impl_approx!(Vec3 { x, y, z })

impl<T> Vec3<T> {
    /// Construct a new vector from the supplied components.
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x: x, y: y, z: z }
    }
}

impl<T:Clone> Vec3<T> {
    /// Construct a new vector with each component set to `value`.
    #[inline]
    pub fn from_value(value: T) -> Vec3<T> {
        Vec3::new(value.clone(),
                  value.clone(),
                  value.clone())
    }
}

impl<T:Clone + Num> ToVec4<T> for Vec3<T> {
    /// Converts the vector to a four-dimensional homogeneous vector:
    /// `[x, y, z] -> [x, y, z, 0]`
    pub fn to_vec4(&self) -> Vec4<T> {
        Vec4::new((*self).index(0).clone(),
                  (*self).index(1).clone(),
                  (*self).index(2).clone(),
                  zero!(T))
    }
}

/// Constants for three-dimensional vectors.
impl<T:Num> Vec3<T> {
    /// Returns a three-dimensional vector with each component set to `1`.
    #[inline]
    pub fn identity() -> Vec3<T> {
        Vec3::new(one!(T), one!(T), one!(T))
    }

    /// Returns a three-dimensional vector with each component set to `0`.
    #[inline]
    pub fn zero() -> Vec3<T> {
        Vec3::new(zero!(T), zero!(T), zero!(T))
    }

    /// Returns a zeroed three-dimensional vector with the `x` component set to `1`.
    #[inline]
    pub fn unit_x() -> Vec3<T> {
        Vec3::new(one!(T), zero!(T), zero!(T))
    }

    /// Returns a zeroed three-dimensional vector with the `y` component set to `1`.
    #[inline]
    pub fn unit_y() -> Vec3<T> {
        Vec3::new(zero!(T), one!(T), zero!(T))
    }

    /// Returns a zeroed three-dimensional vector with the `z` component set to `1`.
    #[inline]
    pub fn unit_z() -> Vec3<T> {
        Vec3::new(zero!(T), zero!(T), one!(T))
    }
}

/// Numeric operations specific to three-dimensional vectors.
impl<T:Num> Vec3<T> {
    /// Returns the cross product of the vector and `other`.
    #[inline]
    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new((*self.index(1) * *other.index(2)) - (*self.index(2) * *other.index(1)),
                  (*self.index(2) * *other.index(0)) - (*self.index(0) * *other.index(2)),
                  (*self.index(0) * *other.index(1)) - (*self.index(1) * *other.index(0)))
    }

    /// Calculates the cross product of the vector and `other`, then stores the
    /// result in `self`.
    #[inline]
    pub fn cross_self(&mut self, other: &Vec3<T>) {
        *self = self.cross(other)
    }
}

impl<T> Vec<T,[T,..3]> for Vec3<T> {}

impl<T:Num> NumVec<T,[T,..3]> for Vec3<T> {
    /// Returns a new vector with `value` added to each component.
    #[inline]
    pub fn add_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) + value,
                  *self.index(1) + value,
                  *self.index(2) + value)
    }

    /// Returns a new vector with `value` subtracted from each component.
    #[inline]
    pub fn sub_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) - value,
                  *self.index(1) - value,
                  *self.index(2) - value)
    }

    /// Returns the scalar multiplication of the vector with `value`.
    #[inline]
    pub fn mul_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) * value,
                  *self.index(1) * value,
                  *self.index(2) * value)
    }

    /// Returns a new vector with each component divided by `value`.
    #[inline]
    pub fn div_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) / value,
                  *self.index(1) / value,
                  *self.index(2) / value)
    }

    /// Returns the remainder of each component divided by `value`.
    #[inline]
    pub fn rem_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) % value,
                  *self.index(1) % value,
                  *self.index(2) % value)
    }

    /// Returns the sum of the two vectors.
    #[inline]
    pub fn add_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1),
                  *self.index(2) + *other.index(2))
    }

    /// Ruturns the result of subtrating `other` from the vector.
    #[inline]
    pub fn sub_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1),
                  *self.index(2) - *other.index(2))
    }

    /// Returns the component-wise product of the vector and `other`.
    #[inline]
    pub fn mul_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) * *other.index(0),
                  *self.index(1) * *other.index(1),
                  *self.index(2) * *other.index(2))
    }

    /// Returns the component-wise quotient of the vectors.
    #[inline]
    pub fn div_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) / *other.index(0),
                  *self.index(1) / *other.index(1),
                  *self.index(2) / *other.index(2))
    }

    /// Returns the component-wise remainder of the vector divided by `other`.
    #[inline]
    pub fn rem_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) % *other.index(0),
                  *self.index(1) % *other.index(1),
                  *self.index(2) % *other.index(2))
    }

    /// Negates each component of the vector.
    #[inline]
    pub fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index(0);
        *self.index_mut(1) = -*self.index(1);
        *self.index_mut(2) = -*self.index(2);
    }

    /// Adds `value` to each component of the vector.
    #[inline]
    pub fn add_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) + value;
        *self.index_mut(1) = *self.index(1) + value;
        *self.index_mut(2) = *self.index(2) + value;
    }

    /// Subtracts `value` from each component of the vector.
    #[inline]
    pub fn sub_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) - value;
        *self.index_mut(1) = *self.index(1) - value;
        *self.index_mut(2) = *self.index(2) - value;
    }

    #[inline]
    pub fn mul_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) * value;
        *self.index_mut(1) = *self.index(1) * value;
        *self.index_mut(2) = *self.index(2) * value;
    }

    #[inline]
    pub fn div_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) / value;
        *self.index_mut(1) = *self.index(1) / value;
        *self.index_mut(2) = *self.index(2) / value;
    }

    #[inline]
    pub fn rem_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) % value;
        *self.index_mut(1) = *self.index(1) % value;
        *self.index_mut(2) = *self.index(2) % value;
    }

    #[inline]
    pub fn add_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) = *self.index(0) + *other.index(0);
        *self.index_mut(1) = *self.index(1) + *other.index(1);
        *self.index_mut(2) = *self.index(2) + *other.index(2);
    }

    #[inline]
    pub fn sub_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) = *self.index(0) - *other.index(0);
        *self.index_mut(1) = *self.index(1) - *other.index(1);
        *self.index_mut(2) = *self.index(2) - *other.index(2);
    }

    #[inline]
    pub fn mul_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) = *self.index(0) * *other.index(0);
        *self.index_mut(1) = *self.index(1) * *other.index(1);
        *self.index_mut(2) = *self.index(2) * *other.index(2);
    }

    #[inline]
    pub fn div_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) = *self.index(0) / *other.index(0);
        *self.index_mut(1) = *self.index(1) / *other.index(1);
        *self.index_mut(2) = *self.index(2) / *other.index(2);
    }

    #[inline]
    pub fn rem_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) = *self.index(0) % *other.index(0);
        *self.index_mut(1) = *self.index(1) % *other.index(1);
        *self.index_mut(2) = *self.index(2) % *other.index(2);
    }

    /// Returns the dot product of the vector and `other`.
    #[inline]
    pub fn dot(&self, other: &Vec3<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1) +
        *self.index(2) * *other.index(2)
    }

    /// Returns the sum of the vector's components.
    #[inline]
    pub fn comp_add(&self) -> T {
        *self.index(0) + *self.index(1) + *self.index(2)
    }

    /// Returns the product of the vector's components.
    #[inline]
    pub fn comp_mul(&self) -> T {
        *self.index(0) * *self.index(1) * *self.index(2)
    }
}

impl<T:Num> Neg<Vec3<T>> for Vec3<T> {
    /// Returns the vector with each component negated.
    #[inline]
    pub fn neg(&self) -> Vec3<T> {
        Vec3::new(-*self.index(0),
                  -*self.index(1),
                  -*self.index(2))
    }
}

impl<T:Float> FloatVec<T,[T,..3]> for Vec3<T> {
    /// Returns the squared magnitude of the vector. This does not perform a
    /// square root operation like in the `magnitude` method and can therefore
    /// be more efficient for comparing the magnitudes of two vectors.
    #[inline]
    pub fn magnitude2(&self) -> T {
        self.dot(self)
    }

    /// Returns the magnitude (length) of the vector.
    #[inline]
    pub fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }

    /// Returns the angle between the vector and `other`.
    #[inline]
    pub fn angle(&self, other: &Vec3<T>) -> T {
        self.cross(other).magnitude().atan2(&self.dot(other))
    }

    /// Returns the result of normalizing the vector to a magnitude of `1`.
    #[inline]
    pub fn normalize(&self) -> Vec3<T> {
        self.normalize_to(one!(T))
    }

    /// Returns the result of normalizing the vector to `magnitude`.
    #[inline]
    pub fn normalize_to(&self, magnitude: T) -> Vec3<T> {
        self.mul_t(magnitude / self.magnitude())
    }

    /// Returns the result of linarly interpolating the magnitude of the vector
    /// to the magnitude of `other` by the specified amount.
    #[inline]
    pub fn lerp(&self, other: &Vec3<T>, amount: T) -> Vec3<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    /// Normalises the vector to a magnitude of `1`.
    #[inline]
    pub fn normalize_self(&mut self) {
        let rlen = self.magnitude().recip();
        self.mul_self_t(rlen);
    }

    /// Normalizes the vector to `magnitude`.
    #[inline]
    pub fn normalize_self_to(&mut self, magnitude: T) {
        let n = magnitude / self.magnitude();
        self.mul_self_t(n);
    }

    /// Linearly interpolates the magnitude of the vector to the magnitude of
    /// `other` by the specified amount.
    pub fn lerp_self(&mut self, other: &Vec3<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Orderable> OrdVec<T,[T,..3],Vec3<bool>> for Vec3<T> {
    #[inline]
    pub fn lt_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) < value,
                  *self.index(1) < value,
                  *self.index(2) < value)
    }

    #[inline]
    pub fn le_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) <= value,
                  *self.index(1) <= value,
                  *self.index(2) <= value)
    }

    #[inline]
    pub fn ge_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) >= value,
                  *self.index(1) >= value,
                  *self.index(2) >= value)
    }

    #[inline]
    pub fn gt_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) > value,
                  *self.index(1) > value,
                  *self.index(2) > value)
    }

    #[inline]
    pub fn lt_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) < *other.index(0),
                  *self.index(1) < *other.index(1),
                  *self.index(2) < *other.index(2))
    }

    #[inline]
    pub fn le_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) <= *other.index(0),
                  *self.index(1) <= *other.index(1),
                  *self.index(2) <= *other.index(2))
    }

    #[inline]
    pub fn ge_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) >= *other.index(0),
                  *self.index(1) >= *other.index(1),
                  *self.index(2) >= *other.index(2))
    }

    #[inline]
    pub fn gt_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) > *other.index(0),
                  *self.index(1) > *other.index(1),
                  *self.index(2) > *other.index(2))
    }

    #[inline]
    pub fn min_t(&self, value: T) -> Vec3<T> {
        Vec3::new(self.index(0).min(&value),
                  self.index(1).min(&value),
                  self.index(2).min(&value))
    }

    #[inline]
    pub fn max_t(&self, value: T) -> Vec3<T> {
        Vec3::new(self.index(0).max(&value),
                  self.index(1).max(&value),
                  self.index(2).max(&value))
    }

    #[inline]
    pub fn min_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.index(0).min(other.index(0)),
                  self.index(1).min(other.index(1)),
                  self.index(2).min(other.index(2)))
    }

    #[inline]
    pub fn max_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.index(0).max(other.index(0)),
                  self.index(1).max(other.index(1)),
                  self.index(2).max(other.index(2)))
    }

    /// Returns the smallest component of the vector.
    #[inline]
    pub fn comp_min(&self) -> T {
        self.index(0).min(self.index(1)).min(self.index(2))
    }

    /// Returns the largest component of the vector.
    #[inline]
    pub fn comp_max(&self) -> T {
        self.index(0).max(self.index(1)).max(self.index(2))
    }
}

impl<T:Eq> EqVec<T,[T,..3],Vec3<bool>> for Vec3<T> {
    #[inline]
    pub fn eq_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) == value,
                  *self.index(1) == value,
                  *self.index(2) == value)
    }

    #[inline]
    pub fn ne_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) != value,
                  *self.index(1) != value,
                  *self.index(2) != value)
    }

    #[inline]
    pub fn eq_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) == *other.index(0),
                  *self.index(1) == *other.index(1),
                  *self.index(2) == *other.index(2))
    }

    #[inline]
    pub fn ne_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) != *other.index(0),
                  *self.index(1) != *other.index(1),
                  *self.index(2) != *other.index(2))
    }
}

impl BoolVec<[bool,..3]> for Vec3<bool> {
    /// Returns `true` if any of the components of the vector are equal to
    /// `true`, otherwise `false`.
    #[inline]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1) || *self.index(2)
    }

    /// Returns `true` if _all_ of the components of the vector are equal to
    /// `true`, otherwise `false`.
    #[inline]
    pub fn all(&self) -> bool {
        *self.index(0) && *self.index(1) && *self.index(2)
    }
}

impl<T:Not<T>> Not<Vec3<T>> for Vec3<T> {
    pub fn not(&self) -> Vec3<T> {
        Vec3::new(!*self.index(0),
                  !*self.index(1),
                  !*self.index(2))
    }
}

#[cfg(test)]
mod vec3_tests{
    use math::vec::*;

    static A: Vec3<float> = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    static B: Vec3<float> = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    static F1: float = 1.5;
    static F2: float = 0.5;

    #[test]
    fn test_swap() {
        let mut mut_a = A;

        mut_a.swap(0, 2);
        assert_eq!(*mut_a.index(0), *A.index(2));
        assert_eq!(*mut_a.index(2), *A.index(0));
        mut_a = A;

        mut_a.swap(1, 2);
        assert_eq!(*mut_a.index(1), *A.index(2));
        assert_eq!(*mut_a.index(2), *A.index(1));
    }

    #[test]
    fn test_cross() {
        let mut mut_a = A;

        assert_eq!(A.cross(&B), Vec3::new::<float>(-3.0, 6.0, -3.0));

        mut_a.cross_self(&B);
        assert_eq!(mut_a, A.cross(&B));
    }

    #[test]
    fn test_num() {
        let mut mut_a = A;

        assert_eq!(-A, Vec3::new::<float>(-1.0, -2.0, -3.0));
        assert_eq!(A.neg(), Vec3::new::<float>(-1.0, -2.0, -3.0));

        assert_eq!(A.mul_t(F1), Vec3::new::<float>(1.5, 3.0, 4.5));
        assert_eq!(A.div_t(F2), Vec3::new::<float>(2.0, 4.0, 6.0));

        assert_eq!(A.add_v(&B), Vec3::new::<float>(5.0, 7.0, 9.0));
        assert_eq!(A.sub_v(&B), Vec3::new::<float>(-3.0, -3.0, -3.0));
        assert_eq!(A.mul_v(&B), Vec3::new::<float>(4.0, 10.0, 18.0));
        assert_eq!(A.div_v(&B), Vec3::new::<float>(1.0/4.0, 2.0/5.0, 3.0/6.0));

        mut_a.neg_self();
        assert_eq!(mut_a, -A);
        mut_a = A;

        mut_a.mul_self_t(F1);
        assert_eq!(mut_a, A.mul_t(F1));
        mut_a = A;

        mut_a.div_self_t(F2);
        assert_eq!(mut_a, A.div_t(F2));
        mut_a = A;

        mut_a.add_self_v(&B);
        assert_eq!(mut_a, A.add_v(&B));
        mut_a = A;

        mut_a.sub_self_v(&B);
        assert_eq!(mut_a, A.sub_v(&B));
        mut_a = A;

        mut_a.mul_self_v(&B);
        assert_eq!(mut_a, A.mul_v(&B));
        mut_a = A;

        mut_a.div_self_v(&B);
        assert_eq!(mut_a, A.div_v(&B));
    }

    #[test]
    fn test_comp_add() {
        assert_eq!(A.comp_add(), 6.0);
        assert_eq!(B.comp_add(), 15.0);
    }

    #[test]
    fn test_comp_mul() {
        assert_eq!(A.comp_mul(), 6.0);
        assert_eq!(B.comp_mul(), 120.0);
    }

    #[test]
    fn test_approx_eq() {
        assert!(!Vec3::new::<float>(0.000001, 0.000001, 0.000001).approx_eq(&Vec3::new::<float>(0.0, 0.0, 0.0)));
        assert!(Vec3::new::<float>(0.0000001, 0.0000001, 0.0000001).approx_eq(&Vec3::new::<float>(0.0, 0.0, 0.0)));
    }


    #[test]
    fn test_magnitude() {
        let a = Vec3::new(2.0, 3.0, 6.0); // (2, 3, 6, 7) Pythagorean quadruple
        let b = Vec3::new(1.0, 4.0, 8.0); // (1, 4, 8, 9) Pythagorean quadruple

        assert_eq!(a.magnitude(), 7.0);
        assert_eq!(a.magnitude2(), 7.0 * 7.0);

        assert_eq!(b.magnitude(), 9.0);
        assert_eq!(b.magnitude2(), 9.0 * 9.0);
    }

    #[test]
    fn test_angle() {
        assert!(Vec3::new::<float>(1.0, 0.0, 1.0).angle(&Vec3::new::<float>(1.0, 1.0, 0.0)).approx_eq(&Real::frac_pi_3()));
        assert!(Vec3::new::<float>(10.0, 0.0, 10.0).angle(&Vec3::new::<float>(5.0, 5.0, 0.0)).approx_eq(&Real::frac_pi_3()));
        assert!(Vec3::new::<float>(-1.0, 0.0, -1.0).angle(&Vec3::new::<float>(1.0, -1.0, 0.0)).approx_eq(&(2.0 * Real::frac_pi_3())));
    }

    #[test]
    fn test_normalize() {
        assert!(Vec3::new::<float>(2.0, 3.0, 6.0).normalize().approx_eq(&Vec3::new::<float>(2.0/7.0, 3.0/7.0, 6.0/7.0)));
        // TODO: test normalize_to, normalize_self, and normalize_self_to
    }

    #[test]
    fn test_lerp() {
        let c = Vec3::new::<float>(-2.0, -1.0, 1.0);
        let d = Vec3::new::<float>( 1.0,  0.0, 0.5);

        assert_eq!(c.lerp(&d, 0.75), Vec3::new::<float>(0.250, -0.250, 0.625));

        let mut mut_c = c;
        mut_c.lerp_self(&d, 0.75);
        assert_eq!(mut_c, c.lerp(&d, 0.75));
    }

    #[test]
    fn test_comp_min() {
        assert_eq!(A.comp_min(), 1.0);
        assert_eq!(B.comp_min(), 4.0);
    }

    #[test]
    fn test_comp_max() {
        assert_eq!(A.comp_max(), 3.0);
        assert_eq!(B.comp_max(), 6.0);
    }

    #[test]
    fn test_boolean() {
        let tft = Vec3::new(true, false, true);
        let fff = Vec3::new(false, false, false);
        let ttt = Vec3::new(true, true, true);

        assert_eq!(tft.any(), true);
        assert_eq!(tft.all(), false);
        assert_eq!(!tft, Vec3::new(false, true, false));

        assert_eq!(fff.any(), false);
        assert_eq!(fff.all(), false);
        assert_eq!(!fff, Vec3::new(true, true, true));

        assert_eq!(ttt.any(), true);
        assert_eq!(ttt.all(), true);
        assert_eq!(!ttt, Vec3::new(false, false, false));
    }
}

#[deriving(Clone, Eq)]
pub struct Vec4<T> { x: T, y: T, z: T, w: T }

// GLSL-style type aliases
pub type vec4  = Vec4<f32>;
pub type dvec4 = Vec4<f64>;
pub type bvec4 = Vec4<bool>;
pub type ivec4 = Vec4<i32>;
pub type uvec4 = Vec4<u32>;

// Rust-style type aliases
pub type Vec4f   = Vec4<float>;
pub type Vec4f32 = Vec4<f32>;
pub type Vec4f64 = Vec4<f64>;
pub type Vec4i   = Vec4<int>;
pub type Vec4i8  = Vec4<i8>;
pub type Vec4i16 = Vec4<i16>;
pub type Vec4i32 = Vec4<i32>;
pub type Vec4i64 = Vec4<i64>;
pub type Vec4u   = Vec4<uint>;
pub type Vec4u8  = Vec4<u8>;
pub type Vec4u16 = Vec4<u16>;
pub type Vec4u32 = Vec4<u32>;
pub type Vec4u64 = Vec4<u64>;
pub type Vec4b   = Vec4<bool>;

pub trait ToVec4<T> {
    pub fn to_vec4(&self) -> Vec4<T>;
}

pub trait AsVec4<T> {
    pub fn as_vec4<'a>(&'a self) -> &'a Vec4<T>;
    pub fn as_mut_vec4<'a>(&'a mut self) -> &'a mut Vec4<T>;
}

impl_dimensioned!(Vec4, T, 4)
impl_swap_components!(Vec4)
impl_approx!(Vec4 { x, y, z, w })

impl<T> Vec4<T> {
    /// Construct a new vector from the supplied components.
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 { x: x, y: y, z: z, w: w }
    }
}

impl<T:Clone> Vec4<T> {
    /// Construct a new vector with each component set to `value`.
    #[inline]
    pub fn from_value(value: T) -> Vec4<T> {
        Vec4::new(value.clone(),
                  value.clone(),
                  value.clone(),
                  value.clone())
    }
}

/// Constants for four-dimensional vectors.
impl<T:Num> Vec4<T> {
    /// Returns a four-dimensional vector with each component set to `1`.
    #[inline]
    pub fn identity() -> Vec4<T> {
        Vec4::new(one!(T), one!(T), one!(T), one!(T))
    }

    /// Returns a four-dimensional vector with each component set to `0`.
    #[inline]
    pub fn zero() -> Vec4<T> {
        Vec4::new(zero!(T), zero!(T), zero!(T), zero!(T))
    }

    /// Returns a zeroed four-dimensional vector with the `x` component set to `1`.
    #[inline]
    pub fn unit_x() -> Vec4<T> {
        Vec4::new(one!(T), zero!(T), zero!(T), zero!(T))
    }

    /// Returns a zeroed four-dimensional vector with the `y` component set to `1`.
    #[inline]
    pub fn unit_y() -> Vec4<T> {
        Vec4::new(zero!(T), one!(T), zero!(T), zero!(T))
    }

    /// Returns a zeroed four-dimensional vector with the `z` component set to `1`.
    #[inline]
    pub fn unit_z() -> Vec4<T> {
        Vec4::new(zero!(T), zero!(T), one!(T), zero!(T))
    }

    /// Returns a zeroed four-dimensional vector with the `w` component set to `1`.
    #[inline]
    pub fn unit_w() -> Vec4<T> {
        Vec4::new(zero!(T), zero!(T), zero!(T), one!(T))
    }
}

impl<T> Vec<T,[T,..4]> for Vec4<T> {}

impl<T:Num> NumVec<T,[T,..4]> for Vec4<T> {
    /// Returns a new vector with `value` added to each component.
    #[inline]
    pub fn add_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) + value,
                  *self.index(1) + value,
                  *self.index(2) + value,
                  *self.index(3) + value)
    }

    /// Returns a new vector with `value` subtracted from each component.
    #[inline]
    pub fn sub_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) - value,
                  *self.index(1) - value,
                  *self.index(2) - value,
                  *self.index(3) - value)
    }

    /// Returns the scalar multiplication of the vector with `value`.
    #[inline]
    pub fn mul_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) * value,
                  *self.index(1) * value,
                  *self.index(2) * value,
                  *self.index(3) * value)
    }

    /// Returns a new vector with each component divided by `value`.
    #[inline]
    pub fn div_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) / value,
                  *self.index(1) / value,
                  *self.index(2) / value,
                  *self.index(3) / value)
    }

    /// Returns the remainder of each component divided by `value`.
    #[inline]
    pub fn rem_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) % value,
                  *self.index(1) % value,
                  *self.index(2) % value,
                  *self.index(3) % value)
    }

    /// Returns the sum of the two vectors.
    #[inline]
    pub fn add_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1),
                  *self.index(2) + *other.index(2),
                  *self.index(3) + *other.index(3))
    }

    /// Ruturns the result of subtrating `other` from the vector.
    #[inline]
    pub fn sub_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1),
                  *self.index(2) - *other.index(2),
                  *self.index(3) - *other.index(3))
    }

    /// Returns the component-wise product of the vector and `other`.
    #[inline]
    pub fn mul_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) * *other.index(0),
                  *self.index(1) * *other.index(1),
                  *self.index(2) * *other.index(2),
                  *self.index(3) * *other.index(3))
    }

    /// Returns the component-wise quotient of the vectors.
    #[inline]
    pub fn div_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) / *other.index(0),
                  *self.index(1) / *other.index(1),
                  *self.index(2) / *other.index(2),
                  *self.index(3) / *other.index(3))
    }

    /// Returns the component-wise remainder of the vector divided by `other`.
    #[inline]
    pub fn rem_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) % *other.index(0),
                  *self.index(1) % *other.index(1),
                  *self.index(2) % *other.index(2),
                  *self.index(3) % *other.index(3))
    }

    /// Negates each component of the vector.
    #[inline]
    pub fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index(0);
        *self.index_mut(1) = -*self.index(1);
        *self.index_mut(2) = -*self.index(2);
        *self.index_mut(3) = -*self.index(3);
    }

    /// Adds `value` to each component of the vector.
    #[inline]
    pub fn add_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) + value;
        *self.index_mut(1) = *self.index(1) + value;
        *self.index_mut(2) = *self.index(2) + value;
        *self.index_mut(3) = *self.index(3) + value;
    }

    /// Subtracts `value` from each component of the vector.
    #[inline]
    pub fn sub_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) - value;
        *self.index_mut(1) = *self.index(1) - value;
        *self.index_mut(2) = *self.index(2) - value;
        *self.index_mut(3) = *self.index(3) - value;
    }

    #[inline]
    pub fn mul_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) * value;
        *self.index_mut(1) = *self.index(1) * value;
        *self.index_mut(2) = *self.index(2) * value;
        *self.index_mut(3) = *self.index(3) * value;
    }

    #[inline]
    pub fn div_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) / value;
        *self.index_mut(1) = *self.index(1) / value;
        *self.index_mut(2) = *self.index(2) / value;
        *self.index_mut(3) = *self.index(3) / value;
    }

    #[inline]
    pub fn rem_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) % value;
        *self.index_mut(1) = *self.index(1) % value;
        *self.index_mut(2) = *self.index(2) % value;
        *self.index_mut(3) = *self.index(3) % value;
    }

    #[inline]
    pub fn add_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) = *self.index(0) + *other.index(0);
        *self.index_mut(1) = *self.index(1) + *other.index(1);
        *self.index_mut(2) = *self.index(2) + *other.index(2);
        *self.index_mut(3) = *self.index(3) + *other.index(3);
    }

    #[inline]
    pub fn sub_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) = *self.index(0) - *other.index(0);
        *self.index_mut(1) = *self.index(1) - *other.index(1);
        *self.index_mut(2) = *self.index(2) - *other.index(2);
        *self.index_mut(3) = *self.index(3) - *other.index(3);
    }

    #[inline]
    pub fn mul_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) = *self.index(0) * *other.index(0);
        *self.index_mut(1) = *self.index(1) * *other.index(1);
        *self.index_mut(2) = *self.index(2) * *other.index(2);
        *self.index_mut(3) = *self.index(3) * *other.index(3);
    }

    #[inline]
    pub fn div_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) = *self.index(0) / *other.index(0);
        *self.index_mut(1) = *self.index(1) / *other.index(1);
        *self.index_mut(2) = *self.index(2) / *other.index(2);
        *self.index_mut(3) = *self.index(3) / *other.index(3);
    }

    #[inline]
    pub fn rem_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) = *self.index(0) % *other.index(0);
        *self.index_mut(1) = *self.index(1) % *other.index(1);
        *self.index_mut(2) = *self.index(2) % *other.index(2);
        *self.index_mut(3) = *self.index(3) % *other.index(3);
    }

    /// Returns the dot product of the vector and `other`.
    #[inline]
    pub fn dot(&self, other: &Vec4<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1) +
        *self.index(2) * *other.index(2) +
        *self.index(3) * *other.index(3)
    }

    /// Returns the sum of the vector's components.
    #[inline]
    pub fn comp_add(&self) -> T {
        *self.index(0) + *self.index(1) + *self.index(2) + *self.index(3)
    }

    /// Returns the product of the vector's components.
    #[inline]
    pub fn comp_mul(&self) -> T {
        *self.index(0) * *self.index(1) * *self.index(2) * *self.index(3)
    }
}

impl<T:Num> Neg<Vec4<T>> for Vec4<T> {
    /// Returns the vector with each component negated.
    #[inline]
    pub fn neg(&self) -> Vec4<T> {
        Vec4::new(-*self.index(0),
                  -*self.index(1),
                  -*self.index(2),
                  -*self.index(3))
    }
}

impl<T:Float> FloatVec<T,[T,..4]> for Vec4<T> {
    /// Returns the squared magnitude of the vector. This does not perform a
    /// square root operation like in the `magnitude` method and can therefore
    /// be more efficient for comparing the magnitudes of two vectors.
    #[inline]
    pub fn magnitude2(&self) -> T {
        self.dot(self)
    }

    /// Returns the magnitude (length) of the vector.
    #[inline]
    pub fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }

    /// Returns the angle between the vector and `other`.
    #[inline]
    pub fn angle(&self, other: &Vec4<T>) -> T {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }

    /// Returns the result of normalizing the vector to a magnitude of `1`.
    #[inline]
    pub fn normalize(&self) -> Vec4<T> {
        self.normalize_to(one!(T))
    }

    /// Returns the result of normalizing the vector to `magnitude`.
    #[inline]
    pub fn normalize_to(&self, magnitude: T) -> Vec4<T> {
        self.mul_t(magnitude / self.magnitude())
    }

    /// Returns the result of linarly interpolating the magnitude of the vector
    /// to the magnitude of `other` by the specified amount.
    #[inline]
    pub fn lerp(&self, other: &Vec4<T>, amount: T) -> Vec4<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    /// Normalises the vector to a magnitude of `1`.
    #[inline]
    pub fn normalize_self(&mut self) {
        let rlen = self.magnitude().recip();
        self.mul_self_t(rlen);
    }

    /// Normalizes the vector to `magnitude`.
    #[inline]
    pub fn normalize_self_to(&mut self, magnitude: T) {
        let n = magnitude / self.magnitude();
        self.mul_self_t(n);
    }

    /// Linearly interpolates the magnitude of the vector to the magnitude of
    /// `other` by the specified amount.
    pub fn lerp_self(&mut self, other: &Vec4<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Orderable> OrdVec<T,[T,..4],Vec4<bool>> for Vec4<T> {
    #[inline]
    pub fn lt_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) < value,
                  *self.index(1) < value,
                  *self.index(2) < value,
                  *self.index(3) < value)
    }

    #[inline]
    pub fn le_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) <= value,
                  *self.index(1) <= value,
                  *self.index(2) <= value,
                  *self.index(3) <= value)
    }

    #[inline]
    pub fn ge_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) >= value,
                  *self.index(1) >= value,
                  *self.index(2) >= value,
                  *self.index(3) >= value)
    }

    #[inline]
    pub fn gt_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) > value,
                  *self.index(1) > value,
                  *self.index(2) > value,
                  *self.index(3) > value)
    }

    #[inline]
    pub fn lt_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) < *other.index(0),
                  *self.index(1) < *other.index(1),
                  *self.index(2) < *other.index(2),
                  *self.index(3) < *other.index(3))
    }

    #[inline]
    pub fn le_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) <= *other.index(0),
                  *self.index(1) <= *other.index(1),
                  *self.index(2) <= *other.index(2),
                  *self.index(3) <= *other.index(3))
    }

    #[inline]
    pub fn ge_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) >= *other.index(0),
                  *self.index(1) >= *other.index(1),
                  *self.index(2) >= *other.index(2),
                  *self.index(3) >= *other.index(3))
    }

    #[inline]
    pub fn gt_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) > *other.index(0),
                  *self.index(1) > *other.index(1),
                  *self.index(2) > *other.index(2),
                  *self.index(3) > *other.index(3))
    }

    #[inline]
    pub fn min_t(&self, value: T) -> Vec4<T> {
        Vec4::new(self.index(0).min(&value),
                  self.index(1).min(&value),
                  self.index(2).min(&value),
                  self.index(3).min(&value))
    }

    #[inline]
    pub fn max_t(&self, value: T) -> Vec4<T> {
        Vec4::new(self.index(0).max(&value),
                  self.index(1).max(&value),
                  self.index(2).max(&value),
                  self.index(3).max(&value))
    }

    #[inline]
    pub fn min_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self.index(0).min(other.index(0)),
                  self.index(1).min(other.index(1)),
                  self.index(2).min(other.index(2)),
                  self.index(3).min(other.index(3)))
    }

    #[inline]
    pub fn max_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self.index(0).max(other.index(0)),
                  self.index(1).max(other.index(1)),
                  self.index(2).max(other.index(2)),
                  self.index(3).max(other.index(3)))
    }

    /// Returns the smallest component of the vector.
    #[inline]
    pub fn comp_min(&self) -> T {
        self.index(0).min(self.index(1)).min(self.index(2)).min(self.index(3))
    }

    /// Returns the largest component of the vector.
    #[inline]
    pub fn comp_max(&self) -> T {
        self.index(0).max(self.index(1)).max(self.index(2)).max(self.index(3))
    }
}

impl<T:Eq> EqVec<T,[T,..4],Vec4<bool>> for Vec4<T> {
    #[inline]
    pub fn eq_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) == value,
                  *self.index(1) == value,
                  *self.index(2) == value,
                  *self.index(3) == value)
    }

    #[inline]
    pub fn ne_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) != value,
                  *self.index(1) != value,
                  *self.index(2) != value,
                  *self.index(3) != value)
    }

    #[inline]
    pub fn eq_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) == *other.index(0),
                  *self.index(1) == *other.index(1),
                  *self.index(2) == *other.index(2),
                  *self.index(3) == *other.index(3))
    }

    #[inline]
    pub fn ne_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) != *other.index(0),
                  *self.index(1) != *other.index(1),
                  *self.index(2) != *other.index(2),
                  *self.index(3) != *other.index(3))
    }
}

impl BoolVec<[bool,..4]> for Vec4<bool> {
    /// Returns `true` if any of the components of the vector are equal to
    /// `true`, otherwise `false`.
    #[inline]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1) || *self.index(2) || *self.index(3)
    }

    /// Returns `true` if _all_ of the components of the vector are equal to
    /// `true`, otherwise `false`.
    #[inline]
    pub fn all(&self) -> bool {
        *self.index(0) && *self.index(1) && *self.index(2) && *self.index(3)
    }
}

impl<T:Not<T>> Not<Vec4<T>> for Vec4<T> {
    pub fn not(&self) -> Vec4<T> {
        Vec4::new(!*self.index(0),
                  !*self.index(1),
                  !*self.index(2),
                  !*self.index(3))
    }
}

#[cfg(test)]
mod vec4_tests {
    use math::vec::*;

    static A: Vec4<float> = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    static B: Vec4<float> = Vec4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    static F1: float = 1.5;
    static F2: float = 0.5;

    #[test]
    fn test_swap() {
        let mut mut_a = A;

        mut_a.swap(0, 3);
        assert_eq!(*mut_a.index(0), *A.index(3));
        assert_eq!(*mut_a.index(3), *A.index(0));
        mut_a = A;

        mut_a.swap(1, 2);
        assert_eq!(*mut_a.index(1), *A.index(2));
        assert_eq!(*mut_a.index(2), *A.index(1));
    }

    #[test]
    fn test_num() {
        let mut mut_a = A;

        assert_eq!(-A, Vec4::new::<float>(-1.0, -2.0, -3.0, -4.0));
        assert_eq!(A.neg(), Vec4::new::<float>(-1.0, -2.0, -3.0, -4.0));

        assert_eq!(A.mul_t(F1), Vec4::new::<float>(1.5, 3.0, 4.5, 6.0));
        assert_eq!(A.div_t(F2), Vec4::new::<float>(2.0, 4.0, 6.0, 8.0));

        assert_eq!(A.add_v(&B), Vec4::new::<float>(6.0, 8.0, 10.0, 12.0));
        assert_eq!(A.sub_v(&B), Vec4::new::<float>(-4.0, -4.0, -4.0, -4.0));
        assert_eq!(A.mul_v(&B), Vec4::new::<float>(5.0, 12.0, 21.0, 32.0));
        assert_eq!(A.div_v(&B), Vec4::new::<float>(1.0/5.0, 2.0/6.0, 3.0/7.0, 4.0/8.0));

        assert_eq!(A.dot(&B), 70.0);

        mut_a.neg_self();
        assert_eq!(mut_a, -A);
        mut_a = A;

        mut_a.mul_self_t(F1);
        assert_eq!(mut_a, A.mul_t(F1));
        mut_a = A;

        mut_a.div_self_t(F2);
        assert_eq!(mut_a, A.div_t(F2));
        mut_a = A;

        mut_a.add_self_v(&B);
        assert_eq!(mut_a, A.add_v(&B));
        mut_a = A;

        mut_a.sub_self_v(&B);
        assert_eq!(mut_a, A.sub_v(&B));
        mut_a = A;

        mut_a.mul_self_v(&B);
        assert_eq!(mut_a, A.mul_v(&B));
        mut_a = A;

        mut_a.div_self_v(&B);
        assert_eq!(mut_a, A.div_v(&B));
    }

    #[test]
    fn test_comp_add() {
        assert_eq!(A.comp_add(), 10.0);
        assert_eq!(B.comp_add(), 26.0);
    }

    #[test]
    fn test_comp_mul() {
        assert_eq!(A.comp_mul(), 24.0);
        assert_eq!(B.comp_mul(), 1680.0);
    }

    #[test]
    fn test_approx_eq() {
        assert!(!Vec4::new::<float>(0.000001, 0.000001, 0.000001, 0.000001).approx_eq(&Vec4::new::<float>(0.0, 0.0, 0.0, 0.0)));
        assert!(Vec4::new::<float>(0.0000001, 0.0000001, 0.0000001, 0.0000001).approx_eq(&Vec4::new::<float>(0.0, 0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_magnitude() {
        let a = Vec4::new(1.0, 2.0, 4.0, 10.0); // (1, 2, 4, 10, 11) Pythagorean quintuple
        let b = Vec4::new(1.0, 2.0, 8.0, 10.0); // (1, 2, 8, 10, 13) Pythagorean quintuple

        assert_eq!(a.magnitude(), 11.0);
        assert_eq!(a.magnitude2(), 11.0 * 11.0);

        assert_eq!(b.magnitude(), 13.0);
        assert_eq!(b.magnitude2(), 13.0 * 13.0);
    }

    #[test]
    fn test_angle() {
        assert!(Vec4::new::<float>(1.0, 0.0, 1.0, 0.0).angle(&Vec4::new::<float>(0.0, 1.0, 0.0, 1.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec4::new::<float>(10.0, 0.0, 10.0, 0.0).angle(&Vec4::new::<float>(0.0, 5.0, 0.0, 5.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec4::new::<float>(-1.0, 0.0, -1.0, 0.0).angle(&Vec4::new::<float>(0.0, 1.0, 0.0, 1.0)).approx_eq(&Real::frac_pi_2()));
    }

    #[test]
    fn test_normalize() {
        assert!(Vec4::new::<float>(1.0, 2.0, 4.0, 10.0).normalize().approx_eq(&Vec4::new::<float>(1.0/11.0, 2.0/11.0, 4.0/11.0, 10.0/11.0)));
        // TODO: test normalize_to, normalize_self, and normalize_self_to
    }

    #[test]
    fn test_lerp() {
        let c = Vec4::new::<float>(-2.0, -1.0, 1.0, 2.0);
        let d = Vec4::new::<float>( 1.0,  0.0, 0.5, 1.0);

        assert_eq!(c.lerp(&d, 0.75), Vec4::new::<float>(0.250, -0.250, 0.625, 1.250));

        let mut mut_c = c;
        mut_c.lerp_self(&d, 0.75);
        assert_eq!(mut_c, c.lerp(&d, 0.75));
    }

    #[test]
    fn test_comp_min() {
        assert_eq!(A.comp_min(), 1.0);
        assert_eq!(B.comp_min(), 5.0);
    }

    #[test]
    fn test_comp_max() {
        assert_eq!(A.comp_max(), 4.0);
        assert_eq!(B.comp_max(), 8.0);
    }

    #[test]
    fn test_boolean() {
        let tftf = Vec4::new(true, false, true, false);
        let ffff = Vec4::new(false, false, false, false);
        let tttt = Vec4::new(true, true, true, true);

        assert_eq!(tftf.any(), true);
        assert_eq!(tftf.all(), false);
        assert_eq!(!tftf, Vec4::new(false, true, false, true));

        assert_eq!(ffff.any(), false);
        assert_eq!(ffff.all(), false);
        assert_eq!(!ffff, Vec4::new(true, true, true, true));

        assert_eq!(tttt.any(), true);
        assert_eq!(tttt.all(), true);
        assert_eq!(!tttt, Vec4::new(false, false, false, false));
    }
}
