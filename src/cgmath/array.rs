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

#![macro_escape]

use std::slice::{Items, MutItems};

/// An array containing elements of type `T` represented with `Repr`
///
/// This trait abstracts over [T, ..N], and is manually implemented for arrays
/// of length 2, 3, and 4.
pub trait Array<T: Clone, Repr> {
    /// Get a shared reference to the `i`th value.
    fn i<'a>(&'a self, i: uint) -> &'a T;

    /// Get a mutable reference to the `i`th value.
    fn mut_i<'a>(&'a mut self, i: uint) -> &'a mut T;

    /// Get a shared reference to this array's data as `Repr`.
    fn as_slice<'a>(&'a self) -> &'a Repr;

    /// Get a mutable reference to this array's data as `Repr`.
    fn as_mut_slice<'a>(&'a mut self) -> &'a mut Repr;

    /// Construct a new Array from its representation.
    fn from_repr(repr: Repr) -> Self;

    /// Create a new `Array` using a closure to generate the elements. The
    /// closure is passed the index of the element it should produce.
    fn build(builder: |i: uint| -> T) -> Self;

    /// Iterate over the elements of this `Array`, yielding shared references
    /// to items.
    fn iter<'a>(&'a self) -> Items<'a, T>;

    /// Iterate over the elements of this `Array`, yielding mutable references
    /// to items.
    fn mut_iter<'a>(&'a mut self) -> MutItems<'a, T>;

    #[inline]
    /// Swap the elements at indices `a` and `b` in-place.
    fn swap(&mut self, a: uint, b: uint) {
        let tmp = self.i(a).clone();
        *self.mut_i(a) = self.i(b).clone();
        *self.mut_i(b) = tmp;
    }

    /// Fold over this array, creating the same type as the type of the
    /// elements. Use `.iter().fold(...)` for a more flexible fold. The first
    /// element passed to the fold is the accumulator. It starts as the first
    /// value in the array.
    fn fold(&self, f: |&T, &T| -> T) -> T;

    /// Iterate over this array, yielding mutable references to items. The
    /// closure is passed the index of the element the reference is pointing
    /// at.
    fn each_mut(&mut self, f: |i: uint, x: &mut T|);
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
                unsafe { ::std::mem::transmute(self) }
            }

            #[inline]
            fn as_mut_slice<'a>(&'a mut self) -> &'a mut [$T,..$n] {
                unsafe { ::std::mem::transmute(self) }
            }

            #[inline]
            fn from_repr(slice: [$T,..$n]) -> $Self {
                unsafe { ::std::mem::transmute(slice) }
            }

            #[inline]
            fn build(builder: |i: uint| -> $T) -> $Self {
                Array::from_repr(gen_builder!($_n))
            }

            #[inline]
            fn iter<'a>(&'a self) -> ::std::slice::Items<'a, $T> {
                self.as_slice().iter()
            }

            #[inline]
            fn mut_iter<'a>(&'a mut self) -> ::std::slice::MutItems<'a, $T> {
                self.as_mut_slice().mut_iter()
            }

            #[inline]
            fn fold(&self, f: |&$T, &$T| -> $T) -> $T {
                gen_fold!($_n)
            }

            #[inline]
            fn each_mut(&mut self, f: |i: uint, x: &mut $T|) {
                gen_each_mut!($_n)
            }
        }
    )
)

#[inline]
pub fn build<T: Clone, Slice, A: Array<T, Slice>>(builder: |i: uint| -> T) -> A {
    Array::build(builder)
}

macro_rules! gen_builder(
    (_2) => ({ [builder(0), builder(1)] });
    (_3) => ({ [builder(0), builder(1), builder(2)] });
    (_4) => ({ [builder(0), builder(1), builder(2), builder(3)] });
)

macro_rules! gen_fold(
    (_2) => ({ f(self.i(0), self.i(1)) });
    (_3) => ({ let tmp = f(self.i(0), self.i(1)); f(&tmp, self.i(2)) });
    (_4) => ({ let tmp1 = f(self.i(0), self.i(1)); let tmp2 = f(&tmp1, self.i(2)); f(&tmp2, self.i(3)) });
)

macro_rules! gen_each_mut(
    (_2) => ({ f(0, self.mut_i(0)); f(1, self.mut_i(1)); });
    (_3) => ({ f(0, self.mut_i(0)); f(1, self.mut_i(1)); f(2, self.mut_i(2)); });
    (_4) => ({ f(0, self.mut_i(0)); f(1, self.mut_i(1)); f(2, self.mut_i(2)); f(3, self.mut_i(3)); });
)

