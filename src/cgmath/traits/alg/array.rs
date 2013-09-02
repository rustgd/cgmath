// Copyright 2013 The OMath Developers. For a full listing of the authors,
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

use std::vec::{VecIterator, VecMutIterator};

pub trait Array<T, Slice> {
    fn i<'a>(&'a self, i: uint) -> &'a T;
    fn mut_i<'a>(&'a mut self, i: uint) -> &'a mut T;
    fn as_slice<'a>(&'a self) -> &'a Slice;
    fn as_mut_slice<'a>(&'a mut self) -> &'a mut Slice;
    fn from_slice(slice: Slice) -> Self;
    fn build(builder: &fn(i: uint) -> T) -> Self;
    fn iter<'a>(&'a self) -> VecIterator<'a, T>;
    fn mut_iter<'a>(&'a mut self) -> VecMutIterator<'a, T>;

    #[inline]
    fn map<U, SliceU, UU: Array<U, SliceU>>(&self, f: &fn(&T) -> U) -> UU {
        Array::build(|i| f(self.i(i)))
    }

    #[inline]
    fn bimap<U, SliceU, UU: Array<U, SliceU>,
             V, SliceV, VV: Array<V, SliceV>>(&self, other: &UU, f: &fn(&T, &U) -> V) -> VV {
        Array::build(|i| f(self.i(i), other.i(i)))
    }
}

macro_rules! array(
    (impl<$S:ident> $Self:ty -> [$T:ty, ..$n:expr]) => (
        impl<$S> Array<$T, [$T,..$n]> for $Self {
            #[inline]
            fn i<'a>(&'a self, i: uint) -> &'a $T {
                &'a self.as_slice()[i]
            }

            #[inline]
            fn mut_i<'a>(&'a mut self, i: uint) -> &'a mut $T {
                &'a mut self.as_mut_slice()[i]
            }

            #[inline]
            fn as_slice<'a>(&'a self) -> &'a [$T,..$n] {
                unsafe { ::std::cast::transmute(self) }
            }

            #[inline]
            fn as_mut_slice<'a>(&'a mut self) -> &'a mut [$T,..$n] {
                unsafe { ::std::cast::transmute(self) }
            }

            #[inline]
            fn from_slice(slice: [$T,..$n]) -> $Self {
                unsafe { ::std::cast::transmute(slice) }
            }

            #[inline]
            fn build(builder: &fn(i: uint) -> $T) -> $Self {
                use std::unstable::intrinsics;
                let mut s: [$T,..$n] = unsafe { intrinsics::uninit() };
                for i in range(0u, $n) {
                    s[i] = builder(i);
                }
                Array::from_slice(s)
            }

            #[inline]
            fn iter<'a>(&'a self) -> ::std::vec::VecIterator<'a, $T> {
                self.as_slice().iter()
            }

            #[inline]
            fn mut_iter<'a>(&'a mut self) -> ::std::vec::VecMutIterator<'a, $T> {
                self.as_mut_slice().mut_iter()
            }
        }
    )
)

macro_rules! array_op(
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
    (impl<$S:ident> -$Self:ty -> $Result:ty) => (array_op!(impl<$S> (Neg, neg) for $Self -> $Result));
    (impl<$S:ident> $Self:ty + $Other:ty -> $Result:ty) => (array_op!(impl<$S> (Add, add) for ($Self, $Other) -> $Result));
    (impl<$S:ident> $Self:ty - $Other:ty -> $Result:ty) => (array_op!(impl<$S> (Sub, sub) for ($Self, $Other) -> $Result));
    (impl<$S:ident> $Self:ty * $Other:ty -> $Result:ty) => (array_op!(impl<$S> (Mul, mul) for ($Self, $Other) -> $Result));
    (impl<$S:ident> $Self:ty / $Other:ty -> $Result:ty) => (array_op!(impl<$S> (Div, div) for ($Self, $Other) -> $Result));
    (impl<$S:ident> $Self:ty % $Other:ty -> $Result:ty) => (array_op!(impl<$S> (Rem, rem) for ($Self, $Other) -> $Result));
)

/// An `Array` whose elements can be cloned
pub trait ClonableArray<T: Clone, Slice>: Array<T, Slice> {
    /// Swap two elements of the type in place.
    #[inline]
    fn swap(&mut self, a: uint, b: uint) {
        let tmp = self.i(a).clone();
        *self.mut_i(a) = self.i(b).clone();
        *self.mut_i(b) = tmp;
    }
}
