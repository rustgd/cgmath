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
pub trait Array1<Element: Copy>: Index<uint, Element> + IndexMut<uint, Element> {
    /// Get the pointer to the first element of the array.
    fn ptr<'a>(&'a self) -> &'a Element {
        &(*self)[0]
    }

    /// Get a mutable pointer to the first element of the array.
    fn mut_ptr<'a>(&'a mut self) -> &'a mut Element {
        &mut (*self)[0]
    }

    /// Get a shared reference to the `i`th value.
    #[deprecated = "Use index operator instead"]
    #[inline]
    fn i<'a>(&'a self, i: uint) -> &'a Element {
        &(*self)[i]
    }

    /// Get a mutable reference to the `i`th value.
    #[deprecated = "Use index operator instead"]
    #[inline]
    fn mut_i<'a>(&'a mut self, i: uint) -> &'a mut Element {
        &mut (*self)[i]
    }

    #[inline]
    /// Swap the elements at indices `i` and `j` in-place.
    fn swap_i(&mut self, i: uint, j: uint) {
        // Yeah, ok borrow checker – I know what I'm doing here
        unsafe { ptr::swap(&mut (*self)[i], &mut (*self)[j]) };
    }

    /// Replace an element in the array.
    #[inline]
    fn replace_i(&mut self, i: uint, src: Element) -> Element {
        mem::replace(&mut (*self)[i], src)
    }

    /// Apply a function to each element.
    fn map(&mut self, op: |Element| -> Element) -> Self;
}

/// A column-major array
pub trait Array2<Column: Array1<Element>, Row: Array1<Element>, Element: Copy>:
        Index<uint, Column> + IndexMut<uint, Column> {
    /// Get the pointer to the first element of the array.
    fn ptr<'a>(&'a self) -> &'a Element {
        &(*self)[0][0]
    }

    /// Get a mutable pointer to the first element of the array.
    fn mut_ptr<'a>(&'a mut self) -> &'a mut Element {
        &mut (*self)[0][0]
    }

    /// Get a shared reference to a column of this array.
    #[deprecated = "Use index operator instead"]
    #[inline]
    fn c<'a>(&'a self, c: uint) -> &'a Column {
        &(*self)[c]
    }

    /// Get a mutable reference to a column of this array.
    #[deprecated = "Use index operator instead"]
    #[inline]
    fn mut_c<'a>(&'a mut self, c: uint) -> &'a mut Column {
        &mut (*self)[c]
    }

    /// Swap two columns of this array.
    #[inline]
    fn swap_c(&mut self, a: uint, b: uint) {
        unsafe { ptr::swap(&mut (*self)[a], &mut (*self)[b]) };
    }

    /// Replace a column in the array.
    #[inline]
    fn replace_c(&mut self, c: uint, src: Column) -> Column {
        mem::replace(&mut (*self)[c], src)
    }

    /// Get a row from this array by-value.
    fn r(&self, r: uint) -> Row;

    /// Swap two rows of this array.
    fn swap_r(&mut self, a: uint, b: uint);

    /// Return a shared reference to the element at column `c` and row `r`.
    #[deprecated = "Use index operators instead"]
    #[inline]
    fn cr<'a>(&'a self, c: uint, r: uint) -> &'a Element {
        &(*self)[c][r]
    }

    /// Return a mutable reference to the element at column `c` and row `r`.
    #[deprecated = "Use index operators instead"]
    #[inline]
    fn mut_cr<'a>(&'a mut self, c: uint, r: uint) -> &'a mut Element {
        &mut (*self)[c][r]
    }

    /// Swap the values at index `a` and `b`
    #[inline]
    fn swap_cr(&mut self, a: (uint, uint), b: (uint, uint)) {
        let (ac, ar) = a;
        let (bc, br) = b;
        unsafe { ptr::swap(&mut (*self)[ac][ar], &mut (*self)[bc][br]) };
    }

    /// Apply a function to each column.
    fn map(&mut self, op: |&Column| -> Column) -> Self;
}
