use cmp::{Eq, Ord};
use num::Num;

use ncast::*;
use vector::*;

/**
 * Common Functions for all numeric types
 */
pub trait Extent {
    pure fn min(y: &self) -> self;
    pure fn max(y: &self) -> self;
    pure fn clamp(minv: &self, maxv: &self) -> self;
}

#[inline(always)] pub pure fn min<T:Extent>(x: &T, y: &T) -> T { x.min(y) }
#[inline(always)] pub pure fn max<T:Extent>(x: &T, y: &T) -> T { x.max(y) }
#[inline(always)] pub pure fn clamp<T:Extent>(x: &T, minv: &T, maxv: &T) -> T { x.clamp(minv, maxv) }

pub impl u8: Extent {
    #[inline(always)] pure fn min(y: &u8) -> u8 { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &u8) -> u8 { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &u8, maxv: &u8) -> u8 { min(&max(&self, minv), maxv) }
}

pub impl u16: Extent {
    #[inline(always)] pure fn min(y: &u16) -> u16 { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &u16) -> u16 { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &u16, maxv: &u16) -> u16 { min(&max(&self, minv), maxv) }
}

pub impl u32: Extent {
    #[inline(always)] pure fn min(y: &u32) -> u32 { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &u32) -> u32 { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &u32, maxv: &u32) -> u32 { min(&max(&self, minv), maxv) }
}

pub impl u64: Extent {
    #[inline(always)] pure fn min(y: &u64) -> u64 { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &u64) -> u64 { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &u64, maxv: &u64) -> u64 { min(&max(&self, minv), maxv) }
}

pub impl uint: Extent {
    #[inline(always)] pure fn min(y: &uint) -> uint { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &uint) -> uint { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &uint, maxv: &uint) -> uint { min(&max(&self, minv), maxv) }
}

pub impl i8: Extent {
    #[inline(always)] pure fn min(y: &i8) -> i8 { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &i8) -> i8 { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &i8, maxv: &i8) -> i8 { min(&max(&self, minv), maxv) }
}

pub impl i16: Extent {
    #[inline(always)] pure fn min(y: &i16) -> i16 { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &i16) -> i16 { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &i16, maxv: &i16) -> i16 { min(&max(&self, minv), maxv) }
}

pub impl i32: Extent {
    #[inline(always)] pure fn min(y: &i32) -> i32 { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &i32) -> i32 { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &i32, maxv: &i32) -> i32 { min(&max(&self, minv), maxv) }
}

pub impl i64: Extent {
    #[inline(always)] pure fn min(y: &i64) -> i64 { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &i64) -> i64 { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &i64, maxv: &i64) -> i64 { min(&max(&self, minv), maxv) }
}

pub impl int: Extent {
    #[inline(always)] pure fn min(y: &int) -> int { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &int) -> int { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &int, maxv: &int) -> int { min(&max(&self, minv), maxv) }
}

pub impl f32: Extent {
    #[inline(always)] pure fn min(y: &f32) -> f32 { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &f32) -> f32 { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &f32, maxv: &f32) -> f32 { min(&max(&self, minv), maxv) }
}

pub impl f64: Extent {
    #[inline(always)] pure fn min(y: &f64) -> f64 { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &f64) -> f64 { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &f64, maxv: &f64) -> f64 { min(&max(&self, minv), maxv) }
}

pub impl float: Extent {
    #[inline(always)] pure fn min(y: &float) -> float { if self < *y { self } else { *y } }
    #[inline(always)] pure fn max(y: &float) -> float { if self > *y { self } else { *y } }
    #[inline(always)] pure fn clamp(minv: &float, maxv: &float) -> float { min(&max(&self, minv), maxv) }
}

/**
 * Common Functions for signed numeric types
 */
pub trait Signed {
    pure fn abs() -> self;
    pure fn sign() -> self;
}

#[inline(always)] pub pure fn abs<T:Signed>(x: &T) -> T { x.abs() }
#[inline(always)] pub pure fn sign<T:Signed>(x: &T) -> T { x.sign() }

pub impl i8: Signed {
    #[inline(always)] pure fn abs() -> i8 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i8 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl i16: Signed {
    #[inline(always)] pure fn abs() -> i16 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i16 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl i32: Signed {
    #[inline(always)] pure fn abs() -> i32 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i32 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl i64: Signed {
    #[inline(always)] pure fn abs() -> i64 { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> i64 { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl int: Signed {
    #[inline(always)] pure fn abs() -> int { if self >= 0 { self } else {-self } }
    #[inline(always)] pure fn sign() -> int { if self > 0 { 1 } else if self == 0 { 0 } else { -1 } }
}

