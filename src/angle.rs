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

use rust_num::{Float, One, Zero};
use rust_num::traits::cast;

use approx::ApproxEq;
use num::BaseFloat;

/// An angle, in radians
#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, RustcEncodable, RustcDecodable)]
pub struct Rad<S> { pub s: S }
/// An angle, in degrees
#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, RustcEncodable, RustcDecodable)]
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
pub trait Angle
<
    S: BaseFloat
>
:   Clone + Zero
+   PartialEq + PartialOrd
+   ApproxEq<S>
+   Neg<Output=Self>
+   Into<Rad<S>>
+   Into<Deg<S>>
+   ScalarConv<S>
+   fmt::Debug
{
    /// Create a new angle from any other valid angle.
    fn from<A: Angle<S>>(theta: A) -> Self;

    /// Negate this angle, in-place.
    #[inline] fn neg_self(&mut self) { *self = -(*self).clone() }

    /// Add this angle with another, returning the new angle.
    #[inline] fn add_a(&self, other: Self) -> Self { ScalarConv::from(*self.s() + *other.s()) }
    /// Subtract another angle from this one, returning the new angle.
    #[inline] fn sub_a(&self, other: Self) -> Self { ScalarConv::from(*self.s() - *other.s()) }
    /// Divide this angle by another, returning the ratio.
    #[inline] fn div_a(&self, other: Self) -> S { *self.s() / *other.s() }
    /// Take the remainder of this angle with another.
    #[inline] fn rem_a(&self, other: Self) -> S { *self.s() % *other.s() }

    /// Multiply this angle by a scalar, returning the new angle.
    #[inline] fn mul_s(&self, s: S) -> Self { ScalarConv::from(*self.s() * s) }
    /// Divide this angle by a scalar, returing the new angle.
    #[inline] fn div_s(&self, s: S) -> Self { ScalarConv::from(*self.s() / s) }
    /// Take the remainder of this angle by a scalar, returning the new angle.
    #[inline] fn rem_s(&self, s: S) -> Self { ScalarConv::from(*self.s() % s) }

    /// Add this angle with another, in-place.
    #[inline] fn add_self_a(&mut self, other: Self) { *self.mut_s() = *self.s() + *other.s() }
    /// Subtract another angle from this one, in-place.
    #[inline] fn sub_self_a(&mut self, other: Self) { *self.mut_s() = *self.s() - *other.s() }

    /// Multiply this angle by a scalar, in-place.
    #[inline] fn mul_self_s(&mut self, s: S) { *self.mut_s() = *self.s() * s }
    /// Divide this angle by a scalar, in-place.
    #[inline] fn div_self_s(&mut self, s: S) { *self.mut_s() = *self.s() / s }
    /// Take the remainder of this angle by a scalar, in-place.
    #[inline] fn rem_self_s(&mut self, s: S) { *self.mut_s() = *self.s() % s }

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

    fn full_turn() -> Self;

    #[inline] fn turn_div_2() -> Self { Self::full_turn().div_s(cast(2i8).unwrap()) }
    #[inline] fn turn_div_3() -> Self { Self::full_turn().div_s(cast(3i8).unwrap()) }
    #[inline] fn turn_div_4() -> Self { Self::full_turn().div_s(cast(4i8).unwrap()) }
    #[inline] fn turn_div_6() -> Self { Self::full_turn().div_s(cast(6i8).unwrap()) }

    #[inline] fn equiv(&self, other: &Self) -> bool { self.normalize() == other.normalize() }
}

#[inline] pub fn bisect<S: BaseFloat, A: Angle<S>>(a: A, b: A) -> A { a.bisect(b) }

impl<R: Into<Rad<S>>, S: BaseFloat> Add<R> for Rad<S> {
    type Output = Rad<S>;

    #[inline]
    fn add(self, other: R) -> Rad<S> { rad(self.s + other.into().s) }
}

impl<R: Into<Rad<S>>, S: BaseFloat> Add<R> for Deg<S> {
    type Output = Deg<S>;

