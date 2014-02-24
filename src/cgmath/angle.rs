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

//! Angle units for type-safe, self-documenting code.

pub use std::num::{sinh, cosh, tanh};
pub use std::num::{asinh, acosh, atanh};

use std::fmt;
use std::num::{One, one, Zero, zero, cast};

use approx::ApproxEq;

#[deriving(Clone, Eq, Ord, Hash)] pub struct Rad<S> { s: S }
#[deriving(Clone, Eq, Ord, Hash)] pub struct Deg<S> { s: S }

#[inline] pub fn rad<S: Float>(s: S) -> Rad<S> { Rad { s: s } }
#[inline] pub fn deg<S: Float>(s: S) -> Deg<S> { Deg { s: s } }

pub trait ToRad<S: Float> { fn to_rad(&self) -> Rad<S>; }
pub trait ToDeg<S: Float> { fn to_deg(&self) -> Deg<S>; }

impl<S: Float> ToRad<S> for Rad<S> { #[inline] fn to_rad(&self) -> Rad<S> { self.clone() } }
impl<S: Float> ToRad<S> for Deg<S> { #[inline] fn to_rad(&self) -> Rad<S> { rad(self.s.to_radians()) } }

impl<S: Float> ToDeg<S> for Rad<S> { #[inline] fn to_deg(&self) -> Deg<S> { deg(self.s.to_degrees()) } }
impl<S: Float> ToDeg<S> for Deg<S> { #[inline] fn to_deg(&self) -> Deg<S> { self.clone() } }

/// Private utility functions for converting to/from scalars
trait ScalarConv<S> {
    fn from(s: S) -> Self;
    fn s<'a>(&'a self) -> &'a S;
    fn mut_s<'a>(&'a mut self) -> &'a mut S;
}

impl<S: Float> ScalarConv<S> for Rad<S> {
    #[inline] fn from(s: S) -> Rad<S> { rad(s) }
    #[inline] fn s<'a>(&'a self) -> &'a S { &'a self.s }
    #[inline] fn mut_s<'a>(&'a mut self) -> &'a mut S { &'a mut self.s }
}

impl<S: Float> ScalarConv<S> for Deg<S> {
    #[inline] fn from(s: S) -> Deg<S> { deg(s) }
    #[inline] fn s<'a>(&'a self) -> &'a S { &'a self.s }
    #[inline] fn mut_s<'a>(&'a mut self) -> &'a mut S { &'a mut self.s }
}

pub trait Angle
<
    S: Float
>
:   Clone + Zero
+   Eq + Equiv<Self> + Ord
+   ApproxEq<S>
+   Neg<Self>
+   ToRad<S>
+   ToDeg<S>
+   ScalarConv<S>
{
    fn from<A: Angle<S>>(theta: A) -> Self;

    #[inline] fn neg_self(&mut self) { *self = -*self }

    #[inline] fn add_a(&self, other: Self) -> Self { ScalarConv::from(*self.s() + *other.s()) }
    #[inline] fn sub_a(&self, other: Self) -> Self { ScalarConv::from(*self.s() - *other.s()) }
    #[inline] fn div_a(&self, other: Self) -> S { *self.s() / *other.s() }
    #[inline] fn rem_a(&self, other: Self) -> S { *self.s() % *other.s() }
    #[inline] fn mul_s(&self, s: S) -> Self { ScalarConv::from(*self.s() * s) }
    #[inline] fn div_s(&self, s: S) -> Self { ScalarConv::from(*self.s() / s) }
    #[inline] fn rem_s(&self, s: S) -> Self { ScalarConv::from(*self.s() % s) }

    #[inline] fn add_self_a(&mut self, other: Self) { *self.mut_s() = *self.s() + *other.s() }
    #[inline] fn sub_self_a(&mut self, other: Self) { *self.mut_s() = *self.s() - *other.s() }
    #[inline] fn mul_self_s(&mut self, s: S) { *self.mut_s() = *self.s() * s }
    #[inline] fn div_self_s(&mut self, s: S) { *self.mut_s() = *self.s() / s }
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
        self.add_a(self.sub_a(other).mul_s(cast(0.5).unwrap())).normalize()
    }

    fn full_turn() -> Self;

