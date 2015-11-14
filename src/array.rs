// Copyright 2013 The OMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
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

use std::mem;
use std::ptr;
use std::ops::*;

use num::PartialOrd;

/// An array containing elements of type `Element`
pub trait Array1 where
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: Index<usize, Output = <Self as Array1>::Element>,
    Self: IndexMut<usize, Output = <Self as Array1>::Element>,
{
    type Element: Copy;

    /// Get the pointer to the first element of the array.
    #[inline]
    fn as_ptr(&self) -> *const Self::Element {
        &self[0]
    }

    /// Get a mutable pointer to the first element of the array.
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Element {
        &mut self[0]
    }

    /// Swap the elements at indices `i` and `j` in-place.
    #[inline]
    fn swap_elems(&mut self, i: usize, j: usize) {
        // Yeah, ok borrow checker – I know what I'm doing here
        unsafe { ptr::swap(&mut self[i], &mut self[j]) };
    }

    /// Replace an element in the array.
    #[inline]
    fn replace_elem(&mut self, i: usize, src: Self::Element) -> Self::Element {
        mem::replace(&mut self[i], src)
    }

    /// The sum of the elements of the array.
    fn sum(self) -> Self::Element where Self::Element: Add<Output = <Self as Array1>::Element>;

    /// The product of the elements of the array.
    fn product(self) -> Self::Element where Self::Element: Mul<Output = <Self as Array1>::Element>;

    /// The minimum element of the array.
    fn min(self) -> Self::Element where Self::Element: PartialOrd;

    /// The maximum element of the array.
    fn max(self) -> Self::Element where Self::Element: PartialOrd;
}
