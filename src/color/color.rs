use core::cast::transmute;
use core::cmp::{Eq, Ord};
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::vec::raw::buf_as_slice;

use angle::Degrees;
use channel::Channel;
use dim::{Dimensional, ToPtr};
use funs::common::Sign;
use num::cast::{cast, NumCast};
use num::ext::Float;


pub trait Color<T>: Dimensional<T>, ToPtr<T>, Eq {
    pure fn inverse(&self) -> self;
    
    pure fn to_rgb_u8(&self) -> RGB<u8>;
    pure fn to_rgb_u16(&self) -> RGB<u16>;
    pure fn to_rgb_u32(&self) -> RGB<u32>;
    pure fn to_rgb_u64(&self) -> RGB<u64>;
    pure fn to_rgb_f32(&self) -> RGB<f32>;
    pure fn to_rgb_f64(&self) -> RGB<f64>;
    
    pure fn to_hsv_f32(&self) -> HSV<f32>;
    pure fn to_hsv_f64(&self) -> HSV<f64>;
}

// pub trait ColorRGB<T> {
//     static pure fn from_hex(hex: u8) -> self;
// }

pub trait Color3<T>: Color<T> {
    pure fn to_rgba_u8(&self, a: u8) -> RGBA<u8>;
    pure fn to_rgba_u16(&self, a: u16) -> RGBA<u16>;
    pure fn to_rgba_u32(&self, a: u32) -> RGBA<u32>;
    pure fn to_rgba_u64(&self, a: u64) -> RGBA<u64>;
    pure fn to_rgba_f32(&self, a: f32) -> RGBA<f32>;
    pure fn to_rgba_f64(&self, a: f64) -> RGBA<f64>;
    
    pure fn to_hsva_f32(&self, a: f32) -> HSVA<f32>;
    pure fn to_hsva_f64(&self, a: f64) -> HSVA<f64>;
}

pub trait Color4<T>: Color<T> {
    pure fn to_rgba_u8(&self) -> RGBA<u8>;
    pure fn to_rgba_u16(&self) -> RGBA<u16>;
    pure fn to_rgba_u32(&self) -> RGBA<u32>;
    pure fn to_rgba_u64(&self) -> RGBA<u64>;
    pure fn to_rgba_f32(&self) -> RGBA<f32>;
    pure fn to_rgba_f64(&self) -> RGBA<f64>;
    
    pure fn to_hsva_f32(&self) -> HSVA<f32>;
    pure fn to_hsva_f64(&self) -> HSVA<f64>;
}




/**
 * A generic rgb to hsv conversion
 *
 * Assumes that T is a floating point type
 *
 * TODO: Use some sort of 'Float' trait bound to make this safer
 */
#[inline(always)]
pub pure fn to_hsv<T:Copy Num NumCast Eq Ord>(color: &RGB<T>) -> HSV<T> {
    // Algorithm taken from the Wikipedia article on HSL and HSV:
    // http://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV
    
    let _0 = cast(0);
    
    let mx = [color.r, color.g, color.b].max();
    let mn = [color.r, color.g, color.b].min();
    let chr = mx - mn;
    
    if chr != cast(0) {
        let h = Degrees(if      color.r == mx   { ((color.g - color.b) / chr) % cast(6) }
                        else if color.g == mx   { ((color.b - color.r) / chr) + cast(2) }
                        else /* color.b == mx */{ ((color.r - color.g) / chr) + cast(4) } * cast(60));
        
        let s = chr / mx;
        
        HSV::new(h, s, mx)
        
    } else {
        HSV::new(Degrees(_0), _0, mx)
    }
}

/**
 * A generic hsv to rgb conversion
 *
 * Assumes that T is a floating point type
 *
 * TODO: Use some sort of 'Float' trait bound to make this safer
 */
