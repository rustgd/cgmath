// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
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

//! Types and traits for two, three, and four-dimensional vectors.
//!
//! ## Working with Vectors
//!
//! Vectors can be created in several different ways. There is, of course, the
//! traditional `new()` method, but unit vectors, zero vectors, and an one
//! vector are also provided:
//!
//! ```rust
//! use cgmath::{Vector, Vector2, Vector3, Vector4, vec2, vec3};
//!
//! assert_eq!(Vector2::new(1.0f64, 0.0f64), Vector2::unit_x());
//! assert_eq!(vec3(0.0f64, 0.0f64, 0.0f64), Vector3::zero());
//! assert_eq!(Vector2::from_value(1.0f64), vec2(1.0, 1.0));
//! ```
//!
//! Vectors can be manipulated with typical mathematical operations (addition,
//! subtraction, element-wise multiplication, element-wise division, negation)
//! using the built-in operators.
//!
//! ```rust
//! use cgmath::{Vector, Vector2, Vector3, Vector4};
//!
//! let a: Vector2<f64> = Vector2::new(3.0, 4.0);
//! let b: Vector2<f64> = Vector2::new(-3.0, -4.0);
//!
//! assert_eq!(&a + &b, Vector2::zero());
//! assert_eq!(-(&a * &b), Vector2::new(9.0f64, 16.0f64));
//! assert_eq!(&a / &Vector2::one(), a);
//!
//! // As with Rust's `int` and `f32` types, Vectors of different types cannot
//! // be added and so on with impunity. The following will fail to compile:
//! // let c = a + Vector3::new(1.0, 0.0, 2.0);
//!
//! // Instead, we need to convert the Vector2 to a Vector3 by "extending" it
//! // with the value for the last coordinate:
//! let c: Vector3<f64> = &a.extend(0.0) + &Vector3::new(1.0, 0.0, 2.0);
//!
//! // Similarly, we can "truncate" a Vector4 down to a Vector3:
//! let d: Vector3<f64> = &c + &Vector4::unit_x().truncate();
//!
//! assert_eq!(d, Vector3::new(5.0f64, 4.0f64, 2.0f64));
//! ```
//!
//! Vectors also provide methods for typical operations such as
//! [scalar multiplication](http://en.wikipedia.org/wiki/Scalar_multiplication),
//! [dot products](http://en.wikipedia.org/wiki/Dot_product),
//! and [cross products](http://en.wikipedia.org/wiki/Cross_product).
//!
//! ```rust
//! use cgmath::{Vector, Vector2, Vector3, Vector4, dot};
//!
//! // All vectors implement the dot product as a method:
//! let a: Vector2<f64> = Vector2::new(3.0, 6.0);
//! let b: Vector2<f64> = Vector2::new(-2.0, 1.0);
//! assert_eq!(a.dot(&b), 0.0);
//!
//! // But there is also a top-level function:
//! assert_eq!(a.dot(&b), dot(a, b));
//!
//! // Scalar multiplication can return a new object, or be done in place
//! // to avoid an allocation:
//! let mut c = Vector4::from_value(3f64);
//! let d: Vector4<f64> = c.mul_s(2.0);
//! c.mul_self_s(2.0);
//! assert_eq!(c, d);
//!
//! // Cross products are defined for 3-dimensional vectors:
//! let e: Vector3<f64> = Vector3::unit_x();
//! let f: Vector3<f64> = Vector3::unit_y();
//! assert_eq!(e.cross(&f), Vector3::unit_z());
//! ```
//!
//! Several other useful methods are provided as well. Vector fields can be
//! accessed using array syntax (i.e. `vector[0] == vector.x`), or by using
//! the methods provided by the [`Array1`](../array/trait.Array1.html) trait.
//! This trait also provides a `map()` method for applying arbitrary functions.
//!
//! The [`Vector`](../trait.Vector.html) trait presents the most general
//! features of the vectors, while [`EuclideanVector`]
//! (../array/trait.EuclideanVector.html) is more specific to Euclidean space.

use std::fmt;
use std::mem;
use std::ops::*;

use rand::{Rand, Rng};

use rust_num::{NumCast, Zero, One};

