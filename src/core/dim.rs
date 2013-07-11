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

use core::{Mat2, Mat3, Mat4};
use core::{Vec2, Vec3, Vec4, Quat};

pub trait Dimensional<T,Slice> {
    pub fn index<'a>(&'a self, i: uint) -> &'a T;
    pub fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T;
    pub fn as_slice<'a>(&'a self) -> &'a Slice;
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut Slice;
}

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

impl_dimensional!(Vec2, T, 2)
impl_dimensional!(Vec3, T, 3)
impl_dimensional!(Vec4, T, 4)
impl_dimensional!(Quat, T, 4)
impl_dimensional!(Mat2, Vec2<T>, 2)
impl_dimensional!(Mat3, Vec3<T>, 3)
impl_dimensional!(Mat4, Vec4<T>, 4)

// This enclosing module is required because attributes don't play nice
// with macros yet
#[cfg(geom)]
pub mod geom_impls {
    use super::Dimensional;
    use geom::{Point2, Point3};

    impl_dimensional!(Point2, T, 2)
    impl_dimensional!(Point3, T, 3)
}

// This enclosing module is required because attributes don't play nice
// with macros yet
#[cfg(color)]
pub mod color_impls {
    use super::Dimensional;
    use color::{HSV, HSVA, YCbCr};
    use color::{RGB, RGBA, SRGB, SRGBA};

    impl_dimensional!(HSV, T, 3)
    impl_dimensional!(HSVA, T, 4)
    impl_dimensional!(RGB, T, 3)
    impl_dimensional!(RGBA, T, 4)
    impl_dimensional!(SRGB, T, 3)
    impl_dimensional!(SRGBA, T, 4)
    impl_dimensional!(YCbCr, T, 3)
}
