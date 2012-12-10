/**
 * Common Functions
 *
 * This module corresponds to Section 8.3 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */

use angle::{Radians, Degrees};
use vec::{Vec2, Vec3, Vec4};

pub trait Sign {
    pure fn abs(&self) -> self;
    pure fn sign(&self) -> self;
}

#[inline(always)] pub pure fn abs<T:Sign>(x: &T)  -> T { x.abs()  }
#[inline(always)] pub pure fn sign<T:Sign>(x: &T) -> T { x.sign() }

pub impl i8: Sign {
    #[inline(always)] pure fn abs(&self)  -> i8 { if (*self) >= 0 { (*self) } else { -(*self) } }
    #[inline(always)] pure fn sign(&self) -> i8 { if (*self) > 0 { 1 } else if (*self) == 0 { 0 } else { -1 } }
}

pub impl i16: Sign {
    #[inline(always)] pure fn abs(&self)  -> i16 { if (*self) >= 0 { (*self) } else { -(*self) } }
    #[inline(always)] pure fn sign(&self) -> i16 { if (*self) > 0 { 1 } else if (*self) == 0 { 0 } else { -1 } }
}

pub impl i32: Sign {
    #[inline(always)] pure fn abs(&self)  -> i32 { if (*self) >= 0 { (*self) } else { -(*self) } }
    #[inline(always)] pure fn sign(&self) -> i32 { if (*self) > 0 { 1 } else if (*self) == 0 { 0 } else { -1 } }
}

pub impl i64: Sign {
    #[inline(always)] pure fn abs(&self)  -> i64 { if (*self) >= 0 { (*self) } else { -(*self) } }
    #[inline(always)] pure fn sign(&self) -> i64 { if (*self) > 0 { 1 } else if (*self) == 0 { 0 } else { -1 } }
}

pub impl int: Sign {
    #[inline(always)] pure fn abs(&self)  -> int { if (*self) >= 0 { (*self) } else { -(*self) } }
    #[inline(always)] pure fn sign(&self) -> int { if (*self) > 0 { 1 } else if (*self) == 0 { 0 } else { -1 } }
}

pub impl f32: Sign {
    #[inline(always)] pure fn abs(&self)  -> f32 { if (*self) >= 0f32 { (*self) } else { -(*self) } }
    #[inline(always)] pure fn sign(&self) -> f32 { if (*self) > 0f32 { 1f32 } else if (*self) == 0f32 { 0f32 } else { -1f32 } }
}

pub impl f64: Sign {
    #[inline(always)] pure fn abs(&self)  -> f64 { if (*self) >= 0f64 { (*self) } else { -(*self) } }
    #[inline(always)] pure fn sign(&self) -> f64 { if (*self) > 0f64 { 1f64 } else if (*self) == 0f64 { 0f64 } else { -1f64 } }
}

pub impl float: Sign {
    #[inline(always)] pure fn abs(&self)  -> float { if (*self) >= 0f { (*self) } else { -(*self) } }
    #[inline(always)] pure fn sign(&self) -> float { if (*self) > 0f { 1f } else if (*self) == 0f { 0f } else { -1f } }
}


pub impl<T:Sign> Radians<T>: Sign{
    #[inline(always)] pure fn abs(&self)  -> Radians<T> { Radians(abs(&**self)) }
    #[inline(always)] pure fn sign(&self) -> Radians<T> { Radians(sign(&**self)) }
}

pub impl<T:Sign> Degrees<T>: Sign{
    #[inline(always)] pure fn abs(&self)  -> Degrees<T> { Degrees(abs(&**self)) }
    #[inline(always)] pure fn sign(&self) -> Degrees<T> { Degrees(sign(&**self)) }
}



pub impl<T:Copy Sign> Vec2<T>: Sign {
    #[inline(always)]
    pure fn abs(&self) -> Vec2<T> {
        Vec2::new(abs(&self[0]),
                  abs(&self[1]))
    }
    