use angle::{Rad, atan2, acos};
use approx::ApproxEq;
use array::Array1;
use num::{BaseNum, BaseFloat};

/// A trait that specifies a range of numeric operations for vectors. Not all
/// of these make sense from a linear algebra point of view, but are included
/// for pragmatic reasons.
pub trait Vector<S: BaseNum>: Array1<S> + Clone // where
    // FIXME: blocked by rust-lang/rust#20671
    //
    // for<'a, 'b> &'a Self: Add<&'b Self, Output = Self>,
    // for<'a, 'b> &'a Self: Sub<&'b Self, Output = Self>,
    // for<'a, 'b> &'a Self: Mul<&'b Self, Output = Self>,
    // for<'a, 'b> &'a Self: Div<&'b Self, Output = Self>,
    // for<'a, 'b> &'a Self: Rem<&'b Self, Output = Self>,
    // for<'a, 'b> &'a Self: Sub<&'b Self, Output = Self>,
    //
    // for<'a> &'a Self: Add<S, Output = Self>,
    // for<'a> &'a Self: Sub<S, Output = Self>,
    // for<'a> &'a Self: Mul<S, Output = Self>,
    // for<'a> &'a Self: Div<S, Output = Self>,
    // for<'a> &'a Self: Rem<S, Output = Self>,
{
    /// Construct a vector from a single value, replicating it.
    fn from_value(s: S) -> Self;

    /// The zero vector (with all components set to zero)
    #[inline]
    fn zero() -> Self { Self::from_value(S::zero()) }
    /// The identity vector (with all components set to one)
    #[inline]
    fn one() -> Self { Self::from_value(S::one()) }

    /// Add a scalar to this vector, returning a new vector.
    #[must_use]
    fn add_s(&self, s: S) -> Self;
    /// Subtract a scalar from this vector, returning a new vector.
    #[must_use]
    fn sub_s(&self, s: S) -> Self;
    /// Multiply this vector by a scalar, returning a new vector.
    #[must_use]
    fn mul_s(&self, s: S) -> Self;
    /// Divide this vector by a scalar, returning a new vector.
    #[must_use]
    fn div_s(&self, s: S) -> Self;
    /// Take the remainder of this vector by a scalar, returning a new vector.
    #[must_use]
    fn rem_s(&self, s: S) -> Self;

    /// Add this vector to another, returning a new vector.
    #[must_use]
    fn add_v(&self, v: &Self) -> Self;
    /// Subtract another vector from this one, returning a new vector.
    #[must_use]
    fn sub_v(&self, v: &Self) -> Self;
    /// Multiply this vector by another, returning a new vector.
    #[must_use]
    fn mul_v(&self, v: &Self) -> Self;
    /// Divide this vector by another, returning a new vector.
    #[must_use]
    fn div_v(&self, v: &Self) -> Self;
    /// Take the remainder of this vector by another, returning a new scalar.
    #[must_use]
    fn rem_v(&self, v: &Self) -> Self;

    /// Add a scalar to this vector in-place.
    fn add_self_s(&mut self, s: S);
    /// Subtract a scalar from this vector, in-place.
    fn sub_self_s(&mut self, s: S);
    /// Multiply this vector by a scalar, in-place.
    fn mul_self_s(&mut self, s: S);
    /// Divide this vector by a scalar, in-place.
    fn div_self_s(&mut self, s: S);
    /// Take the remainder of this vector by a scalar, in-place.
    fn rem_self_s(&mut self, s: S);

    /// Add another vector to this one, in-place.
    fn add_self_v(&mut self, v: &Self);
    /// Subtract another vector from this one, in-place.
    fn sub_self_v(&mut self, v: &Self);
    /// Multiply this matrix by another, in-place.
    fn mul_self_v(&mut self, v: &Self);
    /// Divide this matrix by anothor, in-place.
    fn div_self_v(&mut self, v: &Self);
    /// Take the remainder of this vector by another, in-place.
    fn rem_self_v(&mut self, v: &Self);

    /// The sum of the components of the vector.
    fn sum(&self) -> S;
    /// The product of the components of the vector.
    fn product(&self) -> S;

    /// Vector dot product.
    #[inline]
    fn dot(&self, v: &Self) -> S { self.mul_v(v).sum() }

    /// The minimum component of the vector.
    fn comp_min(&self) -> S;
    /// The maximum component of the vector.
    fn comp_max(&self) -> S;
}

