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

#[macro_escape];

macro_rules! impl_vec(
    ($Vec:ident { $($field:ident),+ }) => (
        impl<T> $Vec<T> {
            #[inline]
            pub fn new($($field: T),+) -> $Vec<T> {
                $Vec { $($field: $field),+ }
            }
        }
    )
)

macro_rules! impl_vec_clonable(
    ($Vec:ident) => (
        impl<T:Clone> $Vec<T> {
            #[inline]
            pub fn from_value(value: T) -> $Vec<T> {
                vec_from_value!($Vec)
            }
        }
    )
)

macro_rules! vec_from_value(
    (Vec2) => (Vec2::new(value.clone(), value.clone()));
    (Vec3) => (Vec3::new(value.clone(), value.clone(), value.clone()));
    (Vec4) => (Vec4::new(value.clone(), value.clone(), value.clone(), value.clone()));
)

macro_rules! impl_vec_numeric(
    ($Vec:ident) => (
        impl<T:Clone + Num> $Vec<T> {
            #[inline] pub fn identity() -> $Vec<T> { $Vec::from_value(one!(T)) }
            #[inline] pub fn zero() -> $Vec<T> { $Vec::from_value(zero!(T)) }

            #[inline] pub fn add_t(&self, value: T) -> $Vec<T> { $Vec::from_slice(self.map(|&x| x + value)) }
            #[inline] pub fn sub_t(&self, value: T) -> $Vec<T> { $Vec::from_slice(self.map(|&x| x - value)) }
            #[inline] pub fn mul_t(&self, value: T) -> $Vec<T> { $Vec::from_slice(self.map(|&x| x * value)) }
            #[inline] pub fn div_t(&self, value: T) -> $Vec<T> { $Vec::from_slice(self.map(|&x| x / value)) }
            #[inline] pub fn rem_t(&self, value: T) -> $Vec<T> { $Vec::from_slice(self.map(|&x| x % value)) }

            #[inline] pub fn add_v(&self, other: &$Vec<T>) -> $Vec<T> { $Vec::from_slice(self.zip(other, |&a, &b| a + b)) }
            #[inline] pub fn sub_v(&self, other: &$Vec<T>) -> $Vec<T> { $Vec::from_slice(self.zip(other, |&a, &b| a - b)) }
            #[inline] pub fn mul_v(&self, other: &$Vec<T>) -> $Vec<T> { $Vec::from_slice(self.zip(other, |&a, &b| a * b)) }
            #[inline] pub fn div_v(&self, other: &$Vec<T>) -> $Vec<T> { $Vec::from_slice(self.zip(other, |&a, &b| a / b)) }
            #[inline] pub fn rem_v(&self, other: &$Vec<T>) -> $Vec<T> { $Vec::from_slice(self.zip(other, |&a, &b| a % b)) }

            #[inline] pub fn neg_self(&mut self) { self.map_mut(|x| *x = -*x) }
            #[inline] pub fn add_self_t(&mut self, value: T) { self.map_mut(|x| *x += value.clone()) }
            #[inline] pub fn sub_self_t(&mut self, value: T) { self.map_mut(|x| *x -= value.clone()) }
            #[inline] pub fn mul_self_t(&mut self, value: T) { self.map_mut(|x| *x *= value.clone()) }
            #[inline] pub fn div_self_t(&mut self, value: T) { self.map_mut(|x| *x /= value.clone()) }
            #[inline] pub fn rem_self_t(&mut self, value: T) { self.map_mut(|x| *x %= value.clone()) }

            #[inline] pub fn add_self_v(&mut self, other: &$Vec<T>) { self.zip_mut(other, |a, &b| *a += b) }
            #[inline] pub fn sub_self_v(&mut self, other: &$Vec<T>) { self.zip_mut(other, |a, &b| *a -= b) }
            #[inline] pub fn mul_self_v(&mut self, other: &$Vec<T>) { self.zip_mut(other, |a, &b| *a *= b) }
            #[inline] pub fn div_self_v(&mut self, other: &$Vec<T>) { self.zip_mut(other, |a, &b| *a /= b) }
            #[inline] pub fn rem_self_v(&mut self, other: &$Vec<T>) { self.zip_mut(other, |a, &b| *a %= b) }

            #[inline] pub fn dot(&self, other: &$Vec<T>) -> T { vec_dot!($Vec) }
        }
    )
)

macro_rules! vec_dot(
    (Vec2) => (
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1)
    );
    (Vec3) => (
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1) +
        *self.index(2) * *other.index(2)
    );
    (Vec4) => (
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1) +
        *self.index(2) * *other.index(2) +
        *self.index(3) * *other.index(3)
    );
)

macro_rules! impl_vec_neg(
    ($Vec:ident) => (
        impl<T:Clone + Num> Neg<$Vec<T>> for $Vec<T> {
            #[inline]
            pub fn neg(&self) -> $Vec<T> {
                $Vec::from_slice(self.map(|&x| -x))
            }
        }
    )
)

