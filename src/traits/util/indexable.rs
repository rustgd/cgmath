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

/// Types that can be accessed via an unsigned index.
pub trait Indexable<T, Slice> {
    fn len(&self) -> uint;
    fn i<'a>(&'a self, i: uint) -> &'a T;
    fn mut_i<'a>(&'a mut self, i: uint) -> &'a mut T;
    fn as_slice<'a>(&'a self) -> &'a Slice;
    fn as_mut_slice<'a>(&'a mut self) -> &'a mut Slice;
    fn from_slice(slice: Slice) -> Self;
    fn build(builder: &fn(i: uint) -> T) -> Self;

    #[inline]
    fn map<U, SliceU, UU: Indexable<U, SliceU>>(&self, f: &fn(&T) -> U) -> UU {
        Indexable::build(|i| f(self.i(i)))
    }

    #[inline]
    fn map_mut(&mut self, f: &fn(&mut T)) {
        for i in range(0, self.len()) {
            f(self.mut_i(i));
        }
    }

    #[inline]
    fn bimap<U, SliceU, UU: Indexable<U, SliceU>,
             V, SliceV, VV: Indexable<V, SliceV>>(&self, other: &UU, f: &fn(&T, &U) -> V) -> VV {
        Indexable::build(|i| f(self.i(i), other.i(i)))
    }

    #[inline]
    fn bimap_mut<U, SliceU, UU: Indexable<U, Slice>>(&mut self, other: &UU, f: &fn(&mut T, &U)) {
        for i in range(0, self.len()) {
            f(self.mut_i(i), other.i(i));
        }
    }

    #[inline]
    fn fold<U>(&self, init: U, f: &fn(acc: &U, x: &T) -> U) -> U {
        let mut acc = init;
        for i in range(0, self.len()) {
            acc = f(&acc, self.i(i));
        }
        acc
    }
}

macro_rules! impl_indexable(
    ($Self:ty, [$T:ident, ..$n:expr]) => (
        impl<$T> Indexable<$T, [$T,..$n]> for $Self {
            #[inline]
            fn len(&self) -> uint { $n }

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
                for i in range::<uint>(0, $n) {
                    s[i] = builder(i);
                }
                Indexable::from_slice(s)
            }
        }
    )
)
