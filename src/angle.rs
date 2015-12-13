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

/// An angle, in radians
#[derive(Copy, Clone, PartialEq, PartialOrd, RustcEncodable, RustcDecodable)]
pub struct Rad<S> { pub s: S }
/// An angle, in degrees
#[derive(Copy, Clone, PartialEq, PartialOrd, RustcEncodable, RustcDecodable)]
pub struct Deg<S> { pub s: S }

/// Create a new angle, in radians
#[inline] pub fn rad<S: BaseFloat>(s: S) -> Rad<S> { Rad { s: s } }
/// Create a new angle, in degrees
#[inline] pub fn deg<S: BaseFloat>(s: S) -> Deg<S> { Deg { s: s } }

impl<S> From<Rad<S>> for Deg<S> where S: BaseFloat {
    #[inline]
    fn from(r: Rad<S>) -> Deg<S> {
        deg(r.s * cast(180.0 / f64::consts::PI).unwrap())
    }
}

impl<S> From<Deg<S>> for Rad<S> where S: BaseFloat {
    #[inline]
    fn from(d: Deg<S>) -> Rad<S> {
        rad(d.s * cast(f64::consts::PI / 180.0).unwrap())
    }
}

/// Private utility functions for converting to/from scalars
trait ScalarConv<S> {
    fn from(s: S) -> Self;
    fn s<'a>(&'a self) -> &'a S;
    fn mut_s<'a>(&'a mut self) -> &'a mut S;
}

impl<S: BaseFloat> ScalarConv<S> for Rad<S> {
    #[inline] fn from(s: S) -> Rad<S> { rad(s) }
    #[inline] fn s<'a>(&'a self) -> &'a S { &self.s }
    #[inline] fn mut_s<'a>(&'a mut self) -> &'a mut S { &mut self.s }
}

impl<S: BaseFloat> ScalarConv<S> for Deg<S> {
    #[inline] fn from(s: S) -> Deg<S> { deg(s) }
    #[inline] fn s<'a>(&'a self) -> &'a S { &self.s }
    #[inline] fn mut_s<'a>(&'a mut self) -> &'a mut S { &mut self.s }
}

/// Operations on angles.
pub trait Angle where
    Self: Copy + Clone,
    Self: PartialEq + PartialOrd,
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: ApproxEq<Epsilon = <Self as Angle>::Unitless>,
    Self: Neg<Output = Self>,
    Self: Into<Rad<<Self as Angle>::Unitless>>,
    Self: Into<Deg<<Self as Angle>::Unitless>>,
    Self: ScalarConv<<Self as Angle>::Unitless>,
    Self: fmt::Debug,
{
    type Unitless: BaseFloat;

    /// Create a new angle from any other valid angle.
    fn from<A: Angle<Unitless = Self::Unitless>>(theta: A) -> Self;

    /// Negate this angle, in-place.
    #[inline] fn neg_self(&mut self) { *self = -*self }

    /// Add this angle with another, returning the new angle.
    #[inline] fn add_a(&self, other: Self) -> Self { ScalarConv::from(*self.s() + *other.s()) }
    /// Subtract another angle from this one, returning the new angle.
    #[inline] fn sub_a(&self, other: Self) -> Self { ScalarConv::from(*self.s() - *other.s()) }
    /// Divide this angle by another, returning the ratio.
    #[inline] fn div_a(&self, other: Self) -> Self::Unitless { *self.s() / *other.s() }
    /// Take the remainder of this angle with another.
    #[inline] fn rem_a(&self, other: Self) -> Self::Unitless { *self.s() % *other.s() }

    /// Multiply this angle by a scalar, returning the new angle.
    #[inline] fn mul_s(&self, scalar: Self::Unitless) -> Self { ScalarConv::from(*self.s() * scalar) }
    /// Divide this angle by a scalar, returing the new angle.
    #[inline] fn div_s(&self, scalar: Self::Unitless) -> Self { ScalarConv::from(*self.s() / scalar) }
    /// Take the remainder of this angle by a scalar, returning the new angle.
    #[inline] fn rem_s(&self, scalar: Self::Unitless) -> Self { ScalarConv::from(*self.s() % scalar) }

    /// Add this angle with another, in-place.
    #[inline] fn add_self_a(&mut self, other: Self) { *self.mut_s() = *self.s() + *other.s() }
    /// Subtract another angle from this one, in-place.
    #[inline] fn sub_self_a(&mut self, other: Self) { *self.mut_s() = *self.s() - *other.s() }

    /// Multiply this angle by a scalar, in-place.
    #[inline] fn mul_self_s(&mut self, scalar: Self::Unitless) { *self.mut_s() = *self.s() * scalar }
    /// Divide this angle by a scalar, in-place.
    #[inline] fn div_self_s(&mut self, scalar: Self::Unitless) { *self.mut_s() = *self.s() / scalar }
    /// Take the remainder of this angle by a scalar, in-place.
    #[inline] fn rem_self_s(&mut self, scalar: Self::Unitless) { *self.mut_s() = *self.s() % scalar }

    /// Return the angle, normalized to the range `[0, full_turn)`.
    #[inline]
    fn normalize(&self) -> Self {
        let mut a = self.clone();
        a.normalize_self();
        a
    }

    /// Normalize the angle to the range `[0, full_turn)`.
    #[inline]
    fn normalize_self(&mut self) {
        let full_turn = Self::full_turn();
        self.rem_self_s(full_turn.s().clone());
        if *self < Self::zero() { self.add_self_a(full_turn) };
    }

    /// Return the angle rotated by half a turn
    #[inline]
    fn opposite(&self) -> Self {
        self.add_a(Self::turn_div_2()).normalize()
    }

    /// Returns the interior bisector of the two angles
    #[inline]
    fn bisect(&self, other: Self) -> Self {
        self.add_a(self.sub_a(other).mul_s(cast(0.5f64).unwrap())).normalize()
    }

    fn zero() -> Self;
    fn full_turn() -> Self;

    #[inline] fn turn_div_2() -> Self { Self::full_turn().div_s(cast(2i8).unwrap()) }
    #[inline] fn turn_div_3() -> Self { Self::full_turn().div_s(cast(3i8).unwrap()) }
    #[inline] fn turn_div_4() -> Self { Self::full_turn().div_s(cast(4i8).unwrap()) }
    #[inline] fn turn_div_6() -> Self { Self::full_turn().div_s(cast(6i8).unwrap()) }

    #[inline] fn equiv(&self, other: &Self) -> bool { self.normalize() == other.normalize() }
}