#[inline(always)]
pub pure fn to_rgb<T:Copy Num NumCast Sign Eq Ord>(color: &HSV<T>) -> RGB<T> {
    // Algorithm taken from the Wikipedia article on HSL and HSV:
    // http://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV
    
    let _0: T = cast(0);
    let _1: T = cast(1);
    let _2: T = cast(2);
    
    let chr = color.v * color.s;
    let h_ = *(color.h) / cast(60);  // TODO: it'd be nice if Degrees / Degrees returned a scalar
    
    // the 2nd largest component
    let x = chr * (_1 - ((h_ % _2) - _1).abs());
    
    
    let mut color_rgb =
        if      h_ < cast(1) { RGB::new(chr,   x,  _0) }
        else if h_ < cast(2) { RGB::new(  x, chr,  _0) }
        else if h_ < cast(3) { RGB::new( _0, chr,   x) }
        else if h_ < cast(4) { RGB::new( _0,   x, chr) }
        else if h_ < cast(5) { RGB::new(  x,  _0, chr) }
        else if h_ < cast(6) { RGB::new(chr,  _0,   x) }
        else                 { RGB::new( _0,  _0,  _0) };
    
    // match the value by adding the same amount to each component
    let mn = color.v - chr;
    
    color_rgb.r += mn;
    color_rgb.g += mn;
    color_rgb.b += mn;
    
    return color_rgb;
}




pub struct RGB<T> { r: T, g: T, b: T }

pub impl<T:Copy> RGB<T> {
    #[inline(always)]
    static pure fn new(r: T, g: T, b: T) -> RGB<T> {
        RGB { r: move r, g: move g, b: move b }
    }
}

pub impl<T> RGB<T>: Dimensional<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 3 }
    
    #[inline(always)]
    static pure fn size_of() -> uint {
        size_of::<RGB<T>>()
    }
}

pub impl<T:Copy> RGB<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*RGB<T>, *T>(
                to_unsafe_ptr(&self)), 3) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy> RGB<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        ptr::to_unsafe_ptr(&self[0])
    }
}

pub impl<T:Copy Num NumCast Channel Eq Ord> RGB<T>: Color<T> {
    #[inline(always)]
    pure fn inverse(&self) -> RGB<T> {
        RGB::new(self.r.inverse(),
                 self.g.inverse(),
                 self.b.inverse())
    }
    
    #[inline(always)]
    pure fn to_rgb_u8(&self) -> RGB<u8> {
        RGB::new(self.r.to_channel_u8(),
                 self.g.to_channel_u8(),
                 self.b.to_channel_u8())
    }
    
    #[inline(always)]
    pure fn to_rgb_u16(&self) -> RGB<u16> {
        RGB::new(self.r.to_channel_u16(),
                 self.g.to_channel_u16(),
                 self.b.to_channel_u16())
    }
    
    #[inline(always)]
    pure fn to_rgb_u32(&self) -> RGB<u32> {
        RGB::new(self.r.to_channel_u32(),
                 self.g.to_channel_u32(),
                 self.b.to_channel_u32())
    }
    
    #[inline(always)]
    pure fn to_rgb_u64(&self) -> RGB<u64> {
        RGB::new(self.r.to_channel_u64(),
                 self.g.to_channel_u64(),
                 self.b.to_channel_u64())
    }
    
    #[inline(always)]
    pure fn to_rgb_f32(&self) -> RGB<f32> {
        RGB::new(self.r.to_channel_f32(),
                 self.g.to_channel_f32(),
                 self.b.to_channel_f32())
    }
    
    #[inline(always)]
    pure fn to_rgb_f64(&self) -> RGB<f64> {
        RGB::new(self.r.to_channel_f64(),
                 self.g.to_channel_f64(),
                 self.b.to_channel_f64())
    }
    
    #[inline(always)] pure fn to_hsv_f32(&self) -> HSV<f32> { to_hsv(&self.to_rgb_f32()) }
    #[inline(always)] pure fn to_hsv_f64(&self) -> HSV<f64> { to_hsv(&self.to_rgb_f64()) }
}

