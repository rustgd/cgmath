use std::cmp::FuzzyEq;


pub trait DefaultEq {
    pure fn default_eq(&self, other: &self) -> bool;
}

pub impl bool:  DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &bool)   -> bool { (*self) == (*other) } }

pub impl u8:    DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &u8)     -> bool { (*self) == (*other) } }
pub impl u16:   DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &u16)    -> bool { (*self) == (*other) } }
pub impl u32:   DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &u32)    -> bool { (*self) == (*other) } }
pub impl u64:   DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &u64)    -> bool { (*self) == (*other) } }
pub impl uint:  DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &uint)   -> bool { (*self) == (*other) } }

pub impl i8:    DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &i8)     -> bool { (*self) == (*other) } }
pub impl i16:   DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &i16)    -> bool { (*self) == (*other) } }
pub impl i32:   DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &i32)    -> bool { (*self) == (*other) } }
pub impl i64:   DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &i64)    -> bool { (*self) == (*other) } }
pub impl int:   DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &int)    -> bool { (*self) == (*other) } }

pub impl f32:   DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &f32)    -> bool { self.fuzzy_eq(other) } }
pub impl f64:   DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &f64)    -> bool { self.fuzzy_eq(other) } }
pub impl float: DefaultEq { #[inline(always)] pure fn default_eq(&self, other: &float)  -> bool { self.fuzzy_eq(other) } }