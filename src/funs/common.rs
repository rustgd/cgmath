/**
 * Common Functions
 *
 * This module corresponds to Section 8.3 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */

use num::cast::cast;
use angle::{Radians, Degrees};
use vec::{Vec2, Vec3, Vec4};

pub trait Sign {
    pure fn abs() -> self;
    pure fn sign() -> self;
}

#[inline(always)] pub pure fn abs<T:Sign>(x: &T)  -> T { x.abs()  }
#[inline(always)] pub pure fn sign<T:Sign>(x: &T) -> T { x.sign() }

pub impl i8: Sign {
    #[inline(always)] pure fn abs()  -> i8 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i8 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl i16: Sign {
    #[inline(always)] pure fn abs()  -> i16 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i16 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl i32: Sign {
    #[inline(always)] pure fn abs()  -> i32 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i32 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl i64: Sign {
    #[inline(always)] pure fn abs()  -> i64 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i64 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl int: Sign {
    #[inline(always)] pure fn abs()  -> int { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> int { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl f32: Sign {
    #[inline(always)] pure fn abs()  -> f32 { if self >= 0f32 { self } else {-self } }
    #[inline(always)] pure fn sign() -> f32 { if self > 0f32 { 1f32 } else if self == 0f32 { 0f32 } else { -1f32 } }
}

pub impl f64: Sign {
    #[inline(always)] pure fn abs()  -> f64 { if self >= 0f64 { self } else {-self } }
    #[inline(always)] pure fn sign() -> f64 { if self > 0f64 { 1f64 } else if self == 0f64 { 0f64 } else { -1f64 } }
}

pub impl float: Sign {
    #[inline(always)] pure fn abs()  -> float { if self >= 0f { self } else {-self } }
    #[inline(always)] pure fn sign() -> float { if self > 0f { 1f } else if self == 0f { 0f } else { -1f } }
}


pub impl<T:Copy Sign> Radians<T>: Sign{
    #[inline(always)] pure fn abs()  -> Radians<T> { Radians(abs(&*self)) }
    #[inline(always)] pure fn sign() -> Radians<T> { Radians(sign(&*self)) }
}

pub impl<T:Copy Sign> Degrees<T>: Sign{
    #[inline(always)] pure fn abs()  -> Degrees<T> { Degrees(abs(&*self)) }
    #[inline(always)] pure fn sign() -> Degrees<T> { Degrees(sign(&*self)) }
}



pub impl<T:Copy Sign> Vec2<T>: Sign {
    #[inline(always)]
    pure fn abs() -> Vec2<T> {
        Vec2::new(abs(&self[0]),
                  abs(&self[1]))
    }
    
    #[inline(always)]
    pure fn sign() -> Vec2<T> {
        Vec2::new(sign(&self[0]),
                  sign(&self[1]))
    }
}

pub impl<T:Copy Sign> Vec3<T>: Sign {
    #[inline(always)]
    pure fn abs() -> Vec3<T> {
        Vec3::new(abs(&self[0]),
                  abs(&self[1]),
                  abs(&self[2]))
    }
    
    #[inline(always)]
    pure fn sign() -> Vec3<T> {
        Vec3::new(sign(&self[0]),
                  sign(&self[1]),
                  sign(&self[2]))
    }
}

pub impl<T:Copy Sign> Vec4<T>: Sign {
    #[inline(always)]
    pure fn abs() -> Vec4<T> {
        Vec4::new(abs(&self[0]),
                  abs(&self[1]),
                  abs(&self[2]),
                  abs(&self[3]))
    }
    
    #[inline(always)]
    pure fn sign() -> Vec4<T> {
        Vec4::new(sign(&self[0]),
                  sign(&self[1]),
                  sign(&self[2]),
                  sign(&self[3]))
    }
}

pub trait Approx {
    pure fn floor() -> self;
    pure fn trunc() -> self;
    pure fn round() -> self;
    // pure fn roundEven() -> self;
    pure fn ceil()  -> self;
    pure fn fract() -> self;
}

