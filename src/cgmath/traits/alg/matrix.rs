// Copyright 2013 The CGMath Developers. For a full listing of the authors,
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

use traits::alg::Array;
use traits::alg::ClonableArray;
use traits::alg::Field;
use traits::alg::Ring;
use traits::alg::VectorSpace;

pub trait Matrix
<
    S: Field + Clone + ApproxEq<S>,
    RV: Clone + VectorSpace<S> + ClonableArray<S, RVSlice>, RVSlice, RSlice,
    CV: Clone + VectorSpace<S> + ClonableArray<S, CVSlice>, CVSlice, CSlice,
    MT//: Matrix<S, CV, CSlice, RV, RSlice, Self>
>
:   Ring<S>
+   ClonableArray<CV, RSlice>
{
    #[inline]
    fn c<'a>(&'a self, c: uint) -> &'a CV { self.i(c) }

    #[inline]
    fn mut_c<'a>(&'a mut self, c: uint) -> &'a mut CV { self.mut_i(c) }

    #[inline]
    fn swap_c(&mut self, a: uint, b: uint) {
        let tmp = self.c(a).clone();
        *self.mut_c(a) = self.c(b).clone();
        *self.mut_c(b) = tmp;
    }

    fn r(&self, r: uint) -> RV;

    #[inline]
    fn swap_r(&mut self, a: uint, b: uint) {
        self.map_mut(|c| c.swap(a, b));
    }

    #[inline]
    fn cr<'a>(&'a self, c: uint, r: uint) -> &'a S { self.i(c).i(r) }

    #[inline]
    fn mut_cr<'a>(&'a mut self, c: uint, r: uint) -> &'a mut S {
        self.mut_i(c).mut_i(r)
    }

    #[inline]
    fn swap_cr(&mut self, a: (uint, uint), b: (uint, uint)) {
        let (ca, ra) = a;
        let (cb, rb) = b;
        let tmp = self.cr(ca, ra).clone();
        *self.mut_cr(ca, ra) = self.cr(cb, rb).clone();
        *self.mut_cr(cb, rb) = tmp;
    }

    // fn swap_cr(&mut self, (ca, ra): (uint, uint), (cb, rb): (uint, uint)) {
    //     let tmp = self.cr(ca, ra).clone();
    //     *self.mut_cr(ca, ra) = self.cr(cb, rb).clone();
    //     *self.mut_cr(cb, rb) = tmp;
    // }

    fn transpose(&self) -> MT;
}