pub impl<T:Copy Num NumCast Channel Eq Ord> RGB<T>: Color3<T> {
    #[inline(always)] pure fn to_rgba_u8(&self, a: u8)   -> RGBA<u8>  { RGBA::from_rgb_a(&self.to_rgb_u8(),  a) }
    #[inline(always)] pure fn to_rgba_u16(&self, a: u16) -> RGBA<u16> { RGBA::from_rgb_a(&self.to_rgb_u16(), a) }
    #[inline(always)] pure fn to_rgba_u32(&self, a: u32) -> RGBA<u32> { RGBA::from_rgb_a(&self.to_rgb_u32(), a) }
    #[inline(always)] pure fn to_rgba_u64(&self, a: u64) -> RGBA<u64> { RGBA::from_rgb_a(&self.to_rgb_u64(), a) }
    #[inline(always)] pure fn to_rgba_f32(&self, a: f32) -> RGBA<f32> { RGBA::from_rgb_a(&self.to_rgb_f32(), a) }
    #[inline(always)] pure fn to_rgba_f64(&self, a: f64) -> RGBA<f64> { RGBA::from_rgb_a(&self.to_rgb_f64(), a) }
    
    #[inline(always)] pure fn to_hsva_f32(&self, a: f32) -> HSVA<f32> { HSVA::from_hsv_a(&self.to_hsv_f32(), a) }
    #[inline(always)] pure fn to_hsva_f64(&self, a: f64) -> HSVA<f64> { HSVA::from_hsv_a(&self.to_hsv_f64(), a) }
}

pub impl<T:Copy Eq> RGB<T>: Eq {
    pure fn eq(&self, other: &RGB<T>) -> bool {
        self.r == other.r &&
        self.g == other.g &&
        self.b == other.b
    }
    
    pure fn ne(&self, other: &RGB<T>) -> bool {
        !(self == other)
    }
}





pub struct RGBA<T> { r: T, g: T, b: T, a: T }

pub impl<T:Copy> RGBA<T> {
    #[inline(always)]
    static pure fn new(r: T, g: T, b: T, a: T) -> RGBA<T> {
        RGBA { r: move r, g: move g, b: move b, a: move a }
    }
    
    #[inline(always)]
    static pure fn from_rgb_a(rgb: &RGB<T>, a: T) -> RGBA<T> {
        RGBA::new(rgb.r, rgb.g, rgb.b, move a)
    }
}

pub impl<T> RGBA<T>: Dimensional<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    static pure fn size_of() -> uint {
        size_of::<RGBA<T>>()
    }
}

pub impl<T:Copy> RGBA<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*RGBA<T>, *T>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy> RGBA<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        ptr::to_unsafe_ptr(&self[0])
    }
}

pub impl<T:Copy Num NumCast Channel Eq Ord> RGBA<T>: Color<T> {
    #[inline(always)]
    pure fn inverse(&self) -> RGBA<T> {
        RGBA::new(self.r.inverse(),
                  self.g.inverse(),
                  self.b.inverse(),
                  self.a.inverse())
    }
    
    #[inline(always)]
    pure fn to_rgb_u8(&self) -> RGB<u8> {
        RGB::new(self.r.to_channel_u8(),
                 self.g.to_channel_u8(),
                 self.b.to_channel_u8())
    }
    
    #[inline(always)]
    pure fn to_rgb_u16(&self) -> RGB<u16> {
        RGB::new(self.r.to_channel_u16(),
                 self.g.to_channel_u16(),
                 self.b.to_channel_u16())
    }
    
    #[inline(always)]
    pure fn to_rgb_u32(&self) -> RGB<u32> {
        RGB::new(self.r.to_channel_u32(),
                 self.g.to_channel_u32(),
                 self.b.to_channel_u32())
    }
    
    #[inline(always)]
    pure fn to_rgb_u64(&self) -> RGB<u64> {
        RGB::new(self.r.to_channel_u64(),
                 self.g.to_channel_u64(),
                 self.b.to_channel_u64())
    }
    