#[inline(always)] pub pure fn floor<T:Approx>(x: &T) -> T { x.floor() }
#[inline(always)] pub pure fn trunc<T:Approx>(x: &T) -> T { x.trunc() }
#[inline(always)] pub pure fn round<T:Approx>(x: &T) -> T { x.round() }
// #[inline(always)] pub pure fn roundEven<T:Approx>(x: &T) -> T { x.roundEven() }
#[inline(always)] pub pure fn ceil<T:Approx>(x: &T)  -> T { x.ceil() }
#[inline(always)] pub pure fn fract<T:Approx>(x: &T) -> T { x.fract() }

pub impl f32: Approx {
    #[inline(always)] pure fn floor() -> f32 { cast(cmath::c_float_utils::floor(self)) }
    #[inline(always)] pure fn trunc() -> f32 { cast(cmath::c_float_utils::trunc(self)) }
    #[inline(always)] pure fn round() -> f32 { cast(cmath::c_float_utils::round(self)) }
    // #[inline(always)] pure fn roundEven() -> f32 {}
    #[inline(always)] pure fn ceil()  -> f32 { cast(cmath::c_float_utils::ceil(self)) }
    #[inline(always)] pure fn fract() -> f32 { self - floor(&self) }
}

pub impl f64: Approx {
    #[inline(always)] pure fn floor() -> f64 { cast(cmath::c_double_utils::floor(self)) }
    #[inline(always)] pure fn trunc() -> f64 { cast(cmath::c_double_utils::trunc(self)) }
    #[inline(always)] pure fn round() -> f64 { cast(cmath::c_double_utils::round(self)) }
    // #[inline(always)] pure fn roundEven() -> f64 {}
    #[inline(always)] pure fn ceil()  -> f64 { cast(cmath::c_double_utils::ceil(self)) }
    #[inline(always)] pure fn fract() -> f64 { self - floor(&self) }
}

pub impl float: Approx {
    #[inline(always)] pure fn floor() -> float { cast(cmath::c_float_utils::floor(cast(self))) }
    #[inline(always)] pure fn trunc() -> float { cast(cmath::c_float_utils::trunc(cast(self))) }
    #[inline(always)] pure fn round() -> float { cast(cmath::c_float_utils::round(cast(self))) }
    // #[inline(always)] pure fn roundEven() -> float {}
    #[inline(always)] pure fn ceil()  -> float { cast(cmath::c_float_utils::ceil(cast(self))) }
    #[inline(always)] pure fn fract() -> float { self - floor(&self) }
}


pub impl<T:Copy Approx> Radians<T>: Approx{
    #[inline(always)] pure fn floor() -> Radians<T> { Radians(floor(&*self)) }
    #[inline(always)] pure fn trunc() -> Radians<T> { Radians(trunc(&*self)) }
    #[inline(always)] pure fn round() -> Radians<T> { Radians(round(&*self)) }
    // #[inline(always)] pure fn roundEven() -> Radians<T> { Radians(roundEven(&*self)) }
    #[inline(always)] pure fn ceil()  -> Radians<T> { Radians(ceil(&*self)) }
    #[inline(always)] pure fn fract() -> Radians<T> { Radians(fract(&*self)) }
}

pub impl<T:Copy Approx> Degrees<T>: Approx{
    #[inline(always)] pure fn floor() -> Degrees<T> { Degrees(floor(&*self)) }
    #[inline(always)] pure fn trunc() -> Degrees<T> { Degrees(trunc(&*self)) }
    #[inline(always)] pure fn round() -> Degrees<T> { Degrees(round(&*self)) }
    // #[inline(always)] pure fn roundEven() -> Degrees<T> { Degrees(roundEven(&*self)) }
    #[inline(always)] pure fn ceil()  -> Degrees<T> { Degrees(ceil(&*self)) }
    #[inline(always)] pure fn fract() -> Degrees<T> { Degrees(fract(&*self)) }
}


