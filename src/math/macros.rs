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

macro_rules! impl_dimensioned(
    ($Self:ident, $T:ty, $n:expr) => (
        impl<T> Dimensioned<$T,[$T,..$n]> for $Self<T> {
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

macro_rules! impl_to_vec(
    ($Self:ident, 2) => (impl_to_vec_helper!(ToVec2, $Self, Vec2, to_vec2, as_vec2));
    ($Self:ident, 3) => (impl_to_vec_helper!(ToVec3, $Self, Vec3, to_vec3, as_vec3));
    ($Self:ident, 4) => (impl_to_vec_helper!(ToVec4, $Self, Vec4, to_vec4, as_vec4));
)

macro_rules! impl_to_vec_helper(
    ($ToVec:ident, $Self:ident, $Vec:ident, $to_vec:ident, $as_vec:ident) => (
        impl<T:Clone> $ToVec<T> for $Self<T> {
            #[inline]
            pub fn $to_vec(&self) -> $Vec<T> {
                self.$as_vec().clone()
            }
        }
    )
)

macro_rules! impl_as_vec(
    ($Self:ident, 2) => (impl_as_vec_helper!(AsVec2, $Self, Vec2, as_vec2, as_mut_vec2, map_as_vec2));
    ($Self:ident, 3) => (impl_as_vec_helper!(AsVec3, $Self, Vec3, as_vec3, as_mut_vec3, map_as_vec3));
    ($Self:ident, 4) => (impl_as_vec_helper!(AsVec4, $Self, Vec4, as_vec4, as_mut_vec4, map_as_vec4));
)

macro_rules! impl_as_vec_helper(
    ($AsVec:ident, $Self:ident, $Vec:ident, $as_vec:ident, $as_mut_vec:ident, $map_as_vec:ident) => (
        impl<T> $AsVec<T> for $Self<T> {
            /// Safely transmute to a vec.
            #[inline]
            pub fn $as_vec<'a>(&'a self) -> &'a $Vec<T> {
                use std::cast::transmute;
                unsafe { transmute(self) }
            }

            /// Safely transmute to a mutable vec.
            #[inline]
            pub fn $as_mut_vec<'a>(&'a mut self) -> &'a mut $Vec<T> {
                use std::cast::transmute;
                unsafe { transmute(self) }
            }

            /// Operate on `self` transmuted to a vec, then return the result as
            /// transmuted back to the `Self` type.
            #[inline]
            pub fn $map_as_vec<'a>(&'a self, f: &fn(&'a $Vec<T>) -> $Vec<T>) -> $Self<T> {
                use std::cast::transmute;
                unsafe { transmute(f(self.$as_vec())) }
            }
        }
    )
)

macro_rules! impl_swap_components(
    ($Self:ident) => (
        impl<T:Clone> SwapComponents for $Self<T> {
            #[inline]
            pub fn swap(&mut self, a: uint, b: uint) {
                let tmp = self.index(a).clone();
                *self.index_mut(a) = self.index(b).clone();
                *self.index_mut(b) = tmp;
            }
        }
    )
)
