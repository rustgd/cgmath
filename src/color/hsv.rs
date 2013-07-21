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

use math::*;

use color::{Color, FloatColor};
use color::{Channel, FloatChannel};
use color::{RGB, ToRGB, RGBA, ToRGBA};

#[deriving(Clone, Eq)]
pub struct HSV<T> { h: T, s: T, v: T }

impl_dimensioned!(HSV, T, 3)
impl_to_vec!(HSV, 3)
impl_as_vec!(HSV, 3)
impl_swap_components!(HSV)
impl_approx!(HSV { h, s, v })

impl<T:FloatChannel> HSV<T> {
    pub fn new(h: T, s: T, v: T) -> HSV<T> {
        HSV { h: h, s: s, v: v }
    }
}

impl<T:FloatChannel> Color<T> for HSV<T> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    pub fn clamp_s(&self, lo: T, hi: T) -> HSV<T> {
        HSV::new(self.h.clamp(&lo, &hi), // Should the hue component be clamped?
                 self.s.clamp(&lo, &hi),
                 self.v.clamp(&lo, &hi))
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    pub fn clamp_c(&self, lo: &HSV<T>, hi: &HSV<T>) -> HSV<T> {
        HSV::new(self.h.clamp(&lo.h, &hi.h),
                 self.s.clamp(&lo.s, &hi.s),
                 self.v.clamp(&lo.v, &hi.v))
    }

    /// Inverts the color.
    #[inline]
    pub fn inverse(&self) -> HSV<T> {
        HSV::new(self.h.invert_degrees(),
                 self.s.invert_channel(),
                 self.v.invert_channel())
    }
}

impl<T:FloatChannel> FloatColor<T> for HSV<T> {
    /// Normalizes the components of the color. Modulo `360` is applied to the
    /// `h` component, and `s` and `v` are clamped to the range `(0,1)`.
    #[inline]
    pub fn normalize(&self) -> HSV<T> {
        HSV::new(self.h.normalize_degrees(),
                 self.s.normalize_channel(),
                 self.v.normalize_channel())
    }
}

pub trait ToHSV {
    pub fn to_hsv<U:FloatChannel>(&self) -> HSV<U>;
}

impl ToHSV for u32 {
    #[inline]
    pub fn to_hsv<U:FloatChannel>(&self) -> HSV<U> {
        fail!("Not yet implemented")
    }
}

impl ToHSV for u64 {
    #[inline]
    pub fn to_hsv<U:FloatChannel>(&self) -> HSV<U> {
        fail!("Not yet implemented")
    }
}

impl<T:Clone + FloatChannel> ToHSV for HSV<T> {
    #[inline]
    pub fn to_hsv<U:FloatChannel>(&self) -> HSV<U> {
        HSV::new(self.h.to_channel(),
                 self.s.to_channel(),
                 self.v.to_channel())
    }
}

impl<T:Clone + FloatChannel> ToRGB for HSV<T> {
    pub fn to_rgb<U:Channel>(&self) -> RGB<U> {
        // Algorithm taken from the Wikipedia article on HSL and HSV:
        // http://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV

        let chr = self.v * self.s;
        let h = self.h / num::cast(60);

        // the 2nd largest component
        let x = chr * (one!(T) - ((h % two!(T)) - one!(T)).abs());

        let mut rgb = cond! (
            (h < num::cast(1)) { RGB::new(chr.clone(), x, zero!(T)) }
            (h < num::cast(2)) { RGB::new(x, chr.clone(), zero!(T)) }
            (h < num::cast(3)) { RGB::new(zero!(T), chr.clone(), x) }
            (h < num::cast(4)) { RGB::new(zero!(T), x, chr.clone()) }
            (h < num::cast(5)) { RGB::new(x, zero!(T), chr.clone()) }
            (h < num::cast(6)) { RGB::new(chr.clone(), zero!(T), x) }
            _                  { RGB::new(zero!(T), zero!(T), zero!(T)) }
        );

        // match the value by adding the same amount to each component
        let mn = self.v - chr;

        rgb.r = rgb.r + mn;
        rgb.g = rgb.g + mn;
        rgb.b = rgb.b + mn;

        rgb.to_rgb::<U>()
    }
}

