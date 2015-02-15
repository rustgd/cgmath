// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
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

//! Types and traits for two, three, and four-dimensional vectors.
//!
//! ## Working with Vectors
//!
//! Vectors can be created in several different ways. There is, of course, the
//! traditional `new()` method, but unit vectors, zero vectors, and an identity
//! vector are also provided:
//!
//! ```rust
//! use cgmath::{Vector2, Vector3, Vector4, one, zero, vec3};
//!
//! assert_eq!(Vector2::new(1.0f64, 0.0f64), Vector2::unit_x());
//! assert_eq!(vec3(0.0f64, 0.0f64, 0.0f64), zero());
//! assert_eq!(Vector4::from_value(1.0f64), one());
//! ```
//!
//! Vectors can be manipulated with typical mathematical operations (addition,
//! subtraction, element-wise multiplication, element-wise division, negation)
//! using the built-in operators. The additive and multiplicative neutral
//! elements (zero and one) are also provided by this library
//!
//! ```rust
//! use cgmath::{Vector2, Vector3, Vector4, one, zero};
//!
//! let a: Vector2<f64> = Vector2::new(3.0, 4.0);
//! let b: Vector2<f64> = Vector2::new(-3.0, -4.0);
//!
//! assert_eq!(a + b, zero());
//! assert_eq!(-(a * b), Vector2::new(9.0f64, 16.0f64));
//! assert_eq!(a / one(), a);
//!
//! // As with Rust's `int` and `f32` types, Vectors of different types cannot
//! // be added and so on with impunity. The following will fail to compile:
//! // let c = a + Vector3::new(1.0, 0.0, 2.0);
//!
//! // Instead, we need to convert the Vector2 to a Vector3 by "extending" it
//! // with the value for the last coordinate:
//! let c: Vector3<f64> = a.extend(0.0) + Vector3::new(1.0, 0.0, 2.0);
//!
//! // Similarly, we can "truncate" a Vector4 down to a Vector3:
//! let d: Vector3<f64> = c + Vector4::unit_x().truncate();
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
//! use cgmath::{Vector, Vector2, Vector3, Vector4, dot, zero};
//!
//! // All vectors implement the dot product as a method:
//! let a: Vector2<f64> = Vector2::new(3.0, 6.0);
//! let b: Vector2<f64> = Vector2::new(-2.0, 1.0);
//! assert_eq!(a.dot(&b), zero());
//!
//! // But there is also a top-level function:
//! assert_eq!(a.dot(&b), dot(a, b));
//!
//! // Scalar multiplication can return a new object, or be done in place
//! // to avoid an allocation:
//! let mut c: Vector4<f64> = Vector4::from_value(3.0);
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
use std::num::NumCast;
use std::ops::*;

use angle::{Rad, atan2, acos};
use approx::ApproxEq;
use array::{Array1, FixedArray};
use num::{BaseNum, BaseFloat, Zero, One, zero, one};

/// A trait that specifies a range of numeric operations for vectors. Not all
/// of these make sense from a linear algebra point of view, but are included
/// for pragmatic reasons.
pub trait Vector<S: BaseNum>: Array1<S> + Zero + One + Neg<Output=Self> {
    /// Add a scalar to this vector, returning a new vector.
    fn add_s(&self, s: S) -> Self;
    /// Subtract a scalar from this vector, returning a new vector.
    fn sub_s(&self, s: S) -> Self;
    /// Multiply this vector by a scalar, returning a new vector.
    fn mul_s(&self, s: S) -> Self;
    /// Divide this vector by a scalar, returning a new vector.
    fn div_s(&self, s: S) -> Self;
    /// Take the remainder of this vector by a scalar, returning a new vector.
    fn rem_s(&self, s: S) -> Self;

    /// Add this vector to another, returning a new vector.
    fn add_v(&self, v: &Self) -> Self;
    /// Subtract another vector from this one, returning a new vector.
    fn sub_v(&self, v: &Self) -> Self;
    /// Multiply this vector by another, returning a new vector.
    fn mul_v(&self, v: &Self) -> Self;
    /// Divide this vector by another, returning a new vector.
    fn div_v(&self, v: &Self) -> Self;
    /// Take the remainder of this vector by another, returning a new scalar.
    fn rem_v(&self, v: &Self) -> Self;

    /// Negate this vector in-place.
    fn neg_self(&mut self);

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

    /// The sum of each component of the vector.
    fn comp_add(&self) -> S;
    /// The product of each component of the vector.
    fn comp_mul(&self) -> S;