macro_rules! impl_vec_euclidean(
    ($Vec:ident) => (
        impl<T:Clone + Real> $Vec<T> {
            #[inline]
            pub fn length2(&self) -> T {
                self.dot(self)
            }

            #[inline]
            pub fn length(&self) -> T {
                self.length2().sqrt()
            }

            #[inline]
            pub fn distance2(&self, other: &$Vec<T>) -> T {
                other.sub_v(self).length2()
            }

            #[inline]
            pub fn distance(&self, other: &$Vec<T>) -> T {
                other.distance2(self).sqrt()
            }

            #[inline]
            pub fn angle(&self, other: &$Vec<T>) -> T {
                vec_angle!($Vec)
            }

            #[inline]
            pub fn normalize(&self) -> $Vec<T> {
                self.mul_t(one!(T)/self.length())
            }

            #[inline]
            pub fn normalize_to(&self, length: T) -> $Vec<T> {
                self.mul_t(length / self.length())
            }

            #[inline]
            pub fn lerp(&self, other: &$Vec<T>, amount: T) -> $Vec<T> {
                self.add_v(&other.sub_v(self).mul_t(amount))
            }

            #[inline]
            pub fn normalize_self(&mut self) {
                let rlen = self.length().recip();
                self.mul_self_t(rlen);
            }

            #[inline]
            pub fn normalize_self_to(&mut self, length: T) {
                let n = length / self.length();
                self.mul_self_t(n);
            }

            pub fn lerp_self(&mut self, other: &$Vec<T>, amount: T) {
                let v = other.sub_v(self).mul_t(amount);
                self.add_self_v(&v);
            }
        }
    )
)

macro_rules! vec_angle(
    (Vec2) => (self.perp_dot(other).atan2(&self.dot(other)));
    (Vec3) => (self.cross(other).length().atan2(&self.dot(other)));
    (Vec4) => ((self.dot(other) / (self.length() * other.length())).acos());
)

macro_rules! impl_vec_ord(
    ($Vec:ident) => (
        impl<T:Clone + Ord> $Vec<T> {
            #[inline] pub fn lt_t(&self, value: T) -> $Vec<bool> { $Vec::from_slice(self.map(|&x| x < value)) }
            #[inline] pub fn le_t(&self, value: T) -> $Vec<bool> { $Vec::from_slice(self.map(|&x| x <= value)) }
            #[inline] pub fn ge_t(&self, value: T) -> $Vec<bool> { $Vec::from_slice(self.map(|&x| x >= value)) }
            #[inline] pub fn gt_t(&self, value: T) -> $Vec<bool> { $Vec::from_slice(self.map(|&x| x > value)) }

            #[inline] pub fn lt_v(&self, other: &$Vec<T>) -> $Vec<bool> { $Vec::from_slice(self.zip(other, |&a, &b| a < b)) }
            #[inline] pub fn le_v(&self, other: &$Vec<T>) -> $Vec<bool> { $Vec::from_slice(self.zip(other, |&a, &b| a <= b)) }
            #[inline] pub fn ge_v(&self, other: &$Vec<T>) -> $Vec<bool> { $Vec::from_slice(self.zip(other, |&a, &b| a >= b)) }
            #[inline] pub fn gt_v(&self, other: &$Vec<T>) -> $Vec<bool> { $Vec::from_slice(self.zip(other, |&a, &b| a > b)) }
        }
    )
)

macro_rules! impl_vec_eq(
    ($Vec:ident) => (
        impl<T:Clone + Eq> $Vec<T> {
            #[inline] pub fn eq_t(&self, value: T) -> $Vec<bool> { $Vec::from_slice(self.map(|&x| x == value)) }
            #[inline] pub fn ne_t(&self, value: T) -> $Vec<bool> { $Vec::from_slice(self.map(|&x| x != value)) }

            #[inline] pub fn eq_v(&self, other: &$Vec<T>) -> $Vec<bool> { $Vec::from_slice(self.zip(other, |&a, &b| a == b)) }
            #[inline] pub fn ne_v(&self, other: &$Vec<T>) -> $Vec<bool> { $Vec::from_slice(self.zip(other, |&a, &b| a != b)) }
        }
    )
)

macro_rules! impl_vec_bool(
    ($Vec:ident) => (
        impl $Vec<bool> {
            #[inline]
            pub fn any(&self) -> bool { vec_any!($Vec) }

            #[inline]
            pub fn all(&self) -> bool { vec_all!($Vec) }

            #[inline]
            pub fn not(&self) -> $Vec<bool> {
                $Vec::from_slice(self.map(|&x| !x))
            }
        }
    )
)

macro_rules! vec_any(
    (Vec2) => (*self.index(0) || *self.index(1));
    (Vec3) => (*self.index(0) || *self.index(1) || *self.index(2));
    (Vec4) => (*self.index(0) || *self.index(1) || *self.index(2) || *self.index(3));
)

macro_rules! vec_all(
    (Vec2) => (*self.index(0) && *self.index(1));
    (Vec3) => (*self.index(0) && *self.index(1) && *self.index(2));
    (Vec4) => (*self.index(0) && *self.index(1) && *self.index(2) && *self.index(3));
)

macro_rules! impl_vec_not(
    ($Vec:ident) => (
        impl<T:Clone + Not<T>> Not<$Vec<T>> for $Vec<T> {
            pub fn not(&self) -> $Vec<T> {
                $Vec::from_slice(self.map(|&x| !x))
            }
        }
    )
)