pub impl<T:Copy Approx> Vec2<T>: Approx {
    #[inline(always)]
    pure fn floor() -> Vec2<T> {
        Vec2::new(floor(&self[0]),
                  floor(&self[1]))
    }
    
    #[inline(always)]
    pure fn trunc() -> Vec2<T> {
        Vec2::new(trunc(&self[0]),
                  trunc(&self[1]))
    }
    
    #[inline(always)]
    pure fn round() -> Vec2<T> {
        Vec2::new(round(&self[0]),
                  round(&self[1]))
    }
    
    // #[inline(always)]
    // pure fn ceil()  -> Vec2<T> {
    //     Vec2::new(roundEven(&self[0]),
    //               roundEven(&self[1]))
    // }
    
    #[inline(always)]
    pure fn ceil()  -> Vec2<T> {
        Vec2::new(ceil(&self[0]),
                  ceil(&self[1]))
    }
    
    #[inline(always)]
    pure fn fract() -> Vec2<T> {
        Vec2::new(fract(&self[0]),
                  fract(&self[1]))
    }
    
}

pub impl<T:Copy Approx> Vec3<T>: Approx {
    #[inline(always)]
    pure fn floor() -> Vec3<T> {
        Vec3::new(floor(&self[0]),
                  floor(&self[1]),
                  floor(&self[2]))
    }
    
    #[inline(always)]
    pure fn trunc() -> Vec3<T> {
        Vec3::new(trunc(&self[0]),
                  trunc(&self[1]),
                  trunc(&self[2]))
    }
    
    #[inline(always)]
    pure fn round() -> Vec3<T> {
        Vec3::new(round(&self[0]),
                  round(&self[1]),
                  round(&self[2]))
    }
    
    // #[inline(always)]
    // pure fn ceil()  -> Vec3<T> {
    //     Vec3::new(roundEven(&self[0]),
    //               roundEven(&self[1]),
    //               roundEven(&self[2]))
    // }
    
    #[inline(always)]
    pure fn ceil()  -> Vec3<T> {
        Vec3::new(ceil(&self[0]),
                  ceil(&self[1]),
                  ceil(&self[2]))
    }
    
    #[inline(always)]
    pure fn fract() -> Vec3<T> {
        Vec3::new(fract(&self[0]),
                  fract(&self[1]),
                  fract(&self[2]))
    }
    
}

pub impl<T:Copy Approx> Vec4<T>: Approx {
    #[inline(always)]
    pure fn floor() -> Vec4<T> {
        Vec4::new(floor(&self[0]),
                  floor(&self[1]),
                  floor(&self[2]),
                  floor(&self[3]))
    }
    
    #[inline(always)]
    pure fn trunc() -> Vec4<T> {
        Vec4::new(trunc(&self[0]),
                  trunc(&self[1]),
                  trunc(&self[2]),
                  trunc(&self[3]))
    }
    
    #[inline(always)]
    pure fn round() -> Vec4<T> {
        Vec4::new(round(&self[0]),
                  round(&self[1]),
                  round(&self[2]),
                  round(&self[3]))
    }
    
    // #[inline(always)]
    // pure fn ceil()  -> Vec4<T> {
    //     Vec4::new(roundEven(&self[0]),
    //               roundEven(&self[1]),
    //               roundEven(&self[2]),
    //               roundEven(&self[3]))
    // }
    
    #[inline(always)]
    pure fn ceil()  -> Vec4<T> {
        Vec4::new(ceil(&self[0]),
                  ceil(&self[1]),
                  ceil(&self[2]),
                  ceil(&self[3]))
    }
    
    #[inline(always)]
    pure fn fract() -> Vec4<T> {
        Vec4::new(fract(&self[0]),
                  fract(&self[1]),
                  fract(&self[2]),
                  fract(&self[3]))
    }
    
}

pub trait MinMax {
    pure fn min(other: &self) -> self;
    pure fn max(other: &self) -> self;
}

