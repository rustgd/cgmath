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

use approx::ApproxEq;

use std::cmp;
use std::fmt;

/// A trait providing a [partial ordering](http://mathworld.wolfram.com/PartialOrder.html).
pub trait PartialOrd {
    fn partial_min(self, other: Self) -> Self;
    fn partial_max(self, other: Self) -> Self;
}

macro_rules! partial_ord_int (
    ($T:ident) => (
        impl PartialOrd for $T {
            fn partial_min(self, other: $T) -> $T { cmp::min(self, other) }
            fn partial_max(self, other: $T) -> $T { cmp::max(self, other) }
        }
    )
)

partial_ord_int!(int)
partial_ord_int!(i8)
partial_ord_int!(i16)
partial_ord_int!(i32)
partial_ord_int!(i64)
partial_ord_int!(uint)
partial_ord_int!(u8)
partial_ord_int!(u16)
partial_ord_int!(u32)
partial_ord_int!(u64)

macro_rules! partial_ord_float (
    ($T:ident) => (
        impl PartialOrd for $T {
            fn partial_min(self, other: $T) -> $T { self.min(other) }
            fn partial_max(self, other: $T) -> $T { self.max(other) }
        }
    )
)

partial_ord_float!(f32)
partial_ord_float!(f64)

/// Base numeric types with partial ordering
pub trait BaseNum: Primitive + PartialOrd + fmt::Show {}

impl BaseNum for i8 {}
impl BaseNum for i16 {}
impl BaseNum for i32 {}
impl BaseNum for i64 {}
impl BaseNum for int {}
impl BaseNum for u8 {}
impl BaseNum for u16 {}
impl BaseNum for u32 {}
impl BaseNum for u64 {}
impl BaseNum for uint {}
impl BaseNum for f32 {}
impl BaseNum for f64 {}

/// Base integer types
pub trait BaseInt : BaseNum + Int {}

impl BaseInt for i8 {}
impl BaseInt for i16 {}
impl BaseInt for i32 {}
impl BaseInt for i64 {}
impl BaseInt for int {}
impl BaseInt for u8 {}
impl BaseInt for u16 {}
impl BaseInt for u32 {}
impl BaseInt for u64 {}
impl BaseInt for uint {}

/// Base floating point types
pub trait BaseFloat : BaseNum + FloatMath + ApproxEq<Self> + fmt::Float {}

impl BaseFloat for f32 {}
impl BaseFloat for f64 {}