    #[inline(always)]
    pure fn sign(&self) -> Vec2<T> {
        Vec2::new(sign(&self[0]),
                  sign(&self[1]))
    }
}

pub impl<T:Copy Sign> Vec3<T>: Sign {
    #[inline(always)]
    pure fn abs(&self) -> Vec3<T> {
        Vec3::new(abs(&self[0]),
                  abs(&self[1]),
                  abs(&self[2]))
    }
    
    #[inline(always)]
    pure fn sign(&self) -> Vec3<T> {
        Vec3::new(sign(&self[0]),
                  sign(&self[1]),
                  sign(&self[2]))
    }
}

pub impl<T:Copy Sign> Vec4<T>: Sign {
    #[inline(always)]
    pure fn abs(&self) -> Vec4<T> {
        Vec4::new(abs(&self[0]),
                  abs(&self[1]),
                  abs(&self[2]),
                  abs(&self[3]))
    }
    
    #[inline(always)]
    pure fn sign(&self) -> Vec4<T> {
        Vec4::new(sign(&self[0]),
                  sign(&self[1]),
                  sign(&self[2]),
                  sign(&self[3]))
    }
}

pub trait Approx {
    pure fn floor(&self) -> self;
    pure fn trunc(&self) -> self;
    pure fn round(&self) -> self;
    // pure fn roundEven(&self) -> self;
    pure fn ceil(&self)  -> self;
    pure fn fract(&self) -> self;
}

#[inline(always)] pub pure fn floor<T:Approx>(x: &T) -> T { x.floor() }
#[inline(always)] pub pure fn trunc<T:Approx>(x: &T) -> T { x.trunc() }
#[inline(always)] pub pure fn round<T:Approx>(x: &T) -> T { x.round() }
// #[inline(always)] pub pure fn roundEven<T:Approx>(x: &T) -> T { x.roundEven() }
#[inline(always)] pub pure fn ceil<T:Approx>(x: &T)  -> T { x.ceil() }
#[inline(always)] pub pure fn fract<T:Approx>(x: &T) -> T { x.fract() }

pub impl f32: Approx {
    #[inline(always)] pure fn floor(&self) -> f32 { f32::floor(*self) }
    #[inline(always)] pure fn trunc(&self) -> f32 { f32::trunc(*self) }
    #[inline(always)] pure fn round(&self) -> f32 { f32::round(*self) }
    // #[inline(always)] pure fn roundEven(&self) -> f32 {}
    #[inline(always)] pure fn ceil(&self)  -> f32 { f32::ceil(*self) }
    #[inline(always)] pure fn fract(&self) -> f32 { (*self) - floor(self) }
}

pub impl f64: Approx {
    #[inline(always)] pure fn floor(&self) -> f64 { f64::floor(*self) }
    #[inline(always)] pure fn trunc(&self) -> f64 { f64::trunc(*self) }
    #[inline(always)] pure fn round(&self) -> f64 { f64::round(*self) }
    // #[inline(always)] pure fn roundEven(&self) -> f64 {}
    #[inline(always)] pure fn ceil(&self)  -> f64 { f64::ceil(*self) }
    #[inline(always)] pure fn fract(&self) -> f64 { (*self) - floor(self) }
}

pub impl float: Approx {
    #[inline(always)] pure fn floor(&self) -> float { f64::floor(*self as f64) as float }
    #[inline(always)] pure fn trunc(&self) -> float { f64::trunc(*self as f64) as float }
    #[inline(always)] pure fn round(&self) -> float { f64::round(*self as f64) as float }
    // #[inline(always)] pure fn roundEven(&self) -> float {}
    #[inline(always)] pure fn ceil(&self)  -> float { f64::ceil(*self as f64) as float }
    #[inline(always)] pure fn fract(&self) -> float { (*self) - floor(self) }
}


