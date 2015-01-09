// Copyright 2014 The CGMath Developers. For a full listing of the authors,
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

use std::num;
use std::num::Float;

pub trait ApproxEq<T: Float>: Sized {
    fn approx_epsilon(_hack: Option<Self>) -> T {
        num::cast(1.0e-5f64).unwrap()
    }

    fn approx_eq(&self, other: &Self) -> bool {
        let eps: T = ApproxEq::approx_epsilon(None::<Self>);
        self.approx_eq_eps(other, &eps)
    }

    fn approx_eq_eps(&self, other: &Self, epsilon: &T) -> bool;
}


macro_rules! approx_float(
    ($S:ident) => (
        impl ApproxEq<$S> for $S {
             #[inline]
            fn approx_eq_eps(&self, other: &$S, epsilon: &$S) -> bool {
                 (*self - *other).abs() < *epsilon
            }
        }
    )
);

approx_float!(f32);
approx_float!(f64);

#[macro_export]
macro_rules! assert_approx_eq_eps(
    ($given: expr, $expected: expr, $eps: expr) => ({
        let eps = &($eps);
        let (given_val, expected_val) = (&($given), &($expected));
        if !given_val.approx_eq_eps(expected_val, eps) {
            panic!("assertion failed: `left ≈ right` (left: `{:?}`, right: `{:?}`, tolerance: `{:?}`)",
                *given_val, *expected_val, *eps
            );
        }
    })
);

#[macro_export]
macro_rules! assert_approx_eq(
    ($given: expr, $expected: expr) => ({
        let (given_val, expected_val) = (&($given), &($expected));
        if !given_val.approx_eq(expected_val) {
            panic!("assertion failed: `left ≈ right` (left: `{:?}`, right: `{:?}`, tolerance: `{:?}`)",
                *given_val, *expected_val,
                ApproxEq::approx_epsilon(Some(*given_val))
            );
        }
    })
);