    #[inline]
    fn add(self, other: R) -> Deg<S> { deg(self.s + other.into().s) }
}

impl<R: Into<Rad<S>>, S: BaseFloat> Sub<R> for Rad<S> {
    type Output = Rad<S>;

    #[inline]
    fn sub(self, other: R) -> Rad<S> { rad(self.s - other.into().s) }
}

impl<R: Into<Rad<S>>, S: BaseFloat> Sub<R> for Deg<S> {
    type Output = Deg<S>;

    #[inline]
    fn sub(self, other: R) -> Deg<S> { deg(self.s - other.into().s) }
}

impl<S: BaseFloat> Neg for Rad<S> {
    type Output = Rad<S>;

    #[inline]
    fn neg(self) -> Rad<S> { rad(-self.s) }
}
impl<S: BaseFloat> Neg for Deg<S> {
    type Output = Deg<S>;

    #[inline]
    fn neg(self) -> Deg<S> { deg(-self.s) }
}

impl<S: BaseFloat> Zero for Rad<S> {
    #[inline]
    fn zero() -> Rad<S> { rad(S::zero()) }
    #[inline]
    fn is_zero(&self) -> bool { *self == Self::zero() }
}
impl<S: BaseFloat> Zero for Deg<S> {
    #[inline]
    fn zero() -> Deg<S> { deg(S::zero()) }
    #[inline]
    fn is_zero(&self) -> bool { *self == Self::zero() }
}

impl<R: Into<Rad<S>>, S: BaseFloat> Mul<R> for Rad<S> {
    type Output = Rad<S>;

    #[inline]
    fn mul(self, other: R) -> Rad<S> { rad(self.s * other.into().s) }
}

impl<R: Into<Rad<S>>, S: BaseFloat> Mul<R> for Deg<S> {
    type Output = Deg<S>;

    #[inline]
    fn mul(self, other: R) -> Deg<S> { deg(self.s * other.into().s) }
}

impl<S: BaseFloat> One for Rad<S> {
    #[inline]
    fn one() -> Rad<S> { rad(S::one()) }
}
impl<S: BaseFloat> One for Deg<S> {
    #[inline]
    fn one() -> Deg<S> { deg(S::one()) }
}

const PI_2: f64 = f64::consts::PI * 2f64;
impl<S: BaseFloat>
Angle<S> for Rad<S> {
    #[inline] fn from<A: Angle<S>>(theta: A) -> Rad<S> { theta.into() }
    #[inline] fn full_turn() -> Rad<S> { rad(cast(PI_2).unwrap()) }
}

impl<S: BaseFloat>
Angle<S> for Deg<S> {
    #[inline] fn from<A: Angle<S>>(theta: A) -> Deg<S> { theta.into() }
    #[inline] fn full_turn() -> Deg<S> { deg(cast(360i32).unwrap()) }
}

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

impl<S: BaseFloat + fmt::Debug>
fmt::Debug for Rad<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} rad", self.s)
    }
}

impl<S: BaseFloat + fmt::Debug>
fmt::Debug for Deg<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}°", self.s)
    }
}

impl<S: BaseFloat>
ApproxEq<S> for Rad<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Rad<S>, epsilon: &S) -> bool {
        self.s.approx_eq_eps(&other.s, epsilon)
    }
}

impl<S: BaseFloat>
ApproxEq<S> for Deg<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Deg<S>, epsilon: &S) -> bool {
        self.s.approx_eq_eps(&other.s, epsilon)
    }
}

impl<S: BaseFloat + PartialOrd + SampleRange + Rand> Rand for Rad<S> {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Rad<S> {
        let angle: S = rng.gen_range(cast(-f64::consts::PI).unwrap(), cast(f64::consts::PI).unwrap());
        rad(angle)
    }
}

impl<S: BaseFloat + PartialOrd + SampleRange + Rand> Rand for Deg<S> {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Deg<S> {
        let angle: S = rng.gen_range(cast(-180f64).unwrap(), cast(180f64).unwrap());
        deg(angle)
    }
}