pub impl<T:Approx> Radians<T>: Approx{
    #[inline(always)] pure fn floor(&self) -> Radians<T> { Radians(floor(&**self)) }
    #[inline(always)] pure fn trunc(&self) -> Radians<T> { Radians(trunc(&**self)) }
    #[inline(always)] pure fn round(&self) -> Radians<T> { Radians(round(&**self)) }
    // #[inline(always)] pure fn roundEven(&self) -> Radians<T> { Radians(roundEven(&**self)) }
    #[inline(always)] pure fn ceil(&self)  -> Radians<T> { Radians(ceil(&**self)) }
    #[inline(always)] pure fn fract(&self) -> Radians<T> { Radians(fract(&**self)) }
}

pub impl<T:Approx> Degrees<T>: Approx{
    #[inline(always)] pure fn floor(&self) -> Degrees<T> { Degrees(floor(&**self)) }
    #[inline(always)] pure fn trunc(&self) -> Degrees<T> { Degrees(trunc(&**self)) }
    #[inline(always)] pure fn round(&self) -> Degrees<T> { Degrees(round(&**self)) }
    // #[inline(always)] pure fn roundEven(&self) -> Degrees<T> { Degrees(roundEven(&**self)) }
    #[inline(always)] pure fn ceil(&self)  -> Degrees<T> { Degrees(ceil(&**self)) }
    #[inline(always)] pure fn fract(&self) -> Degrees<T> { Degrees(fract(&**self)) }
}


pub impl<T:Copy Approx> Vec2<T>: Approx {
    #[inline(always)]
    pure fn floor(&self) -> Vec2<T> {
        Vec2::new(floor(&self[0]),
                  floor(&self[1]))
    }
    
    #[inline(always)]
    pure fn trunc(&self) -> Vec2<T> {
        Vec2::new(trunc(&self[0]),
                  trunc(&self[1]))
    }
    
    #[inline(always)]
    pure fn round(&self) -> Vec2<T> {
        Vec2::new(round(&self[0]),
                  round(&self[1]))
    }
    
    // #[inline(always)]
    // pure fn ceil(&self)  -> Vec2<T> {
    //     Vec2::new(roundEven(&self[0]),
    //               roundEven(&self[1]))
    // }
    
    #[inline(always)]
    pure fn ceil(&self)  -> Vec2<T> {
        Vec2::new(ceil(&self[0]),
                  ceil(&self[1]))
    }
    
    #[inline(always)]
    pure fn fract(&self) -> Vec2<T> {
        Vec2::new(fract(&self[0]),
                  fract(&self[1]))
    }
    
}

pub impl<T:Copy Approx> Vec3<T>: Approx {
    #[inline(always)]
    pure fn floor(&self) -> Vec3<T> {
        Vec3::new(floor(&self[0]),
                  floor(&self[1]),
                  floor(&self[2]))
    }
    
    #[inline(always)]
    pure fn trunc(&self) -> Vec3<T> {
        Vec3::new(trunc(&self[0]),
                  trunc(&self[1]),
                  trunc(&self[2]))
    }
    
    #[inline(always)]
    pure fn round(&self) -> Vec3<T> {
        Vec3::new(round(&self[0]),
                  round(&self[1]),
                  round(&self[2]))
    }
    
    // #[inline(always)]
    // pure fn ceil(&self)  -> Vec3<T> {
    //     Vec3::new(roundEven(&self[0]),
    //               roundEven(&self[1]),
    //               roundEven(&self[2]))
    // }
    
    #[inline(always)]
    pure fn ceil(&self)  -> Vec3<T> {
        Vec3::new(ceil(&self[0]),
                  ceil(&self[1]),
                  ceil(&self[2]))
    }
    
    #[inline(always)]
    pure fn fract(&self) -> Vec3<T> {
        Vec3::new(fract(&self[0]),
                  fract(&self[1]),
                  fract(&self[2]))
    }
    
}

pub impl<T:Copy Approx> Vec4<T>: Approx {
    #[inline(always)]
    pure fn floor(&self) -> Vec4<T> {
        Vec4::new(floor(&self[0]),
                  floor(&self[1]),
                  floor(&self[2]),
                  floor(&self[3]))
    }
    