#[inline(always)] pub pure fn min<T:MinMax>(a: &T, b: &T) -> T { a.min(b) }
#[inline(always)] pub pure fn max<T:MinMax>(a: &T, b: &T) -> T { a.max(b) }

pub impl u8: MinMax {
    #[inline(always)] pure fn min(other: &u8) -> u8 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &u8) -> u8 { if self > *other { self } else { *other } }
}

pub impl u16: MinMax {
    #[inline(always)] pure fn min(other: &u16) -> u16 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &u16) -> u16 { if self > *other { self } else { *other } }
}

pub impl u32: MinMax {
    #[inline(always)] pure fn min(other: &u32) -> u32 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &u32) -> u32 { if self > *other { self } else { *other } }
}

pub impl u64: MinMax {
    #[inline(always)] pure fn min(other: &u64) -> u64 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &u64) -> u64 { if self > *other { self } else { *other } }
}

pub impl uint: MinMax {
    #[inline(always)] pure fn min(other: &uint) -> uint { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &uint) -> uint { if self > *other { self } else { *other } }
}

pub impl i8: MinMax {
    #[inline(always)] pure fn min(other: &i8) -> i8 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &i8) -> i8 { if self > *other { self } else { *other } }
}

pub impl i16: MinMax {
    #[inline(always)] pure fn min(other: &i16) -> i16 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &i16) -> i16 { if self > *other { self } else { *other } }
}

pub impl i32: MinMax {
    #[inline(always)] pure fn min(other: &i32) -> i32 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &i32) -> i32 { if self > *other { self } else { *other } }
}

pub impl i64: MinMax {
    #[inline(always)] pure fn min(other: &i64) -> i64 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &i64) -> i64 { if self > *other { self } else { *other } }
}

pub impl int: MinMax {
    #[inline(always)] pure fn min(other: &int) -> int { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &int) -> int { if self > *other { self } else { *other } }
}

pub impl f32: MinMax {
    #[inline(always)] pure fn min(other: &f32) -> f32 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &f32) -> f32 { if self > *other { self } else { *other } }
}

pub impl f64: MinMax {
    #[inline(always)] pure fn min(other: &f64) -> f64 { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &f64) -> f64 { if self > *other { self } else { *other } }
}

pub impl float: MinMax {
    #[inline(always)] pure fn min(other: &float) -> float { if self < *other { self } else { *other } }
    #[inline(always)] pure fn max(other: &float) -> float { if self > *other { self } else { *other } }
}


pub impl<T:Copy MinMax> Radians<T>: MinMax{
    #[inline(always)] pure fn min(other: &Radians<T>) -> Radians<T> { Radians(min(&*self, &**other)) }
    #[inline(always)] pure fn max(other: &Radians<T>) -> Radians<T> { Radians(max(&*self, &**other)) }
}

pub impl<T:Copy MinMax> Degrees<T>: MinMax{
    #[inline(always)] pure fn min(other: &Degrees<T>) -> Degrees<T> { Degrees(min(&*self, &**other)) }
    #[inline(always)] pure fn max(other: &Degrees<T>) -> Degrees<T> { Degrees(max(&*self, &**other)) }
}


pub impl<T:Copy MinMax> Vec2<T>: MinMax {
    #[inline(always)]
    pure fn min(other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]))
    }
    
    #[inline(always)]
    pure fn max(other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]))
    }
}

pub impl<T:Copy MinMax> Vec3<T>: MinMax {
    #[inline(always)]
    pure fn min(other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]),
                  min(&self[2], &other[2]))
    }
    
    #[inline(always)]
    pure fn max(other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]),
                  max(&self[2], &other[2]))
    }
}

pub impl<T:Copy MinMax> Vec4<T>: MinMax {
    #[inline(always)]
    pure fn min(other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]),
                  min(&self[2], &other[2]),
                  min(&self[3], &other[3]))
    }
    
    #[inline(always)]
    pure fn max(other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]),
                  max(&self[2], &other[2]),
                  max(&self[3], &other[3]))
    }
}

