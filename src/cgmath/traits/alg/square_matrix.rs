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

use traits::alg::Field;
use traits::alg::Matrix;
use traits::alg::VectorSpace;

pub trait SquareMatrix
<
    S: Field,
    V: VectorSpace<S>,
    VVSlice, VSlice
>
:   Matrix<S, V, VVSlice, VSlice, V, VVSlice, VSlice, Self>
{
    fn transpose_self(&mut self);
    fn trace(&self) -> S;
    fn det(&self) -> S;
    fn invert(&self) -> Option<Self>;
    fn invert_self(&mut self) -> Self;
    fn is_invertable(&self) -> bool;
}