    #[inline(always)]
    pure fn trunc(&self) -> Vec4<T> {
        Vec4::new(trunc(&self[0]),
                  trunc(&self[1]),
                  trunc(&self[2]),
                  trunc(&self[3]))
    }
    
    #[inline(always)]
    pure fn round(&self) -> Vec4<T> {
        Vec4::new(round(&self[0]),
                  round(&self[1]),
                  round(&self[2]),
                  round(&self[3]))
    }
    
    // #[inline(always)]
    // pure fn ceil(&self)  -> Vec4<T> {
    //     Vec4::new(roundEven(&self[0]),
    //               roundEven(&self[1]),
    //               roundEven(&self[2]),
    //               roundEven(&self[3]))
    // }
    
    #[inline(always)]
    pure fn ceil(&self)  -> Vec4<T> {
        Vec4::new(ceil(&self[0]),
                  ceil(&self[1]),
                  ceil(&self[2]),
                  ceil(&self[3]))
    }
    
    #[inline(always)]
    pure fn fract(&self) -> Vec4<T> {
        Vec4::new(fract(&self[0]),
                  fract(&self[1]),
                  fract(&self[2]),
                  fract(&self[3]))
    }
    
}

pub trait Extent {
    pure fn min(&self, other: &self) -> self;
    pure fn max(&self, other: &self) -> self;
    pure fn clamp(&self, mn: &self, mx: &self) -> self;
}

#[inline(always)] pub pure fn min<T:Extent>(a: &T, b: &T) -> T { a.min(b) }
#[inline(always)] pub pure fn max<T:Extent>(a: &T, b: &T) -> T { a.max(b) }
#[inline(always)] pub pure fn clamp<T:Extent>(x: &T, mn: &T, mx: &T) -> T { x.clamp(mn, mx) }

pub impl u8: Extent {
    #[inline(always)]
    pure fn min(&self, other: &u8) -> u8 {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &u8) -> u8 {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &u8, mx: &u8) -> u8 {
        min(&max(self, mn), mx)
    }
}

pub impl u16: Extent {
    #[inline(always)]
    pure fn min(&self, other: &u16) -> u16 {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &u16) -> u16 {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &u16, mx: &u16) -> u16 {
        min(&max(self, mn), mx)
    }
}

pub impl u32: Extent {
    #[inline(always)]
    pure fn min(&self, other: &u32) -> u32 {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &u32) -> u32 {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &u32, mx: &u32) -> u32 {
        min(&max(self, mn), mx)
    }
}

pub impl u64: Extent {
    #[inline(always)]
    pure fn min(&self, other: &u64) -> u64 {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &u64) -> u64 {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &u64, mx: &u64) -> u64 {
        min(&max(self, mn), mx)
    }
}

pub impl uint: Extent {
    #[inline(always)]
    pure fn min(&self, other: &uint) -> uint {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &uint) -> uint {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &uint, mx: &uint) -> uint {
        min(&max(self, mn), mx)
    }
}

pub impl i8: Extent {
    #[inline(always)]
    pure fn min(&self, other: &i8) -> i8 {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &i8) -> i8 {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &i8, mx: &i8) -> i8 {
        min(&max(self, mn), mx)
    }
}

pub impl i16: Extent {
    #[inline(always)]
    pure fn min(&self, other: &i16) -> i16 {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &i16) -> i16 {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &i16, mx: &i16) -> i16 {
        min(&max(self, mn), mx)
    }
}

pub impl i32: Extent {
    #[inline(always)]
    pure fn min(&self, other: &i32) -> i32 {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &i32) -> i32 {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &i32, mx: &i32) -> i32 {
        min(&max(self, mn), mx)
    }
}

pub impl i64: Extent {
    #[inline(always)]
    pure fn min(&self, other: &i64) -> i64 {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &i64) -> i64 {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &i64, mx: &i64) -> i64 {
        min(&max(self, mn), mx)
    }
}

pub impl int: Extent {
    #[inline(always)]
    pure fn min(&self, other: &int) -> int {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &int) -> int {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &int, mx: &int) -> int {
        min(&max(self, mn), mx)
    }
}

