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

    #[deprecated = "Use `Array1::swap_elems` instead"]
    #[inline]
    /// Swap the elements at indices `i` and `j` in-place.
    fn swap_i(&mut self, i: uint, j: uint) {
        self.swap_i(i, j)
    }

    /// Swap the elements at indices `i` and `j` in-place.
    #[inline]
    fn swap_elems(&mut self, i: uint, j: uint) {
        // Yeah, ok borrow checker – I know what I'm doing here
        unsafe { ptr::swap(&mut (*self)[i], &mut (*self)[j]) };
    }

    /// Replace an element in the array.
    #[deprecated = "Use `Array1::replace_elem` instead"]
    #[inline]
    fn replace_i(&mut self, i: uint, src: Element) -> Element {
        self.replace_i(i, src)
    }

    /// Replace an element in the array.
    #[inline]
    fn replace_elem(&mut self, i: uint, src: Element) -> Element {
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

    /// Swap two columns of this array.
    #[deprecated = "Use `Array2::swap_cols` instead"]
    #[inline]
    fn swap_c(&mut self, a: uint, b: uint) {
        self.swap_cols(a, b)
    }

    /// Swap two columns of this array.
    #[inline]
    fn swap_cols(&mut self, a: uint, b: uint) {
        unsafe { ptr::swap(&mut (*self)[a], &mut (*self)[b]) };
    }

    /// Replace a column in the array.
    #[deprecated = "Use `Array2::replace_col` instead"]
    #[inline]
    fn replace_c(&mut self, c: uint, src: Column) -> Column {
        self.replace_col(c, src)
    }

    /// Replace a column in the array.
    #[inline]
    fn replace_col(&mut self, c: uint, src: Column) -> Column {
        mem::replace(&mut (*self)[c], src)
    }

    /// Get a row from this array by-value.
    #[deprecated = "Use `Array2::row` instead"]
    #[inline]
    fn r(&self, r: uint) -> Row {
        self.row(r)
    }

    /// Get a row from this array by-value.
    fn row(&self, r: uint) -> Row;

    #[deprecated = "Use `Array2::swap_rows` instead"]
    #[inline]
    fn swap_r(&mut self, a: uint, b: uint) {
        self.swap_rows(a, b)
    }

    /// Swap two rows of this array.
    fn swap_rows(&mut self, a: uint, b: uint);

    /// Swap the values at index `a` and `b`
    #[deprecated = "Use `Array2::swap_elems` instead"]
    #[inline]
    fn swap_cr(&mut self, a: (uint, uint), b: (uint, uint)) {
        self.swap_elems(a, b)
    }

    /// Swap the values at index `a` and `b`
    #[inline]
    fn swap_elems(&mut self, a: (uint, uint), b: (uint, uint)) {
        let (ac, ar) = a;
        let (bc, br) = b;
        unsafe { ptr::swap(&mut (*self)[ac][ar], &mut (*self)[bc][br]) };
    }

    /// Apply a function to each column.
    fn map(&mut self, op: |&Column| -> Column) -> Self;
}

/// Homogeneous arrays of elements that can be converted to and from `[T, ..N]`
/// arrays.
pub trait FixedArray<V> {
    fn into_fixed(self) -> V;
    fn as_fixed<'a>(&'a self) -> &'a V;
    fn as_mut_fixed<'a>(&'a mut self) -> &'a mut V;
    fn from_fixed(v: V) -> Self;
    fn from_fixed_ref<'a>(v: &'a V) -> &'a Self;
    fn from_fixed_mut<'a>(v: &'a mut V) -> &'a mut Self;
}