    #[inline] fn turn_div_2() -> Self { let full_turn: Self = Angle::full_turn(); full_turn.div_s(cast(2).unwrap()) }
    #[inline] fn turn_div_3() -> Self { let full_turn: Self = Angle::full_turn(); full_turn.div_s(cast(3).unwrap()) }
    #[inline] fn turn_div_4() -> Self { let full_turn: Self = Angle::full_turn(); full_turn.div_s(cast(4).unwrap()) }
    #[inline] fn turn_div_6() -> Self { let full_turn: Self = Angle::full_turn(); full_turn.div_s(cast(6).unwrap()) }
}

#[inline] pub fn bisect<S: Float, A: Angle<S>>(a: A, b: A) -> A { a.bisect(b) }

impl<S: Float + ApproxEq<S>>
Rad<S> {
    #[inline] pub fn zero() -> Rad<S> { zero() }
    #[inline] pub fn full_turn() -> Rad<S> { Angle::full_turn() }
    #[inline] pub fn turn_div_2() -> Rad<S> { Angle::turn_div_2() }
    #[inline] pub fn turn_div_3() -> Rad<S> { Angle::turn_div_3() }
    #[inline] pub fn turn_div_4() -> Rad<S> { Angle::turn_div_4() }
    #[inline] pub fn turn_div_6() -> Rad<S> { Angle::turn_div_6() }
}

impl<S: Float + ApproxEq<S>>
Deg<S> {
    #[inline] pub fn zero() -> Deg<S> { zero() }
    #[inline] pub fn full_turn() -> Deg<S> { Angle::full_turn() }
    #[inline] pub fn turn_div_2() -> Deg<S> { Angle::turn_div_2() }
    #[inline] pub fn turn_div_3() -> Deg<S> { Angle::turn_div_3() }
    #[inline] pub fn turn_div_4() -> Deg<S> { Angle::turn_div_4() }
    #[inline] pub fn turn_div_6() -> Deg<S> { Angle::turn_div_6() }
}


impl<S: Float> Add<Rad<S>, Rad<S>> for Rad<S> { #[inline] fn add(&self, other: &Rad<S>) -> Rad<S> { rad(self.s + other.s) } }
impl<S: Float> Add<Deg<S>, Deg<S>> for Deg<S> { #[inline] fn add(&self, other: &Deg<S>) -> Deg<S> { deg(self.s + other.s) } }

impl<S: Float> Sub<Rad<S>, Rad<S>> for Rad<S> { #[inline] fn sub(&self, other: &Rad<S>) -> Rad<S> { rad(self.s - other.s) } }
impl<S: Float> Sub<Deg<S>, Deg<S>> for Deg<S> { #[inline] fn sub(&self, other: &Deg<S>) -> Deg<S> { deg(self.s - other.s) } }

impl<S: Float> Neg<Rad<S>> for Rad<S> { #[inline] fn neg(&self) -> Rad<S> { rad(-self.s) } }
impl<S: Float> Neg<Deg<S>> for Deg<S> { #[inline] fn neg(&self) -> Deg<S> { deg(-self.s) } }

impl<S: Float> Zero for Rad<S> { #[inline] fn zero() -> Rad<S> { rad(zero()) } #[inline] fn is_zero(&self) -> bool { *self == zero() } }
impl<S: Float> Zero for Deg<S> { #[inline] fn zero() -> Deg<S> { deg(zero()) } #[inline] fn is_zero(&self) -> bool { *self == zero() } }

impl<S: Float> Mul<Rad<S>, Rad<S>> for Rad<S> { #[inline] fn mul(&self, other: &Rad<S>) -> Rad<S> { rad(self.s * other.s) } }
impl<S: Float> Mul<Deg<S>, Deg<S>> for Deg<S> { #[inline] fn mul(&self, other: &Deg<S>) -> Deg<S> { deg(self.s * other.s) } }

impl<S: Float> One for Rad<S> { #[inline] fn one() -> Rad<S> { rad(one()) } }
impl<S: Float> One for Deg<S> { #[inline] fn one() -> Deg<S> { deg(one()) } }

impl<S: Float + ApproxEq<S>>
Equiv<Rad<S>> for Rad<S> {
    fn equiv(&self, other: &Rad<S>) -> bool {
        self.normalize() == other.normalize()
    }
}

impl<S: Float + ApproxEq<S>>
Equiv<Deg<S>> for Deg<S> {
    fn equiv(&self, other: &Deg<S>) -> bool {
        self.normalize() == other.normalize()
    }
}

impl<S: Float + ApproxEq<S>>
Angle<S> for Rad<S> {
    #[inline] fn from<A: Angle<S>>(theta: A) -> Rad<S> { theta.to_rad() }
    #[inline] fn full_turn() -> Rad<S> { rad(Float::two_pi()) }
}

impl<S: Float + ApproxEq<S>>
Angle<S> for Deg<S> {
    #[inline] fn from<A: Angle<S>>(theta: A) -> Deg<S> { theta.to_deg() }
    #[inline] fn full_turn() -> Deg<S> { deg(cast(360).unwrap()) }
}

#[inline] pub fn sin<S: Float>(theta: Rad<S>) -> S { theta.s.sin() }
#[inline] pub fn cos<S: Float>(theta: Rad<S>) -> S { theta.s.cos() }
#[inline] pub fn tan<S: Float>(theta: Rad<S>) -> S { theta.s.tan() }
#[inline] pub fn sin_cos<S: Float>(theta: Rad<S>) -> (S, S) { theta.s.sin_cos() }

#[inline] pub fn cot<S: Float>(theta: Rad<S>) -> S { tan(theta).recip() }
#[inline] pub fn sec<S: Float>(theta: Rad<S>) -> S { cos(theta).recip() }
#[inline] pub fn csc<S: Float>(theta: Rad<S>) -> S { sin(theta).recip() }

#[inline] pub fn asin<S: Float>(s: S) -> Rad<S> { rad(s.asin()) }
#[inline] pub fn acos<S: Float>(s: S) -> Rad<S> { rad(s.acos()) }
#[inline] pub fn atan<S: Float>(s: S) -> Rad<S> { rad(s.atan()) }
#[inline] pub fn atan2<S: Float>(a: S, b: S) -> Rad<S> { rad(a.atan2(&b)) }

impl<S: Float + fmt::Show> ToStr for Rad<S> { fn to_str(&self) -> ~str { format!("{} rad", self.s) } }
impl<S: Float + fmt::Show> ToStr for Deg<S> { fn to_str(&self) -> ~str { format!("{}°", self.s) } }

impl<S: Float + ApproxEq<S>>
ApproxEq<S> for Rad<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Rad<S>, epsilon: &S) -> bool {
        self.s.approx_eq_eps(&other.s, epsilon)
    }
}

impl<S: Float + ApproxEq<S>>
ApproxEq<S> for Deg<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Deg<S>, epsilon: &S) -> bool {
        self.s.approx_eq_eps(&other.s, epsilon)
    }
}