#[inline] pub fn bisect<A: Angle>(a: A, b: A) -> A { a.bisect(b) }

#[inline] pub fn sin<S: BaseFloat, R: Into<Rad<S>>>(theta: R) -> S { theta.into().s.sin() }
#[inline] pub fn cos<S: BaseFloat, R: Into<Rad<S>>>(theta: R) -> S { theta.into().s.cos() }
#[inline] pub fn tan<S: BaseFloat, R: Into<Rad<S>>>(theta: R) -> S { theta.into().s.tan() }
#[inline] pub fn sin_cos<S: BaseFloat, R: Into<Rad<S>>>(theta: R) -> (S, S) { theta.into().s.sin_cos() }

#[inline] pub fn cot<S: BaseFloat, R: Into<Rad<S>>>(theta: R) -> S { tan(theta.into()).recip() }
#[inline] pub fn sec<S: BaseFloat, R: Into<Rad<S>>>(theta: R) -> S { cos(theta.into()).recip() }
#[inline] pub fn csc<S: BaseFloat, R: Into<Rad<S>>>(theta: R) -> S { sin(theta.into()).recip() }

#[inline] pub fn asin<S: BaseFloat, R: From<Rad<S>>>(s: S) -> R { rad(s.asin()).into() }
#[inline] pub fn acos<S: BaseFloat, R: From<Rad<S>>>(s: S) -> R { rad(s.acos()).into() }
#[inline] pub fn atan<S: BaseFloat, R: From<Rad<S>>>(s: S) -> R { rad(s.atan()).into() }
#[inline] pub fn atan2<S: BaseFloat, R: From<Rad<S>>>(a: S, b: S) -> R { rad(a.atan2(b)).into() }

macro_rules! impl_angle {
    ($Angle:ident, $fmt:expr, $full_turn:expr, $hi:expr) => {
        impl<S: BaseFloat> Angle for $Angle<S> {
            type Unitless = S;

            #[inline]
            fn zero() -> $Angle<S> { ScalarConv::from(S::zero()) }

            #[inline]
            fn from<A: Angle<Unitless = S>>(theta: A) -> $Angle<S> { theta.into() }

            #[inline]
            fn full_turn() -> $Angle<S> { ScalarConv::from(cast($full_turn).unwrap()) }
        }

        impl<S: BaseFloat> Neg for $Angle<S> {
            type Output = $Angle<S>;

            #[inline]
            fn neg(self) -> $Angle<S> { ScalarConv::from(-self.s) }
        }

        impl<'a, S: BaseFloat> Neg for &'a $Angle<S> {
            type Output = $Angle<S>;

            #[inline]
            fn neg(self) -> $Angle<S> { ScalarConv::from(-self.s) }
        }

        impl_binary_operator!(<S: BaseFloat> Add<$Angle<S> > for $Angle<S> {
            fn add(lhs, rhs) -> $Angle<S> { ScalarConv::from(lhs.s + rhs.s) }
        });
        impl_binary_operator!(<S: BaseFloat> Sub<$Angle<S> > for $Angle<S> {
            fn sub(lhs, rhs) -> $Angle<S> { ScalarConv::from(lhs.s - rhs.s) }
        });
        impl_binary_operator!(<S: BaseFloat> Div<$Angle<S> > for $Angle<S> {
            fn div(lhs, rhs) -> S { lhs.s / rhs.s }
        });
        impl_binary_operator!(<S: BaseFloat> Rem<$Angle<S> > for $Angle<S> {
            fn rem(lhs, rhs) -> S { lhs.s % rhs.s }
        });

        impl_binary_operator!(<S: BaseFloat> Mul<S> for $Angle<S> {
            fn mul(lhs, scalar) -> $Angle<S> { ScalarConv::from(lhs.s * scalar) }
        });
        impl_binary_operator!(<S: BaseFloat> Div<S> for $Angle<S> {
            fn div(lhs, scalar) -> $Angle<S> { ScalarConv::from(lhs.s / scalar) }
        });
        impl_binary_operator!(<S: BaseFloat> Rem<S> for $Angle<S> {
            fn rem(lhs, scalar) -> $Angle<S> { ScalarConv::from(lhs.s % scalar) }
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
                let angle: S = rng.gen_range(cast(-$hi).unwrap(), cast($hi).unwrap());
                ScalarConv::from(angle)
            }
        }

        impl<S: BaseFloat> fmt::Debug for $Angle<S> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, $fmt, self.s)
            }
        }
    }
}

impl_angle!(Rad, "{:?} rad", f64::consts::PI * 2.0, f64::consts::PI);
impl_angle!(Deg, "{:?}°", 360, 180);
