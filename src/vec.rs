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

use std::cast::transmute;
use std::cmp::ApproxEq;
use std::num::{Zero, One};

 #[deriving(Eq)]
pub struct Vec2<T> { x: T, y: T }

impl<T> Vec2<T> {
    #[inline(always)]
    pub fn index<'a>(&'a self, i: uint) -> &'a T {
        &'a self.as_slice()[i]
    }

    #[inline(always)]
    pub fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline(always)]
    pub fn as_slice<'a>(&'a self) -> &'a [T,..2] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T,..2] {
        unsafe { transmute(self) }
    }
}

impl<T:Copy> Vec2<T> {
    #[inline(always)]
    pub fn new(x: T, y: T ) -> Vec2<T> {
        Vec2 { x: x, y: y }
    }

    #[inline(always)]
    pub fn from_value(value: T) -> Vec2<T> {
        Vec2::new(value, value)
    }

    #[inline(always)]
    pub fn swap(&mut self, a: uint, b: uint) {
        let tmp = *self.index(a);
        *self.index_mut(a) = *self.index(b);
        *self.index_mut(b) = tmp;
    }

    #[inline(always)]
    pub fn map(&self, f: &fn(&T) -> T) -> Vec2<T> {
        Vec2::new(f(self.index(0)),
                  f(self.index(1)))
    }
}

impl<T:Copy + Num> Vec2<T> {
    #[inline(always)]
    pub fn identity() -> Vec2<T> {
        Vec2::new(One::one::<T>(), One::one::<T>())
    }

    #[inline(always)]
    pub fn zero() -> Vec2<T> {
        Vec2::new(Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn unit_x() -> Vec2<T> {
        Vec2::new(One::one::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn unit_y() -> Vec2<T> {
        Vec2::new(Zero::zero::<T>(), One::one::<T>())
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self.index(0) == Zero::zero() &&
        *self.index(1) == Zero::zero()
    }

    #[inline(always)]
    pub fn add_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) + value,
                  *self.index(1) + value)
    }

    #[inline(always)]
    pub fn sub_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) - value,
                  *self.index(1) - value)
    }

    #[inline(always)]
    pub fn mul_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) * value,
                  *self.index(1) * value)
    }

    #[inline(always)]
    pub fn div_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) / value,
                  *self.index(1) / value)
    }

    #[inline(always)]
    pub fn rem_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) % value,
                  *self.index(1) % value)
    }

    #[inline(always)]
    pub fn add_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1))
    }

    #[inline(always)]
    pub fn sub_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1))
    }

    #[inline(always)]
    pub fn mul_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) * *other.index(0),
                  *self.index(1) * *other.index(1))
    }

    #[inline(always)]
    pub fn div_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) / *other.index(0),
                  *self.index(1) / *other.index(1))
    }

    #[inline(always)]
    pub fn rem_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) % *other.index(0),
                  *self.index(1) % *other.index(1))
    }

    #[inline(always)]
    pub fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index(0);
        *self.index_mut(1) = -*self.index(1);
    }

    #[inline(always)]
    pub fn add_self_t(&mut self, value: T) {
        *self.index_mut(0) += value;
        *self.index_mut(1) += value;
    }

    #[inline(always)]
    pub fn sub_self_t(&mut self, value: T) {
        *self.index_mut(0) -= value;
        *self.index_mut(1) -= value;
    }

    #[inline(always)]
    pub fn mul_self_t(&mut self, value: T) {
        *self.index_mut(0) *= value;
        *self.index_mut(1) *= value;
    }

    #[inline(always)]
    pub fn div_self_t(&mut self, value: T) {
        *self.index_mut(0) /= value;
        *self.index_mut(1) /= value;
    }

    #[inline(always)]
    pub fn rem_self_t(&mut self, value: T) {
        *self.index_mut(0) %= value;
        *self.index_mut(1) %= value;
    }

    #[inline(always)]
    pub fn add_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) += *other.index(0);
        *self.index_mut(1) += *other.index(1);
    }

    #[inline(always)]
    pub fn sub_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) -= *other.index(0);
        *self.index_mut(1) -= *other.index(1);
    }

    #[inline(always)]
    pub fn mul_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) *= *other.index(0);
        *self.index_mut(1) *= *other.index(1);
    }

    #[inline(always)]
    pub fn div_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) /= *other.index(0);
        *self.index_mut(1) /= *other.index(1);
    }

    #[inline(always)]
    pub fn rem_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) /= *other.index(0);
        *self.index_mut(1) /= *other.index(1);
    }

    #[inline(always)]
    pub fn dot(&self, other: &Vec2<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1)
    }

    #[inline(always)]
    pub fn perp_dot(&self, other: &Vec2<T>) -> T {
        (*self.index(0) * *other.index(1)) -
        (*self.index(1) * *other.index(0))
    }

    #[inline(always)]
    pub fn to_homogeneous(&self) -> Vec3<T> {
        Vec3::new(self.x, self.y, Zero::zero())
    }
}

