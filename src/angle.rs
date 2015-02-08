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

//! Angle units for type-safe, self-documenting code.

use std::fmt;
use std::f64;
use std::num::{cast, Float};
use std::ops::*;

use approx::ApproxEq;
use num::{BaseFloat, One, one, Zero, zero};

/// An angle, in radians
#[derive_Rand]
#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, RustcEncodable, RustcDecodable)]
pub struct Rad<S> { pub s: S }
/// An angle, in degrees
#[derive_Rand]
#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, RustcEncodable, RustcDecodable)]
pub struct Deg<S> { pub s: S }

/// Create a new angle, in radians
#[inline] pub fn rad<S: BaseFloat>(s: S) -> Rad<S> { Rad { s: s } }
/// Create a new angle, in degrees
#[inline] pub fn deg<S: BaseFloat>(s: S) -> Deg<S> { Deg { s: s } }

/// Represents types that can be converted to radians.
pub trait ToRad<S: BaseFloat> {
    /// Convert this value to radians.
    fn to_rad(&self) -> Rad<S>;
}

/// Represents types that can be converted to degrees.
pub trait ToDeg<S: BaseFloat> {
    /// Convert this value to degrees.
    fn to_deg(&self) -> Deg<S>;
}

impl<S: BaseFloat> ToRad<S> for Rad<S> { #[inline] fn to_rad(&self) -> Rad<S> { self.clone() } }
impl<S: BaseFloat> ToRad<S> for Deg<S> { #[inline] fn to_rad(&self) -> Rad<S> { rad(self.s.to_radians()) } }

impl<S: BaseFloat> ToDeg<S> for Rad<S> { #[inline] fn to_deg(&self) -> Deg<S> { deg(self.s.to_degrees()) } }
impl<S: BaseFloat> ToDeg<S> for Deg<S> { #[inline] fn to_deg(&self) -> Deg<S> { self.clone() } }

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
+   ToRad<S>
+   ToDeg<S>
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
        let full_turn: Self = Angle::full_turn();
        self.rem_self_s(full_turn.s().clone());
        if *self < zero() { self.add_self_a(full_turn) };
    }

    /// Return the angle rotated by half a turn
    #[inline]
    fn opposite(&self) -> Self {
        self.add_a(Angle::turn_div_2()).normalize()
    }

    /// Returns the interior bisector of the two angles
    #[inline]
    fn bisect(&self, other: Self) -> Self {
        self.add_a(self.sub_a(other).mul_s(cast(0.5f64).unwrap())).normalize()
    }

    fn full_turn() -> Self;

    #[inline] fn turn_div_2() -> Self { let full_turn: Self = Angle::full_turn(); full_turn.div_s(cast(2i8).unwrap()) }
    #[inline] fn turn_div_3() -> Self { let full_turn: Self = Angle::full_turn(); full_turn.div_s(cast(3i8).unwrap()) }
    #[inline] fn turn_div_4() -> Self { let full_turn: Self = Angle::full_turn(); full_turn.div_s(cast(4i8).unwrap()) }
    #[inline] fn turn_div_6() -> Self { let full_turn: Self = Angle::full_turn(); full_turn.div_s(cast(6i8).unwrap()) }

    #[inline] fn equiv(&self, other: &Self) -> bool { self.normalize() == other.normalize() }
}

#[inline] pub fn bisect<S: BaseFloat, A: Angle<S>>(a: A, b: A) -> A { a.bisect(b) }

impl<S: BaseFloat>
Rad<S> {
    #[inline] pub fn zero() -> Rad<S> { zero() }
    #[inline] pub fn full_turn() -> Rad<S> { Angle::full_turn() }
    #[inline] pub fn turn_div_2() -> Rad<S> { Angle::turn_div_2() }
    #[inline] pub fn turn_div_3() -> Rad<S> { Angle::turn_div_3() }
    #[inline] pub fn turn_div_4() -> Rad<S> { Angle::turn_div_4() }
    #[inline] pub fn turn_div_6() -> Rad<S> { Angle::turn_div_6() }
}

impl<S: BaseFloat>
Deg<S> {
    #[inline] pub fn zero() -> Deg<S> { zero() }
    #[inline] pub fn full_turn() -> Deg<S> { Angle::full_turn() }
    #[inline] pub fn turn_div_2() -> Deg<S> { Angle::turn_div_2() }
    #[inline] pub fn turn_div_3() -> Deg<S> { Angle::turn_div_3() }
    #[inline] pub fn turn_div_4() -> Deg<S> { Angle::turn_div_4() }
    #[inline] pub fn turn_div_6() -> Deg<S> { Angle::turn_div_6() }
}


impl<S: BaseFloat> Add for Rad<S> {
    type Output = Rad<S>;

