// Copyright 2013 The CGMath Developers. For a full listing of the authors,
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

use std::num::{Zero, zero, One, one, sqrt};

use angle::{Rad, atan2};
use array::Array;

/// A 2-dimensional vector.
#[deriving(Eq, Clone, Zero)]
pub struct Vec2<S> { x: S, y: S }

/// A 3-dimensional vector.
#[deriving(Eq, Clone, Zero)]
pub struct Vec3<S> { x: S, y: S, z: S }

/// A 4-dimensional vector.
#[deriving(Eq, Clone, Zero)]
pub struct Vec4<S> { x: S, y: S, z: S, w: S }

// Conversion traits
pub trait ToVec2<S: Clone + Num + Ord> { fn to_vec2(&self) -> Vec2<S>; }
pub trait ToVec3<S: Clone + Num + Ord> { fn to_vec3(&self) -> Vec3<S>; }
pub trait ToVec4<S: Clone + Num + Ord> { fn to_vec4(&self) -> Vec4<S>; }

approx_eq!(impl<S> Vec2<S>)
approx_eq!(impl<S> Vec3<S>)
approx_eq!(impl<S> Vec4<S>)

// Utility macro for generating associated functions for the vectors
macro_rules! vec(
    (impl $Self:ident <$S:ident> { $($field:ident),+ }) => (
        impl<$S: Clone + Num + Ord> $Self<$S> {
            #[inline]
            pub fn new($($field: $S),+) -> $Self<$S> {
                $Self { $($field: $field),+ }
            }

            /// Construct a vector from a single value.
            #[inline]
            pub fn from_value(value: $S) -> $Self<$S> {
                Array::build(|_| value.clone())
            }

            /// The additive identity of the vector.
            #[inline]
            pub fn zero() -> $Self<$S> { $Self::from_value(zero()) }

            /// The additive identity of the vector.
            #[inline]
            pub fn ident() -> $Self<$S> { $Self::from_value(one()) }
        }
    )
)

vec!(impl Vec2<S> { x, y })
vec!(impl Vec3<S> { x, y, z })
vec!(impl Vec4<S> { x, y, z, w })

array!(impl<S> Vec2<S> -> [S, ..2])
array!(impl<S> Vec3<S> -> [S, ..3])
array!(impl<S> Vec4<S> -> [S, ..4])

impl<S: Clone + Num + Ord> One for Vec2<S> { #[inline] fn one() -> Vec2<S> { Vec2::ident() } }
impl<S: Clone + Num + Ord> One for Vec3<S> { #[inline] fn one() -> Vec3<S> { Vec3::ident() } }
impl<S: Clone + Num + Ord> One for Vec4<S> { #[inline] fn one() -> Vec4<S> { Vec4::ident() } }

/// A trait that specifies a range of numeric operations for vectors. Not all
/// of these make sense from a linear algebra point of view, but are included
/// for pragmatic reasons.
pub trait Vector
<
    S: Clone + Num + Ord,
    Slice
>
:   Array<S, Slice>
+   Neg<Self>
+   Zero + One
{
    // TODO: These method impls use iterators and higher order functions to
    // provide generic impls for vector types of different dimensions. We
    // need to check llvm's output to see how well it optimses these.

    #[inline] fn add_s(&self, s: S) -> Self { self.map(|x| x.add(&s)) }
    #[inline] fn sub_s(&self, s: S) -> Self { self.map(|x| x.sub(&s)) }
    #[inline] fn mul_s(&self, s: S) -> Self { self.map(|x| x.mul(&s)) }
    #[inline] fn div_s(&self, s: S) -> Self { self.map(|x| x.div(&s)) }
    #[inline] fn rem_s(&self, s: S) -> Self { self.map(|x| x.rem(&s)) }

    #[inline] fn add_v(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.add(b) ) }
    #[inline] fn sub_v(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.sub(b) ) }
    #[inline] fn mul_v(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.mul(b) ) }
    #[inline] fn div_v(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.div(b) ) }
    #[inline] fn rem_v(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.rem(b) ) }

    #[inline] fn neg_self(&mut self) { for x in self.mut_iter() { *x = x.neg() } }

    #[inline] fn add_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.add(&s) } }
    #[inline] fn sub_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.sub(&s) } }
    #[inline] fn mul_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.mul(&s) } }
    #[inline] fn div_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.div(&s) } }
    #[inline] fn rem_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.rem(&s) } }

    #[inline] fn add_self_v(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.add(b) } }
    #[inline] fn sub_self_v(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.sub(b) } }
    #[inline] fn mul_self_v(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.mul(b) } }
    #[inline] fn div_self_v(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.div(b) } }
    #[inline] fn rem_self_v(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.rem(b) } }

    /// Vector dot product.
    #[inline]
    fn dot(&self, other: &Self) -> S {
        self.iter().zip(other.iter())
            .map(|(a, b)| a.mul(b))
            .fold(zero::<S>(), |a, b| a.add(&b))
    }

    /// The sum of each component of the vector.
    #[inline] fn comp_add(&self) -> S { self.iter().fold(zero::<S>(), |a, b| a.add(b)) }

    /// The product of each component of the vector.
    #[inline] fn comp_mul(&self) -> S { self.iter().fold(one::<S>(), |a, b| a.mul(b)) }

    /// The minimum component of the vector.
    #[inline] fn comp_min(&self) -> S { self.iter().min().unwrap().clone() }

    /// The maximum component of the vector.
    #[inline] fn comp_max(&self) -> S { self.iter().max().unwrap().clone() }
}

impl<S: Clone + Num + Ord> Neg<Vec2<S>> for Vec2<S> { #[inline] fn neg(&self) -> Vec2<S> { self.map(|x| x.neg()) } }
impl<S: Clone + Num + Ord> Neg<Vec3<S>> for Vec3<S> { #[inline] fn neg(&self) -> Vec3<S> { self.map(|x| x.neg()) } }
impl<S: Clone + Num + Ord> Neg<Vec4<S>> for Vec4<S> { #[inline] fn neg(&self) -> Vec4<S> { self.map(|x| x.neg()) } }

impl<S: Clone + Num + Ord> Vector<S, [S, ..2]> for Vec2<S>;
impl<S: Clone + Num + Ord> Vector<S, [S, ..3]> for Vec3<S>;
impl<S: Clone + Num + Ord> Vector<S, [S, ..4]> for Vec4<S>;

/// Operations specific to numeric two-dimensional vectors.
impl<S: Clone + Num + Ord> Vec2<S> {
    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(&self, other: &Vec2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }
}

/// Operations specific to numeric three-dimensional vectors.
impl<S: Clone + Num + Ord> Vec3<S> {
    /// Returns the cross product of the vector and `other`.
    #[inline]
    pub fn cross(&self, other: &Vec3<S>) -> Vec3<S> {
        Vec3::new((self.y * other.z) - (self.z * other.y),
                  (self.z * other.x) - (self.x * other.z),
                  (self.x * other.y) - (self.y * other.x))
    }

