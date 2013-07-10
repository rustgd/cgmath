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
use std::cast;

use color::Color;
use color::{Channel, FloatChannel};
use color::{HSV, ToHSV, HSVA, ToHSVA};

#[path = "../num_macros.rs"]
mod num_macros;

#[deriving(Clone, Eq)]
pub struct RGB<T> { r: T, g: T, b: T }

impl<T:Channel> RGB<T> {
    #[inline]
    pub fn new(r: T, g: T, b: T) -> RGB<T> {
        RGB { r: r, g: g, b: b }
    }
}

impl<T:Channel> Color<T> for RGB<T> {
    #[inline]
    pub fn clamp(&self, lo: T, hi: T) -> RGB<T> {
        RGB::new((*self).r.clamp(&lo, &hi),
                 (*self).g.clamp(&lo, &hi),
                 (*self).b.clamp(&lo, &hi))
    }

    #[inline]
    pub fn inverse(&self) -> RGB<T> {
        RGB::new((*self).r.invert_channel(),
                 (*self).g.invert_channel(),
                 (*self).b.invert_channel())
    }
}

pub trait ToRGB {
    pub fn to_rgb<U:Channel>(&self) -> RGB<U>;
}

impl ToRGB for u32 {
    #[inline]
    pub fn to_rgb<U:Channel>(&self) -> RGB<U> {
        fail!("Not yet implemented")
    }
}

impl ToRGB for u64 {
    #[inline]
    pub fn to_rgb<U:Channel>(&self) -> RGB<U> {
        fail!("Not yet implemented")
    }
}

impl<T:Clone + Channel> ToRGB for RGB<T> {
    #[inline]
    pub fn to_rgb<U:Channel>(&self) -> RGB<U> {
        RGB::new((*self).r.to_channel(),
                 (*self).g.to_channel(),
                 (*self).b.to_channel())
    }
}

impl<T:Clone + Channel> ToHSV for RGB<T> {
    #[inline]
    pub fn to_hsv<U:FloatChannel>(&self) -> HSV<U> {
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

#[deriving(Clone, Eq)]
pub struct RGBA<T> { r: T, g: T, b: T, a: T }

impl<T:Channel> RGBA<T> {
    #[inline]
    pub fn new(r: T, g: T, b: T, a: T) -> RGBA<T> {
        RGBA { r: r, g: g, b: b, a: a }
    }

    #[inline]
    pub fn from_rgb_a(rgb: RGB<T>, a: T) -> RGBA<T> {
        unsafe { cast::transmute((rgb, a)) }
    }

    #[inline]
    pub fn rgb<'a>(&'a self) -> &'a RGB<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn rgb_mut<'a>(&'a mut self) -> &'a mut RGB<T> {
        unsafe { cast::transmute(self) }
    }
}

impl<T:Channel> Color<T> for RGBA<T> {
    #[inline]
    pub fn clamp(&self, lo: T, hi: T) -> RGBA<T> {
        RGBA::new((*self).r.clamp(&lo, &hi),
                  (*self).g.clamp(&lo, &hi),
                  (*self).b.clamp(&lo, &hi),
                  (*self).a.clamp(&lo, &hi))
    }

    #[inline]
    pub fn inverse(&self) -> RGBA<T> {
        RGBA::new((*self).r.invert_channel(),
                  (*self).g.invert_channel(),
                  (*self).b.invert_channel(),
                  (*self).a.invert_channel())
    }
}

pub trait ToRGBA {
    pub fn to_rgba<U:Channel>(&self) -> RGBA<U>;
}

impl ToRGBA for u32 {
    #[inline]
    pub fn to_rgba<U:Channel>(&self) -> RGBA<U> {
        fail!("Not yet implemented")
    }
}

impl ToRGBA for u64 {
    #[inline]
    pub fn to_rgba<U:Channel>(&self) -> RGBA<U> {
        fail!("Not yet implemented")
    }
}

impl<C: ToRGB, T:Clone + Channel> ToRGBA for (C, T) {
    #[inline]
    pub fn to_rgba<U:Channel>(&self) -> RGBA<U> {
        match *self {
            (ref rgb, ref a) =>  {
                RGBA::from_rgb_a(rgb.to_rgb(), a.to_channel())
            }
        }
    }
}

impl<T:Clone + Channel> ToRGBA for RGBA<T> {
    #[inline]
    pub fn to_rgba<U:Channel>(&self) -> RGBA<U> {
        RGBA::new((*self).r.to_channel(),
                  (*self).g.to_channel(),
                  (*self).b.to_channel(),
                  (*self).a.to_channel())
    }
}

impl<T:Clone + Channel> ToHSVA for RGBA<T> {
    #[inline]
    pub fn to_hsva<U:FloatChannel>(&self) -> HSVA<U> {
        HSVA::from_hsv_a(self.rgb().to_hsv(), (*self).a.to_channel())
    }
}

// 0xFF_FF_FF_FF_u32
// 0xFFFF_FFFF_FFFF_FFFF_u64