pub impl f32: Extent {
    #[inline(always)]
    pure fn min(&self, other: &f32) -> f32 {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &f32) -> f32 {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &f32, mx: &f32) -> f32 {
        min(&max(self, mn), mx)
    }
}

pub impl f64: Extent {
    #[inline(always)]
    pure fn min(&self, other: &f64) -> f64 {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &f64) -> f64 {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &f64, mx: &f64) -> f64 {
        min(&max(self, mn), mx)
    }
}

pub impl float: Extent {
    #[inline(always)]
    pure fn min(&self, other: &float) -> float {
        if (*self) < (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn max(&self, other: &float) -> float {
        if (*self) > (*other) { (*self) } else { (*other) }
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &float, mx: &float) -> float {
        min(&max(self, mn), mx)
    }
}


pub impl<T:Copy Extent> Radians<T>: Extent{
    #[inline(always)]
    pure fn min(&self, other: &Radians<T>) -> Radians<T> {
        Radians(min(&**self, &**other))
    }
    
    #[inline(always)]
    pure fn max(&self, other: &Radians<T>) -> Radians<T> {
        Radians(max(&**self, &**other))
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &Radians<T>, mx: &Radians<T>) -> Radians<T> {
        Radians((**self).clamp(&**mn, &**mx))
    }
}

pub impl<T:Copy Extent> Degrees<T>: Extent{
    #[inline(always)]
    pure fn min(&self, other: &Degrees<T>) -> Degrees<T> {
        Degrees(min(&**self, &**other))
    }
    
    #[inline(always)]
    pure fn max(&self, other: &Degrees<T>) -> Degrees<T> {
        Degrees(max(&**self, &**other))
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &Degrees<T>, mx: &Degrees<T>) -> Degrees<T> {
        Degrees((**self).clamp(&**mn, &**mx))
    }
}


pub impl<T:Copy Extent> Vec2<T>: Extent {
    #[inline(always)]
    pure fn min(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]))
    }
    
    #[inline(always)]
    pure fn max(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]))
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &Vec2<T>, mx: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]))
    }
}

pub impl<T:Copy Extent> Vec3<T>: Extent {
    #[inline(always)]
    pure fn min(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]),
                  min(&self[2], &other[2]))
    }
    
    #[inline(always)]
    pure fn max(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]),
                  max(&self[2], &other[2]))
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &Vec3<T>, mx: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]),
                  self[2].clamp(&mn[2], &mx[2]))
    }
}

pub impl<T:Copy Extent> Vec4<T>: Extent {
    #[inline(always)]
    pure fn min(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]),
                  min(&self[2], &other[2]),
                  min(&self[3], &other[3]))
    }
    
    #[inline(always)]
    pure fn max(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]),
                  max(&self[2], &other[2]),
                  max(&self[3], &other[3]))
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &Vec4<T>, mx: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]),
                  self[2].clamp(&mn[2], &mx[2]),
                  self[3].clamp(&mn[3], &mx[3]))
    }
}


pub trait Mix {
    pure fn mix(&self, other: &self, value: &self) -> self;
    pure fn smooth_step(&self, edge0: &self, edge1: &self) -> self;
    pure fn step(&self, edge: &self) -> self;
}

#[inline(always)] pub pure fn mix<T:Mix>(a: &T, b: &T, value: &T) -> T { a.mix(b, value) }
#[inline(always)] pub pure fn smooth_step<T:Mix>(x: &T, edge0: &T, edge1: &T) -> T { x.smooth_step(edge0, edge1) }
#[inline(always)] pub pure fn step<T:Mix>(x: &T, edge: &T) -> T { x.step(edge) }

pub impl f32: Mix {
    #[inline(always)]
    pure fn mix(&self, other: &f32, value: &f32) -> f32 {
        (*self) * (1.0 - (*value)) + (*other) * (*value)
    }
    
