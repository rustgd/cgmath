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

#[macro_escape];

use traits::alg::Field;
use traits::util::Indexable;
use traits::util::Swappable;

pub trait Coordinate
<
    S: Field,
    Slice
>
:   Indexable<S, Slice>
+   Swappable<S, Slice>
{
}

macro_rules! impl_coordinate_binop(
    ($Self:ty, $Other:ty, $Result:ty, $Op:ident, $op:ident) => (
        impl<S: Field> $Op<$Other, $Result> for $Self {
            #[inline(always)]
            fn $op(&self, other: &$Other) -> $Result {
                self.bimap(other, |a, b| a.$op(b))
            }
        }
    )
)

macro_rules! impl_coordinate_op(
    ($Self:ty, $Result:ty, $Op:ident, $op:ident) => (
        impl<S: Field> $Op<$Result> for $Self {
            #[inline(always)]
            fn $op(&self) -> $Result {
                self.map(|a| a.$op())
            }
        }
    )
)