impl<T:Copy + Num> Neg<Vec2<T>> for Vec2<T> {
    #[inline(always)]
    pub fn neg(&self) -> Vec2<T> {
        Vec2::new(-self.index(0), -self.index(1))
    }
}

impl<T:Copy + Real> Vec2<T> {
    #[inline(always)]
    pub fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline(always)]
    pub fn distance2(&self, other: &Vec2<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline(always)]
    pub fn distance(&self, other: &Vec2<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline(always)]
    pub fn angle(&self, other: &Vec2<T>) -> T {
        self.perp_dot(other).atan2(self.dot(other))
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vec2<T> {
        self.mul_t(One::one::<T>()/self.length())
    }

    #[inline(always)]
    pub fn normalize_to(&self, length: T) -> Vec2<T> {
        self.mul_t(length / self.length())
    }

    #[inline(always)]
    pub fn lerp(&self, other: &Vec2<T>, amount: T) -> Vec2<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    #[inline(always)]
    pub fn normalize_self(&mut self) {
        let n = One::one::<T>() / self.length();
        self.mul_self_t(n);
    }

    #[inline(always)]
    pub fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    pub fn lerp_self(&mut self, other: &Vec2<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Vec2<T> {
    #[inline(always)]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    pub fn approx_eq(&self, other: &Vec2<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    pub fn approx_eq_eps(&self, other: &Vec2<T>, epsilon: &T) -> bool {
        self.index(0).approx_eq_eps(other.index(0), epsilon) &&
        self.index(1).approx_eq_eps(other.index(1), epsilon)
    }
}

impl<T:Copy + Ord> Vec2<T> {
    #[inline(always)]
    pub fn lt_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) < value,
                  *self.index(1) < value)
    }

    #[inline(always)]
    pub fn le_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) <= value,
                  *self.index(1) <= value)
    }

    #[inline(always)]
    pub fn ge_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) >= value,
                  *self.index(1) >= value)
    }

    #[inline(always)]
    pub fn gt_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) > value,
                  *self.index(1) > value)
    }

    #[inline(always)]
    pub fn lt_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) < *other.index(0),
                  *self.index(1) < *other.index(1))
    }

    #[inline(always)]
    pub fn le_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) <= *other.index(0),
                  *self.index(1) <= *other.index(1))
    }

    #[inline(always)]
    pub fn ge_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) >= *other.index(0),
                  *self.index(1) >= *other.index(1))
    }

    #[inline(always)]
    pub fn gt_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) > *other.index(0),
                  *self.index(1) > *other.index(1))
    }
}

impl<T:Copy + Eq> Vec2<T> {
    #[inline(always)]
    pub fn eq_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) == value,
                  *self.index(1) == value)
    }

    #[inline(always)]
    pub fn ne_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) != value,
                  *self.index(1) != value)
    }

    #[inline(always)]
    pub fn eq_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) == *other.index(0),
                  *self.index(1) == *other.index(1))
    }

    #[inline(always)]
    pub fn ne_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) != *other.index(0),
                  *self.index(1) != *other.index(1))
    }
}

impl Vec2<bool> {
    #[inline(always)]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1)
    }

    #[inline(always)]
    pub fn all(&self) -> bool {
        *self.index(0) && *self.index(1)
    }

    #[inline(always)]
    pub fn not(&self) -> Vec2<bool> {
        Vec2::new(!*self.index(0), !*self.index(1))
    }
}

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

#[deriving(Eq)]
pub struct Vec3<T> { x: T, y: T, z: T }

impl<T> Vec3<T> {
    #[inline(always)]
    pub fn index<'a>(&'a self, i: uint) -> &'a T {
        &'a self.as_slice()[i]
    }

    #[inline(always)]
    pub fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline(always)]
    pub fn as_slice<'a>(&'a self) -> &'a [T,..3] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T,..3] {
        unsafe { transmute(self) }
    }
}

