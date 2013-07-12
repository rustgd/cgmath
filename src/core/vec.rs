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

#[cfg(geom)]
use std::cast;

use core::Dimensional;
#[cfg(geom)]
use geom::{Point2, Point3};

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

impl_approx!(Vec2 { x, y })

impl<T> Vec2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x: x, y: y }
    }
}

#[cfg(geom)]
impl<T> Vec2<T> {
    #[inline]
    pub fn from_point(point: Point2<T>) -> Vec2<T> {
        unsafe { cast::transmute(point) }
    }

    #[inline]
    pub fn as_point<'a>(&'a self) -> &'a Point2<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn as_mut_point<'a>(&'a mut self) -> &'a mut Point2<T> {
        unsafe { cast::transmute(self) }
    }
}

impl<T:Clone> Vec2<T> {
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

impl<T:Num> Vec2<T> {
    #[inline]
    pub fn identity() -> Vec2<T> {
        Vec2::new(one!(T), one!(T))
    }

    #[inline]
    pub fn zero() -> Vec2<T> {
        Vec2::new(zero!(T), zero!(T))
    }

    #[inline]
    pub fn unit_x() -> Vec2<T> {
        Vec2::new(one!(T), zero!(T))
    }

    #[inline]
    pub fn unit_y() -> Vec2<T> {
        Vec2::new(zero!(T), one!(T))
    }

    #[inline]
    pub fn perp_dot(&self, other: &Vec2<T>) -> T {
        (*self.index(0) * *other.index(1)) -
        (*self.index(1) * *other.index(0))
    }

    #[inline]
    pub fn add_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) + value,
                  *self.index(1) + value)
    }

    #[inline]
    pub fn sub_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) - value,
                  *self.index(1) - value)
    }

    #[inline]
    pub fn mul_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) * value,
                  *self.index(1) * value)
    }

    #[inline]
    pub fn div_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) / value,
                  *self.index(1) / value)
    }

    #[inline]
    pub fn rem_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) % value,
                  *self.index(1) % value)
    }

    #[inline]
    pub fn add_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1))
    }

    #[inline]
    pub fn sub_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1))
    }

    #[inline]
    pub fn mul_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) * *other.index(0),
                  *self.index(1) * *other.index(1))
    }

    #[inline]
    pub fn div_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) / *other.index(0),
                  *self.index(1) / *other.index(1))
    }

    #[inline]
    pub fn rem_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) % *other.index(0),
                  *self.index(1) % *other.index(1))
    }

    #[inline]
    pub fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index(0);
        *self.index_mut(1) = -*self.index(1);
    }

    #[inline]
    pub fn add_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) + value;
        *self.index_mut(1) = *self.index(1) + value;
    }

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

    #[inline] pub fn dot(&self, other: &Vec2<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1)
    }
}

impl<T:Num> Neg<Vec2<T>> for Vec2<T> {
    #[inline]
    pub fn neg(&self) -> Vec2<T> {
        Vec2::new(-*self.index(0),
                  -*self.index(1))
    }
}

impl<T:Real> Vec2<T> {
    #[inline]
    pub fn magnitude2(&self) -> T {
        self.dot(self)
    }

