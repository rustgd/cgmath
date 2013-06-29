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

#[macro_escape];

macro_rules! impl_dimensional(
    ($Self:ident, $T:ty, $n:expr) => (
        impl<T> Dimensional<$T,[$T,..$n]> for $Self<T> {
            #[inline]
            pub fn index<'a>(&'a self, i: uint) -> &'a $T {
                &'a self.as_slice()[i]
            }

            #[inline]
            pub fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut $T {
                &'a mut self.as_mut_slice()[i]
            }

            #[inline]
            pub fn as_slice<'a>(&'a self) -> &'a [$T,..$n] {
                use std::cast::transmute;
                unsafe { transmute(self) }
            }

            #[inline]
            pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [$T,..$n] {
                use std::cast::transmute;
                unsafe { transmute(self) }
            }
        }
    )
)

macro_rules! impl_dimensional_fns(
    ($Self:ident, $T:ty, 2) => (
        impl<T> $Self<T> {
            #[inline]
            pub fn from_slice<'a>(slice: [$T,..2]) -> $Self<T> {
                use std::cast::transmute;
                unsafe { transmute(slice) }
            }

            #[inline(always)]
            pub fn map<U>(&self, f: &fn(&$T) -> U) -> [U,..2] {
                [f(self.index(0)),
                 f(self.index(1))]
            }

            #[inline(always)]
            pub fn map_mut(&mut self, f: &fn(&mut $T)) {
                f(self.index_mut(0));
                f(self.index_mut(1));
            }

            #[inline(always)]
            pub fn zip<U, SU: Dimensional<U,[U,..2]>, V>(&self, other: &SU, f: &fn(&$T, &U) -> V) -> [V,..2] {
                [f(self.index(0), other.index(0)),
                 f(self.index(1), other.index(1))]
            }

            #[inline(always)]
            pub fn zip_mut<U, SU: Dimensional<U,[U,..2]>>(&mut self, other: &SU, f: &fn(&mut $T, &U)) {
                f(self.index_mut(0), other.index(0));
                f(self.index_mut(1), other.index(1));
            }

            #[inline(always)]
            pub fn foldl<U>(&self, init: &U, f: &fn(&$T, &U) -> U) -> U {
                f(self.index(0), &f(self.index(1), init))
            }

            #[inline(always)]
            pub fn foldr<U>(&self, init: &U, f: &fn(&$T, &U) -> U) -> U {
                f(self.index(1), &f(self.index(0), init))
            }
        }
    );
    ($Self:ident, $T:ty, 3) => (
        impl<T> $Self<T> {
            #[inline]
            pub fn from_slice<'a>(slice: [$T,..3]) -> $Self<T> {
                use std::cast::transmute;
                unsafe { transmute(slice) }
            }

            #[inline(always)]
            pub fn map<U>(&self, f: &fn(&$T) -> U) -> [U,..3] {
                [f(self.index(0)),
                 f(self.index(1)),
                 f(self.index(2))]
            }

            #[inline(always)]
            pub fn map_mut(&mut self, f: &fn(&mut $T)) {
                f(self.index_mut(0));
                f(self.index_mut(1));
                f(self.index_mut(2));
            }

            #[inline(always)]
            pub fn zip<U, SU: Dimensional<U,[U,..3]>, V>(&self, other: &SU, f: &fn(&$T, &U) -> V) -> [V,..3] {
                [f(self.index(0), other.index(0)),
                 f(self.index(1), other.index(1)),
                 f(self.index(2), other.index(2))]
            }

            #[inline(always)]
            pub fn zip_mut<U, SU: Dimensional<U,[U,..3]>>(&mut self, other: &SU, f: &fn(&mut $T, &U)) {
                f(self.index_mut(0), other.index(0));
                f(self.index_mut(1), other.index(1));
                f(self.index_mut(2), other.index(2));
            }

            #[inline(always)]
            pub fn foldl<U>(&self, init: &U, f: &fn(&$T, &U) -> U) -> U {
                f(self.index(0), &f(self.index(1), &f(self.index(2), init)))
            }

            #[inline(always)]
            pub fn foldr<U>(&self, init: &U, f: &fn(&$T, &U) -> U) -> U {
                f(self.index(2), &f(self.index(1), &f(self.index(0), init)))
            }
        }
    );
    ($Self:ident, $T:ty, 4) => (
        impl<T> $Self<T> {
            #[inline]
            pub fn from_slice<'a>(slice: [$T,..4]) -> $Self<T> {
                use std::cast::transmute;
                unsafe { transmute(slice) }
            }

            #[inline(always)]
            pub fn map<U>(&self, f: &fn(&$T) -> U) -> [U,..4] {
                [f(self.index(0)),
                 f(self.index(1)),
                 f(self.index(2)),
                 f(self.index(3))]
            }

            #[inline(always)]
            pub fn map_mut(&mut self, f: &fn(&mut $T)) {
                f(self.index_mut(0));
                f(self.index_mut(1));
                f(self.index_mut(2));
                f(self.index_mut(3));
            }

            #[inline(always)]
            pub fn zip<U, SU: Dimensional<U,[U,..4]>, V>(&self, other: &SU, f: &fn(&$T, &U) -> V) -> [V,..4] {
                [f(self.index(0), other.index(0)),
                 f(self.index(1), other.index(1)),
                 f(self.index(2), other.index(2)),
                 f(self.index(3), other.index(3))]
            }

            #[inline(always)]
            pub fn zip_mut<U, SU: Dimensional<U,[U,..4]>>(&mut self, other: &SU, f: &fn(&mut $T, &U)) {
                f(self.index_mut(0), other.index(0));
                f(self.index_mut(1), other.index(1));
                f(self.index_mut(2), other.index(2));
                f(self.index_mut(3), other.index(3));
            }

            #[inline(always)]
            pub fn foldl<U>(&self, init: &U, f: &fn(&$T, &U) -> U) -> U {
                f(self.index(0), &f(self.index(1), &f(self.index(2), &f(self.index(3), init))))
            }

            #[inline(always)]
            pub fn foldr<U>(&self, init: &U, f: &fn(&$T, &U) -> U) -> U {
                f(self.index(3), &f(self.index(2), &f(self.index(1), &f(self.index(0), init))))
            }
        }
    )
)

macro_rules! impl_swap(
    ($Self:ident) => (
        impl<T:Copy> $Self<T> {
            #[inline]
            pub fn swap(&mut self, a: uint, b: uint) {
                let tmp = copy *self.index(a);
                *self.index_mut(a) = copy *self.index(b);
                *self.index_mut(b) = tmp;
            }
        }
    )
)

macro_rules! impl_approx(
    ($Self:ident) => (
        impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for $Self<T> {
            #[inline]
            pub fn approx_epsilon() -> T {
                ApproxEq::approx_epsilon::<T,T>()
            }

            #[inline]
            pub fn approx_eq(&self, other: &$Self<T>) -> bool {
                self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
            }

            #[inline]
            pub fn approx_eq_eps(&self, other: &$Self<T>, epsilon: &T) -> bool {
                
                self.zip(other, |a, b| a.approx_eq_eps(b, epsilon)).all(|&x| x)
            }
        }
    )
)