    #[inline]
    fn add(self, other: Rad<S>) -> Rad<S> { rad(self.s + other.s) }
}
impl<S: BaseFloat> Add for Deg<S> {
    type Output = Deg<S>;

    #[inline]
    fn add(self, other: Deg<S>) -> Deg<S> { deg(self.s + other.s) }
}

impl<S: BaseFloat> Sub for Rad<S> {
    type Output = Rad<S>;

    #[inline]
    fn sub(self, other: Rad<S>) -> Rad<S> { rad(self.s - other.s) }
}
impl<S: BaseFloat> Sub for Deg<S> {
    type Output = Deg<S>;

    #[inline]
    fn sub(self, other: Deg<S>) -> Deg<S> { deg(self.s - other.s) }
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
    fn zero() -> Rad<S> { rad(zero()) }
    #[inline]
    fn is_zero(&self) -> bool { *self == zero() }
}
impl<S: BaseFloat> Zero for Deg<S> {
    #[inline]
    fn zero() -> Deg<S> { deg(zero()) }
    #[inline]
    fn is_zero(&self) -> bool { *self == zero() }
}

impl<S: BaseFloat> Mul for Rad<S> {
    type Output = Rad<S>;

    #[inline]
    fn mul(self, other: Rad<S>) -> Rad<S> { rad(self.s * other.s) }
}
impl<S: BaseFloat> Mul for Deg<S> {
    type Output = Deg<S>;

    #[inline]
    fn mul(self, other: Deg<S>) -> Deg<S> { deg(self.s * other.s) }
}

impl<S: BaseFloat> One for Rad<S> {
    #[inline]
    fn one() -> Rad<S> { rad(one()) }
}
impl<S: BaseFloat> One for Deg<S> {
    #[inline]
    fn one() -> Deg<S> { deg(one()) }
}

impl<S: BaseFloat>
Angle<S> for Rad<S> {
    #[inline] fn from<A: Angle<S>>(theta: A) -> Rad<S> { theta.to_rad() }
    #[inline] fn full_turn() -> Rad<S> { rad(cast(f64::consts::PI_2).unwrap()) }
}

impl<S: BaseFloat>
Angle<S> for Deg<S> {
    #[inline] fn from<A: Angle<S>>(theta: A) -> Deg<S> { theta.to_deg() }
    #[inline] fn full_turn() -> Deg<S> { deg(cast(360i32).unwrap()) }
}

#[inline] pub fn sin<S: BaseFloat>(theta: Rad<S>) -> S { theta.s.sin() }
#[inline] pub fn cos<S: BaseFloat>(theta: Rad<S>) -> S { theta.s.cos() }
#[inline] pub fn tan<S: BaseFloat>(theta: Rad<S>) -> S { theta.s.tan() }
#[inline] pub fn sin_cos<S: BaseFloat>(theta: Rad<S>) -> (S, S) { theta.s.sin_cos() }

#[inline] pub fn cot<S: BaseFloat>(theta: Rad<S>) -> S { tan(theta).recip() }
#[inline] pub fn sec<S: BaseFloat>(theta: Rad<S>) -> S { cos(theta).recip() }
#[inline] pub fn csc<S: BaseFloat>(theta: Rad<S>) -> S { sin(theta).recip() }

#[inline] pub fn asin<S: BaseFloat>(s: S) -> Rad<S> { rad(s.asin()) }
#[inline] pub fn acos<S: BaseFloat>(s: S) -> Rad<S> { rad(s.acos()) }
#[inline] pub fn atan<S: BaseFloat>(s: S) -> Rad<S> { rad(s.atan()) }
#[inline] pub fn atan2<S: BaseFloat>(a: S, b: S) -> Rad<S> { rad(a.atan2(b)) }

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
