use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::vec::raw::buf_as_slice;

use angle::Degrees;
use channel::Channel;
use dim::{Dimensional, ToPtr};
use funs::common::Sign;
use num::kinds::{Float, Number};

/**
 * A generic color trait.
 */
pub trait Color<T>: Dimensional<T>, ToPtr<T>, Eq {
    /**
     * # Return value
     *
     * The color with each component inverted
     */
    pure fn inverse(&self) -> self;
    
    /**
     * Convert the color to a `RGB<u8>`
     *
     * # Returns
     *
     * The color as a `RGB<u8>` color with components ranging from
     * `0x00u8` to `0xFFu8`
     */
    pure fn to_rgb_u8(&self) -> RGB<u8>;
    
    /**
     * Convert the color to a `RGB<u16>`
     *
     * # Returns
     *
     * The color as a `RGB<u16>` color with components ranging from
     * `0x0000u16` to `0xFFFFu16`
     */
    pure fn to_rgb_u16(&self) -> RGB<u16>;
    
    /**
     * Convert the color to a `RGB<u32>`
     *
     * # Returns
     *
     * The color as a `RGB<u32>` color with components ranging from
     * `0x0000_0000_u32` to `0xFFFF_FFFF_u32`
     */
    pure fn to_rgb_u32(&self) -> RGB<u32>;
    
    /**
     * Convert the color to a `RGB<u64>`
     *
     * # Returns
     *
     * The color as a `RGB<u32>` color with components ranging from
     * `0x0000_0000_u32` to `0xFFFF_FFFF_u32`
     */
    pure fn to_rgb_u64(&self) -> RGB<u64>;
    
    /**
     * Convert the color to a `RGB<f32>`
     *
     * # Returns
     *
     * The color as a `RGB<f32>` color with components ranging from
     * `0f32` to `1f32`
     */
    pure fn to_rgb_f32(&self) -> RGB<f32>;
    
    /**
     * Convert the color to a `RGB<f64>`
     *
     * # Returns
     *
     * The color as a `RGB<f64>` color with components ranging from
     * `0f64` to `1f64`
     */
    pure fn to_rgb_f64(&self) -> RGB<f64>;
    
    
    /**
     * Convert the color to a `HSV<f32>`
     *
     * # Returns
     *
     * The color as a `HSV<f32>` with the `h` component as a `f32` angle and
     * saturation and value components ranging from `0f32` to `1f32`
     */
    pure fn to_hsv_f32(&self) -> HSV<f32>;
    
    /**
     * Convert the color to a `HSV<f64>`
     *
     * # Returns
     *
     * The color as a `HSV<f64>` with the `h` component as a `f64` angle and
     * saturation and value components ranging from `0f64` to `1f64`
     */
    pure fn to_hsv_f64(&self) -> HSV<f64>;
}

pub trait MutableColor<T>: Color<T> {
    /**
     * Get a mutable reference to the component at `i`
     */
    fn index_mut(&mut self, i: uint) -> &self/mut T;
    
    /**
     * Swap two components of the color in place
     */
    fn swap(&mut self, a: uint, b: uint);
    
    /**
     * Invert each component of the color
     */
    fn invert_self(&mut self);
}

pub trait Color3<T>: Color<T> {
    // TODO: documentation (bleh, so much writing)
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
    // TODO: documentation (arrg...)
    pure fn to_rgba_u8(&self) -> RGBA<u8>;
    pure fn to_rgba_u16(&self) -> RGBA<u16>;
    pure fn to_rgba_u32(&self) -> RGBA<u32>;
    pure fn to_rgba_u64(&self) -> RGBA<u64>;
    pure fn to_rgba_f32(&self) -> RGBA<f32>;
    pure fn to_rgba_f64(&self) -> RGBA<f64>;
    
    pure fn to_hsva_f32(&self) -> HSVA<f32>;
    pure fn to_hsva_f64(&self) -> HSVA<f64>;
}

// TODO!!!
// pub trait ColorRGB<T> {
//     static pure fn from_hex(hex: u8) -> self;
// }




/**
 * RGB to HSV conversion
 */
#[inline(always)]
pub pure fn to_hsv<T:Copy Float>(color: &RGB<T>) -> HSV<T> {
    // Algorithm taken from the Wikipedia article on HSL and HSV:
    // http://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV
    
    let _0 = Number::from(0f);
    
    let mx = [color.r, color.g, color.b].max();
    let mn = [color.r, color.g, color.b].min();
    let chr = mx - mn;
    
    if chr != Number::from(0f) {
        let h = Degrees(
            if      color.r == mx   { ((color.g - color.b) / chr) % Number::from(6f) }
            else if color.g == mx   { ((color.b - color.r) / chr) + Number::from(2f) }
            else /* color.b == mx */{ ((color.r - color.g) / chr) + Number::from(4f) }
        * Number::from(60f));
        
        let s = chr / mx;
        
        HSV::new(h, s, mx)
        
    } else {
        HSV::new(Degrees(_0), _0, mx)
    }
}

/**
 * HSV to RGB conversion
 */
