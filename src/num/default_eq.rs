use std::cmp::FuzzyEq;


pub trait DefaultEq {
    pure fn default_eq(other: &self) -> bool;
}

pub impl bool:  DefaultEq { #[inline(always)] pure fn default_eq(other: &bool)   -> bool { self == *other } }

pub impl u8:    DefaultEq { #[inline(always)] pure fn default_eq(other: &u8)     -> bool { self == *other } }
pub impl u16:   DefaultEq { #[inline(always)] pure fn default_eq(other: &u16)    -> bool { self == *other } }
pub impl u32:   DefaultEq { #[inline(always)] pure fn default_eq(other: &u32)    -> bool { self == *other } }
pub impl u64:   DefaultEq { #[inline(always)] pure fn default_eq(other: &u64)    -> bool { self == *other } }
pub impl uint:  DefaultEq { #[inline(always)] pure fn default_eq(other: &uint)   -> bool { self == *other } }

pub impl i8:    DefaultEq { #[inline(always)] pure fn default_eq(other: &i8)     -> bool { self == *other } }
pub impl i16:   DefaultEq { #[inline(always)] pure fn default_eq(other: &i16)    -> bool { self == *other } }
pub impl i32:   DefaultEq { #[inline(always)] pure fn default_eq(other: &i32)    -> bool { self == *other } }
pub impl i64:   DefaultEq { #[inline(always)] pure fn default_eq(other: &i64)    -> bool { self == *other } }
pub impl int:   DefaultEq { #[inline(always)] pure fn default_eq(other: &int)    -> bool { self == *other } }

pub impl f32:   DefaultEq { #[inline(always)] pure fn default_eq(other: &f32)    -> bool { self.fuzzy_eq(other) } }
pub impl f64:   DefaultEq { #[inline(always)] pure fn default_eq(other: &f64)    -> bool { self.fuzzy_eq(other) } }
pub impl float: DefaultEq { #[inline(always)] pure fn default_eq(other: &float)  -> bool { self.fuzzy_eq(other) } }