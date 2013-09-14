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

pub trait Array
<
    T: Clone,
    Slice
>
{
    fn i<'a>(&'a self, i: uint) -> &'a T;
    fn mut_i<'a>(&'a mut self, i: uint) -> &'a mut T;
    fn as_slice<'a>(&'a self) -> &'a Slice;
    fn as_mut_slice<'a>(&'a mut self) -> &'a mut Slice;
    fn from_slice(slice: Slice) -> Self;
    fn build(builder: &fn(i: uint) -> T) -> Self;
    fn iter<'a>(&'a self) -> VecIterator<'a, T>;
    fn mut_iter<'a>(&'a mut self) -> VecMutIterator<'a, T>;

    /// Swap two elements of the type in place.
    #[inline]
    fn swap(&mut self, a: uint, b: uint) {
        let tmp = self.i(a).clone();
        *self.mut_i(a) = self.i(b).clone();
        *self.mut_i(b) = tmp;
    }

    fn fold(&self, f: &fn(&T, &T) -> T) -> T;
}

macro_rules! array(
    (impl<$S:ident> $Self:ty -> [$T:ty, ..$n:expr] $_n:ident) => (
        impl<$S: Clone> Array<$T, [$T,..$n]> for $Self {
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
                Array::from_slice(gen_builder!($_n))
            }

            #[inline]
            fn iter<'a>(&'a self) -> ::std::vec::VecIterator<'a, $T> {
                self.as_slice().iter()
            }

            #[inline]
            fn mut_iter<'a>(&'a mut self) -> ::std::vec::VecMutIterator<'a, $T> {
                self.as_mut_slice().mut_iter()
            }

            #[inline]
            fn fold(&self, f: &fn(&$T, &$T) -> $T) -> $T {
                gen_fold!($_n)
            }
        }
    )
)

#[inline]
pub fn build<T: Clone, Slice, A: Array<T, Slice>>(builder: &fn(i: uint) -> T) -> A {
    Array::build(builder)
}

macro_rules! gen_builder(
    (_2) => ([builder(0), builder(1)]);
    (_3) => ([builder(0), builder(1), builder(2)]);
    (_4) => ([builder(0), builder(1), builder(2), builder(3)]);
)

macro_rules! gen_fold(
    (_2) => (f(self.i(0), self.i(1)));
    (_3) => (f(&f(self.i(0), self.i(1)), self.i(2)));
    (_4) => (f(&f(&f(self.i(0), self.i(1)), self.i(2)), self.i(3)));
)

macro_rules! approx_eq(
    (impl<$S:ident> $Self:ty) => (
        impl<$S: Clone + ApproxEq<$S>> ApproxEq<$S> for $Self {
            #[inline]
            fn approx_epsilon() -> $S {
                // TODO: fix this after static methods are fixed in rustc
                fail!(~"Doesn't work!");
            }

            #[inline]
            fn approx_eq(&self, other: &$Self) -> bool {
                self.iter().zip(other.iter())
                           .all(|(a, b)| a.approx_eq(b))
            }

            #[inline]
            fn approx_eq_eps(&self, other: &$Self, approx_epsilon: &$S) -> bool {
                self.iter().zip(other.iter())
                           .all(|(a, b)| a.approx_eq_eps(b, approx_epsilon))
            }
        }
    )
)