    #[inline]
    pub fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }

    #[inline]
    pub fn angle(&self, other: &Vec2<T>) -> T {
        self.perp_dot(other).atan2(&self.dot(other))
    }

    #[inline]
    pub fn normalize(&self) -> Vec2<T> {
        self.mul_t(one!(T)/self.magnitude())
    }

    #[inline]
    pub fn normalize_to(&self, magnitude: T) -> Vec2<T> {
        self.mul_t(magnitude / self.magnitude())
    }

    #[inline]
    pub fn lerp(&self, other: &Vec2<T>, amount: T) -> Vec2<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    #[inline]
    pub fn normalize_self(&mut self) {
        let rlen = self.magnitude().recip();
        self.mul_self_t(rlen);
    }

    #[inline]
    pub fn normalize_self_to(&mut self, magnitude: T) {
        let n = magnitude / self.magnitude();
        self.mul_self_t(n);
    }

    pub fn lerp_self(&mut self, other: &Vec2<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Ord> Vec2<T> {
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
}

impl<T:Eq> Vec2<T> {
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

impl Vec2<bool> {
    #[inline]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1)
    }

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
    use core::vec::*;

    static A: Vec2<float> = Vec2 { x: 1.0, y: 2.0 };
    static B: Vec2<float> = Vec2 { x: 3.0, y: 4.0 };
    static F1: float = 1.5;
    static F2: float = 0.5;

    #[test]
    fn test_vec2() {
        let mut mut_a = A;

        assert_eq!(Vec2::new::<float>(1.0, 2.0), A);
        assert_eq!(Vec2::from_value(1.0), Vec2::new::<float>(1.0, 1.0));

        assert_eq!(Vec2::zero(), Vec2::new::<float>(0.0, 0.0));
        assert_eq!(Vec2::unit_x(), Vec2::new::<float>(1.0, 0.0));
        assert_eq!(Vec2::unit_y(), Vec2::new::<float>(0.0, 1.0));
        assert_eq!(Vec2::identity(), Vec2::new::<float>(1.0, 1.0));

        *mut_a.index_mut(0) = 42.0;
        *mut_a.index_mut(1) = 43.0;
        assert_eq!(mut_a, Vec2::new::<float>(42.0, 43.0));
        mut_a = A;

        mut_a.swap(0, 1);
        assert_eq!(*mut_a.index(0), *A.index(1));
        assert_eq!(*mut_a.index(1), *A.index(0));
        mut_a = A;

        assert_eq!(A.x, 1.0);
        assert_eq!(A.y, 2.0);
        assert_eq!(*A.index(0), 1.0);
        assert_eq!(*A.index(1), 2.0);

        assert_eq!(-A, Vec2::new::<float>(-1.0, -2.0));
        assert_eq!(A.neg(), Vec2::new::<float>(-1.0, -2.0));

        assert_eq!(A.mul_t(F1), Vec2::new::<float>( 1.5, 3.0));
        assert_eq!(A.div_t(F2), Vec2::new::<float>( 2.0, 4.0));

        assert_eq!(A.add_v(&B), Vec2::new::<float>(    4.0,     6.0));
        assert_eq!(A.sub_v(&B), Vec2::new::<float>(   -2.0,    -2.0));
        assert_eq!(A.mul_v(&B), Vec2::new::<float>(    3.0,     8.0));
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
    fn test_vec2_approx_eq() {
        assert!(!Vec2::new::<float>(0.000001, 0.000001).approx_eq(&Vec2::new::<float>(0.0, 0.0)));
        assert!(Vec2::new::<float>(0.0000001, 0.0000001).approx_eq(&Vec2::new::<float>(0.0, 0.0)));
    }

    static E_A: Vec2<float> = Vec2 { x: 5.0, y: 12.0 }; // (5, 12, 13) Pythagorean triple
    static E_B: Vec2<float> = Vec2 { x: 3.0, y: 4.0 }; // (3, 4, 5) Pythagorean triple

    #[test]
    fn test_vec2_euclidean() {
        assert_eq!(E_A.magnitude(), 13.0);
        assert_eq!(E_A.magnitude2(), 13.0 * 13.0);

        assert_eq!(E_B.magnitude(), 5.0);
        assert_eq!(E_B.magnitude2(), 5.0 * 5.0);

        assert!(Vec2::new::<float>(1.0, 0.0).angle(&Vec2::new::<float>(0.0, 1.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec2::new::<float>(10.0, 0.0).angle(&Vec2::new::<float>(0.0, 5.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec2::new::<float>(-1.0, 0.0).angle(&Vec2::new::<float>(0.0, 1.0)).approx_eq(&-Real::frac_pi_2::<float>()));

        assert!(Vec2::new::<float>(3.0, 4.0).normalize().approx_eq(&Vec2::new::<float>(3.0/5.0, 4.0/5.0)));
        // TODO: test normalize_to, normalize_self, and normalize_self_to

        let c = Vec2::new::<float>(-2.0, -1.0);
        let d = Vec2::new::<float>( 1.0,  0.0);

        assert_eq!(c.lerp(&d, 0.75), Vec2::new::<float>(0.250, -0.250));

        let mut mut_c = c;
        mut_c.lerp_self(&d, 0.75);
        assert_eq!(mut_c, c.lerp(&d, 0.75));
    }

    #[test]
    fn test_vec2_boolean() {
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

impl_approx!(Vec3 { x, y, z })

impl<T> Vec3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x: x, y: y, z: z }
    }
}

#[cfg(geom)]
impl<T> Vec3<T> {
    #[inline]
    pub fn from_point(point: Point3<T>) -> Vec3<T> {
        unsafe { cast::transmute(point) }
    }

    #[inline]
    pub fn as_point<'a>(&'a self) -> &'a Point3<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn as_mut_point<'a>(&'a mut self) -> &'a mut Point3<T> {
        unsafe { cast::transmute(self) }
    }
}

impl<T:Clone> Vec3<T> {
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

impl<T:Num> Vec3<T> {
    #[inline]
    pub fn identity() -> Vec3<T> {
        Vec3::new(one!(T), one!(T), one!(T))
    }

    #[inline]
    pub fn zero() -> Vec3<T> {
        Vec3::new(zero!(T), zero!(T), zero!(T))
    }

    #[inline]
    pub fn unit_x() -> Vec3<T> {
        Vec3::new(one!(T), zero!(T), zero!(T))
    }

    #[inline]
    pub fn unit_y() -> Vec3<T> {
        Vec3::new(zero!(T), one!(T), zero!(T))
    }

    #[inline]
    pub fn unit_z() -> Vec3<T> {
        Vec3::new(zero!(T), zero!(T), one!(T))
    }

    #[inline]
    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new((*self.index(1) * *other.index(2)) - (*self.index(2) * *other.index(1)),
                  (*self.index(2) * *other.index(0)) - (*self.index(0) * *other.index(2)),
                  (*self.index(0) * *other.index(1)) - (*self.index(1) * *other.index(0)))
    }

    #[inline]
    pub fn cross_self(&mut self, other: &Vec3<T>) {
        *self = self.cross(other)
    }

    #[inline]
    pub fn add_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) + value,
                  *self.index(1) + value,
                  *self.index(2) + value)
    }

    #[inline]
    pub fn sub_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) - value,
                  *self.index(1) - value,
                  *self.index(2) - value)
    }

    #[inline]
    pub fn mul_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) * value,
                  *self.index(1) * value,
                  *self.index(2) * value)
    }

    #[inline]
    pub fn div_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) / value,
                  *self.index(1) / value,
                  *self.index(2) / value)
    }

    #[inline]
    pub fn rem_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) % value,
                  *self.index(1) % value,
                  *self.index(2) % value)
    }

    #[inline]
    pub fn add_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1),
                  *self.index(2) + *other.index(2))
    }

    #[inline]
    pub fn sub_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1),
                  *self.index(2) - *other.index(2))
    }

    #[inline]
    pub fn mul_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) * *other.index(0),
                  *self.index(1) * *other.index(1),
                  *self.index(2) * *other.index(2))
    }

    #[inline]
    pub fn div_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) / *other.index(0),
                  *self.index(1) / *other.index(1),
                  *self.index(2) / *other.index(2))
    }

    #[inline]
    pub fn rem_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) % *other.index(0),
                  *self.index(1) % *other.index(1),
                  *self.index(2) % *other.index(2))
    }

    #[inline]
    pub fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index(0);
        *self.index_mut(1) = -*self.index(1);
        *self.index_mut(2) = -*self.index(2);
    }

    #[inline]
    pub fn add_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) + value;
        *self.index_mut(1) = *self.index(1) + value;
        *self.index_mut(2) = *self.index(2) + value;
    }

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

    #[inline] pub fn dot(&self, other: &Vec3<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1) +
        *self.index(2) * *other.index(2)
    }
}