/// Dot product of two vectors.
#[inline] pub fn dot<S: BaseNum, V: Vector<S>>(a: V, b: V) -> S { a.dot(&b) }

// Utility macro for generating associated functions for the vectors
macro_rules! vec {
    ($VectorN:ident <$S:ident> { $($field:ident),+ }, $n:expr, $constructor:ident) => {
        #[derive(PartialEq, Eq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
        pub struct $VectorN<S> { $(pub $field: S),+ }

        impl<$S> $VectorN<$S> {
            /// Construct a new vector, using the provided values.
            #[inline]
            pub fn new($($field: $S),+) -> $VectorN<$S> {
                $VectorN { $($field: $field),+ }
            }
        }

        impl<$S: Copy + Neg<Output = $S>> $VectorN<$S> {
            /// Negate this vector in-place (multiply by -1).
            #[inline]
            pub fn neg_self(&mut self) {
                $(self.$field = -self.$field);+
            }
        }

        /// The short constructor.
        #[inline]
        pub fn $constructor<S>($($field: S),+) -> $VectorN<S> {
            $VectorN::new($($field),+)
        }

        impl<$S: NumCast + Copy> $VectorN<$S> {
            /// Component-wise casting to another type
            #[inline]
            pub fn cast<T: NumCast>(&self) -> $VectorN<T> {
                $VectorN { $($field: NumCast::from(self.$field).unwrap()),+ }
            }
        }

        impl<$S: Copy> Array1<$S> for $VectorN<$S> {}

        impl<S: BaseNum> Vector<S> for $VectorN<S> {
            #[inline] fn from_value(s: S) -> $VectorN<S> { $VectorN { $($field: s),+ } }

            #[inline] fn add_s(&self, s: S) -> $VectorN<S> { self + s }
            #[inline] fn sub_s(&self, s: S) -> $VectorN<S> { self - s }
            #[inline] fn mul_s(&self, s: S) -> $VectorN<S> { self * s }
            #[inline] fn div_s(&self, s: S) -> $VectorN<S> { self / s }
            #[inline] fn rem_s(&self, s: S) -> $VectorN<S> { self % s }

            #[inline] fn add_v(&self, v: &$VectorN<S>) -> $VectorN<S> { self + v }
            #[inline] fn sub_v(&self, v: &$VectorN<S>) -> $VectorN<S> { self - v }
            #[inline] fn mul_v(&self, v: &$VectorN<S>) -> $VectorN<S> { self * v }
            #[inline] fn div_v(&self, v: &$VectorN<S>) -> $VectorN<S> { self / v }
            #[inline] fn rem_v(&self, v: &$VectorN<S>) -> $VectorN<S> { self % v }

            #[inline] fn add_self_s(&mut self, s: S) { *self = &*self + s; }
            #[inline] fn sub_self_s(&mut self, s: S) { *self = &*self - s; }
            #[inline] fn mul_self_s(&mut self, s: S) { *self = &*self * s; }
            #[inline] fn div_self_s(&mut self, s: S) { *self = &*self / s; }
            #[inline] fn rem_self_s(&mut self, s: S) { *self = &*self % s; }

            #[inline] fn add_self_v(&mut self, v: &$VectorN<S>) { *self = &*self + v; }
            #[inline] fn sub_self_v(&mut self, v: &$VectorN<S>) { *self = &*self - v; }
            #[inline] fn mul_self_v(&mut self, v: &$VectorN<S>) { *self = &*self * v; }
            #[inline] fn div_self_v(&mut self, v: &$VectorN<S>) { *self = &*self / v; }
            #[inline] fn rem_self_v(&mut self, v: &$VectorN<S>) { *self = &*self % v; }

            #[inline] fn sum(&self) -> S { fold!(add, { $(self.$field),+ }) }
            #[inline] fn product(&self) -> S { fold!(mul, { $(self.$field),+ }) }
            #[inline] fn comp_min(&self) -> S { fold!(partial_min, { $(self.$field),+ }) }
            #[inline] fn comp_max(&self) -> S { fold!(partial_max, { $(self.$field),+ }) }
        }

        impl<S: Neg<Output = S>> Neg for $VectorN<S> {
            type Output = $VectorN<S>;

            #[inline]
            fn neg(self) -> $VectorN<S> { $VectorN::new($(-self.$field),+) }
        }

        impl<S: BaseFloat> ApproxEq<S> for $VectorN<S> {
            #[inline]
            fn approx_eq_eps(&self, other: &$VectorN<S>, epsilon: &S) -> bool {
                $(self.$field.approx_eq_eps(&other.$field, epsilon))&&+
            }
        }

        impl<S: BaseFloat + Rand> Rand for $VectorN<S> {
            #[inline]
            fn rand<R: Rng>(rng: &mut R) -> $VectorN<S> {
                $VectorN { $($field: rng.gen()),+ }
            }
        }
    }
}

