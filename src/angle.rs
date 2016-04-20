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

use structure::Angle;

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
            fn add_assign(&mut self, other) { self.s += other.s; }
        });
        impl_assignment_operator!(<S: BaseFloat> SubAssign<$Angle<S> > for $Angle<S> {
            fn sub_assign(&mut self, other) { self.s -= other.s; }
        });
        impl_assignment_operator!(<S: BaseFloat> RemAssign<$Angle<S> > for $Angle<S> {
            fn rem_assign(&mut self, other) { self.s %= other.s; }
        });

        impl_operator!(<S: BaseFloat> Mul<S> for $Angle<S> {
            fn mul(lhs, scalar) -> $Angle<S> { $Angle::new(lhs.s * scalar) }
        });
        impl_operator!(<S: BaseFloat> Div<S> for $Angle<S> {
            fn div(lhs, scalar) -> $Angle<S> { $Angle::new(lhs.s / scalar) }
        });
        impl_assignment_operator!(<S: BaseFloat> MulAssign<S> for $Angle<S> {
            fn mul_assign(&mut self, scalar) { self.s *= scalar; }
        });
        impl_assignment_operator!(<S: BaseFloat> DivAssign<S> for $Angle<S> {
            fn div_assign(&mut self, scalar) { self.s /= scalar; }
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