impl<T:Copy> Vec3<T> {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T ) -> Vec3<T> {
        Vec3 { x: x, y: y, z: z }
    }

    #[inline(always)]
    pub fn from_value(value: T) -> Vec3<T> {
        Vec3::new(value, value, value)
    }

    #[inline(always)]
    pub fn swap(&mut self, a: uint, b: uint) {
        let tmp = *self.index(a);
        *self.index_mut(a) = *self.index(b);
        *self.index_mut(b) = tmp;
    }

    #[inline(always)]
    pub fn map(&self, f: &fn(&T) -> T) -> Vec3<T> {
        Vec3::new(f(self.index(0)),
                  f(self.index(1)),
                  f(self.index(2)))
    }
}

impl<T:Copy + Num> Vec3<T> {
    #[inline(always)]
    pub fn identity() -> Vec3<T> {
        Vec3::new(One::one::<T>(), One::one::<T>(), One::one::<T>())
    }

    #[inline(always)]
    pub fn zero() -> Vec3<T> {
        Vec3::new(Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn unit_x() -> Vec3<T> {
        Vec3::new(One::one::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn unit_y() -> Vec3<T> {
        Vec3::new(Zero::zero::<T>(), One::one::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn unit_z() -> Vec3<T> {
        Vec3::new(Zero::zero::<T>(), Zero::zero::<T>(), One::one::<T>())
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self.index(0) == Zero::zero() &&
        *self.index(1) == Zero::zero() &&
        *self.index(2) == Zero::zero()
    }

    #[inline(always)]
    pub fn add_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) + value,
                  *self.index(1) + value,
                  *self.index(2) + value)
    }

    #[inline(always)]
    pub fn sub_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) - value,
                  *self.index(1) - value,
                  *self.index(2) - value)
    }

    #[inline(always)]
    pub fn mul_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) * value,
                  *self.index(1) * value,
                  *self.index(2) * value)
    }

    #[inline(always)]
    pub fn div_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) / value,
                  *self.index(1) / value,
                  *self.index(2) / value)
    }

    #[inline(always)]
    pub fn rem_t(&self, value: T) -> Vec3<T> {
        Vec3::new(*self.index(0) % value,
                  *self.index(1) % value,
                  *self.index(2) % value)
    }

    #[inline(always)]
    pub fn add_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1),
                  *self.index(2) + *other.index(2))
    }

    #[inline(always)]
    pub fn sub_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1),
                  *self.index(2) - *other.index(2))
    }

    #[inline(always)]
    pub fn mul_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) * *other.index(0),
                  *self.index(1) * *other.index(1),
                  *self.index(2) * *other.index(2))
    }

    #[inline(always)]
    pub fn div_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) / *other.index(0),
                  *self.index(1) / *other.index(1),
                  *self.index(2) / *other.index(2))
    }

    #[inline(always)]
    pub fn rem_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(*self.index(0) % *other.index(0),
                  *self.index(1) % *other.index(1),
                  *self.index(2) % *other.index(2))
    }

    #[inline(always)]
    pub fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index(0);
        *self.index_mut(1) = -*self.index(1);
        *self.index_mut(2) = -*self.index(2);
    }

    #[inline(always)]
    pub fn add_self_t(&mut self, value: T) {
        *self.index_mut(0) += value;
        *self.index_mut(1) += value;
        *self.index_mut(2) += value;
    }

    #[inline(always)]
    pub fn sub_self_t(&mut self, value: T) {
        *self.index_mut(0) -= value;
        *self.index_mut(1) -= value;
        *self.index_mut(2) -= value;
    }

    #[inline(always)]
    pub fn mul_self_t(&mut self, value: T) {
        *self.index_mut(0) *= value;
        *self.index_mut(1) *= value;
        *self.index_mut(2) *= value;
    }

    #[inline(always)]
    pub fn div_self_t(&mut self, value: T) {
        *self.index_mut(0) /= value;
        *self.index_mut(1) /= value;
        *self.index_mut(2) /= value;
    }

    #[inline(always)]
    pub fn rem_self_t(&mut self, value: T) {
        *self.index_mut(0) %= value;
        *self.index_mut(1) %= value;
        *self.index_mut(2) %= value;
    }

    #[inline(always)]
    pub fn add_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) += *other.index(0);
        *self.index_mut(1) += *other.index(1);
        *self.index_mut(2) += *other.index(2);
    }

    #[inline(always)]
    pub fn sub_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) -= *other.index(0);
        *self.index_mut(1) -= *other.index(1);
        *self.index_mut(2) -= *other.index(2);
    }

    #[inline(always)]
    pub fn mul_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) *= *other.index(0);
        *self.index_mut(1) *= *other.index(1);
        *self.index_mut(2) *= *other.index(2);
    }

    #[inline(always)]
    pub fn div_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) /= *other.index(0);
        *self.index_mut(1) /= *other.index(1);
        *self.index_mut(2) /= *other.index(2);
    }

    #[inline(always)]
    pub fn rem_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) /= *other.index(0);
        *self.index_mut(1) /= *other.index(1);
        *self.index_mut(2) /= *other.index(2);
    }

    #[inline(always)]
    pub fn dot(&self, other: &Vec3<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1) +
        *self.index(2) * *other.index(2)
    }

    #[inline(always)]
    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new((*self.index(1) * *other.index(2)) - (*self.index(2) * *other.index(1)),
                  (*self.index(2) * *other.index(0)) - (*self.index(0) * *other.index(2)),
                  (*self.index(0) * *other.index(1)) - (*self.index(1) * *other.index(0)))
    }

    #[inline(always)]
    pub fn cross_self(&mut self, other: &Vec3<T>) {
        *self = self.cross(other)
    }

    #[inline(always)]
    pub fn to_homogeneous(&self) -> Vec4<T> {
        Vec4::new(self.x, self.y, self.z, Zero::zero())
    }
}