pub trait Clamp {
    pure fn clamp(mn: &self, mx: &self) -> self;
}

#[inline(always)] pub pure fn clamp<T:Clamp>(x: &T, mn: &T, mx: &T) -> T { x.clamp(mn, mx) }

pub impl u8:    Clamp { #[inline(always)] pure fn clamp(mn: &u8,    mx: &u8)    -> u8    { min(&max(&self, mn), mx) } }
pub impl u16:   Clamp { #[inline(always)] pure fn clamp(mn: &u16,   mx: &u16)   -> u16   { min(&max(&self, mn), mx) } }
pub impl u32:   Clamp { #[inline(always)] pure fn clamp(mn: &u32,   mx: &u32)   -> u32   { min(&max(&self, mn), mx) } }
pub impl u64:   Clamp { #[inline(always)] pure fn clamp(mn: &u64,   mx: &u64)   -> u64   { min(&max(&self, mn), mx) } }
pub impl uint:  Clamp { #[inline(always)] pure fn clamp(mn: &uint,  mx: &uint)  -> uint  { min(&max(&self, mn), mx) } }
pub impl i8:    Clamp { #[inline(always)] pure fn clamp(mn: &i8,    mx: &i8)    -> i8    { min(&max(&self, mn), mx) } }
pub impl i16:   Clamp { #[inline(always)] pure fn clamp(mn: &i16,   mx: &i16)   -> i16   { min(&max(&self, mn), mx) } }
pub impl i32:   Clamp { #[inline(always)] pure fn clamp(mn: &i32,   mx: &i32)   -> i32   { min(&max(&self, mn), mx) } }
pub impl i64:   Clamp { #[inline(always)] pure fn clamp(mn: &i64,   mx: &i64)   -> i64   { min(&max(&self, mn), mx) } }
pub impl int:   Clamp { #[inline(always)] pure fn clamp(mn: &int,   mx: &int)   -> int   { min(&max(&self, mn), mx) } }
pub impl f32:   Clamp { #[inline(always)] pure fn clamp(mn: &f32,   mx: &f32)   -> f32   { min(&max(&self, mn), mx) } }
pub impl f64:   Clamp { #[inline(always)] pure fn clamp(mn: &f64,   mx: &f64)   -> f64   { min(&max(&self, mn), mx) } }
pub impl float: Clamp { #[inline(always)] pure fn clamp(mn: &float, mx: &float) -> float { min(&max(&self, mn), mx) } }


pub impl<T:Clamp> Radians<T>: Clamp {
    #[inline(always)]
    pure fn clamp(mn: &Radians<T>, mx: &Radians<T>) -> Radians<T> {
        Radians((*self).clamp(&**mn, &**mx))
    }
}

pub impl<T:Clamp> Degrees<T>: Clamp {
    #[inline(always)]
    pure fn clamp(mn: &Degrees<T>, mx: &Degrees<T>) -> Degrees<T> {
        Degrees((*self).clamp(&**mn, &**mx))
    }
}


pub impl<T:Copy Clamp> Vec2<T>: Clamp {
    #[inline(always)]
    pure fn clamp(mn: &Vec2<T>, mx: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]))
    }
}

pub impl<T:Copy Clamp> Vec3<T>: Clamp {
    #[inline(always)]
    pure fn clamp(mn: &Vec3<T>, mx: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]),
                  self[2].clamp(&mn[2], &mx[2]))
    }
}

pub impl<T:Copy Clamp> Vec4<T>: Clamp {
    #[inline(always)]
    pure fn clamp(mn: &Vec4<T>, mx: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]),
                  self[2].clamp(&mn[2], &mx[2]),
                  self[3].clamp(&mn[3], &mx[3]))
    }
}

pub trait Mix {
    pure fn mix(other: &self, value: &self) -> self;
    pure fn smooth_step(edge0: &self, edge1: &self) -> self;
    pure fn step(edge: &self) -> self;
}

#[inline(always)] pub pure fn mix<T:Mix>(a: &T, b: &T, value: &T) -> T { a.mix(b, value) }
#[inline(always)] pub pure fn smooth_step<T:Mix>(x: &T, edge0: &T, edge1: &T) -> T { x.smooth_step(edge0, edge1) }
#[inline(always)] pub pure fn step<T:Mix>(x: &T, edge: &T) -> T { x.step(edge) }

pub impl f32: Mix {
    #[inline(always)]
    pure fn mix(other: &f32, value: &f32) -> f32 {
        self * (1.0 - (*value)) + (*other) * (*value)
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &f32, edge1: &f32) -> f32 {
        let t = clamp(&((self - *edge0) / (*edge1 - *edge0)), &0.0, &1.0);
        return t * t * (3.0 - 2.0 * t);
    }
    
