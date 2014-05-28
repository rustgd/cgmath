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

use std::mem;
use std::ptr;

/// An array containing elements of type `Element`
pub trait Array1<Element: Copy> {
    /// Get the pointer to the first element of the array.
    fn ptr<'a>(&'a self) -> &'a Element;

    /// Get a mutable pointer to the first element of the array.
    fn mut_ptr<'a>(&'a mut self) -> &'a mut Element;

    /// Get a shared reference to the `i`th value.
    fn i(&self, i: uint) -> Element;

    /// Get a mutable reference to the `i`th value.
    fn mut_i<'a>(&'a mut self, i: uint) -> &'a mut Element;

    #[inline]
    /// Swap the elements at indices `a` and `b` in-place.
    fn swap_i(&mut self, a: uint, b: uint) {
        // Yeah, ok borrow checker – I know what I'm doing here
        unsafe { ptr::swap(self.mut_i(a), self.mut_i(b)) };
    }

    /// Replace an element in the array.
    #[inline]
    fn replace_i(&mut self, i: uint, src: Element) -> Element {
        mem::replace(self.mut_i(i), src)
    }
}

/// A column-major array
pub trait Array2<Column: Array1<Element>, Row: Array1<Element>, Element: Copy> {
    /// Get the pointer to the first element of the array.
    fn ptr<'a>(&'a self) -> &'a Element;

    /// Get a mutable pointer to the first element of the array.
    fn mut_ptr<'a>(&'a mut self) -> &'a mut Element;

    /// Get a shared reference to a column of this array.
    fn c<'a>(&'a self, c: uint) -> &'a Column;

    /// Get a mutable reference to a column of this array.
    fn mut_c<'a>(&'a mut self, c: uint) -> &'a mut Column;

    /// Swap two columns of this array.
    #[inline]
    fn swap_c(&mut self, a: uint, b: uint) {
        unsafe { ptr::swap(self.mut_c(a), self.mut_c(b)) };
    }

    /// Replace a column in the array.
    #[inline]
    fn replace_c(&mut self, c: uint, src: Column) -> Column {
        mem::replace(self.mut_c(c), src)
    }

    /// Get a row from this array by-value.
    fn r(&self, r: uint) -> Row;

    /// Swap two rows of this array.
    fn swap_r(&mut self, a: uint, b: uint);

    /// Return a shared reference to the element at column `c` and row `r`.
    #[inline]
    fn cr(&self, c: uint, r: uint) -> Element { self.c(c).i(r) }

    /// Return a mutable reference to the element at column `c` and row `r`.
    #[inline]
    fn mut_cr<'a>(&'a mut self, c: uint, r: uint) -> &'a mut Element {
        self.mut_c(c).mut_i(r)
    }

    /// Swap the values at index `a` and `b`
    #[inline]
    fn swap_cr(&mut self, a: (uint, uint), b: (uint, uint)) {
        let (ac, ar) = a;
        let (bc, br) = b;
        unsafe { ptr::swap(self.mut_cr(ac, ar), self.mut_cr(bc, br)) };
    }
}
