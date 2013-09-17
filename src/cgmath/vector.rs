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

use angle::{Rad, atan2, acos};
use array::{Array, build};

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
pub trait ToVec2<S: Primitive> { fn to_vec2(&self) -> Vec2<S>; }
pub trait ToVec3<S: Primitive> { fn to_vec3(&self) -> Vec3<S>; }
pub trait ToVec4<S: Primitive> { fn to_vec4(&self) -> Vec4<S>; }

// Utility macro for generating associated functions for the vectors
macro_rules! vec(
    (impl $Self:ident <$S:ident> { $($field:ident),+ }) => (
        impl<$S: Primitive> $Self<$S> {
            #[inline]
            pub fn new($($field: $S),+) -> $Self<$S> {
                $Self { $($field: $field),+ }
            }

            /// Construct a vector from a single value.
            #[inline]
            pub fn from_value(value: $S) -> $Self<$S> {
                $Self { $($field: value.clone()),+ }
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

impl<S: Primitive> Vec2<S> {
    #[inline] pub fn unit_x() -> Vec2<S> { Vec2::new(one(), zero()) }
    #[inline] pub fn unit_y() -> Vec2<S> { Vec2::new(zero(), one()) }
}

impl<S: Primitive> Vec3<S> {
    #[inline] pub fn unit_x() -> Vec3<S> { Vec3::new(one(), zero(), zero()) }
    #[inline] pub fn unit_y() -> Vec3<S> { Vec3::new(zero(), one(), zero()) }
    #[inline] pub fn unit_z() -> Vec3<S> { Vec3::new(zero(), zero(), one()) }
}

impl<S: Primitive> Vec4<S> {
    #[inline] pub fn unit_x() -> Vec4<S> { Vec4::new(one(), zero(), zero(), zero()) }
    #[inline] pub fn unit_y() -> Vec4<S> { Vec4::new(zero(), one(), zero(), zero()) }
    #[inline] pub fn unit_z() -> Vec4<S> { Vec4::new(zero(), zero(), one(), zero()) }
    #[inline] pub fn unit_w() -> Vec4<S> { Vec4::new(zero(), zero(), zero(), one()) }
}

array!(impl<S> Vec2<S> -> [S, ..2] _2)
array!(impl<S> Vec3<S> -> [S, ..3] _3)
array!(impl<S> Vec4<S> -> [S, ..4] _4)

approx_eq!(impl<S> Vec2<S>)
approx_eq!(impl<S> Vec3<S>)
approx_eq!(impl<S> Vec4<S>)

/// A trait that specifies a range of numeric operations for vectors. Not all
/// of these make sense from a linear algebra point of view, but are included
/// for pragmatic reasons.
pub trait Vector
<
    S: Primitive,
    Slice
>
:   Array<S, Slice>
+   Neg<Self>
+   Zero + One
{
    #[inline] fn add_s(&self, s: S) -> Self { build(|i| self.i(i).add(&s)) }
    #[inline] fn sub_s(&self, s: S) -> Self { build(|i| self.i(i).sub(&s)) }
    #[inline] fn mul_s(&self, s: S) -> Self { build(|i| self.i(i).mul(&s)) }
    #[inline] fn div_s(&self, s: S) -> Self { build(|i| self.i(i).div(&s)) }
    #[inline] fn rem_s(&self, s: S) -> Self { build(|i| self.i(i).rem(&s)) }

    #[inline] fn add_v(&self, other: &Self) -> Self { build(|i| self.i(i).add(other.i(i))) }
    #[inline] fn sub_v(&self, other: &Self) -> Self { build(|i| self.i(i).sub(other.i(i))) }
    #[inline] fn mul_v(&self, other: &Self) -> Self { build(|i| self.i(i).mul(other.i(i))) }
    #[inline] fn div_v(&self, other: &Self) -> Self { build(|i| self.i(i).div(other.i(i))) }
    #[inline] fn rem_v(&self, other: &Self) -> Self { build(|i| self.i(i).rem(other.i(i))) }

    #[inline] fn neg_self(&mut self) { self.each_mut(|_, x| *x = x.neg()) }

    #[inline] fn add_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.add(&s)) }
    #[inline] fn sub_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.sub(&s)) }
    #[inline] fn mul_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.mul(&s)) }
    #[inline] fn div_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.div(&s)) }
    #[inline] fn rem_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.rem(&s)) }

    #[inline] fn add_self_v(&mut self, other: &Self) { self.each_mut(|i, x| *x = x.add(other.i(i))) }
    #[inline] fn sub_self_v(&mut self, other: &Self) { self.each_mut(|i, x| *x = x.sub(other.i(i))) }
    #[inline] fn mul_self_v(&mut self, other: &Self) { self.each_mut(|i, x| *x = x.mul(other.i(i))) }
    #[inline] fn div_self_v(&mut self, other: &Self) { self.each_mut(|i, x| *x = x.div(other.i(i))) }
    #[inline] fn rem_self_v(&mut self, other: &Self) { self.each_mut(|i, x| *x = x.rem(other.i(i))) }

    /// The sum of each component of the vector.
    #[inline] fn comp_add(&self) -> S { self.fold(|a, b| a.add(b)) }

    /// The product of each component of the vector.
    #[inline] fn comp_mul(&self) -> S { self.fold(|a, b| a.mul(b)) }

    /// Vector dot product.
    #[inline] fn dot(&self, other: &Self) -> S { self.mul_v(other).comp_add() }

    /// The minimum component of the vector.
    #[inline] fn comp_min(&self) -> S { self.iter().min().unwrap().clone() }

    /// The maximum component of the vector.
    #[inline] fn comp_max(&self) -> S { self.iter().max().unwrap().clone() }
}

#[inline] fn dot<S: Primitive, Slice, V: Vector<S, Slice>>(a: V, b: V) -> S { a.dot(&b) }

impl<S: Primitive> One for Vec2<S> { #[inline] fn one() -> Vec2<S> { Vec2::ident() } }
impl<S: Primitive> One for Vec3<S> { #[inline] fn one() -> Vec3<S> { Vec3::ident() } }
impl<S: Primitive> One for Vec4<S> { #[inline] fn one() -> Vec4<S> { Vec4::ident() } }

impl<S: Primitive> Neg<Vec2<S>> for Vec2<S> { #[inline] fn neg(&self) -> Vec2<S> { build(|i| self.i(i).neg()) } }
impl<S: Primitive> Neg<Vec3<S>> for Vec3<S> { #[inline] fn neg(&self) -> Vec3<S> { build(|i| self.i(i).neg()) } }
impl<S: Primitive> Neg<Vec4<S>> for Vec4<S> { #[inline] fn neg(&self) -> Vec4<S> { build(|i| self.i(i).neg()) } }

macro_rules! vector(
    (impl $Self:ident <$S:ident> $Slice:ty { $x:ident, $($xs:ident),+ }) => (
        impl<$S: Primitive> Vector<$S, $Slice> for $Self<$S>;
    )
)

vector!(impl Vec2<S> [S, ..2] { x, y })
vector!(impl Vec3<S> [S, ..3] { x, y, z })
vector!(impl Vec4<S> [S, ..4] { x, y, z, w })

/// Operations specific to numeric two-dimensional vectors.
impl<S: Primitive> Vec2<S> {
    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(&self, other: &Vec2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }
}

/// Operations specific to numeric three-dimensional vectors.
impl<S: Primitive> Vec3<S> {
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
    S: Float,
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

impl<S: Float> EuclideanVector<S, [S, ..2]> for Vec2<S> {
    #[inline]
    fn angle(&self, other: &Vec2<S>) -> Rad<S> {
        atan2(self.perp_dot(other), self.dot(other))
    }
}

impl<S: Float> EuclideanVector<S, [S, ..3]> for Vec3<S> {
    #[inline]
    fn angle(&self, other: &Vec3<S>) -> Rad<S> {
        atan2(self.cross(other).length(), self.dot(other))
    }
}

impl<S: Float> EuclideanVector<S, [S, ..4]> for Vec4<S> {
    #[inline]
    fn angle(&self, other: &Vec4<S>) -> Rad<S> {
        acos(self.dot(other) / (self.length() * other.length()))
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