#[inline(always)]
pub pure fn to_rgb<T:Copy Float Sign>(color: &HSV<T>) -> RGB<T> {
    // Algorithm taken from the Wikipedia article on HSL and HSV:
    // http://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV
    
    let _0: T = Number::from(0f);
    let _1: T = Number::from(1f);
    let _2: T = Number::from(2f);
    
    let chr = color.v * color.s;
    let h_ = (* color.h) / Number::from(60f);  // TODO: it'd be nice if Degrees / Degrees returned a scalar
    
    // the 2nd largest component
    let x = chr * (_1 - ((h_ % _2) - _1).abs());
    
    
    let mut color_rgb =
        if      h_ < Number::from(1f) { RGB::new(chr,   x,  _0) }
        else if h_ < Number::from(2f) { RGB::new(  x, chr,  _0) }
        else if h_ < Number::from(3f) { RGB::new( _0, chr,   x) }
        else if h_ < Number::from(4f) { RGB::new( _0,   x, chr) }
        else if h_ < Number::from(5f) { RGB::new(  x,  _0, chr) }
        else if h_ < Number::from(6f) { RGB::new(chr,  _0,   x) }
        else                          { RGB::new( _0,  _0,  _0) };
    
    // match the value by adding the same amount to each component
    let mn = color.v - chr;
    
    color_rgb.r += mn;
    color_rgb.g += mn;
    color_rgb.b += mn;
    
    return color_rgb;
}



/**
 *  A RGB color type (red, green, blue)
 *
 * # Type parameters
 *
 * * `T` - A color component which should be one of the following primitive
 *         types: `u8`, `u16`, `u32`, `u64`, `f32` or `f64`.
 *
 * # Fields
 *
 * * `r` - the red component
 * * `g` - the green component
 * * `b` - the blue component
 */
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
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 3) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> RGB<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*RGB<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T:Copy Number Channel> RGB<T>: Color<T> {
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

pub impl<T:Copy Channel> RGB<T>: MutableColor<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &self/mut T {
        match i {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 2, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        util::swap(self.index_mut(a),
                   self.index_mut(b));
    }
    
    #[inline(always)]
    fn invert_self(&mut self) {
        self.r = self.r.inverse();
        self.g = self.g.inverse();
        self.b = self.b.inverse();
    }
}

pub impl<T:Copy Number Channel> RGB<T>: Color3<T> {
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



/**
 *  A RGBA color type (red, green, blue, alpha)
 *
 * # Type parameters
 *
 * * `T` - A color component which should be one of the following primitive
 *         types: `u8`, `u16`, `u32`, `u64`, `f32` or `f64`.
 *
 * # Fields
 *
 * * `r` - the red component
 * * `g` - the green component
 * * `b` - the blue component
 * * `a` - the alpha component
 */
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
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 4) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> RGBA<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*RGBA<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T:Copy Number Channel> RGBA<T>: Color<T> {
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

pub impl<T:Copy Channel> RGBA<T>: MutableColor<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &self/mut T {
        match i {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 2, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        util::swap(self.index_mut(a),
                   self.index_mut(b));
    }
    
    #[inline(always)]
    fn invert_self(&mut self) {
        self.r = self.r.inverse();
        self.g = self.g.inverse();
        self.b = self.b.inverse();
        self.a = self.a.inverse();
    }
}

pub impl<T:Copy Number Channel> RGBA<T>: Color4<T> {
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



/**
 *  A HSV color type (hue, saturation, value)
 *
 * # Type parameters
 *
 * * `T` - A color component which should be either an `f32` or `f64`.
 *
 * # Fields
 *
 * * `h` - the hue component in degrees (from 0.0 to 360.0)
 * * `s` - the saturation component
 * * `v` - the value (brightness) component
 */
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
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 3) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> HSV<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*HSV<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
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

pub impl<T:Copy Channel Float> HSV<T>: MutableColor<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &self/mut T {
        match i {
            0 => fail(~"can't swap the hue component at index 0 in a HSVA type"),
            1 => &mut self.s,
            2 => &mut self.v,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 2, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        if a != 0 && b != 0 { fail(fmt!("can't swap the hue component (at index 0) in a HSV type: found a: %u, b: %u", a, b)); }
        util::swap(self.index_mut(a),
                   self.index_mut(b));
    }
    
    #[inline(always)]
    fn invert_self(&mut self) {
        self.h = self.h.opposite();
        self.s = self.s.inverse();
        self.v = self.v.inverse();
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



/**
 *  A HSVA color type (hue, saturation, value, alpha)
 *
 * # Type parameters
 *
 * * `T` - A color component which should be either an `f32` or `f64`.
 *
 * # Fields
 *
 * * `h` - the hue component in degrees (from 0.0 to 360.0)
 * * `s` - the saturation component
 * * `v` - the value (brightness) component
 * * `v` - the alpha component
 */
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
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 4) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> HSVA<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*HSVA<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
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

pub impl<T:Copy Channel Float> HSVA<T>: MutableColor<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &self/mut T {
        match i {
            0 => fail(~"can't swap the hue component at index 0 in a HSVA type"),
            1 => &mut self.s,
            2 => &mut self.v,
            3 => &mut self.a,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 2, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        util::swap(self.index_mut(a),
                   self.index_mut(b));
    }
    
    #[inline(always)]
    fn invert_self(&mut self) {
        self.h = self.h.opposite();
        self.s = self.s.inverse();
        self.v = self.v.inverse();
        self.a = self.a.inverse();
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
