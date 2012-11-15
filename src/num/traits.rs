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

trait UnSignedNum: NumExt {}

pub impl u8:   UnSignedNum {}
pub impl u16:  UnSignedNum {}
pub impl u32:  UnSignedNum {}
pub impl u64:  UnSignedNum {}
pub impl uint: UnSignedNum {}


trait SignedNum: NumExt {}

pub impl i8:    SignedNum {}
pub impl i16:   SignedNum {}
pub impl i32:   SignedNum {}
pub impl i64:   SignedNum {}
pub impl int:   SignedNum {}

pub impl f32:   SignedNum {}
pub impl f64:   SignedNum {}
pub impl float: SignedNum {}


trait IntegerNum: NumExt, IntConsts {}

pub impl u8:   IntegerNum {}
pub impl u16:  IntegerNum {}
pub impl u32:  IntegerNum {}
pub impl u64:  IntegerNum {}
pub impl uint: IntegerNum {}

pub impl i8:   IntegerNum {}
pub impl i16:  IntegerNum {}
pub impl i32:  IntegerNum {}
pub impl i64:  IntegerNum {}
pub impl int:  IntegerNum {}


trait FloatNum: NumExt, FloatConsts, FuzzyEq {}

pub impl f32:   FloatNum {}
pub impl f64:   FloatNum {}
pub impl float: FloatNum {}