impl<T:Copy + Num> Neg<Vec3<T>> for Vec3<T> {
    #[inline(always)]
    pub fn neg(&self) -> Vec3<T> {
        Vec3::new(-self.index(0), -self.index(1), -self.index(2))
    }
}

impl<T:Copy + Real> Vec3<T> {
    #[inline(always)]
    pub fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline(always)]
    pub fn distance2(&self, other: &Vec3<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline(always)]
    pub fn distance(&self, other: &Vec3<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline(always)]
    pub fn angle(&self, other: &Vec3<T>) -> T {
        self.cross(other).length().atan2(self.dot(other))
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vec3<T> {
        self.mul_t(One::one::<T>()/self.length())
    }

    #[inline(always)]
    pub fn normalize_to(&self, length: T) -> Vec3<T> {
        self.mul_t(length / self.length())
    }

    #[inline(always)]
    pub fn lerp(&self, other: &Vec3<T>, amount: T) -> Vec3<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    #[inline(always)]
    pub fn normalize_self(&mut self) {
        let n = One::one::<T>() / self.length();
        self.mul_self_t(n);
    }

    #[inline(always)]
    pub fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    pub fn lerp_self(&mut self, other: &Vec3<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Vec3<T> {
    #[inline(always)]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    pub fn approx_eq(&self, other: &Vec3<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    pub fn approx_eq_eps(&self, other: &Vec3<T>, epsilon: &T) -> bool {
        self.index(0).approx_eq_eps(other.index(0), epsilon) &&
        self.index(1).approx_eq_eps(other.index(1), epsilon) &&
        self.index(2).approx_eq_eps(other.index(2), epsilon)
    }
}

impl<T:Copy + Ord> Vec3<T> {
    #[inline(always)]
    pub fn lt_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) < value,
                  *self.index(1) < value,
                  *self.index(2) < value)
    }

    #[inline(always)]
    pub fn le_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) <= value,
                  *self.index(1) <= value,
                  *self.index(2) <= value)
    }

    #[inline(always)]
    pub fn ge_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) >= value,
                  *self.index(1) >= value,
                  *self.index(2) >= value)
    }

    #[inline(always)]
    pub fn gt_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) > value,
                  *self.index(1) > value,
                  *self.index(2) > value)
    }

    #[inline(always)]
    pub fn lt_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) < *other.index(0),
                  *self.index(1) < *other.index(1),
                  *self.index(2) < *other.index(2))
    }

    #[inline(always)]
    pub fn le_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) <= *other.index(0),
                  *self.index(1) <= *other.index(1),
                  *self.index(2) <= *other.index(2))
    }

    #[inline(always)]
    pub fn ge_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) >= *other.index(0),
                  *self.index(1) >= *other.index(1),
                  *self.index(2) >= *other.index(2))
    }

    #[inline(always)]
    pub fn gt_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) > *other.index(0),
                  *self.index(1) > *other.index(1),
                  *self.index(2) > *other.index(2))
    }
}