    /// Calculates the cross product of the vector and `other`, then stores the
    /// result in `self`.
    #[inline]
    pub fn cross_self(&mut self, other: &Vec3<S>) {
        *self = self.cross(other)
    }
}

/// Specifies geometric operations for vectors. This is only implemented for
/// 2-dimensional and 3-dimensional vectors.
pub trait EuclideanVector
<
    S: Clone + Float,
    Slice
>
:   Vector<S, Slice>
+   ApproxEq<S>
{
    /// Returns `true` if the vector is perpendicular (at right angles to)
    /// the other vector.
    fn is_perpendicular(&self, other: &Self) -> bool {
        self.dot(other).approx_eq(&zero())
    }

    /// Returns the squared length of the vector. This does not perform an
    /// expensive square root operation like in the `length` method and can
    /// therefore be more efficient for comparing the lengths of two vectors.
    #[inline]
    fn length2(&self) -> S {
        self.dot(self)
    }

    /// The norm of the vector.
    #[inline]
    fn length(&self) -> S {
        sqrt(self.dot(self))
    }

    /// The angle between the vector and `other`.
    fn angle(&self, other: &Self) -> Rad<S>;

    /// Returns a vector with the same direction, but with a `length` (or
    /// `norm`) of `1`.
    #[inline]
    fn normalize(&self) -> Self {
        self.normalize_to(one::<S>())
    }

    /// Returns a vector with the same direction and a given `length`.
    #[inline]
    fn normalize_to(&self, length: S) -> Self {
        self.mul_s(length / self.length())
    }

    /// Returns the result of linarly interpolating the length of the vector
    /// towards the length of `other` by the specified amount.
    #[inline]
    fn lerp(&self, other: &Self, amount: S) -> Self {
        self.add_v(&other.sub_v(self).mul_s(amount))
    }

    /// Normalises the vector to a length of `1`.
    #[inline]
    fn normalize_self(&mut self) {
        let rlen = self.length().recip();
        self.mul_self_s(rlen);
    }

    /// Normalizes the vector to `length`.
    #[inline]
    fn normalize_self_to(&mut self, length: S) {
        let n = length / self.length();
        self.mul_self_s(n);
    }

    /// Linearly interpolates the length of the vector towards the length of
    /// `other` by the specified amount.
    fn lerp_self(&mut self, other: &Self, amount: S) {
        let v = other.sub_v(self).mul_s(amount);
        self.add_self_v(&v);
    }
}

impl<S: Clone + Float> EuclideanVector<S, [S, ..2]> for Vec2<S> {
    #[inline]
    fn angle(&self, other: &Vec2<S>) -> Rad<S> {
        atan2(self.perp_dot(other), self.dot(other))
    }
}

impl<S: Clone + Float> EuclideanVector<S, [S, ..3]> for Vec3<S> {
    #[inline]
    fn angle(&self, other: &Vec3<S>) -> Rad<S> {
        atan2(self.cross(other).length(), self.dot(other))
    }
}

impl<S> ToStr for Vec2<S> {
    fn to_str(&self) -> ~str {
        fmt!("[%?, %?]", self.x, self.y)
    }
}

impl<S> ToStr for Vec3<S> {
    fn to_str(&self) -> ~str {
        fmt!("[%?, %?, %?]", self.x, self.y, self.z)
    }
}

impl<S> ToStr for Vec4<S> {
    fn to_str(&self) -> ~str {
        fmt!("[%?, %?, %?, %?]", self.x, self.y, self.z, self.w)
    }
}
