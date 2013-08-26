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

// use traits::alg::Field;

/// Enforces the multiplication of an type by a scalar.
pub trait ScalarMul
<
    S/*: Field*/
>
:   Mul<S, Self>
+   Div<S, Self>
+   Rem<S, Self>
{
}

// impls for concrete types

impl ScalarMul<u8> for u8;
impl ScalarMul<u16> for u16;
impl ScalarMul<u32> for u32;
impl ScalarMul<u64> for u64;
impl ScalarMul<uint> for uint;
impl ScalarMul<i8> for i8;
impl ScalarMul<i16> for i16;
impl ScalarMul<i32> for i32;
impl ScalarMul<i64> for i64;
impl ScalarMul<int> for int;
impl ScalarMul<f32> for f32;
impl ScalarMul<f64> for f64;
impl ScalarMul<float> for float;
