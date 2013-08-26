// Copyright 2013 The Lmath Developers. For a full listing of the authors,
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

use traits::alg::ScalarMul;

/// An algebraic structure that generalizes the notion of a vector space.
pub trait Module<S>: Eq
                   + Add<Self,Self>
                   + ScalarMul<S>
                   + Zero {}

// impls for concrete types

impl Module<u8> for u8;
impl Module<u16> for u16;
impl Module<u32> for u32;
impl Module<u64> for u64;
impl Module<uint> for uint;
impl Module<i8> for i8;
impl Module<i16> for i16;
impl Module<i32> for i32;
impl Module<i64> for i64;
impl Module<int> for int;
impl Module<f32> for f32;
impl Module<f64> for f64;
impl Module<float> for float;
