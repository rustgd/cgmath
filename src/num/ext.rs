/**
 * Various traits intended to be used with the built in numeric types. These
 * allow one to be more specific with trait bounds when using generics.
 *
 * Note: These traits won't be able to be used to their full potential until
 * trait inheritence is implemented.
 */

use core::cmp::{Eq, Ord};

use std::cmp::FuzzyEq;

use num::cast::*;
use num::consts::*;

trait NumExt: Copy, Eq, Num, NumCast, Ord {}

trait UnSignedExt: NumExt {}

pub impl u8:   UnSignedExt {}
pub impl u16:  UnSignedExt {}
pub impl u32:  UnSignedExt {}
pub impl u64:  UnSignedExt {}
pub impl uint: UnSignedExt {}


trait SignedExt: NumExt {}

pub impl i8:    SignedExt {}
pub impl i16:   SignedExt {}
pub impl i32:   SignedExt {}
pub impl i64:   SignedExt {}
pub impl int:   SignedExt {}

pub impl f32:   SignedExt {}
pub impl f64:   SignedExt {}
pub impl float: SignedExt {}


trait IntegerExt: NumExt, IntConsts {}

pub impl u8:   IntegerExt {}
pub impl u16:  IntegerExt {}
pub impl u32:  IntegerExt {}
pub impl u64:  IntegerExt {}
pub impl uint: IntegerExt {}

pub impl i8:   IntegerExt {}
pub impl i16:  IntegerExt {}
pub impl i32:  IntegerExt {}
pub impl i64:  IntegerExt {}
pub impl int:  IntegerExt {}


trait FloatExt: NumExt, FloatConsts, FuzzyEq {}

pub impl f32:   FloatExt {}
pub impl f64:   FloatExt {}
pub impl float: FloatExt {}