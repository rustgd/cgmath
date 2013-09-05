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

use std::num::Zero;

#[deriving(Clone, Eq, Ord, Zero)] pub struct Rad<S> { s: S }
#[deriving(Clone, Eq, Ord, Zero)] pub struct Deg<S> { s: S }

#[inline] pub fn rad<S: Clone + Float>(s: S) -> Rad<S> { Rad { s: s } }
#[inline] pub fn deg<S: Clone + Float>(s: S) -> Deg<S> { Deg { s: s } }

pub trait ToRad<S: Clone + Float> { fn to_rad(&self) -> Rad<S>; }
pub trait ToDeg<S: Clone + Float> { fn to_deg(&self) -> Deg<S>; }

impl<S: Clone + Float> ToRad<S> for Rad<S> { #[inline] fn to_rad(&self) -> Rad<S> { self.clone() } }
impl<S: Clone + Float> ToRad<S> for Deg<S> { #[inline] fn to_rad(&self) -> Rad<S> { rad(self.s.to_radians()) } }

impl<S: Clone + Float> ToDeg<S> for Rad<S> { #[inline] fn to_deg(&self) -> Deg<S> { deg(self.s.to_degrees()) } }
impl<S: Clone + Float> ToDeg<S> for Deg<S> { #[inline] fn to_deg(&self) -> Deg<S> { self.clone() } }

impl<S: Clone + Float> Neg<Rad<S>> for Rad<S> { #[inline] fn neg(&self) -> Rad<S> { rad(-self.s) } }
impl<S: Clone + Float> Neg<Deg<S>> for Deg<S> { #[inline] fn neg(&self) -> Deg<S> { deg(-self.s) } }

/// Private utility functions for converting to/from scalars
trait ScalarConv<S> {
    fn from(s: S) -> Self;
    fn s<'a>(&'a self) -> &'a S;
    fn mut_s<'a>(&'a mut self) -> &'a mut S;
}

impl<S: Clone + Float> ScalarConv<S> for Rad<S> {
    #[inline] fn from(s: S) -> Rad<S> { rad(s) }
    #[inline] fn s<'a>(&'a self) -> &'a S { &'a self.s }
    #[inline] fn mut_s<'a>(&'a mut self) -> &'a mut S { &'a mut self.s }
}

impl<S: Clone + Float> ScalarConv<S> for Deg<S> {
    #[inline] fn from(s: S) -> Deg<S> { deg(s) }
    #[inline] fn s<'a>(&'a self) -> &'a S { &'a self.s }
    #[inline] fn mut_s<'a>(&'a mut self) -> &'a mut S { &'a mut self.s }
}

pub trait Angle
<
    S: Clone + Float
>
:   Clone + Zero
+   Eq + Ord
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
}

impl<S: Clone + Float> Angle<S> for Rad<S> {
    #[inline] fn from<A: Angle<S>>(theta: A) -> Rad<S> { theta.to_rad() }
}

impl<S: Clone + Float> Angle<S> for Deg<S> {
    #[inline] fn from<A: Angle<S>>(theta: A) -> Deg<S> { theta.to_deg() }
}

#[inline] pub fn sin<S: Clone + Float, A: Angle<S>>(theta: A) -> S { theta.to_rad().s.sin() }
#[inline] pub fn cos<S: Clone + Float, A: Angle<S>>(theta: A) -> S { theta.to_rad().s.cos() }
#[inline] pub fn tan<S: Clone + Float, A: Angle<S>>(theta: A) -> S { theta.to_rad().s.tan() }
#[inline] pub fn sin_cos<S: Clone + Float, A: Angle<S>>(theta: A) -> (S, S) { theta.to_rad().s.sin_cos() }

#[inline] pub fn cot<S: Clone + Float, A: Angle<S>>(theta: A) -> S { tan(theta).recip() }
#[inline] pub fn sec<S: Clone + Float, A: Angle<S>>(theta: A) -> S { cos(theta).recip() }
#[inline] pub fn csc<S: Clone + Float, A: Angle<S>>(theta: A) -> S { sin(theta).recip() }

#[inline] pub fn asin<S: Clone + Float, A: Angle<S>>(s: S) -> A { Angle::from(rad(s.asin())) }
#[inline] pub fn acos<S: Clone + Float, A: Angle<S>>(s: S) -> A { Angle::from(rad(s.acos())) }
#[inline] pub fn atan<S: Clone + Float, A: Angle<S>>(s: S) -> A { Angle::from(rad(s.atan())) }
#[inline] pub fn atan2<S: Clone + Float, A: Angle<S>>(a: S, b: S) -> A { Angle::from(rad(a.atan2(&b))) }

impl<S: Clone + Float> ToStr for Rad<S> { fn to_str(&self) -> ~str { fmt!("%? rad", self.s) } }
impl<S: Clone + Float> ToStr for Deg<S> { fn to_str(&self) -> ~str { fmt!("%?°", self.s) } }

impl<S: Clone + Float> ApproxEq<S> for Rad<S> {
    #[inline]
    fn approx_epsilon() -> S {
        // TODO: fix this after static methods are fixed in rustc
        fail!(~"Doesn't work!");
    }

    #[inline]
    fn approx_eq(&self, other: &Rad<S>) -> bool {
        self.s.approx_eq(&other.s)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Rad<S>, approx_epsilon: &S) -> bool {
        self.s.approx_eq_eps(&other.s, approx_epsilon)
    }
}

impl<S: Clone + Float> ApproxEq<S> for Deg<S> {
    #[inline]
    fn approx_epsilon() -> S {
        // TODO: fix this after static methods are fixed in rustc
        fail!(~"Doesn't work!");
    }

    #[inline]
    fn approx_eq(&self, other: &Deg<S>) -> bool {
        self.s.approx_eq(&other.s)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Deg<S>, approx_epsilon: &S) -> bool {
        self.s.approx_eq_eps(&other.s, approx_epsilon)
    }
}
