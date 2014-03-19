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

use approx::ApproxEq;
use std::cmp;

pub trait PartOrdPrim : Primitive {
    fn min(&self, b: Self) -> Self;
    fn max(&self, b: Self) -> Self;
}

macro_rules! gen_minmax_for_floats (
    ( $($T:ident),+ ) => (
        $(
            impl PartOrdPrim for $T {
                fn min(&self, b: $T) -> $T { (*self).min(b) }
                fn max(&self, b: $T) -> $T { (*self).max(b) }
            }
        )+
    )
)

macro_rules! gen_minmax_for_not_floats (
    ( $($T:ident),+ ) => (
        $(
            impl PartOrdPrim for $T {
                fn min(&self, b: $T) -> $T { cmp::min((*self), b) }
                fn max(&self, b: $T) -> $T { cmp::max((*self), b) }
            }
        )+
    )
)

gen_minmax_for_floats!(f32, f64)
gen_minmax_for_not_floats!(int, i8, i16, i32, i64, uint, u8, u16, u32, u64)

pub trait PartOrdFloat<S> : Float + ApproxEq<S> + PartOrdPrim {}
impl PartOrdFloat<f32> for f32 {}
impl PartOrdFloat<f64> for f64 {}

