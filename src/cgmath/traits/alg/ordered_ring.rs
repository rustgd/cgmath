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

use traits::alg::Ring;

/// A ring that can also be ordered.
pub trait OrderedRing
<
    S
>
:   Ring<S>
+   Orderable
{
}

// impls for concrete types

impl OrderedRing<u8> for u8;
impl OrderedRing<u16> for u16;
impl OrderedRing<u32> for u32;
impl OrderedRing<u64> for u64;
impl OrderedRing<uint> for uint;
impl OrderedRing<i8> for i8;
impl OrderedRing<i16> for i16;
impl OrderedRing<i32> for i32;
impl OrderedRing<i64> for i64;
impl OrderedRing<int> for int;
impl OrderedRing<f32> for f32;
impl OrderedRing<f64> for f64;
impl OrderedRing<float> for float;