impl<T:Num> Neg<Vec3<T>> for Vec3<T> {
    #[inline]
    pub fn neg(&self) -> Vec3<T> {
        Vec3::new(-*self.index(0),
                  -*self.index(1),
                  -*self.index(2))
    }
}

impl<T:Real> Vec3<T> {
    #[inline]
    pub fn magnitude2(&self) -> T {
        self.dot(self)
    }

    #[inline]
    pub fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }

    #[inline]
    pub fn angle(&self, other: &Vec3<T>) -> T {
        self.cross(other).magnitude().atan2(&self.dot(other))
    }

    #[inline]
    pub fn normalize(&self) -> Vec3<T> {
        self.mul_t(one!(T)/self.magnitude())
    }

    #[inline]
    pub fn normalize_to(&self, magnitude: T) -> Vec3<T> {
        self.mul_t(magnitude / self.magnitude())
    }

    #[inline]
    pub fn lerp(&self, other: &Vec3<T>, amount: T) -> Vec3<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    #[inline]
    pub fn normalize_self(&mut self) {
        let rlen = self.magnitude().recip();
        self.mul_self_t(rlen);
    }

    #[inline]
    pub fn normalize_self_to(&mut self, magnitude: T) {
        let n = magnitude / self.magnitude();
        self.mul_self_t(n);
    }

    pub fn lerp_self(&mut self, other: &Vec3<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Ord> Vec3<T> {
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
}

impl<T:Eq> Vec3<T> {
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

impl Vec3<bool> {
    #[inline]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1) || *self.index(2)
    }

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
    use core::vec::*;

    static A: Vec3<float> = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    static B: Vec3<float> = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    static F1: float = 1.5;
    static F2: float = 0.5;

    #[test]
    fn test_vec3() {
        let mut mut_a = A;

        assert_eq!(Vec3::new::<float>(1.0, 2.0, 3.0), A);
        assert_eq!(Vec3::from_value(1.0), Vec3::new::<float>(1.0, 1.0, 1.0));

        assert_eq!(Vec3::zero(), Vec3::new::<float>(0.0, 0.0, 0.0));
        assert_eq!(Vec3::unit_x(), Vec3::new::<float>(1.0, 0.0, 0.0));
        assert_eq!(Vec3::unit_y(), Vec3::new::<float>(0.0, 1.0, 0.0));
        assert_eq!(Vec3::unit_z(), Vec3::new::<float>(0.0, 0.0, 1.0));
        assert_eq!(Vec3::identity(), Vec3::new::<float>(1.0, 1.0, 1.0));

        *mut_a.index_mut(0) = 42.0;
        *mut_a.index_mut(1) = 43.0;
        *mut_a.index_mut(2) = 44.0;
        assert_eq!(mut_a, Vec3::new::<float>(42.0, 43.0, 44.0));
        mut_a = A;

        mut_a.swap(0, 2);
        assert_eq!(*mut_a.index(0), *A.index(2));
        assert_eq!(*mut_a.index(2), *A.index(0));
        mut_a = A;

        mut_a.swap(1, 2);
        assert_eq!(*mut_a.index(1), *A.index(2));
        assert_eq!(*mut_a.index(2), *A.index(1));
        mut_a = A;

        assert_eq!(A.x, 1.0);
        assert_eq!(A.y, 2.0);
        assert_eq!(A.z, 3.0);
        assert_eq!(*A.index(0), 1.0);
        assert_eq!(*A.index(1), 2.0);
        assert_eq!(*A.index(2), 3.0);

        assert_eq!(A.cross(&B), Vec3::new::<float>(-3.0, 6.0, -3.0));

        mut_a.cross_self(&B);
        assert_eq!(mut_a, A.cross(&B));
        mut_a = A;

        assert_eq!(-A, Vec3::new::<float>(-1.0, -2.0, -3.0));
        assert_eq!(A.neg(), Vec3::new::<float>(-1.0, -2.0, -3.0));

        assert_eq!(A.mul_t(F1), Vec3::new::<float>( 1.5, 3.0, 4.5));
        assert_eq!(A.div_t(F2), Vec3::new::<float>( 2.0, 4.0, 6.0));

        assert_eq!(A.add_v(&B), Vec3::new::<float>(    5.0,     7.0,     9.0));
        assert_eq!(A.sub_v(&B), Vec3::new::<float>(   -3.0,    -3.0,    -3.0));
        assert_eq!(A.mul_v(&B), Vec3::new::<float>(    4.0,    10.0,    18.0));
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
    fn test_vec3_approx_eq() {
        assert!(!Vec3::new::<float>(0.000001, 0.000001, 0.000001).approx_eq(&Vec3::new::<float>(0.0, 0.0, 0.0)));
        assert!(Vec3::new::<float>(0.0000001, 0.0000001, 0.0000001).approx_eq(&Vec3::new::<float>(0.0, 0.0, 0.0)));
    }

    static E_A: Vec3<float> = Vec3 { x: 2.0, y: 3.0, z: 6.0 }; // (2, 3, 6, 7) Pythagorean quadruple
    static E_B: Vec3<float> = Vec3 { x: 1.0, y: 4.0, z: 8.0 }; // (1, 4, 8, 9) Pythagorean quadruple

    #[test]
    fn test_vec3_euclidean() {
        assert_eq!(E_A.magnitude(), 7.0);
        assert_eq!(E_A.magnitude2(), 7.0 * 7.0);

        assert_eq!(E_B.magnitude(), 9.0);
        assert_eq!(E_B.magnitude2(), 9.0 * 9.0);

        assert!(Vec3::new::<float>(1.0, 0.0, 1.0).angle(&Vec3::new::<float>(1.0, 1.0, 0.0)).approx_eq(&Real::frac_pi_3()));
        assert!(Vec3::new::<float>(10.0, 0.0, 10.0).angle(&Vec3::new::<float>(5.0, 5.0, 0.0)).approx_eq(&Real::frac_pi_3()));
        assert!(Vec3::new::<float>(-1.0, 0.0, -1.0).angle(&Vec3::new::<float>(1.0, -1.0, 0.0)).approx_eq(&(2.0 * Real::frac_pi_3())));

        assert!(Vec3::new::<float>(2.0, 3.0, 6.0).normalize().approx_eq(&Vec3::new::<float>(2.0/7.0, 3.0/7.0, 6.0/7.0)));
        // TODO: test normalize_to, normalize_self, and normalize_self_to

        let c = Vec3::new::<float>(-2.0, -1.0, 1.0);
        let d = Vec3::new::<float>( 1.0,  0.0, 0.5);

        assert_eq!(c.lerp(&d, 0.75), Vec3::new::<float>(0.250, -0.250, 0.625));

        let mut mut_c = c;
        mut_c.lerp_self(&d, 0.75);
        assert_eq!(mut_c, c.lerp(&d, 0.75));
    }

    #[test]
    fn test_vec3_boolean() {
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

impl_approx!(Vec4 { x, y, z, w })

impl<T> Vec4<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 { x: x, y: y, z: z, w: w }
    }
}

impl<T:Clone> Vec4<T> {
    #[inline]
    pub fn from_value(value: T) -> Vec4<T> {
        Vec4::new(value.clone(),
                  value.clone(),
                  value.clone(),
                  value.clone())
    }
}

impl<T:Num> Vec4<T> {
    #[inline]
    pub fn identity() -> Vec4<T> {
        Vec4::new(one!(T), one!(T), one!(T), one!(T))
    }

    #[inline]
    pub fn zero() -> Vec4<T> {
        Vec4::new(zero!(T), zero!(T), zero!(T), zero!(T))
    }

    #[inline]
    pub fn unit_x() -> Vec4<T> {
        Vec4::new(one!(T), zero!(T), zero!(T), zero!(T))
    }

    #[inline]
    pub fn unit_y() -> Vec4<T> {
        Vec4::new(zero!(T), one!(T), zero!(T), zero!(T))
    }

    #[inline]
    pub fn unit_z() -> Vec4<T> {
        Vec4::new(zero!(T), zero!(T), one!(T), zero!(T))
    }

    #[inline]
    pub fn unit_w() -> Vec4<T> {
        Vec4::new(zero!(T), zero!(T), zero!(T), one!(T))
    }

    #[inline]
    pub fn add_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) + value,
                  *self.index(1) + value,
                  *self.index(2) + value,
                  *self.index(3) + value)
    }

    #[inline]
    pub fn sub_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) - value,
                  *self.index(1) - value,
                  *self.index(2) - value,
                  *self.index(3) - value)
    }

    #[inline]
    pub fn mul_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) * value,
                  *self.index(1) * value,
                  *self.index(2) * value,
                  *self.index(3) * value)
    }

    #[inline]
    pub fn div_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) / value,
                  *self.index(1) / value,
                  *self.index(2) / value,
                  *self.index(3) / value)
    }

    #[inline]
    pub fn rem_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) % value,
                  *self.index(1) % value,
                  *self.index(2) % value,
                  *self.index(3) % value)
    }

    #[inline]
    pub fn add_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1),
                  *self.index(2) + *other.index(2),
                  *self.index(3) + *other.index(3))
    }

    #[inline]
    pub fn sub_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1),
                  *self.index(2) - *other.index(2),
                  *self.index(3) - *other.index(3))
    }

    #[inline]
    pub fn mul_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) * *other.index(0),
                  *self.index(1) * *other.index(1),
                  *self.index(2) * *other.index(2),
                  *self.index(3) * *other.index(3))
    }

    #[inline]
    pub fn div_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) / *other.index(0),
                  *self.index(1) / *other.index(1),
                  *self.index(2) / *other.index(2),
                  *self.index(3) / *other.index(3))
    }

    #[inline]
    pub fn rem_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) % *other.index(0),
                  *self.index(1) % *other.index(1),
                  *self.index(2) % *other.index(2),
                  *self.index(3) % *other.index(3))
    }

    #[inline]
    pub fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index(0);
        *self.index_mut(1) = -*self.index(1);
        *self.index_mut(2) = -*self.index(2);
        *self.index_mut(3) = -*self.index(3);
    }

    #[inline]
    pub fn add_self_t(&mut self, value: T) {
        *self.index_mut(0) = *self.index(0) + value;
        *self.index_mut(1) = *self.index(1) + value;
        *self.index_mut(2) = *self.index(2) + value;
        *self.index_mut(3) = *self.index(3) + value;
    }

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

    #[inline] pub fn dot(&self, other: &Vec4<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1) +
        *self.index(2) * *other.index(2) +
        *self.index(3) * *other.index(3)
    }
}

