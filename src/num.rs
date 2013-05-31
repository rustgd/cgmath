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

pub trait NumAssign {
    fn add_assign(&mut self, other: &Self);
    fn sub_assign(&mut self, other: &Self);
    fn mul_assign(&mut self, other: &Self);
    fn div_assign(&mut self, other: &Self);
    fn rem_assign(&mut self, other: &Self);
}

macro_rules! impl_NumAssign(
    ($T:ty) => (
        impl NumAssign for $T {
            #[inline(always)] fn add_assign(&mut self, other: &$T) { *self += *other }
            #[inline(always)] fn sub_assign(&mut self, other: &$T) { *self -= *other }
            #[inline(always)] fn mul_assign(&mut self, other: &$T) { *self *= *other }
            #[inline(always)] fn div_assign(&mut self, other: &$T) { *self /= *other }
            #[inline(always)] fn rem_assign(&mut self, other: &$T) { *self %= *other }
        }
    )
)

impl_NumAssign!(float)
impl_NumAssign!(f32)
impl_NumAssign!(f64)

impl_NumAssign!(int)
impl_NumAssign!(i8)
impl_NumAssign!(i16)
impl_NumAssign!(i32)
impl_NumAssign!(i64)

impl_NumAssign!(uint)
impl_NumAssign!(u8)
impl_NumAssign!(u16)
impl_NumAssign!(u32)
impl_NumAssign!(u64)