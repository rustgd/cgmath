/**
 * Various traits intended to be used with the built in numeric types. These
 * allow one to be more specific with trait bounds when using generics.
 *
 * Note: These traits won't be able to be used to their full potential until
 * trait inheritence is implemented.
 */

use cmp::{Eq, Ord};
use std::cmp::FuzzyEq;
use ncast::*;
use nconsts::{IntConsts, FloatConsts};

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


pub impl u8    : Add<u8,u8>        { #[inline(always)] pure fn add(rhs: &u8)    -> u8    { self + *rhs } }
pub impl u16   : Add<u16,u16>      { #[inline(always)] pure fn add(rhs: &u16)   -> u16   { self + *rhs } }
pub impl u32   : Add<u32,u32>      { #[inline(always)] pure fn add(rhs: &u32)   -> u32   { self + *rhs } }
pub impl u64   : Add<u64,u64>      { #[inline(always)] pure fn add(rhs: &u64)   -> u64   { self + *rhs } }
pub impl uint  : Add<uint,uint>    { #[inline(always)] pure fn add(rhs: &uint)  -> uint  { self + *rhs } }

pub impl i8    : Add<i8,i8>        { #[inline(always)] pure fn add(rhs: &i8)    -> i8    { self + *rhs } }
pub impl i16   : Add<i16,i16>      { #[inline(always)] pure fn add(rhs: &i16)   -> i16   { self + *rhs } }
pub impl i32   : Add<i32,i32>      { #[inline(always)] pure fn add(rhs: &i32)   -> i32   { self + *rhs } }
pub impl i64   : Add<i64,i64>      { #[inline(always)] pure fn add(rhs: &i64)   -> i64   { self + *rhs } }
pub impl int   : Add<int,int>      { #[inline(always)] pure fn add(rhs: &int)   -> int   { self + *rhs } }

pub impl f32   : Add<f32,f32>      { #[inline(always)] pure fn add(rhs: &f32)   -> f32   { self + *rhs } }
pub impl f64   : Add<f64,f64>      { #[inline(always)] pure fn add(rhs: &f64)   -> f64   { self + *rhs } }
pub impl float : Add<float,float>  { #[inline(always)] pure fn add(rhs: &float) -> float { self + *rhs } }


pub impl u8    : Sub<u8,u8>        { #[inline(always)] pure fn sub(rhs: &u8)    -> u8    { self - *rhs } }
pub impl u16   : Sub<u16,u16>      { #[inline(always)] pure fn sub(rhs: &u16)   -> u16   { self - *rhs } }
pub impl u32   : Sub<u32,u32>      { #[inline(always)] pure fn sub(rhs: &u32)   -> u32   { self - *rhs } }
pub impl u64   : Sub<u64,u64>      { #[inline(always)] pure fn sub(rhs: &u64)   -> u64   { self - *rhs } }
pub impl uint  : Sub<uint,uint>    { #[inline(always)] pure fn sub(rhs: &uint)  -> uint  { self - *rhs } }

pub impl i8    : Sub<i8,i8>        { #[inline(always)] pure fn sub(rhs: &i8)    -> i8    { self - *rhs } }
pub impl i16   : Sub<i16,i16>      { #[inline(always)] pure fn sub(rhs: &i16)   -> i16   { self - *rhs } }
pub impl i32   : Sub<i32,i32>      { #[inline(always)] pure fn sub(rhs: &i32)   -> i32   { self - *rhs } }
pub impl i64   : Sub<i64,i64>      { #[inline(always)] pure fn sub(rhs: &i64)   -> i64   { self - *rhs } }
pub impl int   : Sub<int,int>      { #[inline(always)] pure fn sub(rhs: &int)   -> int   { self - *rhs } }

pub impl f32   : Sub<f32,f32>      { #[inline(always)] pure fn sub(rhs: &f32)   -> f32   { self - *rhs } }
pub impl f64   : Sub<f64,f64>      { #[inline(always)] pure fn sub(rhs: &f64)   -> f64   { self - *rhs } }
pub impl float : Sub<float,float>  { #[inline(always)] pure fn sub(rhs: &float) -> float { self - *rhs } }


pub impl u8    : Mul<u8,u8>        { #[inline(always)] pure fn mul(rhs: &u8)    -> u8    { self * *rhs } }
pub impl u16   : Mul<u16,u16>      { #[inline(always)] pure fn mul(rhs: &u16)   -> u16   { self * *rhs } }
pub impl u32   : Mul<u32,u32>      { #[inline(always)] pure fn mul(rhs: &u32)   -> u32   { self * *rhs } }
pub impl u64   : Mul<u64,u64>      { #[inline(always)] pure fn mul(rhs: &u64)   -> u64   { self * *rhs } }
pub impl uint  : Mul<uint,uint>    { #[inline(always)] pure fn mul(rhs: &uint)  -> uint  { self * *rhs } }

pub impl i8    : Mul<i8,i8>        { #[inline(always)] pure fn mul(rhs: &i8)    -> i8    { self * *rhs } }
pub impl i16   : Mul<i16,i16>      { #[inline(always)] pure fn mul(rhs: &i16)   -> i16   { self * *rhs } }
pub impl i32   : Mul<i32,i32>      { #[inline(always)] pure fn mul(rhs: &i32)   -> i32   { self * *rhs } }
pub impl i64   : Mul<i64,i64>      { #[inline(always)] pure fn mul(rhs: &i64)   -> i64   { self * *rhs } }
pub impl int   : Mul<int,int>      { #[inline(always)] pure fn mul(rhs: &int)   -> int   { self * *rhs } }

pub impl f32   : Mul<f32,f32>      { #[inline(always)] pure fn mul(rhs: &f32)   -> f32   { self * *rhs } }
pub impl f64   : Mul<f64,f64>      { #[inline(always)] pure fn mul(rhs: &f64)   -> f64   { self * *rhs } }
pub impl float : Mul<float,float>  { #[inline(always)] pure fn mul(rhs: &float) -> float { self * *rhs } }


pub impl u8    : Div<u8,u8>        { #[inline(always)] pure fn div(rhs: &u8)    -> u8    { self / *rhs } }
pub impl u16   : Div<u16,u16>      { #[inline(always)] pure fn div(rhs: &u16)   -> u16   { self / *rhs } }
pub impl u32   : Div<u32,u32>      { #[inline(always)] pure fn div(rhs: &u32)   -> u32   { self / *rhs } }
pub impl u64   : Div<u64,u64>      { #[inline(always)] pure fn div(rhs: &u64)   -> u64   { self / *rhs } }
pub impl uint  : Div<uint,uint>    { #[inline(always)] pure fn div(rhs: &uint)  -> uint  { self / *rhs } }

pub impl i8    : Div<i8,i8>        { #[inline(always)] pure fn div(rhs: &i8)    -> i8    { self / *rhs } }
pub impl i16   : Div<i16,i16>      { #[inline(always)] pure fn div(rhs: &i16)   -> i16   { self / *rhs } }
pub impl i32   : Div<i32,i32>      { #[inline(always)] pure fn div(rhs: &i32)   -> i32   { self / *rhs } }
pub impl i64   : Div<i64,i64>      { #[inline(always)] pure fn div(rhs: &i64)   -> i64   { self / *rhs } }
pub impl int   : Div<int,int>      { #[inline(always)] pure fn div(rhs: &int)   -> int   { self / *rhs } }

pub impl f32   : Div<f32,f32>      { #[inline(always)] pure fn div(rhs: &f32)   -> f32   { self / *rhs } }
pub impl f64   : Div<f64,f64>      { #[inline(always)] pure fn div(rhs: &f64)   -> f64   { self / *rhs } }
pub impl float : Div<float,float>  { #[inline(always)] pure fn div(rhs: &float) -> float { self / *rhs } }


pub impl u8    : Modulo<u8,u8>        { #[inline(always)] pure fn modulo(rhs: &u8)    -> u8    { self % *rhs } }
pub impl u16   : Modulo<u16,u16>      { #[inline(always)] pure fn modulo(rhs: &u16)   -> u16   { self % *rhs } }
pub impl u32   : Modulo<u32,u32>      { #[inline(always)] pure fn modulo(rhs: &u32)   -> u32   { self % *rhs } }
pub impl u64   : Modulo<u64,u64>      { #[inline(always)] pure fn modulo(rhs: &u64)   -> u64   { self % *rhs } }
pub impl uint  : Modulo<uint,uint>    { #[inline(always)] pure fn modulo(rhs: &uint)  -> uint  { self % *rhs } }

pub impl i8    : Modulo<i8,i8>        { #[inline(always)] pure fn modulo(rhs: &i8)    -> i8    { self % *rhs } }
pub impl i16   : Modulo<i16,i16>      { #[inline(always)] pure fn modulo(rhs: &i16)   -> i16   { self % *rhs } }
pub impl i32   : Modulo<i32,i32>      { #[inline(always)] pure fn modulo(rhs: &i32)   -> i32   { self % *rhs } }
pub impl i64   : Modulo<i64,i64>      { #[inline(always)] pure fn modulo(rhs: &i64)   -> i64   { self % *rhs } }
pub impl int   : Modulo<int,int>      { #[inline(always)] pure fn modulo(rhs: &int)   -> int   { self % *rhs } }

pub impl f32   : Modulo<f32,f32>      { #[inline(always)] pure fn modulo(rhs: &f32)   -> f32   { self % *rhs } }
pub impl f64   : Modulo<f64,f64>      { #[inline(always)] pure fn modulo(rhs: &f64)   -> f64   { self % *rhs } }
pub impl float : Modulo<float,float>  { #[inline(always)] pure fn modulo(rhs: &float) -> float { self % *rhs } }


pub impl i8    : Neg<i8>    { #[inline(always)] pure fn neg() -> i8    { -self } }
pub impl i16   : Neg<i16>   { #[inline(always)] pure fn neg() -> i16   { -self } }
pub impl i32   : Neg<i32>   { #[inline(always)] pure fn neg() -> i32   { -self } }
pub impl i64   : Neg<i64>   { #[inline(always)] pure fn neg() -> i64   { -self } }
pub impl int   : Neg<int>   { #[inline(always)] pure fn neg() -> int   { -self } }

pub impl f32   : Neg<f32>   { #[inline(always)] pure fn neg() -> f32   { -self } }
pub impl f64   : Neg<f64>   { #[inline(always)] pure fn neg() -> f64   { -self } }
pub impl float : Neg<float> { #[inline(always)] pure fn neg() -> float { -self } }