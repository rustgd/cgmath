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

use std::fmt;
use std::num::{Zero, zero, One, one};

use angle::{Rad, atan2, acos};
use approx::ApproxEq;
use array::{Array, build};
use partial_ord::{PartOrdPrim, PartOrdFloat};

/// A trait that specifies a range of numeric operations for vectors. Not all
/// of these make sense from a linear algebra point of view, but are included
/// for pragmatic reasons.
pub trait Vector
<
    S: PartOrdPrim,
    Slice
>
:   Array<S, Slice>
+   Neg<Self>
+   Zero + One
{
    /// Add a scalar to this vector, returning a new vector.
    #[inline] fn add_s(&self, s: S) -> Self { build(|i| self.i(i).add(&s)) }
    /// Subtract a scalar from this vector, returning a new vector.
    #[inline] fn sub_s(&self, s: S) -> Self { build(|i| self.i(i).sub(&s)) }
    /// Multiply this vector by a scalar, returning a new vector.
    #[inline] fn mul_s(&self, s: S) -> Self { build(|i| self.i(i).mul(&s)) }
    /// Divide this vector by a scalar, returning a new vector.
    #[inline] fn div_s(&self, s: S) -> Self { build(|i| self.i(i).div(&s)) }
    /// Take the remainder of this vector by a scalar, returning a new vector.
    #[inline] fn rem_s(&self, s: S) -> Self { build(|i| self.i(i).rem(&s)) }

    /// Add this vector to another, returning a new vector.
    #[inline] fn add_v(&self, other: &Self) -> Self { build(|i| self.i(i).add(other.i(i))) }
    /// Subtract another vector from this one, returning a new vector.
    #[inline] fn sub_v(&self, other: &Self) -> Self { build(|i| self.i(i).sub(other.i(i))) }
    /// Multiply this vector by another, returning a new vector.
    #[inline] fn mul_v(&self, other: &Self) -> Self { build(|i| self.i(i).mul(other.i(i))) }
    /// Divide this vector by another, returning a new vector.
    #[inline] fn div_v(&self, other: &Self) -> Self { build(|i| self.i(i).div(other.i(i))) }
    /// Take the remainder of this vector by another, returning a new scalar.
    #[inline] fn rem_v(&self, other: &Self) -> Self { build(|i| self.i(i).rem(other.i(i))) }

    /// Negate this vector in-place.
    #[inline] fn neg_self(&mut self) { self.each_mut(|_, x| *x = x.neg()) }

    /// Add a scalar to this vector in-place.
    #[inline] fn add_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.add(&s)) }
    /// Subtract a scalar from this vector, in-place.
    #[inline] fn sub_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.sub(&s)) }
    /// Multiply this vector by a scalar, in-place.
    #[inline] fn mul_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.mul(&s)) }
    /// Divide this vector by a scalar, in-place.
    #[inline] fn div_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.div(&s)) }
    /// Take the remainder of this vector by a scalar, in-place.
    #[inline] fn rem_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.rem(&s)) }

    /// Add another vector to this one, in-place.
    #[inline] fn add_self_v(&mut self, other: &Self) { self.each_mut(|i, x| *x = x.add(other.i(i))) }
    /// Subtract another vector from this one, in-place.
    #[inline] fn sub_self_v(&mut self, other: &Self) { self.each_mut(|i, x| *x = x.sub(other.i(i))) }
    /// Multiply this matrix by another, in-place.
    #[inline] fn mul_self_v(&mut self, other: &Self) { self.each_mut(|i, x| *x = x.mul(other.i(i))) }
    /// Divide this matrix by anothor, in-place.
    #[inline] fn div_self_v(&mut self, other: &Self) { self.each_mut(|i, x| *x = x.div(other.i(i))) }
    /// Take the remainder of this vector by another, in-place.
    #[inline] fn rem_self_v(&mut self, other: &Self) { self.each_mut(|i, x| *x = x.rem(other.i(i))) }

    /// The sum of each component of the vector.
    #[inline] fn comp_add(&self) -> S { self.fold(|a, b| a.add(b)) }

    /// The product of each component of the vector.
    #[inline] fn comp_mul(&self) -> S { self.fold(|a, b| a.mul(b)) }

    /// Vector dot product.
    #[inline] fn dot(&self, other: &Self) -> S { self.mul_v(other).comp_add() }

    /// The minimum component of the vector.
    #[inline] fn comp_min(&self) -> S { self.fold(|a, b| if *a < *b { *a } else {*b }) }

    /// The maximum component of the vector.
    #[inline] fn comp_max(&self) -> S { self.fold(|a, b| if *a > *b { *a } else {*b }) }
}

