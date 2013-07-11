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

macro_rules! impl_swap(
    ($Self:ident) => (
        impl<T:Clone> $Self<T> {
            #[inline]
            pub fn swap(&mut self, a: uint, b: uint) {
                let tmp = self.index(a).clone();
                *self.index_mut(a) = self.index(b).clone();
                *self.index_mut(b) = tmp;
            }
        }
    )
)