macro_rules! impl_binary_operator {
    ($Binop:ident :: $binop:ident, $VectorN:ident { $($field:ident),+ }) => {
        impl<'a, S: BaseNum> $Binop<S> for &'a $VectorN<S> {
            type Output = $VectorN<S>;

            #[inline]
            fn $binop(self, s: S) -> $VectorN<S> {
                $VectorN::new($(self.$field.$binop(s)),+)
            }
        }

        impl<'a, 'b, S: BaseNum> $Binop<&'a $VectorN<S>> for &'b $VectorN<S> {
            type Output = $VectorN<S>;

            #[inline]
            fn $binop(self, other: &'a $VectorN<S>) -> $VectorN<S> {
                $VectorN::new($(self.$field.$binop(other.$field)),+)
            }
        }
    }
}

impl_binary_operator!(Add::add, Vector2 { x, y });
impl_binary_operator!(Add::add, Vector3 { x, y, z });
impl_binary_operator!(Add::add, Vector4 { x, y, z, w });
impl_binary_operator!(Sub::sub, Vector2 { x, y });
impl_binary_operator!(Sub::sub, Vector3 { x, y, z });
impl_binary_operator!(Sub::sub, Vector4 { x, y, z, w });
impl_binary_operator!(Mul::mul, Vector2 { x, y });
impl_binary_operator!(Mul::mul, Vector3 { x, y, z });
impl_binary_operator!(Mul::mul, Vector4 { x, y, z, w });
impl_binary_operator!(Div::div, Vector2 { x, y });
impl_binary_operator!(Div::div, Vector3 { x, y, z });
impl_binary_operator!(Div::div, Vector4 { x, y, z, w });
impl_binary_operator!(Rem::rem, Vector2 { x, y });
impl_binary_operator!(Rem::rem, Vector3 { x, y, z });
impl_binary_operator!(Rem::rem, Vector4 { x, y, z, w });

macro_rules! fold {
    (&$method:ident, { $x:expr, $y:expr })                   => { $x.$method(&$y) };
    (&$method:ident, { $x:expr, $y:expr, $z:expr })          => { $x.$method(&$y).$method(&$z) };
    (&$method:ident, { $x:expr, $y:expr, $z:expr, $w:expr }) => { $x.$method(&$y).$method(&$z).$method(&$w) };
    ($method:ident, { $x:expr, $y:expr })                    => { $x.$method($y) };
    ($method:ident, { $x:expr, $y:expr, $z:expr })           => { $x.$method($y).$method($z) };
    ($method:ident, { $x:expr, $y:expr, $z:expr, $w:expr })  => { $x.$method($y).$method($z).$method($w) };
}

vec!(Vector2<S> { x, y }, 2, vec2);
vec!(Vector3<S> { x, y, z }, 3, vec3);
vec!(Vector4<S> { x, y, z, w }, 4, vec4);