    /// Vector dot product.
    #[inline]
    fn dot(&self, v: &Self) -> S { self.mul_v(v).comp_add() }

    /// The minimum component of the vector.
    fn comp_min(&self) -> S;
    /// The maximum component of the vector.
    fn comp_max(&self) -> S;
}

/// Dot product of two vectors.
#[inline] pub fn dot<S: BaseNum, V: Vector<S>>(a: V, b: V) -> S { a.dot(&b) }

// Utility macro for generating associated functions for the vectors
macro_rules! vec(
    ($Self_:ident <$S:ident> { $($field:ident),+ }, $n:expr, $constructor:ident) => (
        #[derive_Rand]
        #[derive(PartialEq, Eq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
        pub struct $Self_<S> { $(pub $field: S),+ }

        impl<$S> $Self_<$S> {
            /// Construct a new vector, using the provided values.
            #[inline]
            pub fn new($($field: $S),+) -> $Self_<$S> {
                $Self_ { $($field: $field),+ }
            }
        }

        /// The short constructor.
        #[inline]
        pub fn $constructor<S>($($field: S),+) -> $Self_<S> {
            $Self_::new($($field),+)
        }

        impl<$S: Copy> $Self_<$S> {
            /// Construct a vector from a single value, replicating it.
            #[inline]
            pub fn from_value(value: $S) -> $Self_<$S> {
                $Self_ { $($field: value),+ }
            }
        }

        impl<$S: Zero> Zero for $Self_<$S> {
            #[inline]
            fn zero() -> $Self_<S> { $Self_ { $($field: zero()),+ } }

            #[inline]
            fn is_zero(&self) -> bool { $((self.$field.is_zero()) )&&+ }
        }

        impl<$S: One> One for $Self_<$S> {
            #[inline]
            fn one() -> $Self_<$S> { $Self_ { $($field: one()),+ } }
        }

        impl<$S: NumCast + Copy> $Self_<$S> {
            /// Component-wise casting to another type
            #[inline]
            pub fn cast<T: NumCast>(&self) -> $Self_<T> {
                $Self_ { $($field: NumCast::from(self.$field).unwrap()),+ }
            }
        }

        impl<$S> FixedArray<[$S; $n]> for $Self_<$S> {
            #[inline]
            fn into_fixed(self) -> [$S; $n] {
                match self { $Self_ { $($field),+ } => [$($field),+] }
            }

            #[inline]
            fn as_fixed<'a>(&'a self) -> &'a [$S; $n] {
                unsafe { mem::transmute(self) }
            }

            #[inline]
            fn as_mut_fixed<'a>(&'a mut self) -> &'a mut [$S; $n] {
                unsafe { mem::transmute(self) }
            }

            #[inline]
            fn from_fixed(_v: [$S; $n]) -> $Self_<$S> {
                // match v { [$($field),+] => $Self { $($field: $field),+ } }
                panic!("Unimplemented, pending a fix for rust-lang/rust#16418");
            }

            #[inline]
            fn from_fixed_ref<'a>(v: &'a [$S; $n]) -> &'a $Self_<$S> {
                unsafe { mem::transmute(v) }
            }

            #[inline]
            fn from_fixed_mut<'a>(v: &'a mut [$S; $n]) -> &'a mut $Self_<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<$S: Copy> Index<usize> for $Self_<$S> {
            type Output = S;

            #[inline]
            fn index<'a>(&'a self, i: &usize) -> &'a $S {
                &self.as_fixed()[*i]
            }
        }

        impl<$S: Copy> IndexMut<usize> for $Self_<$S> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: &usize) -> &'a mut $S {
                &mut self.as_mut_fixed()[*i]
            }
        }

        impl<$S: Copy> Array1<$S> for $Self_<$S> {
            #[inline]
            fn map<F>(&mut self, mut op: F) -> $Self_<$S> where F: FnMut($S) -> $S {
                $(self.$field = op(self.$field);)+ *self
            }
        }

        impl<S: BaseNum> Vector<S> for $Self_<S> {
            #[inline] fn add_s(&self, s: S) -> $Self_<S> { $Self_::new($(self.$field + s),+) }
            #[inline] fn sub_s(&self, s: S) -> $Self_<S> { $Self_::new($(self.$field - s),+) }
            #[inline] fn mul_s(&self, s: S) -> $Self_<S> { $Self_::new($(self.$field * s),+) }
            #[inline] fn div_s(&self, s: S) -> $Self_<S> { $Self_::new($(self.$field / s),+) }
            #[inline] fn rem_s(&self, s: S) -> $Self_<S> { $Self_::new($(self.$field % s),+) }

            #[inline] fn add_v(&self, v: &$Self_<S>) -> $Self_<S> { $Self_::new($(self.$field + v.$field),+) }
            #[inline] fn sub_v(&self, v: &$Self_<S>) -> $Self_<S> { $Self_::new($(self.$field - v.$field),+) }
            #[inline] fn mul_v(&self, v: &$Self_<S>) -> $Self_<S> { $Self_::new($(self.$field * v.$field),+) }
            #[inline] fn div_v(&self, v: &$Self_<S>) -> $Self_<S> { $Self_::new($(self.$field / v.$field),+) }
            #[inline] fn rem_v(&self, v: &$Self_<S>) -> $Self_<S> { $Self_::new($(self.$field % v.$field),+) }

            #[inline] fn neg_self(&mut self) { $(self.$field = -self.$field;)+ }

            #[inline] fn add_self_s(&mut self, s: S) { $(self.$field = self.$field + s;)+ }
            #[inline] fn sub_self_s(&mut self, s: S) { $(self.$field = self.$field - s;)+ }
            #[inline] fn mul_self_s(&mut self, s: S) { $(self.$field = self.$field * s;)+ }
            #[inline] fn div_self_s(&mut self, s: S) { $(self.$field = self.$field / s;)+ }
            #[inline] fn rem_self_s(&mut self, s: S) { $(self.$field = self.$field % s;)+ }

            #[inline] fn add_self_v(&mut self, v: &$Self_<S>) { $(self.$field = self.$field + v.$field;)+ }
            #[inline] fn sub_self_v(&mut self, v: &$Self_<S>) { $(self.$field = self.$field - v.$field;)+ }
            #[inline] fn mul_self_v(&mut self, v: &$Self_<S>) { $(self.$field = self.$field * v.$field;)+ }
            #[inline] fn div_self_v(&mut self, v: &$Self_<S>) { $(self.$field = self.$field / v.$field;)+ }
            #[inline] fn rem_self_v(&mut self, v: &$Self_<S>) { $(self.$field = self.$field % v.$field;)+ }

            #[inline] fn comp_add(&self) -> S { fold!(add, { $(self.$field),+ }) }
            #[inline] fn comp_mul(&self) -> S { fold!(mul, { $(self.$field),+ }) }
            #[inline] fn comp_min(&self) -> S { fold!(partial_min, { $(self.$field),+ }) }
            #[inline] fn comp_max(&self) -> S { fold!(partial_max, { $(self.$field),+ }) }
        }

        impl<S: BaseNum> Add for $Self_<S> {
            type Output = $Self_<S>;

            #[inline]
            fn add(self, v: $Self_<S>) -> $Self_<S> { self.add_v(&v) }
        }

        impl<S: BaseNum> Sub for $Self_<S> {
            type Output = $Self_<S>;

            #[inline]
            fn sub(self, v: $Self_<S>) -> $Self_<S> { self.sub_v(&v) }
        }

        impl<S: BaseNum> Neg for $Self_<S> {
            type Output = $Self_<S>;

            #[inline]
            fn neg(self) -> $Self_<S> { $Self_::new($(-self.$field),+) }
        }

        impl<S: BaseNum> Mul for $Self_<S> {
            type Output = $Self_<S>;

            #[inline]
            fn mul(self, v: $Self_<S>) -> $Self_<S> { self.mul_v(&v) }
        }

        impl<S: BaseNum> Div for $Self_<S> {
            type Output = $Self_<S>;

            #[inline]
            fn div(self, v: $Self_<S>) -> $Self_<S> { self.div_v(&v) }
        }

        impl<S: BaseNum> Rem for $Self_<S> {
            type Output = $Self_<S>;

            #[inline]
            fn rem(self, v: $Self_<S>) -> $Self_<S> { self.rem_v(&v) }
        }

        impl<S: BaseFloat> ApproxEq<S> for $Self_<S> {
            #[inline]
            fn approx_eq_eps(&self, other: &$Self_<S>, epsilon: &S) -> bool {
                $(self.$field.approx_eq_eps(&other.$field, epsilon))&&+
            }
        }
    )
);

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

/// Operations specific to numeric two-dimensional vectors.
impl<S: BaseNum> Vector2<S> {
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
        Vector3::new(self.x, self.y, z)
    }
}

/// Operations specific to numeric three-dimensional vectors.
impl<S: BaseNum> Vector3<S> {
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