    #[inline(always)]
    pure fn to_rgb_f32(&self) -> RGB<f32> {
        RGB::new(self.r.to_channel_f32(),
                 self.g.to_channel_f32(),
                 self.b.to_channel_f32())
    }
    
    #[inline(always)]
    pure fn to_rgb_f64(&self) -> RGB<f64> {
        RGB::new(self.r.to_channel_f64(),
                 self.g.to_channel_f64(),
                 self.b.to_channel_f64())
    }
    
    #[inline(always)] pure fn to_hsv_f32(&self) -> HSV<f32> { to_hsv(&self.to_rgb_f32()) }
    #[inline(always)] pure fn to_hsv_f64(&self) -> HSV<f64> { to_hsv(&self.to_rgb_f64()) }
}

pub impl<T:Copy Num NumCast Channel Eq Ord> RGBA<T>: Color4<T> {
    #[inline(always)] pure fn to_rgba_u8(&self)  -> RGBA<u8>  { RGBA::from_rgb_a(&self.to_rgb_u8(),  self.a.to_channel_u8()) }
    #[inline(always)] pure fn to_rgba_u16(&self) -> RGBA<u16> { RGBA::from_rgb_a(&self.to_rgb_u16(), self.a.to_channel_u16()) }
    #[inline(always)] pure fn to_rgba_u32(&self) -> RGBA<u32> { RGBA::from_rgb_a(&self.to_rgb_u32(), self.a.to_channel_u32()) }
    #[inline(always)] pure fn to_rgba_u64(&self) -> RGBA<u64> { RGBA::from_rgb_a(&self.to_rgb_u64(), self.a.to_channel_u64()) }
    #[inline(always)] pure fn to_rgba_f32(&self) -> RGBA<f32> { RGBA::from_rgb_a(&self.to_rgb_f32(), self.a.to_channel_f32()) }
    #[inline(always)] pure fn to_rgba_f64(&self) -> RGBA<f64> { RGBA::from_rgb_a(&self.to_rgb_f64(), self.a.to_channel_f64()) }
    
    #[inline(always)] pure fn to_hsva_f32(&self) -> HSVA<f32> { HSVA::from_hsv_a(&self.to_hsv_f32(), self.a.to_channel_f32()) }
    #[inline(always)] pure fn to_hsva_f64(&self) -> HSVA<f64> { HSVA::from_hsv_a(&self.to_hsv_f64(), self.a.to_channel_f64()) }
}

pub impl<T:Copy Eq> RGBA<T>: Eq {
    pure fn eq(&self, other: &RGBA<T>) -> bool {
        self.r == other.r &&
        self.g == other.g &&
        self.b == other.b &&
        self.a == other.a
    }
    
    pure fn ne(&self, other: &RGBA<T>) -> bool {
        !(self == other)
    }
}






pub struct HSV<T> { h: Degrees<T>, s: T, v: T }

pub impl<T:Copy> HSV<T> {
    static pure fn new(h: Degrees<T>, s: T, v: T) -> HSV<T> {
        HSV { h: move h, s: move s, v: move v }
    }
}

pub impl<T> HSV<T>: Dimensional<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 3 }
    
    #[inline(always)]
    static pure fn size_of() -> uint {
        size_of::<HSV<T>>()
    }
}

pub impl<T:Copy> HSV<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*HSV<T>, *T>(
                to_unsafe_ptr(&self)), 3) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy> HSV<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        ptr::to_unsafe_ptr(&self[0])
    }
}

pub impl<T:Copy Float Channel> HSV<T>: Color<T> {
    #[inline(always)]
    pure fn inverse(&self) -> HSV<T> {
        HSV::new(self.h.opposite(),
                 self.s.inverse(),
                 self.v.inverse())
    }
    