impl<T:Num> Neg<Vec4<T>> for Vec4<T> {
    #[inline]
    pub fn neg(&self) -> Vec4<T> {
        Vec4::new(-*self.index(0),
                  -*self.index(1),
                  -*self.index(2),
                  -*self.index(3))
    }
}

impl<T:Real> Vec4<T> {
    #[inline]
    pub fn magnitude2(&self) -> T {
        self.dot(self)
    }

    #[inline]
    pub fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }

    #[inline]
    pub fn angle(&self, other: &Vec4<T>) -> T {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }

    #[inline]
    pub fn normalize(&self) -> Vec4<T> {
        self.mul_t(one!(T)/self.magnitude())
    }

    #[inline]
    pub fn normalize_to(&self, magnitude: T) -> Vec4<T> {
        self.mul_t(magnitude / self.magnitude())
    }

    #[inline]
    pub fn lerp(&self, other: &Vec4<T>, amount: T) -> Vec4<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    #[inline]
    pub fn normalize_self(&mut self) {
        let rlen = self.magnitude().recip();
        self.mul_self_t(rlen);
    }

    #[inline]
    pub fn normalize_self_to(&mut self, magnitude: T) {
        let n = magnitude / self.magnitude();
        self.mul_self_t(n);
    }

    pub fn lerp_self(&mut self, other: &Vec4<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Ord> Vec4<T> {
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
}

impl<T:Eq> Vec4<T> {
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

impl Vec4<bool> {
    #[inline]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1) || *self.index(2) || *self.index(3)
    }

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
    use core::vec::*;

    static A: Vec4<float> = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    static B: Vec4<float> = Vec4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    static F1: float = 1.5;
    static F2: float = 0.5;

    #[test]
    fn test_vec4() {
        let mut mut_a = A;

        assert_eq!(Vec4::new::<float>(1.0, 2.0, 3.0, 4.0), A);
        assert_eq!(Vec4::from_value(1.0), Vec4::new::<float>(1.0, 1.0, 1.0, 1.0));

        *mut_a.index_mut(0) = 42.0;
        *mut_a.index_mut(1) = 43.0;
        *mut_a.index_mut(2) = 44.0;
        *mut_a.index_mut(3) = 45.0;
        assert_eq!(mut_a, Vec4::new::<float>(42.0, 43.0, 44.0, 45.0));
        mut_a = A;

        mut_a.swap(0, 3);
        assert_eq!(*mut_a.index(0), *A.index(3));
        assert_eq!(*mut_a.index(3), *A.index(0));
        mut_a = A;

        mut_a.swap(1, 2);
        assert_eq!(*mut_a.index(1), *A.index(2));
        assert_eq!(*mut_a.index(2), *A.index(1));
        mut_a = A;

        assert_eq!(Vec4::zero(), Vec4::new::<float>(0.0, 0.0, 0.0, 0.0));
        assert_eq!(Vec4::unit_x(), Vec4::new::<float>(1.0, 0.0, 0.0, 0.0));
        assert_eq!(Vec4::unit_y(), Vec4::new::<float>(0.0, 1.0, 0.0, 0.0));
        assert_eq!(Vec4::unit_z(), Vec4::new::<float>(0.0, 0.0, 1.0, 0.0));
        assert_eq!(Vec4::unit_w(), Vec4::new::<float>(0.0, 0.0, 0.0, 1.0));
        assert_eq!(Vec4::identity(), Vec4::new::<float>(1.0, 1.0, 1.0, 1.0));

        assert_eq!(A.x, 1.0);
        assert_eq!(A.y, 2.0);
        assert_eq!(A.z, 3.0);
        assert_eq!(A.w, 4.0);
        assert_eq!(*A.index(0), 1.0);
        assert_eq!(*A.index(1), 2.0);
        assert_eq!(*A.index(2), 3.0);
        assert_eq!(*A.index(3), 4.0);

        assert_eq!(-A, Vec4::new::<float>(-1.0, -2.0, -3.0, -4.0));
        assert_eq!(A.neg(), Vec4::new::<float>(-1.0, -2.0, -3.0, -4.0));

        assert_eq!(A.mul_t(F1), Vec4::new::<float>( 1.5, 3.0, 4.5, 6.0));
        assert_eq!(A.div_t(F2), Vec4::new::<float>( 2.0, 4.0, 6.0, 8.0));

        assert_eq!(A.add_v(&B), Vec4::new::<float>(    6.0,     8.0,    10.0,    12.0));
        assert_eq!(A.sub_v(&B), Vec4::new::<float>(   -4.0,    -4.0,    -4.0,    -4.0));
        assert_eq!(A.mul_v(&B), Vec4::new::<float>(    5.0,    12.0,    21.0,    32.0));
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
    fn test_vec4_approx_eq() {
        assert!(!Vec4::new::<float>(0.000001, 0.000001, 0.000001, 0.000001).approx_eq(&Vec4::new::<float>(0.0, 0.0, 0.0, 0.0)));
        assert!(Vec4::new::<float>(0.0000001, 0.0000001, 0.0000001, 0.0000001).approx_eq(&Vec4::new::<float>(0.0, 0.0, 0.0, 0.0)));
    }

    static E_A: Vec4<float> = Vec4 { x: 1.0, y: 2.0, z: 4.0, w: 10.0 }; // (1, 2, 4, 10, 11) Pythagorean quintuple
    static E_B: Vec4<float> = Vec4 { x: 1.0, y: 2.0, z: 8.0, w: 10.0 }; // (1, 2, 8, 10, 13) Pythagorean quintuple

    #[test]
    fn test_vec4_euclidean() {
        assert_eq!(E_A.magnitude(), 11.0);
        assert_eq!(E_A.magnitude2(), 11.0 * 11.0);

        assert_eq!(E_B.magnitude(), 13.0);
        assert_eq!(E_B.magnitude2(), 13.0 * 13.0);

        assert!(Vec4::new::<float>(1.0, 0.0, 1.0, 0.0).angle(&Vec4::new::<float>(0.0, 1.0, 0.0, 1.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec4::new::<float>(10.0, 0.0, 10.0, 0.0).angle(&Vec4::new::<float>(0.0, 5.0, 0.0, 5.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec4::new::<float>(-1.0, 0.0, -1.0, 0.0).angle(&Vec4::new::<float>(0.0, 1.0, 0.0, 1.0)).approx_eq(&Real::frac_pi_2()));

        assert!(Vec4::new::<float>(1.0, 2.0, 4.0, 10.0).normalize().approx_eq(&Vec4::new::<float>(1.0/11.0, 2.0/11.0, 4.0/11.0, 10.0/11.0)));
        // TODO: test normalize_to, normalize_self, and normalize_self_to

        let c = Vec4::new::<float>(-2.0, -1.0, 1.0, 2.0);
        let d = Vec4::new::<float>( 1.0,  0.0, 0.5, 1.0);

        assert_eq!(c.lerp(&d, 0.75), Vec4::new::<float>(0.250, -0.250, 0.625, 1.250));

        let mut mut_c = c;
        mut_c.lerp_self(&d, 0.75);
        assert_eq!(mut_c, c.lerp(&d, 0.75));
    }

    #[test]
    fn test_vec4_boolean() {
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
