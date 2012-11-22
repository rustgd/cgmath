use num::cast::cast;

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