#[deriving(Clone, Eq)]
pub struct HSVA<T> { h: T, s: T, v: T, a: T }

impl_dimensioned!(HSVA, T, 4)
impl_to_vec!(HSVA, 4)
impl_as_vec!(HSVA, 4)
impl_swap_components!(HSVA)
impl_approx!(HSVA { h, s, v, a })

impl<T:FloatChannel> HSVA<T> {
    #[inline]
    pub fn new(h: T, s: T, v: T, a: T) -> HSVA<T> {
        HSVA { h: h, s: s, v: v, a: a }
    }

    #[inline]
    pub fn from_hsv_a(hsv: HSV<T>, a: T) -> HSVA<T> {
        unsafe { cast::transmute((hsv, a)) }
    }

    #[inline]
    pub fn hsv<'a>(&'a self) -> &'a HSV<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn hsv_mut<'a>(&'a mut self) -> &'a mut HSV<T> {
        unsafe { cast::transmute(self) }
    }
}

impl<T:FloatChannel> Color<T> for HSVA<T> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    pub fn clamp_s(&self, lo: T, hi: T) -> HSVA<T> {
        HSVA::new(self.h.clamp(&lo, &hi),    // Should the hue component be clamped?
                  self.s.clamp(&lo, &hi),
                  self.v.clamp(&lo, &hi),
                  self.a.clamp(&lo, &hi))
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    pub fn clamp_c(&self, lo: &HSVA<T>, hi: &HSVA<T>) -> HSVA<T> {
        HSVA::new(self.h.clamp(&lo.h, &hi.h),
                  self.s.clamp(&lo.s, &hi.s),
                  self.v.clamp(&lo.v, &hi.v),
                  self.a.clamp(&lo.a, &hi.a))
    }

    /// Inverts the color.
    #[inline]
    pub fn inverse(&self) -> HSVA<T> {
        HSVA::new(self.h.invert_degrees(),
                  self.s.invert_channel(),
                  self.v.invert_channel(),
                  self.a.invert_channel())
    }
}

impl<T:FloatChannel> FloatColor<T> for HSVA<T> {
    /// Normalizes the components of the color. Modulo `360` is applied to the
    /// `h` component, and `s`, `v` and `a` are clamped to the range `(0,1)`.
    #[inline]
    pub fn normalize(&self) -> HSVA<T> {
        HSVA::new(self.h.normalize_degrees(),
                  self.s.normalize_channel(),
                  self.v.normalize_channel(),
                  self.a.normalize_channel())
    }
}

pub trait ToHSVA {
    pub fn to_hsva<U:FloatChannel>(&self) -> HSVA<U>;
}

impl ToHSVA for u32 {
    #[inline]
    pub fn to_hsva<U:FloatChannel>(&self) -> HSVA<U> {
        fail!("Not yet implemented")
    }
}

impl ToHSVA for u64 {
    #[inline]
    pub fn to_hsva<U:FloatChannel>(&self) -> HSVA<U> {
        fail!("Not yet implemented")
    }
}

impl<C: ToHSV, T:Clone + FloatChannel> ToHSVA for (C, T) {
    #[inline]
    pub fn to_hsva<U:FloatChannel>(&self) -> HSVA<U> {
        match *self {
            (ref hsv, ref a) =>  {
                HSVA::from_hsv_a(hsv.to_hsv(), a.to_channel())
            }
        }
    }
}

impl<T:Clone + FloatChannel> ToHSVA for HSVA<T> {
    #[inline]
    pub fn to_hsva<U:FloatChannel>(&self) -> HSVA<U> {
        HSVA::new(self.h.to_channel(),
                  self.s.to_channel(),
                  self.v.to_channel(),
                  self.a.to_channel())
    }
}

impl<T:Clone + FloatChannel> ToRGBA for HSVA<T> {
    #[inline]
    pub fn to_rgba<U:Channel>(&self) -> RGBA<U> {
        RGBA::from_rgb_a(self.hsv().to_rgb(), self.a.to_channel())
    }
}

#[cfg(test)]
mod tests {
    use color::*;

