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

use color::{Channel, ToChannel, FloatChannel};
use color::{HSV, ToHSV};

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

pub trait ToRGB {
    pub fn to_rgb<U:Clone + Channel>(&self) -> RGB<U>;
}

impl<T:Clone + ToChannel> ToRGB for RGB<T> {
    #[inline]
    pub fn to_rgb<U:Clone + Channel>(&self) -> RGB<U> {
        RGB::new(Channel::from((*self).r.clone()),
                 Channel::from((*self).g.clone()),
                 Channel::from((*self).b.clone()))
    }
}

impl<T:Clone + ToChannel> ToHSV for RGB<T> {
    #[inline]
    pub fn to_hsv<U:Clone + FloatChannel>(&self) -> HSV<U> {
        // Algorithm taken from the Wikipedia article on HSL and HSV:
        // http://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV

        let rgb_u = self.to_rgb::<U>();

        let mx = rgb_u.r.max(&rgb_u.g).max(&rgb_u.b);
        let mn = rgb_u.r.min(&rgb_u.g).min(&rgb_u.b);
        let chr = mx - mn;

        if chr != zero!(U) {
            let h = cond! (
                (rgb_u.r == mx)       { ((rgb_u.g - rgb_u.b) / chr) % num::cast(6) }
                (rgb_u.g == mx)       { ((rgb_u.b - rgb_u.r) / chr) + num::cast(2) }
                _ /* rgb_u.b == mx */ { ((rgb_u.r - rgb_u.g) / chr) + num::cast(4) }
            ) * num::cast(60);

            let s = chr / mx;

            HSV::new(h, s, mx)

        } else {
            HSV::new(zero!(U), zero!(U), mx)
        }
    }
}
