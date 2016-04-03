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

//! Angle units for type-safe, self-documenting code.

use std::fmt;
use std::f64;
use std::ops::*;

use rand::{Rand, Rng};
use rand::distributions::range::SampleRange;

use rust_num::{Float, Zero};
use rust_num::traits::cast;

use approx::ApproxEq;
use num::BaseFloat;

/// An angle, in radians.
///
/// This type is marked as `#[repr(C, packed)]`.
#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, PartialOrd, RustcEncodable, RustcDecodable)]
pub struct Rad<S> { pub s: S }

/// An angle, in degrees.
///
/// This type is marked as `#[repr(C, packed)]`.
#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, PartialOrd, RustcEncodable, RustcDecodable)]
pub struct Deg<S> { pub s: S }

/// Create a new angle, in radians
#[inline] pub fn rad<S: BaseFloat>(s: S) -> Rad<S> { Rad { s: s } }
/// Create a new angle, in degrees
#[inline] pub fn deg<S: BaseFloat>(s: S) -> Deg<S> { Deg { s: s } }

impl<S> From<Rad<S>> for Deg<S> where S: BaseFloat {
    #[inline]
    fn from(r: Rad<S>) -> Deg<S> {
        Deg::new(r.s * cast(180.0 / f64::consts::PI).unwrap())
    }
}

impl<S> From<Deg<S>> for Rad<S> where S: BaseFloat {
    #[inline]
    fn from(d: Deg<S>) -> Rad<S> {
        Rad::new(d.s * cast(f64::consts::PI / 180.0).unwrap())
    }
}

