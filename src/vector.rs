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
//! use cgmath::{Vector, Vector2, Vector3, Vector4, vec3};
//!
//! assert_eq!(Vector2::new(1.0f64, 0.0f64), Vector2::unit_x());
//! assert_eq!(vec3(0.0f64, 0.0f64, 0.0f64), Vector3::zero());
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
//! assert_eq!(a + b, Vector2::zero());
//! assert_eq!(-(a * 2.0), Vector2::new(-6.0, -8.0));
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
//! use cgmath::{Vector, EuclideanVector};
//! use cgmath::{Vector2, Vector3, Vector4};
//!
//! // All vectors implement the dot product as a method:
//! let a: Vector2<f64> = Vector2::new(3.0, 6.0);
//! let b: Vector2<f64> = Vector2::new(-2.0, 1.0);
//! assert_eq!(a.dot(b), 0.0);
//!
//! // But there is also a top-level function:
//! assert_eq!(a.dot(b), cgmath::dot(a, b));
//!
//! // Cross products are defined for 3-dimensional vectors:
//! let e: Vector3<f64> = Vector3::unit_x();
//! let f: Vector3<f64> = Vector3::unit_y();
//! assert_eq!(e.cross(f), Vector3::unit_z());
//! ```
//!
//! Several other useful methods are provided as well. Vector fields can be
//! accessed using array syntax (i.e. `vector[0] == vector.x`), or by using
//! the methods provided by the [`Array`](../array/trait.Array.html) trait.
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

use angle::{Angle, Rad};
use approx::ApproxEq;
use array::{Array, ElementWise};
use num::{BaseNum, BaseFloat, PartialOrd};

/// Vectors that can be [added](http://mathworld.wolfram.com/VectorAddition.html)
/// together and [multiplied](https://en.wikipedia.org/wiki/Scalar_multiplication)
/// by scalars.
///
/// # Required operators
///
/// ## Vector addition
///
/// Vectors can be added, subtracted, or negated via the following traits:
///
/// - `Add<Output = Self>`
/// - `Sub<Output = Self>`
/// - `Neg<Output = Self>`
///
/// ```rust
/// use cgmath::Vector3;
///
/// let velocity0 = Vector3::new(1, 2, 0);
/// let velocity1 = Vector3::new(1, 1, 0);
///
/// let total_velocity = velocity0 + velocity1;
/// let velocity_diff = velocity1 - velocity0;
/// let reversed_velocity0 = -velocity0;
/// ```
///
/// ## Scalar multiplication
///
/// Vectors can be multiplied or divided by their associated scalars via the
/// following traits:
///
/// - `Mul<Self::Scalar, Output = Self>`
/// - `Div<Self::Scalar, Output = Self>`
/// - `Rem<Self::Scalar, Output = Self>`
///
/// ```rust
/// use cgmath::Vector2;
///
/// let translation = Vector2::new(3.0, 4.0);
/// let scale_factor = 2.0;
///
/// let upscaled_translation = translation * scale_factor;
/// let downscaled_translation = translation / scale_factor;
/// ```
pub trait Vector: Copy + Clone where
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: Array<Element = <Self as Vector>::Scalar>,

    Self: Add<Self, Output = Self>,
    Self: Sub<Self, Output = Self>,

    Self: Mul<<Self as Vector>::Scalar, Output = Self>,
    Self: Div<<Self as Vector>::Scalar, Output = Self>,
    Self: Rem<<Self as Vector>::Scalar, Output = Self>,
{
    /// The associated scalar.
    type Scalar: BaseNum;

    /// The additive identity.
    ///
    /// Adding this to another `Self` value has no effect.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Vector2;
    ///
    /// let v = Vector2::new(1, 2);
    /// assert_eq!(v + Vector2::zero(), v);
    /// ```
    fn zero() -> Self;
}

/// A 2-dimensional vector.
///
/// This type is marked as `#[repr(C, packed)]`.
#[repr(C, packed)]
#[derive(PartialEq, Eq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
pub struct Vector2<S> {
    pub x: S,
    pub y: S,
}

/// A 3-dimensional vector.
///
/// This type is marked as `#[repr(C, packed)]`.
#[repr(C, packed)]
#[derive(PartialEq, Eq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
pub struct Vector3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

/// A 4-dimensional vector.
///
/// This type is marked as `#[repr(C, packed)]`.
#[repr(C, packed)]
#[derive(PartialEq, Eq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
pub struct Vector4<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

