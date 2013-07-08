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

use std::num;

use color::{Channel, HSV, ToHSV};

#[path = "../num_macros.rs"]
mod num_macros;

#[deriving(Clone, Eq)]
pub struct RGB<T> { r: T, g: T, b: T }

impl<T> RGB<T> {
    #[inline]
    pub fn new(r: T, g: T, b: T) -> RGB<T> {
        RGB { r: r, g: g, b: b }
    }
}

pub trait ToRGB<T> {
    pub fn to_rgb(&self) -> RGB<T>;
}

impl<T:Clone + Channel + Float> ToHSV<T> for RGB<T> {
    pub fn to_hsv(&self) -> HSV<T> {
        // Algorithm taken from the Wikipedia article on HSL and HSV:
        // http://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV

        let mx = (*self).r.max(&(*self).g).max(&(*self).b);
        let mn = (*self).r.min(&(*self).g).min(&(*self).b);
        let chr = mx - mn;

        if chr != zero!(T) {
            let h = cond! (
                ((*self).r == mx)       { (((*self).g - (*self).b) / chr) % num::cast(6) }
                ((*self).g == mx)       { (((*self).b - (*self).r) / chr) + num::cast(2) }
                _ /* (*self).b == mx */ { (((*self).r - (*self).g) / chr) + num::cast(4) }
            ) * num::cast(60);

            let s = chr / mx;

            HSV::new(h, s, mx)

        } else {
            HSV::new(zero!(T), zero!(T), mx)
        }
    }
}