/// Dot product of two vectors.
#[inline] pub fn dot<S: PartOrdPrim, Slice, V: Vector<S, Slice>>(a: V, b: V) -> S { a.dot(&b) }

// Utility macro for generating associated functions for the vectors
macro_rules! vec(
    ($Self:ident <$S:ident> { $($field:ident),+ }, $n:expr) => (
        #[deriving(Eq, TotalEq, Clone, Hash)]
        pub struct $Self<S> { $(pub $field: S),+ }

        impl<$S: Primitive> $Self<$S> {
            /// Construct a new vector, using the provided values.
            #[inline]
            pub fn new($($field: $S),+) -> $Self<$S> {
                $Self { $($field: $field),+ }
            }

            /// Construct a vector from a single value, replicating it.
            #[inline]
            pub fn from_value(value: $S) -> $Self<$S> {
                $Self { $($field: value.clone()),+ }
            }

            /// The additive identity of the vector.
            #[inline]
            pub fn zero() -> $Self<$S> { $Self::from_value(zero()) }

            /// The multiplicative identity of the vector.
            #[inline]
            pub fn ident() -> $Self<$S> { $Self::from_value(one()) }
        }

        impl<S: PartOrdPrim> Add<$Self<S>, $Self<S>> for $Self<S> {
            #[inline] fn add(&self, other: &$Self<S>) -> $Self<S> { self.add_v(other) }
        }

        impl<S: PartOrdPrim> Sub<$Self<S>, $Self<S>> for $Self<S> {
            #[inline] fn sub(&self, other: &$Self<S>) -> $Self<S> { self.sub_v(other) }
        }

        impl<S: PartOrdPrim> Zero for $Self<S> {
            #[inline] fn zero() -> $Self<S> { $Self::from_value(zero()) }
            #[inline] fn is_zero(&self) -> bool { *self == zero() }
        }

        impl<S: PartOrdPrim> Neg<$Self<S>> for $Self<S> {
            #[inline] fn neg(&self) -> $Self<S> { build(|i| self.i(i).neg()) }
        }

        impl<S: PartOrdPrim> Mul<$Self<S>, $Self<S>> for $Self<S> {
            #[inline] fn mul(&self, other: &$Self<S>) -> $Self<S> { self.mul_v(other) }
        }

        impl<S: PartOrdPrim> One for $Self<S> {
            #[inline] fn one() -> $Self<S> { $Self::from_value(one()) }
        }

        impl<S: PartOrdPrim> Vector<S, [S, ..$n]> for $Self<S> {}
    )
)

vec!(Vector2<S> { x, y }, 2)
vec!(Vector3<S> { x, y, z }, 3)
vec!(Vector4<S> { x, y, z, w }, 4)

array!(impl<S> Vector2<S> -> [S, ..2] _2)
array!(impl<S> Vector3<S> -> [S, ..3] _3)
array!(impl<S> Vector4<S> -> [S, ..4] _4)

/// Operations specific to numeric two-dimensional vectors.
impl<S: Primitive> Vector2<S> {
    /// A unit vector in the `x` direction.
    #[inline] pub fn unit_x() -> Vector2<S> { Vector2::new(one(), zero()) }
    /// A unit vector in the `y` direction.
    #[inline] pub fn unit_y() -> Vector2<S> { Vector2::new(zero(), one()) }

    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(&self, other: &Vector2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }

    /// Create a `Vector3`, using the `x` and `y` values from this vector, and the
    /// provided `z`.
    #[inline]
    pub fn extend(&self, z: S)-> Vector3<S> {
        Vector3::new(self.x.clone(), self.y.clone(), z)
    }
}

