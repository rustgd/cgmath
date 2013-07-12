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

use core::{Dimensional, Swap};
use core::{Vec3, ToVec3, AsVec3};
use core::{Vec4, ToVec4, AsVec4};

#[deriving(Clone, Eq)]
pub struct SRGB<T> { r: T, g: T, b: T }

impl<T> SRGB<T> {
    #[inline]
    pub fn new(r: T, g: T, b: T) -> SRGB<T> {
        SRGB { r: r, g: g, b: b }
    }
}

impl_dimensional!(SRGB, T, 3)
impl_to_vec!(SRGB, 3)
impl_as_vec!(SRGB, 3)
impl_swap!(SRGB)
impl_approx!(SRGB { r, g, b })

#[deriving(Clone, Eq)]
pub struct SRGBA<T> { r: T, g: T, b: T, a: T }

impl_dimensional!(SRGBA, T, 4)
impl_to_vec!(SRGBA, 4)
impl_as_vec!(SRGBA, 4)
impl_swap!(SRGBA)
impl_approx!(SRGBA { r, g, b, a })

impl<T> SRGBA<T> {
    #[inline]
    pub fn new(r: T, g: T, b: T, a: T) -> SRGBA<T> {
        SRGBA { r: r, g: g, b: b, a: a }
    }
}