pub impl f32: Signed {
    #[inline(always)] pure fn abs() -> f32 { if self >= 0f32 { self } else {-self } }
    #[inline(always)] pure fn sign() -> f32 { if self > 0f32 { 1f32 } else if self == 0f32 { 0f32 } else { -1f32 } }
}

pub impl f64: Signed {
    #[inline(always)] pure fn abs() -> f64 { if self >= 0f64 { self } else {-self } }
    #[inline(always)] pure fn sign() -> f64 { if self > 0f64 { 1f64 } else if self == 0f64 { 0f64 } else { -1f64 } }
}

pub impl float: Signed {
    #[inline(always)] pure fn abs() -> float { if self >= 0f { self } else {-self } }
    #[inline(always)] pure fn sign() -> float { if self > 0f { 1f } else if self == 0f { 0f } else { -1f } }
}

/**
 * Common Functions for floating point types
 */
pub trait Float {
    pure fn floor() -> self;
    pure fn trunc() -> self;
    pure fn round() -> self;
    // pure fn roundEven() -> self;
    pure fn ceil() -> self;
    // pure fn fract() -> self;
}

#[inline(always)] pub pure fn floor<T:Float>(x: T) -> T { x.floor() }
#[inline(always)] pub pure fn trunc<T:Float>(x: T) -> T { x.trunc() }
#[inline(always)] pub pure fn round<T:Float>(x: T) -> T { x.round() }
// #[inline(always)] pub pure fn roundEven<T:Float>(x: T) -> T { x.roundEven() }
#[inline(always)] pub pure fn ceil<T:Float>(x: T) -> T { x.ceil() }
// #[inline(always)] pub pure fn fract<T:Float>(x: T) -> T { x.fract() }

pub impl f32: Float {
    #[inline(always)] pure fn floor() -> f32 { f32::floor(self) }
    #[inline(always)] pure fn trunc() -> f32 { f32::trunc(self) }
    #[inline(always)] pure fn round() -> f32 { f32::round(self) }
    // #[inline(always)] pure fn roundEven() -> f32 {}
    #[inline(always)] pure fn ceil() -> f32 { f32::ceil(self) }
    // #[inline(always)] pure fn fract() -> f32 {}
}

pub impl f64: Float {
    #[inline(always)] pure fn floor() -> f64 { f64::floor(self) }
    #[inline(always)] pure fn trunc() -> f64 { f64::trunc(self) }
    #[inline(always)] pure fn round() -> f64 { f64::round(self) }
    // #[inline(always)] pure fn roundEven() -> f64 {}
    #[inline(always)] pure fn ceil() -> f64 { f64::ceil(self) }
    // #[inline(always)] pure fn fract() -> f64 {}
}

pub impl float: Float {
    #[inline(always)] pure fn floor() -> float { cast(float::floor(cast(self))) }
    #[inline(always)] pure fn trunc() -> float { cast(float::trunc(cast(self))) }
    #[inline(always)] pure fn round() -> float { cast(float::round(cast(self))) }
    // #[inline(always)] pure fn roundEven() -> float {}
    #[inline(always)] pure fn ceil() -> float { cast(float::ceil(cast(self))) }
    // #[inline(always)] pure fn fract() -> float {}
}

pub trait Mix<B> {
    pure fn mix(y: &self, a: &self) -> self;
    pure fn mixb(y: &self, a: &B) -> self;
}

#[inline(always)] pub pure fn mix<T:Mix<B>, B>(x: &T, y: &T, a: &T) -> T { x.mix(y, a) }
#[inline(always)] pub pure fn mixb<T:Mix<B>, B>(x: &T, y: &T, a: &B) -> T { x.mixb(y, a) }

pub impl f32: Mix<bool> {
    #[inline(always)] pure fn mix(y: &f32, a: &f32) -> f32 { self * (1f32 - (*a)) + y * (*a) }
    #[inline(always)] pure fn mixb(y: &f32, a: &bool) -> f32 { if *a { *y } else { self } }
}

pub impl f64: Mix<bool> {
    #[inline(always)] pure fn mix(y: &f64, a: &f64) -> f64 { self * (1f64 - (*a)) + y * (*a) }
    #[inline(always)] pure fn mixb(y: &f64, a: &bool) -> f64 { if *a { *y } else { self } }
}

pub impl float: Mix<bool> {
    #[inline(always)] pure fn mix(y: &float, a: &float) -> float { self * (1f - (*a)) + y * (*a) }
    #[inline(always)] pure fn mixb(y: &float, a: &bool) -> float { if *a { *y } else { self } }
}