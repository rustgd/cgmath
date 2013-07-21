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
use color::{HSV, ToHSV, HSVA, ToHSVA};

#[deriving(Clone, Eq)]
pub struct RGB<T> { r: T, g: T, b: T }

impl_dimensioned!(RGB, T, 3)
impl_to_vec!(RGB, 3)
impl_as_vec!(RGB, 3)
impl_swap_components!(RGB)
impl_approx!(RGB { r, g, b })

impl<T:Channel> RGB<T> {
    #[inline]
    pub fn new(r: T, g: T, b: T) -> RGB<T> {
        RGB { r: r, g: g, b: b }
    }
}

impl<T:Channel> Color<T> for RGB<T> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    pub fn clamp_s(&self, lo: T, hi: T) -> RGB<T> {
        RGB::new(self.r.clamp(&lo, &hi),
                 self.g.clamp(&lo, &hi),
                 self.b.clamp(&lo, &hi))
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    pub fn clamp_c(&self, lo: &RGB<T>, hi: &RGB<T>) -> RGB<T> {
        RGB::new(self.r.clamp(&lo.r, &hi.r),
                 self.g.clamp(&lo.g, &hi.g),
                 self.b.clamp(&lo.b, &hi.b))
    }

    /// Inverts the color.
    #[inline]
    pub fn inverse(&self) -> RGB<T> {
        RGB::new(self.r.invert_channel(),
                 self.g.invert_channel(),
                 self.b.invert_channel())
    }
}

