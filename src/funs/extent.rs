use num::cast::cast;
use vec::{Vec2, Vec3, Vec4};

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