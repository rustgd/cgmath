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

use array::Array;
use matrix::{Mat2, Mat3, Mat4};
use point::{Point2, Point3};
use quaternion::Quat;
use vector::{Vec2, Vec3, Vec4};

pub trait ApproxEq<T: Float> {
    fn approx_epsilon(_hack: Option<Self>) -> T {
        num::cast(1.0e-5).unwrap()
    }

    fn approx_eq(&self, other: &Self) -> bool {
        let eps: T = ApproxEq::approx_epsilon(None::<Self>);
        self.approx_eq_eps(other, &eps)
    }

    fn approx_eq_eps(&self, other: &Self, epsilon: &T) -> bool;
}


macro_rules! approx_simple(
    ($S:ident) => (
        impl ApproxEq<$S> for $S {
             #[inline]
            fn approx_eq_eps(&self, other: &$S, epsilon: &$S) -> bool {
                 num::abs(*self - *other) < *epsilon
            }
        }
    )
)

approx_simple!(f32)
approx_simple!(f64)


macro_rules! approx_array(
    (impl<$S:ident> $Self:ty) => (
        impl<$S: Float + Clone + ApproxEq<$S>> ApproxEq<$S> for $Self {
            #[inline]
            fn approx_eq_eps(&self, other: &$Self, epsilon: &$S) -> bool {
                self.iter().zip(other.iter())
                           .all(|(a, b)| a.approx_eq_eps(b, epsilon))
            }
        }
    )
)

approx_array!(impl<S> Mat2<S>)
approx_array!(impl<S> Mat3<S>)
approx_array!(impl<S> Mat4<S>)

approx_array!(impl<S> Quat<S>)

approx_array!(impl<S> Vec2<S>)
approx_array!(impl<S> Vec3<S>)
approx_array!(impl<S> Vec4<S>)

approx_array!(impl<S> Point2<S>)
approx_array!(impl<S> Point3<S>)