    #[inline(always)] pure fn to_rgb_u8(&self)  -> RGB<u8>  { to_rgb(&self.to_hsv_f32()).to_rgb_u8()  }
    #[inline(always)] pure fn to_rgb_u16(&self) -> RGB<u16> { to_rgb(&self.to_hsv_f32()).to_rgb_u16() }
    #[inline(always)] pure fn to_rgb_u32(&self) -> RGB<u32> { to_rgb(&self.to_hsv_f32()).to_rgb_u32() }
    #[inline(always)] pure fn to_rgb_u64(&self) -> RGB<u64> { to_rgb(&self.to_hsv_f32()).to_rgb_u64() }
    #[inline(always)] pure fn to_rgb_f32(&self) -> RGB<f32> { to_rgb(&self.to_hsv_f32()).to_rgb_f32() }
    #[inline(always)] pure fn to_rgb_f64(&self) -> RGB<f64> { to_rgb(&self.to_hsv_f64()).to_rgb_f64() }
    
    #[inline(always)]
    pure fn to_hsv_f32(&self) -> HSV<f32> {
        HSV::new(Degrees((*self.h).to_f32()),
                 self.s.to_channel_f32(),
                 self.v.to_channel_f32())
    }
    
    #[inline(always)]
    pure fn to_hsv_f64(&self) -> HSV<f64> {
        HSV::new(Degrees((*self.h).to_f64()),
                 self.s.to_channel_f64(),
                 self.v.to_channel_f64())
    }
}

pub impl<T:Copy Float Channel> HSV<T>: Color3<T> {
    #[inline(always)] pure fn to_rgba_u8(&self, a: u8)   -> RGBA<u8>  { RGBA::from_rgb_a(&self.to_rgb_u8(),  a) }
    #[inline(always)] pure fn to_rgba_u16(&self, a: u16) -> RGBA<u16> { RGBA::from_rgb_a(&self.to_rgb_u16(), a) }
    #[inline(always)] pure fn to_rgba_u32(&self, a: u32) -> RGBA<u32> { RGBA::from_rgb_a(&self.to_rgb_u32(), a) }
    #[inline(always)] pure fn to_rgba_u64(&self, a: u64) -> RGBA<u64> { RGBA::from_rgb_a(&self.to_rgb_u64(), a) }
    #[inline(always)] pure fn to_rgba_f32(&self, a: f32) -> RGBA<f32> { RGBA::from_rgb_a(&self.to_rgb_f32(), a) }
    #[inline(always)] pure fn to_rgba_f64(&self, a: f64) -> RGBA<f64> { RGBA::from_rgb_a(&self.to_rgb_f64(), a) }
    
    #[inline(always)] pure fn to_hsva_f32(&self, a: f32) -> HSVA<f32> { HSVA::from_hsv_a(&self.to_hsv_f32(), a) }
    #[inline(always)] pure fn to_hsva_f64(&self, a: f64) -> HSVA<f64> { HSVA::from_hsv_a(&self.to_hsv_f64(), a) }
}

pub impl<T:Copy Float> HSV<T>: Eq {
    pure fn eq(&self, other: &HSV<T>) -> bool {
        self.h == other.h &&
        self.s == other.s &&
        self.v == other.v
    }
    
    pure fn ne(&self, other: &HSV<T>) -> bool {
        !(self == other)
    }
}





pub struct HSVA<T> { h: Degrees<T>, s: T, v: T, a: T }

pub impl<T:Copy> HSVA<T> {
    #[inline(always)]
    static pure fn new(h: Degrees<T>, s: T, v: T, a: T) -> HSVA<T> {
        HSVA { h: move h, s: move s, v: move v, a: move a }
    }
    
    #[inline(always)]
    static pure fn from_hsv_a(hsv: &HSV<T>, a: T) -> HSVA<T> {
        HSVA::new(hsv.h, hsv.s, hsv.v, move a)
    }
}

pub impl<T> HSVA<T>: Dimensional<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    static pure fn size_of() -> uint {
        size_of::<HSVA<T>>()
    }
}

