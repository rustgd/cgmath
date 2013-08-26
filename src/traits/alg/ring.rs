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

use std::num::One;

use traits::alg::Module;

/// A module that also requires the additive inverse operation (subtraction)
/// and the additive inverse.
pub trait Ring: Module<Self>
              + Neg<Self>
              + Sub<Self,Self>
              + One {}

// impls for concrete types

impl Ring for u8;
impl Ring for u16;
impl Ring for u32;
impl Ring for u64;
impl Ring for uint;
impl Ring for i8;
impl Ring for i16;
impl Ring for i32;
impl Ring for i64;
impl Ring for int;
impl Ring for f32;
impl Ring for f64;
impl Ring for float;