/// Angles and their associated trigonometric functions.
///
/// Typed angles allow for the writing of self-documenting code that makes it
/// clear when semantic violations have occured - for example, adding degrees to
/// radians, or adding a number to an angle.
///
pub trait Angle where
    Self: Copy + Clone,
    Self: PartialEq + PartialOrd,
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: ApproxEq<Epsilon = <Self as Angle>::Unitless>,

    Self: Neg<Output = Self>,
    Self: Add<Self, Output = Self>,
    Self: Sub<Self, Output = Self>,
    Self: Rem<Self, Output = Self>,
    Self: Mul<<Self as Angle>::Unitless, Output = Self>,
    Self: Div<Self, Output = <Self as Angle>::Unitless>,
    Self: Div<<Self as Angle>::Unitless, Output = Self>,
{
    type Unitless: BaseFloat;

    /// Return the angle, normalized to the range `[0, full_turn)`.
    #[inline]
    fn normalize(self) -> Self {
        let rem = self % Self::full_turn();
        if rem < Self::zero() { rem + Self::full_turn() } else { rem }
    }

    /// Return the angle rotated by half a turn.
    #[inline]
    fn opposite(self) -> Self {
        Self::normalize(self + Self::turn_div_2())
    }

    /// Returns the interior bisector of the two angles.
    #[inline]
    fn bisect(self, other: Self) -> Self {
        let half = cast(0.5f64).unwrap();
        Self::normalize((self - other) * half + self)
    }

    /// The additive identity.
    ///
    /// Adding this to another angle has no affect.
    ///
    /// For example:
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Deg;
    ///
    /// let v = Deg::new(180.0);
    /// assert_eq!(v + Deg::zero(), v);
    /// ```
    fn zero() -> Self;

    /// A full rotation.
    fn full_turn() -> Self;

    /// Half of a full rotation.
    fn turn_div_2() -> Self;

    /// A third of a full rotation.
    fn turn_div_3() -> Self;

    /// A quarter of a full rotation.
    fn turn_div_4() -> Self;

    /// A sixth of a full rotation.
    fn turn_div_6() -> Self;

    /// Compute the sine of the angle, returning a unitless ratio.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad::new(35.0);
    /// let ratio: f32 = Rad::sin(angle);
    /// ```
    fn sin(self) -> Self::Unitless;

    /// Compute the cosine of the angle, returning a unitless ratio.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad::new(35.0);
    /// let ratio: f32 = Rad::cos(angle);
    /// ```
    fn cos(self) -> Self::Unitless;

    /// Compute the tangent of the angle, returning a unitless ratio.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad::new(35.0);
    /// let ratio: f32 = Rad::tan(angle);
    /// ```
    fn tan(self) -> Self::Unitless;

    /// Compute the sine and cosine of the angle, returning the result as a
    /// pair.
    ///
    /// This does not have any performance benefits, but calculating both the
    /// sine and cosine of a single angle is a common operation.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad::new(35.0);
    /// let (s, c) = Rad::sin_cos(angle);
    /// ```
    fn sin_cos(self) -> (Self::Unitless, Self::Unitless);

    /// Compute the cosecant of the angle.
    ///
    /// This is the same as computing the reciprocal of `Self::sin`.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad::new(35.0);
    /// let ratio: f32 = Rad::csc(angle);
    /// ```
    #[inline]
    fn csc(self) -> Self::Unitless {
        Self::sin(self).recip()
    }

    /// Compute the secant of the angle.
    ///
    /// This is the same as computing the reciprocal of `Self::tan`.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad::new(35.0);
    /// let ratio: f32 = Rad::cot(angle);
    /// ```
    #[inline]
    fn cot(self) -> Self::Unitless {
        Self::tan(self).recip()
    }

    /// Compute the cotatangent of the angle.
    ///
    /// This is the same as computing the reciprocal of `Self::cos`.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle = Rad::new(35.0);
    /// let ratio: f32 = Rad::sec(angle);
    /// ```
    #[inline]
    fn sec(self) -> Self::Unitless {
        Self::cos(self).recip()
    }

    /// Compute the arcsine of the ratio, returning the resulting angle.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle: Rad<f32> = Rad::asin(0.5);
    /// ```
    fn asin(ratio: Self::Unitless) -> Self;

    /// Compute the arccosine of the ratio, returning the resulting angle.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle: Rad<f32> = Rad::acos(0.5);
    /// ```
    fn acos(ratio: Self::Unitless) -> Self;

    /// Compute the arctangent of the ratio, returning the resulting angle.
    ///
    /// ```rust
    /// use cgmath::prelude::*;
    /// use cgmath::Rad;
    ///
    /// let angle: Rad<f32> = Rad::atan(0.5);
    /// ```
    fn atan(ratio: Self::Unitless) -> Self;

    fn atan2(a: Self::Unitless, b: Self::Unitless) -> Self;
}

