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
use std::ops::*;

/// An array containing elements of type `Element`
pub trait Array1<Element: Copy>: Index<usize, Output=Element> + IndexMut<usize, Output=Element> {
    /// Get the pointer to the first element of the array.
    fn ptr<'a>(&'a self) -> &'a Element {
        &(*self)[0]
    }

    /// Get a mutable pointer to the first element of the array.
    fn mut_ptr<'a>(&'a mut self) -> &'a mut Element {
        &mut (*self)[0]
    }

    /// Swap the elements at indices `i` and `j` in-place.
    #[inline]
    fn swap_elems(&mut self, i: usize, j: usize) {
        // Yeah, ok borrow checker – I know what I'm doing here
        unsafe { ptr::swap(&mut (*self)[i], &mut (*self)[j]) };
    }

    /// Replace an element in the array.
    #[inline]
    fn replace_elem(&mut self, i: usize, src: Element) -> Element {
        mem::replace(&mut (*self)[i], src)
    }

    /// Apply a function to each element.
    fn map<F>(&mut self, op: F) -> Self
        where F:  FnMut(Element) -> Element;
}

/// A column-major array
pub trait Array2<Column: Array1<Element>+'static, Row: Array1<Element>, Element: Copy>:
        Index<usize, Output=Column> + IndexMut<usize, Output=Column> {
    /// Get the pointer to the first element of the array.
    fn ptr<'a>(&'a self) -> &'a Element {
        &(*self)[0][0]
    }

    /// Get a mutable pointer to the first element of the array.
    fn mut_ptr<'a>(&'a mut self) -> &'a mut Element {
        &mut (*self)[0][0]
    }

    /// Swap two columns of this array.
    #[inline]
    fn swap_cols(&mut self, a: usize, b: usize) {
        unsafe { ptr::swap(&mut (*self)[a], &mut (*self)[b]) };
    }

    /// Replace a column in the array.
    #[inline]
    fn replace_col(&mut self, c: usize, src: Column) -> Column {
        mem::replace(&mut (*self)[c], src)
    }

    /// Get a row from this array by-value.
    fn row(&self, r: usize) -> Row;

    /// Swap two rows of this array.
    fn swap_rows(&mut self, a: usize, b: usize);

    /// Swap the values at index `a` and `b`
    #[inline]
    fn swap_elems(&mut self, a: (usize, usize), b: (usize, usize)) {
        let (ac, ar) = a;
        let (bc, br) = b;
        unsafe { ptr::swap(&mut (*self)[ac][ar], &mut (*self)[bc][br]) };
    }

    /// Apply a function to each column.
    fn map<F>(&mut self, op: F) -> Self
        where F: FnMut(&Column) -> Column;
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
