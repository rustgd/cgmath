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

use std::num::Zero;

#[deriving(Clone, Eq, Ord, Zero)] struct Rad<S> { s: S }
#[deriving(Clone, Eq, Ord, Zero)] struct Deg<S> { s: S }

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
{
    fn from_s(s: S) -> Self;
    fn from<A: Angle<S>>(theta: A) -> Self;
    fn s<'a>(&'a self) -> &'a S;
    fn mut_s<'a>(&'a mut self) -> &'a mut S;

    #[inline] fn neg_self(&mut self) { *self = -*self }

    #[inline] fn add_a(&self, other: Self) -> Self { Angle::from_s(*self.s() + *other.s()) }
    #[inline] fn sub_a(&self, other: Self) -> Self { Angle::from_s(*self.s() - *other.s()) }
    #[inline] fn div_a(&self, other: Self) -> S { *self.s() / *other.s() }
    #[inline] fn rem_a(&self, other: Self) -> S { *self.s() % *other.s() }
    #[inline] fn mul_s(&self, s: S) -> Self { Angle::from_s(*self.s() * s) }
    #[inline] fn div_s(&self, s: S) -> Self { Angle::from_s(*self.s() / s) }
    #[inline] fn rem_s(&self, s: S) -> Self { Angle::from_s(*self.s() % s) }

    #[inline] fn add_self_a(&mut self, other: Self) { *self.mut_s() = *self.s() + *other.s() }
    #[inline] fn sub_self_a(&mut self, other: Self) { *self.mut_s() = *self.s() - *other.s() }
    #[inline] fn mul_self_s(&mut self, s: S) { *self.mut_s() = *self.s() * s }
    #[inline] fn div_self_s(&mut self, s: S) { *self.mut_s() = *self.s() / s }
    #[inline] fn rem_self_s(&mut self, s: S) { *self.mut_s() = *self.s() % s }

    #[inline] fn sin(&self) -> S { self.s().sin() }
    #[inline] fn cos(&self) -> S { self.s().cos() }
    #[inline] fn tan(&self) -> S { self.s().tan() }
}

#[inline] fn sin<S: Clone + Float, A: Angle<S>>(theta: A) -> S { theta.sin() }
#[inline] fn cos<S: Clone + Float, A: Angle<S>>(theta: A) -> S { theta.cos() }
#[inline] fn tan<S: Clone + Float, A: Angle<S>>(theta: A) -> S { theta.tan() }

impl<S: Clone + Float> Angle<S> for Rad<S> {
    #[inline] fn from_s(s: S) -> Rad<S> { rad(s) }
    #[inline] fn from<A: Angle<S>>(theta: A) -> Rad<S> { theta.to_rad() }
    #[inline] fn s<'a>(&'a self) -> &'a S { &'a self.s }
    #[inline] fn mut_s<'a>(&'a mut self) -> &'a mut S { &'a mut self.s }
}

impl<S: Clone + Float> Angle<S> for Deg<S> {
    #[inline] fn from_s(s: S) -> Deg<S> { deg(s) }
    #[inline] fn from<A: Angle<S>>(theta: A) -> Deg<S> { theta.to_deg() }
    #[inline] fn s<'a>(&'a self) -> &'a S { &'a self.s }
    #[inline] fn mut_s<'a>(&'a mut self) -> &'a mut S { &'a mut self.s }
}

pub trait ScalarTrig: Clone + Float {
    #[inline] fn asin_<A: Angle<Self>>(&self) -> A { Angle::from(rad(self.asin())) }
    #[inline] fn acos_<A: Angle<Self>>(&self) -> A { Angle::from(rad(self.acos())) }
    #[inline] fn atan_<A: Angle<Self>>(&self) -> A { Angle::from(rad(self.atan())) }
    #[inline] fn atan2_<A: Angle<Self>>(&self, other: &Self) -> A { Angle::from(rad(self.atan2(other))) }
}

#[inline] fn asin<S: Clone + ScalarTrig, A: Angle<S>>(s: S) -> A { s.asin_() }
#[inline] fn acos<S: Clone + ScalarTrig, A: Angle<S>>(s: S) -> A { s.acos_() }
#[inline] fn atan<S: Clone + ScalarTrig, A: Angle<S>>(s: S) -> A { s.atan_() }
#[inline] fn atan2<S: Clone + ScalarTrig, A: Angle<S>>(a: S, b: S) -> A { a.atan2_(&b) }

impl ScalarTrig for f32;
impl ScalarTrig for f64;
impl ScalarTrig for float;

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