impl<T:FloatChannel> FloatColor<T> for RGB<T> {
    /// Normalizes the components of the color by clamping them to the range `(0,1)`.
    #[inline]
    pub fn normalize(&self) -> RGB<T> {
        RGB::new(self.r.normalize_channel(),
                 self.g.normalize_channel(),
                 self.b.normalize_channel())
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
        RGB::new(self.r.to_channel(),
                 self.g.to_channel(),
                 self.b.to_channel())
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

impl_dimensioned!(RGBA, T, 4)
impl_to_vec!(RGBA, 4)
impl_as_vec!(RGBA, 4)
impl_swap_components!(RGBA)
impl_approx!(RGBA { r, g, b, a })

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
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    pub fn clamp_s(&self, lo: T, hi: T) -> RGBA<T> {
        RGBA::new(self.r.clamp(&lo, &hi),
                  self.g.clamp(&lo, &hi),
                  self.b.clamp(&lo, &hi),
                  self.a.clamp(&lo, &hi))
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    pub fn clamp_c(&self, lo: &RGBA<T>, hi: &RGBA<T>) -> RGBA<T> {
        RGBA::new(self.r.clamp(&lo.r, &hi.r),
                  self.g.clamp(&lo.g, &hi.g),
                  self.b.clamp(&lo.b, &hi.b),
                  self.a.clamp(&lo.a, &hi.a))
    }

    /// Inverts the color.
    #[inline]
    pub fn inverse(&self) -> RGBA<T> {
        RGBA::new(self.r.invert_channel(),
                  self.g.invert_channel(),
                  self.b.invert_channel(),
                  self.a.invert_channel())
    }
}

impl<T:FloatChannel> FloatColor<T> for RGBA<T> {
    /// Normalizes the components of the color by clamping them to the range `(0,1)`.
    #[inline]
    pub fn normalize(&self) -> RGBA<T> {
        RGBA::new(self.r.normalize_channel(),
                  self.g.normalize_channel(),
                  self.b.normalize_channel(),
                  self.a.normalize_channel())
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
        RGBA::new(self.r.to_channel(),
                  self.g.to_channel(),
                  self.b.to_channel(),
                  self.a.to_channel())
    }
}

impl<T:Clone + Channel> ToHSVA for RGBA<T> {
    #[inline]
    pub fn to_hsva<U:FloatChannel>(&self) -> HSVA<U> {
        HSVA::from_hsv_a(self.rgb().to_hsv(), self.a.to_channel())
    }
}

/// SVG 1.0 color constants: http://www.w3.org/TR/SVG/types.html#ColorKeywords
pub mod consts {
    use color::RGB;

    static ALICEBLUE:               RGB<u8> = RGB { r: 0xF0, g: 0xF8, b: 0xFF };
    static ANTIQUEWHITE:            RGB<u8> = RGB { r: 0xFA, g: 0xEB, b: 0xD7 };
    static AQUA:                    RGB<u8> = RGB { r: 0x00, g: 0xFF, b: 0xFF };
    static AQUAMARINE:              RGB<u8> = RGB { r: 0x7F, g: 0xFF, b: 0xD4 };
    static AZURE:                   RGB<u8> = RGB { r: 0xF0, g: 0xFF, b: 0xFF };
    static BEIGE:                   RGB<u8> = RGB { r: 0xF5, g: 0xF5, b: 0xDC };
    static BISQUE:                  RGB<u8> = RGB { r: 0xFF, g: 0xE4, b: 0xC4 };
    static BLACK:                   RGB<u8> = RGB { r: 0x00, g: 0x00, b: 0x00 };
    static BLANCHEDALMOND:          RGB<u8> = RGB { r: 0xFF, g: 0xEB, b: 0xCD };
    static BLUE:                    RGB<u8> = RGB { r: 0x00, g: 0x00, b: 0xFF };
    static BLUEVIOLET:              RGB<u8> = RGB { r: 0x8A, g: 0x2B, b: 0xE2 };
    static BROWN:                   RGB<u8> = RGB { r: 0xA5, g: 0x2A, b: 0x2A };
    static BURLYWOOD:               RGB<u8> = RGB { r: 0xDE, g: 0xB8, b: 0x87 };
    static CADETBLUE:               RGB<u8> = RGB { r: 0x5F, g: 0x9E, b: 0xA0 };
    static CHARTREUSE:              RGB<u8> = RGB { r: 0x7F, g: 0xFF, b: 0x00 };
    static CHOCOLATE:               RGB<u8> = RGB { r: 0xD2, g: 0x69, b: 0x1E };
    static CORAL:                   RGB<u8> = RGB { r: 0xFF, g: 0x7F, b: 0x50 };
    static CORNFLOWERBLUE:          RGB<u8> = RGB { r: 0x64, g: 0x95, b: 0xED };
    static CORNSILK:                RGB<u8> = RGB { r: 0xFF, g: 0xF8, b: 0xDC };
    static CRIMSON:                 RGB<u8> = RGB { r: 0xDC, g: 0x14, b: 0x3C };
    static CYAN:                    RGB<u8> = RGB { r: 0x00, g: 0xFF, b: 0xFF };
    static DARKBLUE:                RGB<u8> = RGB { r: 0x00, g: 0x00, b: 0x8B };
    static DARKCYAN:                RGB<u8> = RGB { r: 0x00, g: 0x8B, b: 0x8B };
    static DARKGOLDENROD:           RGB<u8> = RGB { r: 0xB8, g: 0x86, b: 0x0B };
    static DARKGRAY:                RGB<u8> = RGB { r: 0xA9, g: 0xA9, b: 0xA9 };
    static DARKGREEN:               RGB<u8> = RGB { r: 0x00, g: 0x64, b: 0x00 };
    static DARKKHAKI:               RGB<u8> = RGB { r: 0xBD, g: 0xB7, b: 0x6B };
    static DARKMAGENTA:             RGB<u8> = RGB { r: 0x8B, g: 0x00, b: 0x8B };
    static DARKOLIVEGREEN:          RGB<u8> = RGB { r: 0x55, g: 0x6B, b: 0x2F };
    static DARKORANGE:              RGB<u8> = RGB { r: 0xFF, g: 0x8C, b: 0x00 };
    static DARKORCHID:              RGB<u8> = RGB { r: 0x99, g: 0x32, b: 0xCC };
    static DARKRED:                 RGB<u8> = RGB { r: 0x8B, g: 0x00, b: 0x00 };
    static DARKSALMON:              RGB<u8> = RGB { r: 0xE9, g: 0x96, b: 0x7A };
    static DARKSEAGREEN:            RGB<u8> = RGB { r: 0x8F, g: 0xBC, b: 0x8F };
    static DARKSLATEBLUE:           RGB<u8> = RGB { r: 0x48, g: 0x3D, b: 0x8B };
    static DARKSLATEGRAY:           RGB<u8> = RGB { r: 0x2F, g: 0x4F, b: 0x4F };
    static DARKTURQUOISE:           RGB<u8> = RGB { r: 0x00, g: 0xCE, b: 0xD1 };
    static DARKVIOLET:              RGB<u8> = RGB { r: 0x94, g: 0x00, b: 0xD3 };
    static DEEPPINK:                RGB<u8> = RGB { r: 0xFF, g: 0x14, b: 0x93 };
    static DEEPSKYBLUE:             RGB<u8> = RGB { r: 0x00, g: 0xBF, b: 0xFF };
    static DIMGRAY:                 RGB<u8> = RGB { r: 0x69, g: 0x69, b: 0x69 };
    static DODGERBLUE:              RGB<u8> = RGB { r: 0x1E, g: 0x90, b: 0xFF };
    static FIREBRICK:               RGB<u8> = RGB { r: 0xB2, g: 0x22, b: 0x22 };
    static FLORALWHITE:             RGB<u8> = RGB { r: 0xFF, g: 0xFA, b: 0xF0 };
    static FORESTGREEN:             RGB<u8> = RGB { r: 0x22, g: 0x8B, b: 0x22 };
    static FUCHSIA:                 RGB<u8> = RGB { r: 0xFF, g: 0x00, b: 0xFF };
    static GAINSBORO:               RGB<u8> = RGB { r: 0xDC, g: 0xDC, b: 0xDC };
    static GHOSTWHITE:              RGB<u8> = RGB { r: 0xF8, g: 0xF8, b: 0xFF };
    static GOLD:                    RGB<u8> = RGB { r: 0xFF, g: 0xD7, b: 0x00 };
    static GOLDENROD:               RGB<u8> = RGB { r: 0xDA, g: 0xA5, b: 0x20 };
    static GRAY:                    RGB<u8> = RGB { r: 0x80, g: 0x80, b: 0x80 };
    static GREEN:                   RGB<u8> = RGB { r: 0x00, g: 0x80, b: 0x00 };
    static GREENYELLOW:             RGB<u8> = RGB { r: 0xAD, g: 0xFF, b: 0x2F };
    static HONEYDEW:                RGB<u8> = RGB { r: 0xF0, g: 0xFF, b: 0xF0 };
    static HOTPINK:                 RGB<u8> = RGB { r: 0xFF, g: 0x69, b: 0xB4 };
    static INDIANRED:               RGB<u8> = RGB { r: 0xCD, g: 0x5C, b: 0x5C };
    static INDIGO:                  RGB<u8> = RGB { r: 0x4B, g: 0x00, b: 0x82 };
    static IVORY:                   RGB<u8> = RGB { r: 0xFF, g: 0xFF, b: 0xF0 };
    static KHAKI:                   RGB<u8> = RGB { r: 0xF0, g: 0xE6, b: 0x8C };
    static LAVENDER:                RGB<u8> = RGB { r: 0xE6, g: 0xE6, b: 0xFA };
    static LAVENDERBLUSH:           RGB<u8> = RGB { r: 0xFF, g: 0xF0, b: 0xF5 };
    static LAWNGREEN:               RGB<u8> = RGB { r: 0x7C, g: 0xFC, b: 0x00 };
    static LEMONCHIFFON:            RGB<u8> = RGB { r: 0xFF, g: 0xFA, b: 0xCD };
    static LIGHTBLUE:               RGB<u8> = RGB { r: 0xAD, g: 0xD8, b: 0xE6 };
    static LIGHTCORAL:              RGB<u8> = RGB { r: 0xF0, g: 0x80, b: 0x80 };
    static LIGHTCYAN:               RGB<u8> = RGB { r: 0xE0, g: 0xFF, b: 0xFF };
    static LIGHTGOLDENRODYELLOW:    RGB<u8> = RGB { r: 0xFA, g: 0xFA, b: 0xD2 };
    static LIGHTGREEN:              RGB<u8> = RGB { r: 0x90, g: 0xEE, b: 0x90 };
    static LIGHTGREY:               RGB<u8> = RGB { r: 0xD3, g: 0xD3, b: 0xD3 };
    static LIGHTPINK:               RGB<u8> = RGB { r: 0xFF, g: 0xB6, b: 0xC1 };
    static LIGHTSALMON:             RGB<u8> = RGB { r: 0xFF, g: 0xA0, b: 0x7A };
    static LIGHTSEAGREEN:           RGB<u8> = RGB { r: 0x20, g: 0xB2, b: 0xAA };
    static LIGHTSKYBLUE:            RGB<u8> = RGB { r: 0x87, g: 0xCE, b: 0xFA };
    static LIGHTSLATEGRAY:          RGB<u8> = RGB { r: 0x77, g: 0x88, b: 0x99 };
    static LIGHTSTEELBLUE:          RGB<u8> = RGB { r: 0xB0, g: 0xC4, b: 0xDE };
    static LIGHTYELLOW:             RGB<u8> = RGB { r: 0xFF, g: 0xFF, b: 0xE0 };
    static LIME:                    RGB<u8> = RGB { r: 0x00, g: 0xFF, b: 0x00 };
    static LIMEGREEN:               RGB<u8> = RGB { r: 0x32, g: 0xCD, b: 0x32 };
    static LINEN:                   RGB<u8> = RGB { r: 0xFA, g: 0xF0, b: 0xE6 };
    static MAGENTA:                 RGB<u8> = RGB { r: 0xFF, g: 0x00, b: 0xFF };
    static MAROON:                  RGB<u8> = RGB { r: 0x80, g: 0x00, b: 0x00 };
    static MEDIUMAQUAMARINE:        RGB<u8> = RGB { r: 0x66, g: 0xCD, b: 0xAA };
    static MEDIUMBLUE:              RGB<u8> = RGB { r: 0x00, g: 0x00, b: 0xCD };
    static MEDIUMORCHID:            RGB<u8> = RGB { r: 0xBA, g: 0x55, b: 0xD3 };
    static MEDIUMPURPLE:            RGB<u8> = RGB { r: 0x93, g: 0x70, b: 0xDB };
    static MEDIUMSEAGREEN:          RGB<u8> = RGB { r: 0x3C, g: 0xB3, b: 0x71 };
    static MEDIUMSLATEBLUE:         RGB<u8> = RGB { r: 0x7B, g: 0x68, b: 0xEE };
    static MEDIUMSPRINGGREEN:       RGB<u8> = RGB { r: 0x00, g: 0xFA, b: 0x9A };
    static MEDIUMTURQUOISE:         RGB<u8> = RGB { r: 0x48, g: 0xD1, b: 0xCC };
    static MEDIUMVIOLETRED:         RGB<u8> = RGB { r: 0xC7, g: 0x15, b: 0x85 };
    static MIDNIGHTBLUE:            RGB<u8> = RGB { r: 0x19, g: 0x19, b: 0x70 };
    static MINTCREAM:               RGB<u8> = RGB { r: 0xF5, g: 0xFF, b: 0xFA };
    static MISTYROSE:               RGB<u8> = RGB { r: 0xFF, g: 0xE4, b: 0xE1 };
    static MOCCASIN:                RGB<u8> = RGB { r: 0xFF, g: 0xE4, b: 0xB5 };
    static NAVAJOWHITE:             RGB<u8> = RGB { r: 0xFF, g: 0xDE, b: 0xAD };
    static NAVY:                    RGB<u8> = RGB { r: 0x00, g: 0x00, b: 0x80 };
    static OLDLACE:                 RGB<u8> = RGB { r: 0xFD, g: 0xF5, b: 0xE6 };
    static OLIVE:                   RGB<u8> = RGB { r: 0x80, g: 0x80, b: 0x00 };
    static OLIVEDRAB:               RGB<u8> = RGB { r: 0x6B, g: 0x8E, b: 0x23 };
    static ORANGE:                  RGB<u8> = RGB { r: 0xFF, g: 0xA5, b: 0x00 };
    static ORANGERED:               RGB<u8> = RGB { r: 0xFF, g: 0x45, b: 0x00 };
    static ORCHID:                  RGB<u8> = RGB { r: 0xDA, g: 0x70, b: 0xD6 };
    static PALEGOLDENROD:           RGB<u8> = RGB { r: 0xEE, g: 0xE8, b: 0xAA };
    static PALEGREEN:               RGB<u8> = RGB { r: 0x98, g: 0xFB, b: 0x98 };
    static PALEVIOLETRED:           RGB<u8> = RGB { r: 0xDB, g: 0x70, b: 0x93 };
    static PAPAYAWHIP:              RGB<u8> = RGB { r: 0xFF, g: 0xEF, b: 0xD5 };
    static PEACHPUFF:               RGB<u8> = RGB { r: 0xFF, g: 0xDA, b: 0xB9 };
    static PERU:                    RGB<u8> = RGB { r: 0xCD, g: 0x85, b: 0x3F };
    static PINK:                    RGB<u8> = RGB { r: 0xFF, g: 0xC0, b: 0xCB };
    static PLUM:                    RGB<u8> = RGB { r: 0xDD, g: 0xA0, b: 0xDD };
    static POWDERBLUE:              RGB<u8> = RGB { r: 0xB0, g: 0xE0, b: 0xE6 };
    static PURPLE:                  RGB<u8> = RGB { r: 0x80, g: 0x00, b: 0x80 };
    static RED:                     RGB<u8> = RGB { r: 0xFF, g: 0x00, b: 0x00 };
    static ROSYBROWN:               RGB<u8> = RGB { r: 0xBC, g: 0x8F, b: 0x8F };
    static ROYALBLUE:               RGB<u8> = RGB { r: 0x41, g: 0x69, b: 0xE1 };
    static SADDLEBROWN:             RGB<u8> = RGB { r: 0x8B, g: 0x45, b: 0x13 };
    static SALMON:                  RGB<u8> = RGB { r: 0xFA, g: 0x80, b: 0x72 };
    static SANDYBROWN:              RGB<u8> = RGB { r: 0xFA, g: 0xA4, b: 0x60 };
    static SEAGREEN:                RGB<u8> = RGB { r: 0x2E, g: 0x8B, b: 0x57 };
    static SEASHELL:                RGB<u8> = RGB { r: 0xFF, g: 0xF5, b: 0xEE };
    static SIENNA:                  RGB<u8> = RGB { r: 0xA0, g: 0x52, b: 0x2D };
    static SILVER:                  RGB<u8> = RGB { r: 0xC0, g: 0xC0, b: 0xC0 };
    static SKYBLUE:                 RGB<u8> = RGB { r: 0x87, g: 0xCE, b: 0xEB };
    static SLATEBLUE:               RGB<u8> = RGB { r: 0x6A, g: 0x5A, b: 0xCD };
    static SLATEGRAY:               RGB<u8> = RGB { r: 0x70, g: 0x80, b: 0x90 };
    static SNOW:                    RGB<u8> = RGB { r: 0xFF, g: 0xFA, b: 0xFA };
    static SPRINGGREEN:             RGB<u8> = RGB { r: 0x00, g: 0xFF, b: 0x7F };
    static STEELBLUE:               RGB<u8> = RGB { r: 0x46, g: 0x82, b: 0xB4 };
    static TAN:                     RGB<u8> = RGB { r: 0xD2, g: 0xB4, b: 0x8C };
    static TEAL:                    RGB<u8> = RGB { r: 0x00, g: 0x80, b: 0x80 };
    static THISTLE:                 RGB<u8> = RGB { r: 0xD8, g: 0xBF, b: 0xD8 };
    static TOMATO:                  RGB<u8> = RGB { r: 0xFF, g: 0x63, b: 0x47 };
    static TURQUOISE:               RGB<u8> = RGB { r: 0x40, g: 0xE0, b: 0xD0 };
    static VIOLET:                  RGB<u8> = RGB { r: 0xEE, g: 0x82, b: 0xEE };
    static WHEAT:                   RGB<u8> = RGB { r: 0xF5, g: 0xDE, b: 0xB3 };
    static WHITE:                   RGB<u8> = RGB { r: 0xFF, g: 0xFF, b: 0xFF };
    static WHITESMOKE:              RGB<u8> = RGB { r: 0xF5, g: 0xF5, b: 0xF5 };
    static YELLOW:                  RGB<u8> = RGB { r: 0xFF, g: 0xFF, b: 0x00 };
    static YELLOWGREEN:             RGB<u8> = RGB { r: 0x9A, g: 0xCD, b: 0x32 };
}

#[cfg(test)]
mod tests {
    use color::*;

    #[test]
    fn test_rgb_to_rgb() {
        assert_eq!(RGB::new::<u8>(0xA0, 0xA0, 0xA0).to_rgb::<u8>(), RGB::new::<u8>(0xA0, 0xA0, 0xA0));
        assert_eq!(RGB::new::<u8>(0xA0, 0xA0, 0xA0).to_rgb::<u16>(), RGB::new::<u16>(0xA0A0, 0xA0A0, 0xA0A0));
    }

    #[test]
    fn test_rgb_to_hsv() {
        assert_eq!(RGB::new::<u8>(0xFF, 0xFF, 0xFF).to_hsv::<f32>(), HSV::new::<f32>(0.0, 0.0, 1.0));
        assert_eq!(RGB::new::<u8>(0x99, 0x00, 0x00).to_hsv::<f32>(), HSV::new::<f32>(0.0, 1.0, 0.6));
        assert_eq!(RGB::new::<u8>(0x00, 0x99, 0x00).to_hsv::<f32>(), HSV::new::<f32>(120.0, 1.0, 0.6));
        assert_eq!(RGB::new::<u8>(0x00, 0x00, 0x99).to_hsv::<f32>(), HSV::new::<f32>(240.0, 1.0, 0.6));
    }

    #[test]
    fn test_tuple_to_rgba() {
        assert_eq!((RGB::new::<f64>(1.0, 1.0, 1.0), 0xFFu8).to_rgba::<f32>(), RGBA::new::<f32>(1.0, 1.0, 1.0, 1.0));
        assert_eq!((RGB::new::<f64>(1.0, 1.0, 1.0), 0xFFu8).to_rgba::<f32>(), RGBA::new::<f32>(1.0, 1.0, 1.0, 1.0));
        assert_eq!((RGB::new::<f64>(1.0, 1.0, 1.0), 0xFFu8).to_rgba::<f32>(), RGBA::new::<f32>(1.0, 1.0, 1.0, 1.0));
        assert_eq!((RGB::new::<f64>(1.0, 1.0, 1.0), 0xFFu8).to_rgba::<f32>(), RGBA::new::<f32>(1.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn test_rgba_to_rgba() {
        assert_eq!(RGBA::new::<u8>(0xA0, 0xA0, 0xA0, 0xA0).to_rgba::<u8>(), RGBA::new::<u8>(0xA0, 0xA0, 0xA0, 0xA0));
        assert_eq!(RGBA::new::<u8>(0xA0, 0xA0, 0xA0, 0xA0).to_rgba::<u16>(), RGBA::new::<u16>(0xA0A0, 0xA0A0, 0xA0A0, 0xA0A0));
    }

    #[test]
    fn test_rgba_to_hsva() {
        assert_eq!(RGBA::new::<u8>(0xFF, 0xFF, 0xFF, 0xFF).to_hsva::<f32>(), HSVA::new::<f32>(0.0, 0.0, 1.0, 1.0));
        assert_eq!(RGBA::new::<u8>(0x99, 0x00, 0x00, 0xFF).to_hsva::<f32>(), HSVA::new::<f32>(0.0, 1.0, 0.6, 1.0));
        assert_eq!(RGBA::new::<u8>(0x00, 0x99, 0x00, 0xFF).to_hsva::<f32>(), HSVA::new::<f32>(120.0, 1.0, 0.6, 1.0));
        assert_eq!(RGBA::new::<u8>(0x00, 0x00, 0x99, 0xFF).to_hsva::<f32>(), HSVA::new::<f32>(240.0, 1.0, 0.6, 1.0));
    }
}
