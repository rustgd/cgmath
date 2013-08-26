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

/// A commutative ring that contains a multiplicative inverse.
pub trait Field
:   Ring
+   Div<Self,Self>
+   Rem<Self,Self>
{
}

// impls for concrete types

impl Field for u8;
impl Field for u16;
impl Field for u32;
impl Field for u64;
impl Field for uint;
impl Field for i8;
impl Field for i16;
impl Field for i32;
impl Field for i64;
impl Field for int;
impl Field for f32;
impl Field for f64;
impl Field for float;