impl<T:Copy + Eq> Vec3<T> {
    #[inline(always)]
    pub fn eq_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) == value,
                  *self.index(1) == value,
                  *self.index(2) == value)
    }

    #[inline(always)]
    pub fn ne_t(&self, value: T) -> Vec3<bool> {
        Vec3::new(*self.index(0) != value,
                  *self.index(1) != value,
                  *self.index(2) != value)
    }

    #[inline(always)]
    pub fn eq_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) == *other.index(0),
                  *self.index(1) == *other.index(1),
                  *self.index(2) == *other.index(2))
    }

    #[inline(always)]
    pub fn ne_v(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(*self.index(0) != *other.index(0),
                  *self.index(1) != *other.index(1),
                  *self.index(2) != *other.index(2))
    }
}

impl Vec3<bool> {
    #[inline(always)]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1) || *self.index(2)
    }

    #[inline(always)]
    pub fn all(&self) -> bool {
        *self.index(0) && *self.index(1) && *self.index(2)
    }

    #[inline(always)]
    pub fn not(&self) -> Vec3<bool> {
        Vec3::new(!*self.index(0), !*self.index(1), !*self.index(2))
    }
}

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

#[deriving(Eq)]
pub struct Vec4<T> { x: T, y: T, z: T, w: T }

impl<T> Vec4<T> {
    #[inline(always)]
    pub fn index<'a>(&'a self, i: uint) -> &'a T {
        &'a self.as_slice()[i]
    }

    #[inline(always)]
    pub fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline(always)]
    pub fn as_slice<'a>(&'a self) -> &'a [T,..4] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T,..4] {
        unsafe { transmute(self) }
    }
}

impl<T:Copy> Vec4<T> {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T, w: T ) -> Vec4<T> {
        Vec4 { x: x, y: y, z: z, w: w }
    }

    #[inline(always)]
    pub fn from_value(value: T) -> Vec4<T> {
        Vec4::new(value, value, value, value)
    }

    #[inline(always)]
    pub fn swap(&mut self, a: uint, b: uint) {
        let tmp = *self.index(a);
        *self.index_mut(a) = *self.index(b);
        *self.index_mut(b) = tmp;
    }

    #[inline(always)]
    pub fn map(&self, f: &fn(&T) -> T) -> Vec4<T> {
        Vec4::new(f(self.index(0)),
                  f(self.index(1)),
                  f(self.index(2)),
                  f(self.index(3)))
    }
}

impl<T:Copy + Num> Vec4<T> {
    #[inline(always)]
    pub fn identity() -> Vec4<T> {
        Vec4::new(One::one::<T>(), One::one::<T>(), One::one::<T>(), One::one::<T>())
    }