    #[inline(always)]
    pure fn step(edge: &f32) -> f32 {
        if self < *edge { 0.0 } else { 1.0 }
    }
}

pub impl f64: Mix {
    #[inline(always)]
    pure fn mix(other: &f64, value: &f64) -> f64 {
        self * (1.0 - (*value)) + (*other) * (*value)
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &f64, edge1: &f64) -> f64 {
        let t = clamp(&((self - *edge0) / (*edge1 - *edge0)), &0.0, &1.0);
        return t * t * (3.0 - 2.0 * t);
    }
    
    #[inline(always)]
    pure fn step(edge: &f64) -> f64 {
        if self < *edge { 0.0 } else { 1.0 }
    }
}

pub impl float: Mix {
    #[inline(always)]
    pure fn mix(other: &float, value: &float) -> float {
        self * (1.0 - (*value)) + (*other) * (*value)
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &float, edge1: &float) -> float {
        let t = clamp(&((self - *edge0) / (*edge1 - *edge0)), &0.0, &1.0);
        return t * t * (3.0 - 2.0 * t);
    }
    
    #[inline(always)]
    pure fn step(edge: &float) -> float {
        if self < *edge { 0.0 } else { 1.0 }
    }
}


pub impl<T:Mix> Radians<T>: Mix {
    #[inline(always)]
    pure fn mix(other: &Radians<T>, value: &Radians<T>) -> Radians<T> {
        Radians((*self).mix(&**other, &**value))
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &Radians<T>, edge1: &Radians<T>) -> Radians<T> {
        Radians((*self).smooth_step(&**edge0, &**edge1))
    }
    
    #[inline(always)]
    pure fn step(edge: &Radians<T>) -> Radians<T> {
        Radians((*self).step(&**edge))
    }
}


pub impl<T:Copy Mix> Vec2<T>: Mix {
    #[inline(always)]
    pure fn mix(other: &Vec2<T>, value: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]))
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &Vec2<T>, edge1: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]))
    }
    
    #[inline(always)]
    pure fn step(edge: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]))
    }
}

pub impl<T:Copy Mix> Vec3<T>: Mix {
    #[inline(always)]
    pure fn mix(other: &Vec3<T>, value: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]),
                  self[2].mix(&other[2], &value[2]))
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &Vec3<T>, edge1: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]),
                  self[2].smooth_step(&edge0[2], &edge1[2]))
    }
    
    #[inline(always)]
    pure fn step(edge: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]),
                  self[2].step(&edge[2]))
    }
}

pub impl<T:Copy Mix> Vec4<T>: Mix {
    #[inline(always)]
    pure fn mix(other: &Vec4<T>, value: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]),
                  self[2].mix(&other[2], &value[2]),
                  self[3].mix(&other[3], &value[3]))
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &Vec4<T>, edge1: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]),
                  self[2].smooth_step(&edge0[2], &edge1[2]),
                  self[3].smooth_step(&edge0[3], &edge1[3]))
    }
    
    #[inline(always)]
    pure fn step(edge: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]),
                  self[2].step(&edge[2]),
                  self[3].step(&edge[3]))
    }
}