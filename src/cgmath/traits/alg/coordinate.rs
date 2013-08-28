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

macro_rules! coordinate_op(
    (impl<$S:ident> ($Op:ident, $op:ident) for ($Self:ty, $Other:ty) -> $Result:ty) => (
        impl<$S: Field> $Op<$Other, $Result> for $Self {
            #[inline(always)]
            fn $op(&self, other: &$Other) -> $Result {
                self.bimap(other, |a, b| a.$op(b))
            }
        }
    );
    (impl<$S:ident> ($Op:ident, $op:ident) for $Self:ty -> $Result:ty) => (
        impl<$S: Field> $Op<$Result> for $Self {
            #[inline(always)]
            fn $op(&self) -> $Result {
                self.map(|a| a.$op())
            }
        }
    );
    (impl<$S:ident> -$Self:ty -> $Result:ty) => (coordinate_op!(impl<$S> (Neg, neg) for $Self -> $Result));
    (impl<$S:ident> $Self:ty + $Other:ty -> $Result:ty) => (coordinate_op!(impl<$S> (Add, add) for ($Self, $Other) -> $Result));
    (impl<$S:ident> $Self:ty - $Other:ty -> $Result:ty) => (coordinate_op!(impl<$S> (Sub, sub) for ($Self, $Other) -> $Result));
    (impl<$S:ident> $Self:ty * $Other:ty -> $Result:ty) => (coordinate_op!(impl<$S> (Mul, mul) for ($Self, $Other) -> $Result));
    (impl<$S:ident> $Self:ty / $Other:ty -> $Result:ty) => (coordinate_op!(impl<$S> (Div, div) for ($Self, $Other) -> $Result));
    (impl<$S:ident> $Self:ty % $Other:ty -> $Result:ty) => (coordinate_op!(impl<$S> (Rem, rem) for ($Self, $Other) -> $Result));
)