    #[inline(always)]
    pub fn zero() -> Vec4<T> {
        Vec4::new(Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn unit_x() -> Vec4<T> {
        Vec4::new(One::one::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn unit_y() -> Vec4<T> {
        Vec4::new(Zero::zero::<T>(), One::one::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn unit_z() -> Vec4<T> {
        Vec4::new(Zero::zero::<T>(), Zero::zero::<T>(), One::one::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn unit_w() -> Vec4<T> {
        Vec4::new(Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), One::one::<T>())
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self.index(0) == Zero::zero() &&
        *self.index(1) == Zero::zero() &&
        *self.index(2) == Zero::zero() &&
        *self.index(3) == Zero::zero()
    }

    #[inline(always)]
    pub fn add_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) + value,
                  *self.index(1) + value,
                  *self.index(2) + value,
                  *self.index(3) + value)
    }

    #[inline(always)]
    pub fn sub_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) - value,
                  *self.index(1) - value,
                  *self.index(2) - value,
                  *self.index(3) - value)
    }

    #[inline(always)]
    pub fn mul_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) * value,
                  *self.index(1) * value,
                  *self.index(2) * value,
                  *self.index(3) * value)
    }

    #[inline(always)]
    pub fn div_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) / value,
                  *self.index(1) / value,
                  *self.index(2) / value,
                  *self.index(3) / value)
    }

    #[inline(always)]
    pub fn rem_t(&self, value: T) -> Vec4<T> {
        Vec4::new(*self.index(0) % value,
                  *self.index(1) % value,
                  *self.index(2) % value,
                  *self.index(3) % value)
    }

    #[inline(always)]
    pub fn add_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1),
                  *self.index(2) + *other.index(2),
                  *self.index(3) + *other.index(3))
    }

    #[inline(always)]
    pub fn sub_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1),
                  *self.index(2) - *other.index(2),
                  *self.index(3) - *other.index(3))
    }

    #[inline(always)]
    pub fn mul_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) * *other.index(0),
                  *self.index(1) * *other.index(1),
                  *self.index(2) * *other.index(2),
                  *self.index(3) * *other.index(3))
    }

    #[inline(always)]
    pub fn div_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) / *other.index(0),
                  *self.index(1) / *other.index(1),
                  *self.index(2) / *other.index(2),
                  *self.index(3) / *other.index(3))
    }

    #[inline(always)]
    pub fn rem_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(*self.index(0) % *other.index(0),
                  *self.index(1) % *other.index(1),
                  *self.index(2) % *other.index(2),
                  *self.index(3) % *other.index(3))
    }

    #[inline(always)]
    pub fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index(0);
        *self.index_mut(1) = -*self.index(1);
        *self.index_mut(2) = -*self.index(2);
        *self.index_mut(3) = -*self.index(3);
    }

    #[inline(always)]
    pub fn add_self_t(&mut self, value: T) {
        *self.index_mut(0) += value;
        *self.index_mut(1) += value;
        *self.index_mut(2) += value;
        *self.index_mut(3) += value;
    }

    #[inline(always)]
    pub fn sub_self_t(&mut self, value: T) {
        *self.index_mut(0) -= value;
        *self.index_mut(1) -= value;
        *self.index_mut(2) -= value;
        *self.index_mut(3) -= value;
    }

    #[inline(always)]
    pub fn mul_self_t(&mut self, value: T) {
        *self.index_mut(0) *= value;
        *self.index_mut(1) *= value;
        *self.index_mut(2) *= value;
        *self.index_mut(3) *= value;
    }

    #[inline(always)]
    pub fn div_self_t(&mut self, value: T) {
        *self.index_mut(0) /= value;
        *self.index_mut(1) /= value;
        *self.index_mut(2) /= value;
        *self.index_mut(3) /= value;
    }

    #[inline(always)]
    pub fn rem_self_t(&mut self, value: T) {
        *self.index_mut(0) %= value;
        *self.index_mut(1) %= value;
        *self.index_mut(2) %= value;
        *self.index_mut(3) %= value;
    }

    #[inline(always)]
    pub fn add_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) += *other.index(0);
        *self.index_mut(1) += *other.index(1);
        *self.index_mut(2) += *other.index(2);
        *self.index_mut(3) += *other.index(3);
    }

    #[inline(always)]
    pub fn sub_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) -= *other.index(0);
        *self.index_mut(1) -= *other.index(1);
        *self.index_mut(2) -= *other.index(2);
        *self.index_mut(3) -= *other.index(3);
    }

    #[inline(always)]
    pub fn mul_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) *= *other.index(0);
        *self.index_mut(1) *= *other.index(1);
        *self.index_mut(2) *= *other.index(2);
        *self.index_mut(3) *= *other.index(3);
    }

    #[inline(always)]
    pub fn div_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) /= *other.index(0);
        *self.index_mut(1) /= *other.index(1);
        *self.index_mut(2) /= *other.index(2);
        *self.index_mut(3) /= *other.index(3);
    }

    #[inline(always)]
    pub fn rem_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) /= *other.index(0);
        *self.index_mut(1) /= *other.index(1);
        *self.index_mut(2) /= *other.index(2);
        *self.index_mut(3) /= *other.index(3);
    }

    #[inline(always)]
    pub fn dot(&self, other: &Vec4<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1) +
        *self.index(2) * *other.index(2) +
        *self.index(3) * *other.index(3)
    }
}

impl<T:Copy + Num> Neg<Vec4<T>> for Vec4<T> {
    #[inline(always)]
    pub fn neg(&self) -> Vec4<T> {
        Vec4::new(-self.index(0), -self.index(1), -self.index(2), -self.index(3))
    }
}