    #[inline(always)]
    pure fn smooth_step(&self, edge0: &f32, edge1: &f32) -> f32 {
        let t = clamp(&((*self - *edge0) / (*edge1 - *edge0)), &0.0, &1.0);
        return t * t * (3.0 - 2.0 * t);
    }
    
    #[inline(always)]
    pure fn step(&self, edge: &f32) -> f32 {
        if (*self) < (*edge) { 0.0 } else { 1.0 }
    }
}

pub impl f64: Mix {
    #[inline(always)]
    pure fn mix(&self, other: &f64, value: &f64) -> f64 {
        (*self) * (1.0 - (*value)) + (*other) * (*value)
    }
    
    #[inline(always)]
    pure fn smooth_step(&self, edge0: &f64, edge1: &f64) -> f64 {
        let t = clamp(&((*self - *edge0) / (*edge1 - *edge0)), &0.0, &1.0);
        return t * t * (3.0 - 2.0 * t);
    }
    
    #[inline(always)]
    pure fn step(&self, edge: &f64) -> f64 {
        if (*self) < (*edge) { 0.0 } else { 1.0 }
    }
}

pub impl float: Mix {
    #[inline(always)]
    pure fn mix(&self, other: &float, value: &float) -> float {
        (*self) * (1.0 - (*value)) + (*other) * (*value)
    }
    
    #[inline(always)]
    pure fn smooth_step(&self, edge0: &float, edge1: &float) -> float {
        let t = clamp(&((*self - *edge0) / (*edge1 - *edge0)), &0.0, &1.0);
        return t * t * (3.0 - 2.0 * t);
    }
    
    #[inline(always)]
    pure fn step(&self, edge: &float) -> float {
        if (*self) < (*edge) { 0.0 } else { 1.0 }
    }
}


// FIXME: ICE!!

pub impl<T:Mix> Radians<T>: Mix {
    #[inline(always)]
    pure fn mix(&self, other: &Radians<T>, value: &Radians<T>) -> Radians<T> {
        Radians((**self).mix(&**other, &**value))
    }
    
    #[inline(always)]
    pure fn smooth_step(&self, edge0: &Radians<T>, edge1: &Radians<T>) -> Radians<T> {
        Radians((**self).smooth_step(&**edge0, &**edge1))
    }
    
    #[inline(always)]
    pure fn step(&self, edge: &Radians<T>) -> Radians<T> {
        Radians((**self).step(&**edge))
    }
}


pub impl<T:Copy Mix> Vec2<T>: Mix {
    #[inline(always)]
    pure fn mix(&self, other: &Vec2<T>, value: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]))
    }
    
    #[inline(always)]
    pure fn smooth_step(&self, edge0: &Vec2<T>, edge1: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]))
    }
    
    #[inline(always)]
    pure fn step(&self, edge: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]))
    }
}

pub impl<T:Copy Mix> Vec3<T>: Mix {
    #[inline(always)]
    pure fn mix(&self, other: &Vec3<T>, value: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]),
                  self[2].mix(&other[2], &value[2]))
    }
    
    #[inline(always)]
    pure fn smooth_step(&self, edge0: &Vec3<T>, edge1: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]),
                  self[2].smooth_step(&edge0[2], &edge1[2]))
    }
    
    #[inline(always)]
    pure fn step(&self, edge: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]),
                  self[2].step(&edge[2]))
    }
}

pub impl<T:Copy Mix> Vec4<T>: Mix {
    #[inline(always)]
    pure fn mix(&self, other: &Vec4<T>, value: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]),
                  self[2].mix(&other[2], &value[2]),
                  self[3].mix(&other[3], &value[3]))
    }
    
    #[inline(always)]
    pure fn smooth_step(&self, edge0: &Vec4<T>, edge1: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]),
                  self[2].smooth_step(&edge0[2], &edge1[2]),
                  self[3].smooth_step(&edge0[3], &edge1[3]))
    }
    
    #[inline(always)]
    pure fn step(&self, edge: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]),
                  self[2].step(&edge[2]),
                  self[3].step(&edge[3]))
    }
}