macro_rules! impl_angle {
    ($Angle:ident, $fmt:expr, $full_turn:expr, $hi:expr) => {
        impl<S: BaseFloat> $Angle<S> {
            #[inline]
            pub fn new(value: S) -> $Angle<S> {
                $Angle { s: value }
            }
        }

        impl<S: BaseFloat> Angle for $Angle<S> {
            type Unitless = S;

            #[inline]
            fn zero() -> $Angle<S> {
                $Angle::new(S::zero())
            }

            #[inline] fn full_turn() -> $Angle<S> { $Angle::new(cast($full_turn).unwrap()) }
            #[inline] fn turn_div_2() -> $Angle<S> { let factor: S = cast(2).unwrap(); $Angle::full_turn() / factor }
            #[inline] fn turn_div_3() -> $Angle<S> { let factor: S = cast(3).unwrap(); $Angle::full_turn() / factor }
            #[inline] fn turn_div_4() -> $Angle<S> { let factor: S = cast(4).unwrap(); $Angle::full_turn() / factor }
            #[inline] fn turn_div_6() -> $Angle<S> { let factor: S = cast(6).unwrap(); $Angle::full_turn() / factor }

            #[inline] fn sin(self) -> S { Rad::from(self).s.sin() }
            #[inline] fn cos(self) -> S { Rad::from(self).s.cos() }
            #[inline] fn tan(self) -> S { Rad::from(self).s.tan() }
            #[inline] fn sin_cos(self) -> (S, S) { Rad::from(self).s.sin_cos() }

            #[inline] fn asin(a: S) -> $Angle<S> { Rad::new(a.asin()).into() }
            #[inline] fn acos(a: S) -> $Angle<S> { Rad::new(a.acos()).into() }
            #[inline] fn atan(a: S) -> $Angle<S> { Rad::new(a.atan()).into() }
            #[inline] fn atan2(a: S, b: S) -> $Angle<S> { Rad::new(a.atan2(b)).into() }
        }

        impl<S: BaseFloat> Neg for $Angle<S> {
            type Output = $Angle<S>;

            #[inline]
            fn neg(self) -> $Angle<S> { $Angle::new(-self.s) }
        }

        impl<'a, S: BaseFloat> Neg for &'a $Angle<S> {
            type Output = $Angle<S>;

            #[inline]
            fn neg(self) -> $Angle<S> { $Angle::new(-self.s) }
        }

        impl_operator!(<S: BaseFloat> Add<$Angle<S> > for $Angle<S> {
            fn add(lhs, rhs) -> $Angle<S> { $Angle::new(lhs.s + rhs.s) }
        });
        impl_operator!(<S: BaseFloat> Sub<$Angle<S> > for $Angle<S> {
            fn sub(lhs, rhs) -> $Angle<S> { $Angle::new(lhs.s - rhs.s) }
        });
        impl_operator!(<S: BaseFloat> Div<$Angle<S> > for $Angle<S> {
            fn div(lhs, rhs) -> S { lhs.s / rhs.s }
        });
        impl_operator!(<S: BaseFloat> Rem<$Angle<S> > for $Angle<S> {
            fn rem(lhs, rhs) -> $Angle<S> { $Angle::new(lhs.s % rhs.s) }
        });
        impl_assignment_operator!(<S: BaseFloat> AddAssign<$Angle<S> > for $Angle<S> {
            fn add_assign(&mut self, other) { self.s + other.s; }
        });
        impl_assignment_operator!(<S: BaseFloat> SubAssign<$Angle<S> > for $Angle<S> {
            fn sub_assign(&mut self, other) { self.s - other.s; }
        });
        impl_assignment_operator!(<S: BaseFloat> RemAssign<$Angle<S> > for $Angle<S> {
            fn rem_assign(&mut self, other) { self.s % other.s; }
        });

        impl_operator!(<S: BaseFloat> Mul<S> for $Angle<S> {
            fn mul(lhs, scalar) -> $Angle<S> { $Angle::new(lhs.s * scalar) }
        });
        impl_operator!(<S: BaseFloat> Div<S> for $Angle<S> {
            fn div(lhs, scalar) -> $Angle<S> { $Angle::new(lhs.s / scalar) }
        });
        impl_assignment_operator!(<S: BaseFloat> MulAssign<S> for $Angle<S> {
            fn mul_assign(&mut self, scalar) { self.s * scalar; }
        });
        impl_assignment_operator!(<S: BaseFloat> DivAssign<S> for $Angle<S> {
            fn div_assign(&mut self, scalar) { self.s / scalar; }
        });

        impl<S: BaseFloat> ApproxEq for $Angle<S> {
            type Epsilon = S;

            #[inline]
            fn approx_eq_eps(&self, other: &$Angle<S>, epsilon: &S) -> bool {
                self.s.approx_eq_eps(&other.s, epsilon)
            }
        }

        impl<S: BaseFloat + SampleRange> Rand for $Angle<S> {
            #[inline]
            fn rand<R: Rng>(rng: &mut R) -> $Angle<S> {
                $Angle::new(rng.gen_range(cast(-$hi).unwrap(), cast($hi).unwrap()))
            }
        }

        impl<S: fmt::Debug> fmt::Debug for $Angle<S> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, $fmt, self.s)
            }
        }
    }
}

impl_angle!(Rad, "{:?} rad", f64::consts::PI * 2.0, f64::consts::PI);
impl_angle!(Deg, "{:?}°", 360, 180);