impl<T:Copy + Real> Vec4<T> {
    #[inline(always)]
    pub fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline(always)]
    pub fn distance2(&self, other: &Vec4<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline(always)]
    pub fn distance(&self, other: &Vec4<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline(always)]
    pub fn angle(&self, other: &Vec4<T>) -> T {
        (self.dot(other) / (self.length() * other.length())).acos()
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vec4<T> {
        self.mul_t(One::one::<T>()/self.length())
    }

    #[inline(always)]
    pub fn normalize_to(&self, length: T) -> Vec4<T> {
        self.mul_t(length / self.length())
    }

    #[inline(always)]
    pub fn lerp(&self, other: &Vec4<T>, amount: T) -> Vec4<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    #[inline(always)]
    pub fn normalize_self(&mut self) {
        let n = One::one::<T>() / self.length();
        self.mul_self_t(n);
    }

    #[inline(always)]
    pub fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    pub fn lerp_self(&mut self, other: &Vec4<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Vec4<T> {
    #[inline(always)]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    pub fn approx_eq(&self, other: &Vec4<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    pub fn approx_eq_eps(&self, other: &Vec4<T>, epsilon: &T) -> bool {
        self.index(0).approx_eq_eps(other.index(0), epsilon) &&
        self.index(1).approx_eq_eps(other.index(1), epsilon) &&
        self.index(2).approx_eq_eps(other.index(2), epsilon) &&
        self.index(3).approx_eq_eps(other.index(3), epsilon)
    }
}

impl<T:Copy + Ord> Vec4<T> {
    #[inline(always)]
    pub fn lt_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) < value,
                  *self.index(1) < value,
                  *self.index(2) < value,
                  *self.index(3) < value)
    }

    #[inline(always)]
    pub fn le_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) <= value,
                  *self.index(1) <= value,
                  *self.index(2) <= value,
                  *self.index(3) <= value)
    }

    #[inline(always)]
    pub fn ge_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) >= value,
                  *self.index(1) >= value,
                  *self.index(2) >= value,
                  *self.index(3) >= value)
    }

    #[inline(always)]
    pub fn gt_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) > value,
                  *self.index(1) > value,
                  *self.index(2) > value,
                  *self.index(3) > value)
    }

    #[inline(always)]
    pub fn lt_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) < *other.index(0),
                  *self.index(1) < *other.index(1),
                  *self.index(2) < *other.index(2),
                  *self.index(3) < *other.index(3))
    }

    #[inline(always)]
    pub fn le_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) <= *other.index(0),
                  *self.index(1) <= *other.index(1),
                  *self.index(2) <= *other.index(2),
                  *self.index(3) <= *other.index(3))
    }

    #[inline(always)]
    pub fn ge_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) >= *other.index(0),
                  *self.index(1) >= *other.index(1),
                  *self.index(2) >= *other.index(2),
                  *self.index(3) >= *other.index(3))
    }

    #[inline(always)]
    pub fn gt_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) > *other.index(0),
                  *self.index(1) > *other.index(1),
                  *self.index(2) > *other.index(2),
                  *self.index(3) > *other.index(3))
    }
}

impl<T:Copy + Eq> Vec4<T> {
    #[inline(always)]
    pub fn eq_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) == value,
                  *self.index(1) == value,
                  *self.index(2) == value,
                  *self.index(3) == value)
    }

    #[inline(always)]
    pub fn ne_t(&self, value: T) -> Vec4<bool> {
        Vec4::new(*self.index(0) != value,
                  *self.index(1) != value,
                  *self.index(2) != value,
                  *self.index(3) != value)
    }

    #[inline(always)]
    pub fn eq_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) == *other.index(0),
                  *self.index(1) == *other.index(1),
                  *self.index(2) == *other.index(2),
                  *self.index(3) == *other.index(3))
    }

    #[inline(always)]
    pub fn ne_v(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(*self.index(0) != *other.index(0),
                  *self.index(1) != *other.index(1),
                  *self.index(2) != *other.index(2),
                  *self.index(3) != *other.index(3))
    }
}

impl Vec4<bool> {
    #[inline(always)]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1) || *self.index(2) || *self.index(3)
    }

    #[inline(always)]
    pub fn all(&self) -> bool {
        *self.index(0) && *self.index(1) && *self.index(2) && *self.index(3)
    }

    #[inline(always)]
    pub fn not(&self) -> Vec4<bool> {
        Vec4::new(!*self.index(0), !*self.index(1), !*self.index(2), !*self.index(3))
    }
}

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