pub impl<T:Copy> HSVA<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*HSVA<T>, *T>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy> HSVA<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        ptr::to_unsafe_ptr(&self[0])
    }
}

pub impl<T:Copy Float Channel> HSVA<T>: Color<T> {
    #[inline(always)]
    pure fn inverse(&self) -> HSVA<T> {
        HSVA::new(self.h.opposite(),
                  self.s.inverse(),
                  self.v.inverse(),
                  self.a.inverse())
    }
    
    #[inline(always)] pure fn to_rgb_u8(&self)  -> RGB<u8>  { to_rgb(&self.to_hsv_f32()).to_rgb_u8()  }
    #[inline(always)] pure fn to_rgb_u16(&self) -> RGB<u16> { to_rgb(&self.to_hsv_f32()).to_rgb_u16() }
    #[inline(always)] pure fn to_rgb_u32(&self) -> RGB<u32> { to_rgb(&self.to_hsv_f32()).to_rgb_u32() }
    #[inline(always)] pure fn to_rgb_u64(&self) -> RGB<u64> { to_rgb(&self.to_hsv_f32()).to_rgb_u64() }
    #[inline(always)] pure fn to_rgb_f32(&self) -> RGB<f32> { to_rgb(&self.to_hsv_f32()).to_rgb_f32() }
    #[inline(always)] pure fn to_rgb_f64(&self) -> RGB<f64> { to_rgb(&self.to_hsv_f64()).to_rgb_f64() }
    
    #[inline(always)]
    pure fn to_hsv_f32(&self) -> HSV<f32> {
        HSV::new(Degrees((*self.h).to_f32()),
                 self.s.to_channel_f32(),
                 self.v.to_channel_f32())
    }
    
    #[inline(always)]
    pure fn to_hsv_f64(&self) -> HSV<f64> {
        HSV::new(Degrees((*self.h).to_f64()),
                 self.s.to_channel_f64(),
                 self.v.to_channel_f64())
    }
}

pub impl<T:Copy Float Channel> HSVA<T>: Color4<T> {
    #[inline(always)] pure fn to_rgba_u8(&self)  -> RGBA<u8>  { RGBA::from_rgb_a(&self.to_rgb_u8(), self.a.to_channel_u8()) }
    #[inline(always)] pure fn to_rgba_u16(&self) -> RGBA<u16> { RGBA::from_rgb_a(&self.to_rgb_u16(), self.a.to_channel_u16()) }
    #[inline(always)] pure fn to_rgba_u32(&self) -> RGBA<u32> { RGBA::from_rgb_a(&self.to_rgb_u32(), self.a.to_channel_u32()) }
    #[inline(always)] pure fn to_rgba_u64(&self) -> RGBA<u64> { RGBA::from_rgb_a(&self.to_rgb_u64(), self.a.to_channel_u64()) }
    #[inline(always)] pure fn to_rgba_f32(&self) -> RGBA<f32> { RGBA::from_rgb_a(&self.to_rgb_f32(), self.a.to_channel_f32()) }
    #[inline(always)] pure fn to_rgba_f64(&self) -> RGBA<f64> { RGBA::from_rgb_a(&self.to_rgb_f64(), self.a.to_channel_f64()) }
    
    #[inline(always)] pure fn to_hsva_f32(&self) -> HSVA<f32> { HSVA::from_hsv_a(&self.to_hsv_f32(), self.a.to_channel_f32()) }
    #[inline(always)] pure fn to_hsva_f64(&self) -> HSVA<f64> { HSVA::from_hsv_a(&self.to_hsv_f64(), self.a.to_channel_f64()) }
}

pub impl<T:Copy Float> HSVA<T>: Eq {
    pure fn eq(&self, other: &HSVA<T>) -> bool {
        self.h == other.h &&
        self.s == other.s &&
        self.v == other.v &&
        self.a == other.a
    }
    
    pure fn ne(&self, other: &HSVA<T>) -> bool {
        !(self == other)
    }
}