    #[test]
    fn test_hsv_to_hsv() {
        assert_eq!(HSV::new::<f64>(0.0, 0.0, 1.0).to_hsv::<f32>(), HSV::new::<f32>(0.0, 0.0, 1.0));
        assert_eq!(HSV::new::<f64>(0.0, 1.0, 0.6).to_hsv::<f32>(), HSV::new::<f32>(0.0, 1.0, 0.6));
        assert_eq!(HSV::new::<f64>(120.0, 1.0, 0.6).to_hsv::<f32>(), HSV::new::<f32>(120.0, 1.0, 0.6));
        assert_eq!(HSV::new::<f64>(240.0, 1.0, 0.6).to_hsv::<f32>(), HSV::new::<f32>(240.0, 1.0, 0.6));
    }

    #[test]
    fn test_hsv_to_rgb() {
        assert_eq!(HSV::new::<f32>(0.0, 0.0, 1.0).to_rgb::<u8>(), RGB::new::<u8>(0xFF, 0xFF, 0xFF));
        assert_eq!(HSV::new::<f32>(0.0, 1.0, 0.6).to_rgb::<u8>(), RGB::new::<u8>(0x99, 0x00, 0x00));
        assert_eq!(HSV::new::<f32>(120.0, 1.0, 0.6).to_rgb::<u8>(), RGB::new::<u8>(0x00, 0x99, 0x00));
        assert_eq!(HSV::new::<f32>(240.0, 1.0, 0.6).to_rgb::<u8>(), RGB::new::<u8>(0x00, 0x00, 0x99));
    }

    #[test]
    fn test_tuple_to_hsva() {
        assert_eq!((RGB::new::<u8>(0xFF, 0xFF, 0xFF), 0.5f32).to_hsva::<f32>(), HSVA::new::<f32>(0.0, 0.0, 1.0, 0.5));
        assert_eq!((RGB::new::<u8>(0x99, 0x00, 0x00), 0.5f32).to_hsva::<f32>(), HSVA::new::<f32>(0.0, 1.0, 0.6, 0.5));
        assert_eq!((RGB::new::<u8>(0x00, 0x99, 0x00), 0.5f32).to_hsva::<f32>(), HSVA::new::<f32>(120.0, 1.0, 0.6, 0.5));
        assert_eq!((RGB::new::<u8>(0x00, 0x00, 0x99), 0.5f32).to_hsva::<f32>(), HSVA::new::<f32>(240.0, 1.0, 0.6, 0.5));
    }

    #[test]
    fn test_hsva_to_hsva() {
        assert_eq!(HSVA::new::<f64>(0.0, 0.0, 1.0, 0.5).to_hsva::<f32>(), HSVA::new::<f32>(0.0, 0.0, 1.0, 0.5));
        assert_eq!(HSVA::new::<f64>(0.0, 1.0, 0.6, 0.5).to_hsva::<f32>(), HSVA::new::<f32>(0.0, 1.0, 0.6, 0.5));
        assert_eq!(HSVA::new::<f64>(120.0, 1.0, 0.6, 0.5).to_hsva::<f32>(), HSVA::new::<f32>(120.0, 1.0, 0.6, 0.5));
        assert_eq!(HSVA::new::<f64>(240.0, 1.0, 0.6, 0.5).to_hsva::<f32>(), HSVA::new::<f32>(240.0, 1.0, 0.6, 0.5));
    }

    #[test]
    fn test_hsva_to_rgba() {
        assert_eq!(HSVA::new::<f32>(0.0, 0.0, 1.0, 0.5).to_rgba::<u8>(), RGBA::new::<u8>(0xFF, 0xFF, 0xFF, 0x7F));
        assert_eq!(HSVA::new::<f32>(0.0, 1.0, 0.6, 0.5).to_rgba::<u8>(), RGBA::new::<u8>(0x99, 0x00, 0x00, 0x7F));
        assert_eq!(HSVA::new::<f32>(120.0, 1.0, 0.6, 0.5).to_rgba::<u8>(), RGBA::new::<u8>(0x00, 0x99, 0x00, 0x7F));
        assert_eq!(HSVA::new::<f32>(240.0, 1.0, 0.6, 0.5).to_rgba::<u8>(), RGBA::new::<u8>(0x00, 0x00, 0x99, 0x7F));
    }
}