/// Operations specific to numeric three-dimensional vectors.
impl<S: Primitive> Vector3<S> {
    /// A unit vector in the `x` direction.
    #[inline] pub fn unit_x() -> Vector3<S> { Vector3::new(one(), zero(), zero()) }
    /// A unit vector in the `y` direction.
    #[inline] pub fn unit_y() -> Vector3<S> { Vector3::new(zero(), one(), zero()) }
    /// A unit vector in the `w` direction.
    #[inline] pub fn unit_z() -> Vector3<S> { Vector3::new(zero(), zero(), one()) }

    /// Returns the cross product of the vector and `other`.
    #[inline]
    pub fn cross(&self, other: &Vector3<S>) -> Vector3<S> {
        Vector3::new((self.y * other.z) - (self.z * other.y),
                  (self.z * other.x) - (self.x * other.z),
                  (self.x * other.y) - (self.y * other.x))
    }

    /// Calculates the cross product of the vector and `other`, then stores the
    /// result in `self`.
    #[inline]
    pub fn cross_self(&mut self, other: &Vector3<S>) {
        *self = self.cross(other)
    }

    /// Create a `Vector4`, using the `x`, `y` and `z` values from this vector, and the
    /// provided `w`.
    #[inline]
    pub fn extend(&self, w: S)-> Vector4<S> {
        Vector4::new(self.x.clone(), self.y.clone(), self.z.clone(), w)
    }

    /// Create a `Vector2`, dropping the `z` value.
    #[inline]
    pub fn truncate(&self)-> Vector2<S> {
        Vector2::new(self.x.clone(), self.y.clone())
    }
}

/// Operations specific to numeric four-dimensional vectors.
impl<S: Primitive> Vector4<S> {
    /// A unit vector in the `x` direction.
    #[inline] pub fn unit_x() -> Vector4<S> { Vector4::new(one(), zero(), zero(), zero()) }
    /// A unit vector in the `y` direction.
    #[inline] pub fn unit_y() -> Vector4<S> { Vector4::new(zero(), one(), zero(), zero()) }
    /// A unit vector in the `z` direction.
    #[inline] pub fn unit_z() -> Vector4<S> { Vector4::new(zero(), zero(), one(), zero()) }
    /// A unit vector in the `w` direction.
    #[inline] pub fn unit_w() -> Vector4<S> { Vector4::new(zero(), zero(), zero(), one()) }

    /// Create a `Vector3`, dropping the `w` value.
    #[inline]
    pub fn truncate(&self)-> Vector3<S> {
        Vector3::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

/// Specifies geometric operations for vectors. This is only implemented for
/// 2-dimensional and 3-dimensional vectors.
pub trait EuclideanVector
<
    S: PartOrdFloat<S>,
    Slice
>
:   Vector<S, Slice>
+   ApproxEq<S>
{
    /// Returns `true` if the vector is perpendicular (at right angles) to the
    /// other vector.
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
        self.dot(self).sqrt()
    }

    /// The angle between the vector and `other`, in radians.
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

impl<S: PartOrdFloat<S>>
EuclideanVector<S, [S, ..2]> for Vector2<S> {
    #[inline]
    fn angle(&self, other: &Vector2<S>) -> Rad<S> {
        atan2(self.perp_dot(other), self.dot(other))
    }
}

impl<S: PartOrdFloat<S>>
EuclideanVector<S, [S, ..3]> for Vector3<S> {
    #[inline]
    fn angle(&self, other: &Vector3<S>) -> Rad<S> {
        atan2(self.cross(other).length(), self.dot(other))
    }
}

impl<S: PartOrdFloat<S>>
EuclideanVector<S, [S, ..4]> for Vector4<S> {
    #[inline]
    fn angle(&self, other: &Vector4<S>) -> Rad<S> {
        acos(self.dot(other) / (self.length() * other.length()))
    }
}

impl<S: fmt::Show> fmt::Show for Vector2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<S: fmt::Show> fmt::Show for Vector3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl<S: fmt::Show> fmt::Show for Vector4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}