macro_rules! fixed_array_conversions {
    ($VectorN:ident <$S:ident> { $($field:ident : $index:expr),+ }, $n:expr) => {

        impl<$S> Into<[$S; $n]> for $VectorN<$S> {
            #[inline]
            fn into(self) -> [$S; $n] {
                match self { $VectorN { $($field),+ } => [$($field),+] }
            }
        }

        impl<$S> AsRef<[$S; $n]> for $VectorN<$S> {
            #[inline]
            fn as_ref(&self) -> &[$S; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[$S; $n]> for $VectorN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$S; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S: Clone> From<[$S; $n]> for $VectorN<$S> {
            #[inline]
            fn from(v: [$S; $n]) -> $VectorN<$S> {
                // We need to use a clone here because we can't pattern match on arrays yet
                $VectorN { $($field: v[$index].clone()),+ }
            }
        }

        impl<'a, $S> From<&'a [$S; $n]> for &'a $VectorN<$S> {
            #[inline]
            fn from(v: &'a [$S; $n]) -> &'a $VectorN<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut [$S; $n]> for &'a mut $VectorN<$S> {
            #[inline]
            fn from(v: &'a mut [$S; $n]) -> &'a mut $VectorN<$S> {
                unsafe { mem::transmute(v) }
            }
        }
    }
}

fixed_array_conversions!(Vector2<S> { x:0, y:1 }, 2);
fixed_array_conversions!(Vector3<S> { x:0, y:1, z:2 }, 3);
fixed_array_conversions!(Vector4<S> { x:0, y:1, z:2, w:3 }, 4);

macro_rules! tuple_conversions {
    ($VectorN:ident <$S:ident> { $($field:ident),+ }, $Tuple:ty) => {
        impl<$S> Into<$Tuple> for $VectorN<$S> {
            #[inline]
            fn into(self) -> $Tuple {
                match self { $VectorN { $($field),+ } => ($($field),+) }
            }
        }

        impl<$S> AsRef<$Tuple> for $VectorN<$S> {
            #[inline]
            fn as_ref(&self) -> &$Tuple {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<$Tuple> for $VectorN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut $Tuple {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> From<$Tuple> for $VectorN<$S> {
            #[inline]
            fn from(v: $Tuple) -> $VectorN<$S> {
                match v { ($($field),+) => $VectorN { $($field: $field),+ } }
            }
        }

        impl<'a, $S> From<&'a $Tuple> for &'a $VectorN<$S> {
            #[inline]
            fn from(v: &'a $Tuple) -> &'a $VectorN<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut $Tuple> for &'a mut $VectorN<$S> {
            #[inline]
            fn from(v: &'a mut $Tuple) -> &'a mut $VectorN<$S> {
                unsafe { mem::transmute(v) }
            }
        }
    }
}

tuple_conversions!(Vector2<S> { x, y }, (S, S));
tuple_conversions!(Vector3<S> { x, y, z }, (S, S, S));
tuple_conversions!(Vector4<S> { x, y, z, w }, (S, S, S, S));

macro_rules! index_operators {
    ($VectorN:ident<$S:ident>, $n:expr, $Output:ty, $I:ty) => {
        impl<$S> Index<$I> for $VectorN<$S> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&'a self, i: $I) -> &'a $Output {
                let v: &[$S; $n] = self.as_ref(); &v[i]
            }
        }

        impl<$S> IndexMut<$I> for $VectorN<$S> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: $I) -> &'a mut $Output {
                let v: &mut [$S; $n] = self.as_mut(); &mut v[i]
            }
        }
    }
}

index_operators!(Vector2<S>, 2, S, usize);
index_operators!(Vector3<S>, 3, S, usize);
index_operators!(Vector4<S>, 4, S, usize);
index_operators!(Vector2<S>, 2, [S], Range<usize>);
index_operators!(Vector3<S>, 3, [S], Range<usize>);
index_operators!(Vector4<S>, 4, [S], Range<usize>);
index_operators!(Vector2<S>, 2, [S], RangeTo<usize>);
index_operators!(Vector3<S>, 3, [S], RangeTo<usize>);
index_operators!(Vector4<S>, 4, [S], RangeTo<usize>);
index_operators!(Vector2<S>, 2, [S], RangeFrom<usize>);
index_operators!(Vector3<S>, 3, [S], RangeFrom<usize>);
index_operators!(Vector4<S>, 4, [S], RangeFrom<usize>);
index_operators!(Vector2<S>, 2, [S], RangeFull);
index_operators!(Vector3<S>, 3, [S], RangeFull);
index_operators!(Vector4<S>, 4, [S], RangeFull);

/// Operations specific to numeric two-dimensional vectors.
impl<S: BaseNum> Vector2<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Vector2<S> {
        Vector2::new(S::one(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Vector2<S> {
        Vector2::new(S::zero(), S::one())
    }

    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(&self, other: &Vector2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }

    /// Create a `Vector3`, using the `x` and `y` values from this vector, and the
    /// provided `z`.
    #[inline]
    pub fn extend(&self, z: S)-> Vector3<S> {
        Vector3::new(self.x, self.y, z)
    }
}

/// Operations specific to numeric three-dimensional vectors.
impl<S: BaseNum> Vector3<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Vector3<S> {
        Vector3::new(S::one(), S::zero(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Vector3<S> {
        Vector3::new(S::zero(), S::one(), S::zero())
    }

    /// A unit vector in the `w` direction.
    #[inline]
    pub fn unit_z() -> Vector3<S> {
        Vector3::new(S::zero(), S::zero(), S::one())
    }

    /// Returns the cross product of the vector and `other`.
    #[inline]
    #[must_use]
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
        Vector4::new(self.x, self.y, self.z, w)
    }

    /// Create a `Vector2`, dropping the `z` value.
    #[inline]
    pub fn truncate(&self)-> Vector2<S> {
        Vector2::new(self.x, self.y)
    }
}

/// Operations specific to numeric four-dimensional vectors.
impl<S: BaseNum> Vector4<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Vector4<S> {
        Vector4::new(S::one(), S::zero(), S::zero(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Vector4<S> {
        Vector4::new(S::zero(), S::one(), S::zero(), S::zero())
    }

    /// A unit vector in the `z` direction.
    #[inline]
    pub fn unit_z() -> Vector4<S> {
        Vector4::new(S::zero(), S::zero(), S::one(), S::zero())
    }

    /// A unit vector in the `w` direction.
    #[inline]
    pub fn unit_w() -> Vector4<S> {
        Vector4::new(S::zero(), S::zero(), S::zero(), S::one())
    }

    /// Create a `Vector3`, dropping the `w` value.
    #[inline]
    pub fn truncate(&self)-> Vector3<S> {
        Vector3::new(self.x, self.y, self.z)
    }

    /// Create a `Vector3`, dropping the nth element
    #[inline]
    pub fn truncate_n(&self, n: isize)-> Vector3<S> {
        match n {
            0 => Vector3::new(self.y, self.z, self.w),
            1 => Vector3::new(self.x, self.z, self.w),
            2 => Vector3::new(self.x, self.y, self.w),
            3 => Vector3::new(self.x, self.y, self.z),
            _ => panic!("{:?} is out of range", n)
        }
    }
}

/// Specifies geometric operations for vectors. This is only implemented for
/// 2-dimensional and 3-dimensional vectors.
pub trait EuclideanVector<S: BaseFloat>: Vector<S>
                                       + ApproxEq<S>
                                       + Sized {
    /// Returns `true` if the vector is perpendicular (at right angles) to the
    /// other vector.
    fn is_perpendicular(&self, other: &Self) -> bool {
        self.dot(other).approx_eq(&S::zero())
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
    #[must_use]
    fn normalize(&self) -> Self {
        self.normalize_to(S::one())
    }

    /// Returns a vector with the same direction and a given `length`.
    #[inline]
    #[must_use]
    fn normalize_to(&self, length: S) -> Self {
        self.mul_s(length / self.length())
    }

    /// Returns the result of linarly interpolating the length of the vector
    /// towards the length of `other` by the specified amount.
    #[inline]
    #[must_use]
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

impl<S: BaseFloat> EuclideanVector<S> for Vector2<S> {
    #[inline]
    fn angle(&self, other: &Vector2<S>) -> Rad<S> {
        atan2(self.perp_dot(other), self.dot(other))
    }
}

impl<S: BaseFloat> EuclideanVector<S> for Vector3<S> {
    #[inline]
    fn angle(&self, other: &Vector3<S>) -> Rad<S> {
        atan2(self.cross(other).length(), self.dot(other))
    }
}

impl<S: BaseFloat> EuclideanVector<S> for Vector4<S> {
    #[inline]
    fn angle(&self, other: &Vector4<S>) -> Rad<S> {
        acos(self.dot(other) / (self.length() * other.length()))
    }
}

impl<S: BaseNum> fmt::Debug for Vector2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?}]", self.x, self.y)
    }
}

impl<S: BaseNum> fmt::Debug for Vector3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?}, {:?}]", self.x, self.y, self.z)
    }
}

impl<S: BaseNum> fmt::Debug for Vector4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?}, {:?}, {:?}]", self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    mod vector2 {
        use vector::*;

        const VECTOR2: Vector2<i32> = Vector2 { x: 1, y: 2 };

        #[test]
        fn test_index() {
            assert_eq!(VECTOR2[0], VECTOR2.x);
            assert_eq!(VECTOR2[1], VECTOR2.y);
        }

        #[test]
        fn test_index_mut() {
            let mut v = VECTOR2;
            *&mut v[0] = 0;
            assert_eq!(v, [0, 2].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            VECTOR2[2];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&VECTOR2[..0], &[]);
            assert_eq!(&VECTOR2[..1], &[1]);
            assert_eq!(VECTOR2[..0].len(), 0);
            assert_eq!(VECTOR2[..1].len(), 1);
            assert_eq!(&VECTOR2[2..], &[]);
            assert_eq!(&VECTOR2[1..], &[2]);
            assert_eq!(VECTOR2[2..].len(), 0);
            assert_eq!(VECTOR2[1..].len(), 1);
            assert_eq!(&VECTOR2[..], &[1, 2]);
            assert_eq!(VECTOR2[..].len(), 2);
        }

        #[test]
        fn test_into() {
            let v = VECTOR2;
            {
                let v: [i32; 2] = v.into();
                assert_eq!(v, [1, 2]);
            }
            {
                let v: (i32, i32) = v.into();
                assert_eq!(v, (1, 2));
            }
        }

        #[test]
        fn test_as_ref() {
            let v = VECTOR2;
            {
                let v: &[i32; 2] = v.as_ref();
                assert_eq!(v, &[1, 2]);
            }
            {
                let v: &(i32, i32) = v.as_ref();
                assert_eq!(v, &(1, 2));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut v = VECTOR2;
            {
                let v: &mut [i32; 2] = v.as_mut();
                assert_eq!(v, &mut [1, 2]);
            }
            {
                let v: &mut (i32, i32) = v.as_mut();
                assert_eq!(v, &mut (1, 2));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Vector2::from([1, 2]), VECTOR2);
            {
                let v = &[1, 2];
                let v: &Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
            {
                let v = &mut [1, 2];
                let v: &mut Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
            assert_eq!(Vector2::from((1, 2)), VECTOR2);
            {
                let v = &(1, 2);
                let v: &Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
            {
                let v = &mut (1, 2);
                let v: &mut Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
        }
    }

    mod vector3 {
        use vector::*;

        const VECTOR3: Vector3<i32> = Vector3 { x: 1, y: 2, z: 3 };

        #[test]
        fn test_index() {
            assert_eq!(VECTOR3[0], VECTOR3.x);
            assert_eq!(VECTOR3[1], VECTOR3.y);
            assert_eq!(VECTOR3[2], VECTOR3.z);
        }

        #[test]
        fn test_index_mut() {
            let mut v = VECTOR3;
            *&mut v[1] = 0;
            assert_eq!(v, [1, 0, 3].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            VECTOR3[3];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&VECTOR3[..1], &[1]);
            assert_eq!(&VECTOR3[..2], &[1, 2]);
            assert_eq!(VECTOR3[..1].len(), 1);
            assert_eq!(VECTOR3[..2].len(), 2);
            assert_eq!(&VECTOR3[2..], &[3]);
            assert_eq!(&VECTOR3[1..], &[2, 3]);
            assert_eq!(VECTOR3[2..].len(), 1);
            assert_eq!(VECTOR3[1..].len(), 2);
            assert_eq!(&VECTOR3[..], &[1, 2, 3]);
            assert_eq!(VECTOR3[..].len(), 3);
        }

        #[test]
        fn test_into() {
            let v = VECTOR3;
            {
                let v: [i32; 3] = v.into();
                assert_eq!(v, [1, 2, 3]);
            }
            {
                let v: (i32, i32, i32) = v.into();
                assert_eq!(v, (1, 2, 3));
            }
        }

        #[test]
        fn test_as_ref() {
            let v = VECTOR3;
            {
                let v: &[i32; 3] = v.as_ref();
                assert_eq!(v, &[1, 2, 3]);
            }
            {
                let v: &(i32, i32, i32) = v.as_ref();
                assert_eq!(v, &(1, 2, 3));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut v = VECTOR3;
            {
                let v: &mut [i32; 3] = v.as_mut();
                assert_eq!(v, &mut [1, 2, 3]);
            }
            {
                let v: &mut (i32, i32, i32) = v.as_mut();
                assert_eq!(v, &mut (1, 2, 3));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Vector3::from([1, 2, 3]), VECTOR3);
            {
                let v = &[1, 2, 3];
                let v: &Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
            {
                let v = &mut [1, 2, 3];
                let v: &mut Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
            assert_eq!(Vector3::from((1, 2, 3)), VECTOR3);
            {
                let v = &(1, 2, 3);
                let v: &Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
            {
                let v = &mut (1, 2, 3);
                let v: &mut Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
        }
    }

    mod vector4 {
        use vector::*;

        const VECTOR4: Vector4<i32> = Vector4 { x: 1, y: 2, z: 3, w: 4 };

        #[test]
        fn test_index() {
            assert_eq!(VECTOR4[0], VECTOR4.x);
            assert_eq!(VECTOR4[1], VECTOR4.y);
            assert_eq!(VECTOR4[2], VECTOR4.z);
            assert_eq!(VECTOR4[3], VECTOR4.w);
        }

        #[test]
        fn test_index_mut() {
            let mut v = VECTOR4;
            *&mut v[2] = 0;
            assert_eq!(v, [1, 2, 0, 4].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            VECTOR4[4];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&VECTOR4[..2], &[1, 2]);
            assert_eq!(&VECTOR4[..3], &[1, 2, 3]);
            assert_eq!(VECTOR4[..2].len(), 2);
            assert_eq!(VECTOR4[..3].len(), 3);
            assert_eq!(&VECTOR4[2..], &[3, 4]);
            assert_eq!(&VECTOR4[1..], &[2, 3, 4]);
            assert_eq!(VECTOR4[2..].len(), 2);
            assert_eq!(VECTOR4[1..].len(), 3);
            assert_eq!(&VECTOR4[..], &[1, 2, 3, 4]);
            assert_eq!(VECTOR4[..].len(), 4);
        }

        #[test]
        fn test_into() {
            let v = VECTOR4;
            {
                let v: [i32; 4] = v.into();
                assert_eq!(v, [1, 2, 3, 4]);
            }
            {
                let v: (i32, i32, i32, i32) = v.into();
                assert_eq!(v, (1, 2, 3, 4));
            }
        }

        #[test]
        fn test_as_ref() {
            let v = VECTOR4;
            {
                let v: &[i32; 4] = v.as_ref();
                assert_eq!(v, &[1, 2, 3, 4]);
            }
            {
                let v: &(i32, i32, i32, i32) = v.as_ref();
                assert_eq!(v, &(1, 2, 3, 4));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut v = VECTOR4;
            {
                let v: &mut[i32; 4] = v.as_mut();
                assert_eq!(v, &mut [1, 2, 3, 4]);
            }
            {
                let v: &mut(i32, i32, i32, i32) = v.as_mut();
                assert_eq!(v, &mut (1, 2, 3, 4));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Vector4::from([1, 2, 3, 4]), VECTOR4);
            {
                let v = &[1, 2, 3, 4];
                let v: &Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
            {
                let v = &mut [1, 2, 3, 4];
                let v: &mut Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
            assert_eq!(Vector4::from((1, 2, 3, 4)), VECTOR4);
            {
                let v = &(1, 2, 3, 4);
                let v: &Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
            {
                let v = &mut (1, 2, 3, 4);
                let v: &mut Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
        }
    }
}