// Utility macro for generating associated functions for the vectors
macro_rules! impl_vector {
    ($VectorN:ident <$S:ident> { $($field:ident),+ }, $n:expr, $constructor:ident) => {
        impl<$S> $VectorN<$S> {
            /// Construct a new vector, using the provided values.
            #[inline]
            pub fn new($($field: $S),+) -> $VectorN<$S> {
                $VectorN { $($field: $field),+ }
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

        impl<S: Copy> Array for $VectorN<S> {
            type Element = S;

            #[inline]
            fn from_value(scalar: S) -> $VectorN<S> {
                $VectorN { $($field: scalar),+ }
            }

            #[inline]
            fn sum(self) -> S where S: Add<Output = S> {
                fold_array!(add, { $(self.$field),+ })
            }

            #[inline]
            fn product(self) -> S where S: Mul<Output = S> {
                fold_array!(mul, { $(self.$field),+ })
            }

            #[inline]
            fn min(self) -> S where S: PartialOrd {
                fold_array!(partial_min, { $(self.$field),+ })
            }

            #[inline]
            fn max(self) -> S where S: PartialOrd {
                fold_array!(partial_max, { $(self.$field),+ })
            }
        }

        impl<S: BaseNum> Vector for $VectorN<S> {
            type Scalar = S;

            #[inline]
            fn zero() -> Self {
                Self::from_value(Self::Scalar::zero())
            }
        }

        impl<S: Neg<Output = S>> Neg for $VectorN<S> {
            type Output = $VectorN<S>;

            #[inline]
            fn neg(self) -> $VectorN<S> { $VectorN::new($(-self.$field),+) }
        }

        impl<S: BaseFloat> ApproxEq for $VectorN<S> {
            type Epsilon = S;

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

        impl_operator!(<S: BaseNum> Add<$VectorN<S> > for $VectorN<S> {
            fn add(lhs, rhs) -> $VectorN<S> { $VectorN::new($(lhs.$field + rhs.$field),+) }
        });
        impl_assignment_operator!(<S: BaseNum> AddAssign<$VectorN<S> > for $VectorN<S> {
            fn add_assign(&mut self, other) { $(self.$field += other.$field);+ }
        });

        impl_operator!(<S: BaseNum> Sub<$VectorN<S> > for $VectorN<S> {
            fn sub(lhs, rhs) -> $VectorN<S> { $VectorN::new($(lhs.$field - rhs.$field),+) }
        });
        impl_assignment_operator!(<S: BaseNum> SubAssign<$VectorN<S> > for $VectorN<S> {
            fn sub_assign(&mut self, other) { $(self.$field -= other.$field);+ }
        });

        impl_operator!(<S: BaseNum> Mul<S> for $VectorN<S> {
            fn mul(vector, scalar) -> $VectorN<S> { $VectorN::new($(vector.$field * scalar),+) }
        });
        impl_assignment_operator!(<S: BaseNum> MulAssign<S> for $VectorN<S> {
            fn mul_assign(&mut self, scalar) { $(self.$field *= scalar);+ }
        });

        impl_operator!(<S: BaseNum> Div<S> for $VectorN<S> {
            fn div(vector, scalar) -> $VectorN<S> { $VectorN::new($(vector.$field / scalar),+) }
        });
        impl_assignment_operator!(<S: BaseNum> DivAssign<S> for $VectorN<S> {
            fn div_assign(&mut self, scalar) { $(self.$field /= scalar);+ }
        });

        impl_operator!(<S: BaseNum> Rem<S> for $VectorN<S> {
            fn rem(vector, scalar) -> $VectorN<S> { $VectorN::new($(vector.$field % scalar),+) }
        });
        impl_assignment_operator!(<S: BaseNum> RemAssign<S> for $VectorN<S> {
            fn rem_assign(&mut self, scalar) { $(self.$field %= scalar);+ }
        });

        impl<S: BaseNum> ElementWise for $VectorN<S> {
            #[inline] fn add_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field + rhs.$field),+) }
            #[inline] fn sub_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field - rhs.$field),+) }
            #[inline] fn mul_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field * rhs.$field),+) }
            #[inline] fn div_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field / rhs.$field),+) }
            #[inline] fn rem_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field % rhs.$field),+) }

            #[cfg(feature = "unstable")] #[inline] fn add_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field += rhs.$field);+ }
            #[cfg(feature = "unstable")] #[inline] fn sub_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field -= rhs.$field);+ }
            #[cfg(feature = "unstable")] #[inline] fn mul_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field *= rhs.$field);+ }
            #[cfg(feature = "unstable")] #[inline] fn div_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field /= rhs.$field);+ }
            #[cfg(feature = "unstable")] #[inline] fn rem_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field %= rhs.$field);+ }
        }

        impl<S: BaseNum> ElementWise<S> for $VectorN<S> {
            #[inline] fn add_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field + rhs),+) }
            #[inline] fn sub_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field - rhs),+) }
            #[inline] fn mul_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field * rhs),+) }
            #[inline] fn div_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field / rhs),+) }
            #[inline] fn rem_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field % rhs),+) }

            #[cfg(feature = "unstable")] #[inline] fn add_assign_element_wise(&mut self, rhs: S) { $(self.$field += rhs);+ }
            #[cfg(feature = "unstable")] #[inline] fn sub_assign_element_wise(&mut self, rhs: S) { $(self.$field -= rhs);+ }
            #[cfg(feature = "unstable")] #[inline] fn mul_assign_element_wise(&mut self, rhs: S) { $(self.$field *= rhs);+ }
            #[cfg(feature = "unstable")] #[inline] fn div_assign_element_wise(&mut self, rhs: S) { $(self.$field /= rhs);+ }
            #[cfg(feature = "unstable")] #[inline] fn rem_assign_element_wise(&mut self, rhs: S) { $(self.$field %= rhs);+ }
        }

        impl_scalar_ops!($VectorN<usize> { $($field),+ });
        impl_scalar_ops!($VectorN<u8> { $($field),+ });
        impl_scalar_ops!($VectorN<u16> { $($field),+ });
        impl_scalar_ops!($VectorN<u32> { $($field),+ });
        impl_scalar_ops!($VectorN<u64> { $($field),+ });
        impl_scalar_ops!($VectorN<isize> { $($field),+ });
        impl_scalar_ops!($VectorN<i8> { $($field),+ });
        impl_scalar_ops!($VectorN<i16> { $($field),+ });
        impl_scalar_ops!($VectorN<i32> { $($field),+ });
        impl_scalar_ops!($VectorN<i64> { $($field),+ });
        impl_scalar_ops!($VectorN<f32> { $($field),+ });
        impl_scalar_ops!($VectorN<f64> { $($field),+ });

        impl_index_operators!($VectorN<S>, $n, S, usize);
        impl_index_operators!($VectorN<S>, $n, [S], Range<usize>);
        impl_index_operators!($VectorN<S>, $n, [S], RangeTo<usize>);
        impl_index_operators!($VectorN<S>, $n, [S], RangeFrom<usize>);
        impl_index_operators!($VectorN<S>, $n, [S], RangeFull);
    }
}

macro_rules! impl_scalar_ops {
    ($VectorN:ident<$S:ident> { $($field:ident),+ }) => {
        impl_operator!(Mul<$VectorN<$S>> for $S {
            fn mul(scalar, vector) -> $VectorN<$S> { $VectorN::new($(scalar * vector.$field),+) }
        });
        impl_operator!(Div<$VectorN<$S>> for $S {
            fn div(scalar, vector) -> $VectorN<$S> { $VectorN::new($(scalar / vector.$field),+) }
        });
        impl_operator!(Rem<$VectorN<$S>> for $S {
            fn rem(scalar, vector) -> $VectorN<$S> { $VectorN::new($(scalar % vector.$field),+) }
        });
    };
}

impl_vector!(Vector2<S> { x, y }, 2, vec2);
impl_vector!(Vector3<S> { x, y, z }, 3, vec3);
impl_vector!(Vector4<S> { x, y, z, w }, 4, vec4);

impl_fixed_array_conversions!(Vector2<S> { x: 0, y: 1 }, 2);
impl_fixed_array_conversions!(Vector3<S> { x: 0, y: 1, z: 2 }, 3);
impl_fixed_array_conversions!(Vector4<S> { x: 0, y: 1, z: 2, w: 3 }, 4);

impl_tuple_conversions!(Vector2<S> { x, y }, (S, S));
impl_tuple_conversions!(Vector3<S> { x, y, z }, (S, S, S));
impl_tuple_conversions!(Vector4<S> { x, y, z, w }, (S, S, S, S));

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
    pub fn perp_dot(self, other: Vector2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }

    /// Create a `Vector3`, using the `x` and `y` values from this vector, and the
    /// provided `z`.
    #[inline]
    pub fn extend(self, z: S)-> Vector3<S> {
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
    pub fn cross(self, other: Vector3<S>) -> Vector3<S> {
        Vector3::new((self.y * other.z) - (self.z * other.y),
                     (self.z * other.x) - (self.x * other.z),
                     (self.x * other.y) - (self.y * other.x))
    }

    /// Create a `Vector4`, using the `x`, `y` and `z` values from this vector, and the
    /// provided `w`.
    #[inline]
    pub fn extend(self, w: S)-> Vector4<S> {
        Vector4::new(self.x, self.y, self.z, w)
    }

    /// Create a `Vector2`, dropping the `z` value.
    #[inline]
    pub fn truncate(self)-> Vector2<S> {
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
    pub fn truncate(self)-> Vector3<S> {
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

/// Vectors that also have a [dot](https://en.wikipedia.org/wiki/Dot_product)
/// (or [inner](https://en.wikipedia.org/wiki/Inner_product_space)) product.
///
/// The dot product allows for the definition of other useful operations, like
/// finding the magnitude of a vector or normalizing it.
pub trait EuclideanVector: Vector + Sized where
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    <Self as Vector>::Scalar: BaseFloat,
    Self: ApproxEq<Epsilon = <Self as Vector>::Scalar>,
{
    /// Vector dot (or inner) product.
    fn dot(self, other: Self) -> Self::Scalar;

    /// Returns `true` if the vector is perpendicular (at right angles) to the
    /// other vector.
    fn is_perpendicular(self, other: Self) -> bool {
        Self::dot(self, other).approx_eq(&Self::Scalar::zero())
    }

    /// Returns the squared magnitude of the vector.
    ///
    /// This does not perform an expensive square root operation like in
    /// `Vector::magnitude` method, and so can be used to compare vectors more
    /// efficiently.
    #[inline]
    fn magnitude2(self) -> Self::Scalar {
        Self::dot(self, self)
    }

    /// The distance from the tail to the tip of the vector.
    #[inline]
    fn magnitude(self) -> Self::Scalar {
        use rust_num::Float;

        // FIXME: Not sure why we can't use method syntax for `sqrt` here...
        Float::sqrt(self.magnitude2())
    }

    /// Returns the angle between two vectors in radians.
    fn angle(self, other: Self) -> Rad<Self::Scalar> {
        Rad::acos(Self::dot(self, other) / (self.magnitude() * other.magnitude()))
    }

    /// Returns a vector with the same direction, but with a magnitude of `1`.
    #[inline]
    #[must_use]
    fn normalize(self) -> Self {
        self.normalize_to(Self::Scalar::one())
    }

    /// Returns a vector with the same direction and a given magnitude.
    #[inline]
    #[must_use]
    fn normalize_to(self, magnitude: Self::Scalar) -> Self {
        self * (magnitude / self.magnitude())
    }

    /// Returns the result of linearly interpolating the magnitude of the vector
    /// towards the magnitude of `other` by the specified amount.
    #[inline]
    #[must_use]
    fn lerp(self, other: Self, amount: Self::Scalar) -> Self {
        self + ((other - self) * amount)
    }
}

/// Dot product of two vectors.
#[inline]
pub fn dot<V: EuclideanVector>(a: V, b: V) -> V::Scalar where
    V::Scalar: BaseFloat,
{
    V::dot(a, b)
}

impl<S: BaseFloat> EuclideanVector for Vector2<S> {
    #[inline]
    fn dot(self, other: Vector2<S>) -> S {
        Vector2::mul_element_wise(self, other).sum()
    }

    #[inline]
    fn angle(self, other: Vector2<S>) -> Rad<S> {
        Rad::atan2(Self::perp_dot(self, other), Self::dot(self, other))
    }
}

impl<S: BaseFloat> EuclideanVector for Vector3<S> {
    #[inline]
    fn dot(self, other: Vector3<S>) -> S {
        Vector3::mul_element_wise(self, other).sum()
    }

    #[inline]
    fn angle(self, other: Vector3<S>) -> Rad<S> {
        Rad::atan2(self.cross(other).magnitude(), Self::dot(self, other))
    }
}

impl<S: BaseFloat> EuclideanVector for Vector4<S> {
    #[inline]
    fn dot(self, other: Vector4<S>) -> S {
        Vector4::mul_element_wise(self, other).sum()
    }
}

impl<S: fmt::Debug> fmt::Debug for Vector2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Vector2 "));
        <[S; 2] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Vector3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Vector3 "));
        <[S; 3] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Vector4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Vector4 "));
        <[S; 4] as fmt::Debug>::fmt(self.as_ref(), f)
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
