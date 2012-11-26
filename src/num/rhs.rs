/**
 * These RHS operator traits would formalise the overloading of the RHS of the 
 * Num operators (as described in [Refining Traits and Impls]
 * (http://smallcultfollowing.com/babysteps/blog/2012/10/04/refining-traits-slash-impls/)
 * by nmatsakis). For example, if you wanted to achieve something like:
 *
 * ~~~
 * enum Radians<T> = T;
 *
 * let half_rotation = Radians(float::consts::pi);
 * let full_rotation = 2.0 * half_rotation;
 * let quarter_rotation = half_rotation * 0.5;
 * ~~~
 *
 * All you'd have to implement is:
 *
 * ~~~
 * pub impl Radians<float>: float::MulRHS<Radians<float>> {
 *     pure fn int_add_rhs(lhs: &int) -> Radians<float> { Radians(lhs * (*self)) }
 * }
 *
 * pub impl<T:Copy Num> Radians<T>: Mul<T, Radians<T>> {
 *     pure fn mul(rhs: &T) -> Radians<T> { Radians((*self) * (*rhs)) }
 * }
 * ~~~
 *
 * I may have got something wrong in my implementations, as I got some 'multiple
 * applicable methods in scope' errors due to Num automatically being imported
 * with core. If that's the case, apologies in advance!
 */

use core::Num;

pub mod f32 {
    pub trait AddRHS<Result>            { pure fn f32_add_rhs(lhs: &f32)        -> Result; }
    pub trait SubRHS<Result>            { pure fn f32_sub_rhs(lhs: &f32)        -> Result; }
    pub trait MulRHS<Result>            { pure fn f32_mul_rhs(lhs: &f32)        -> Result; }
    pub trait DivRHS<Result>            { pure fn f32_div_rhs(lhs: &f32)        -> Result; }
    pub trait ModuloRHS<Result>         { pure fn f32_modulo_rhs(lhs: &f32)     -> Result; }
    pub trait BitAndRHS<Result>         { pure fn f32_bitand_rhs(lhs: &f32)     -> Result; }
    pub trait BitOrRHS<Result>          { pure fn f32_bitor_rhs(lhs: &f32 )     -> Result; }
    pub trait BitXorRHS<Result>         { pure fn f32_bitxor_rhs(lhs: &f32)     -> Result; }
    pub trait ShlRHS<Result>            { pure fn f32_shl_rhs(lhs: &f32)        -> Result; }
    pub trait ShrRHS<Result>            { pure fn f32_shr_rhs(lhs: &f32)        -> Result; }

    pub impl f32: AddRHS<f32>           { pure fn f32_add_rhs(lhs: &f32)        -> f32 { *lhs + self } }
    pub impl f32: SubRHS<f32>           { pure fn f32_sub_rhs(lhs: &f32)        -> f32 { *lhs - self } }
    pub impl f32: MulRHS<f32>           { pure fn f32_mul_rhs(lhs: &f32)        -> f32 { *lhs * self } }
    pub impl f32: DivRHS<f32>           { pure fn f32_div_rhs(lhs: &f32)        -> f32 { *lhs / self } }
    pub impl f32: ModuloRHS<f32>        { pure fn f32_modulo_rhs(lhs: &f32)     -> f32 { *lhs % self } }

    // pub impl f32: Num {
    //     pure fn add(other: &f32)    -> f32 { other.f32_add_rhs(&self) }
    //     pure fn sub(other: &f32)    -> f32 { other.f32_sub_rhs(&self) }
    //     pure fn mul(other: &f32)    -> f32 { other.f32_mul_rhs(&self) }
    //     pure fn div(other: &f32)    -> f32 { other.f32_div_rhs(&self) }
    //     pure fn modulo(other: &f32) -> f32 { other.f32_modulo_rhs(&self) }
    //     pure fn neg()               -> f32 { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> f32 { n as f32 }
    // }
}

pub mod f64 {
    pub trait AddRHS<Result>            { pure fn f64_add_rhs(lhs: &f64)        -> Result; }
    pub trait SubRHS<Result>            { pure fn f64_sub_rhs(lhs: &f64)        -> Result; }
    pub trait MulRHS<Result>            { pure fn f64_mul_rhs(lhs: &f64)        -> Result; }
    pub trait DivRHS<Result>            { pure fn f64_div_rhs(lhs: &f64)        -> Result; }
    pub trait ModuloRHS<Result>         { pure fn f64_modulo_rhs(lhs: &f64)     -> Result; }
    pub trait BitAndRHS<Result>         { pure fn f64_bitand_rhs(lhs: &f64)     -> Result; }
    pub trait BitOrRHS<Result>          { pure fn f64_bitor_rhs(lhs: &f64)      -> Result; }
    pub trait BitXorRHS<Result>         { pure fn f64_bitxor_rhs(lhs: &f64)     -> Result; }
    pub trait ShlRHS<Result>            { pure fn f64_shl_rhs(lhs: &f64)        -> Result; }
    pub trait ShrRHS<Result>            { pure fn f64_shr_rhs(lhs: &f64)        -> Result; }

    pub impl f64: AddRHS<f64>           { pure fn f64_add_rhs(lhs: &f64)        -> f64 { *lhs + self } }
    pub impl f64: SubRHS<f64>           { pure fn f64_sub_rhs(lhs: &f64)        -> f64 { *lhs - self } }
    pub impl f64: MulRHS<f64>           { pure fn f64_mul_rhs(lhs: &f64)        -> f64 { *lhs * self } }
    pub impl f64: DivRHS<f64>           { pure fn f64_div_rhs(lhs: &f64)        -> f64 { *lhs / self } }
    pub impl f64: ModuloRHS<f64>        { pure fn f64_modulo_rhs(lhs: &f64)     -> f64 { *lhs % self } }

    // pub impl f64: Num {
    //     pure fn add(other: &f64)    -> f64 { other.f64_add_rhs(&self) }
    //     pure fn sub(other: &f64)    -> f64 { other.f64_sub_rhs(&self) }
    //     pure fn mul(other: &f64)    -> f64 { other.f64_mul_rhs(&self) }
    //     pure fn div(other: &f64)    -> f64 { other.f64_div_rhs(&self) }
    //     pure fn modulo(other: &f64) -> f64 { other.f64_modulo_rhs(&self) }
    //     pure fn neg()               -> f64 { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> f64 { n as f64 }
    // }
}

pub mod float {
    pub trait AddRHS<Result>          { pure fn float_add_rhs(lhs: &float)    -> Result; }
    pub trait SubRHS<Result>          { pure fn float_sub_rhs(lhs: &float)    -> Result; }
    pub trait MulRHS<Result>          { pure fn float_mul_rhs(lhs: &float)    -> Result; }
    pub trait DivRHS<Result>          { pure fn float_div_rhs(lhs: &float)    -> Result; }
    pub trait ModuloRHS<Result>       { pure fn float_modulo_rhs(lhs: &float) -> Result; }
    pub trait BitAndRHS<Result>       { pure fn float_bitand_rhs(lhs: &float) -> Result; }
    pub trait BitOrRHS<Result>        { pure fn float_bitor_rhs(lhs: &float)  -> Result; }
    pub trait BitXorRHS<Result>       { pure fn float_bitxor_rhs(lhs: &float) -> Result; }
    pub trait ShlRHS<Result>          { pure fn float_shl_rhs(lhs: &float)    -> Result; }
    pub trait ShrRHS<Result>          { pure fn float_shr_rhs(lhs: &float)    -> Result; }

    pub impl float: AddRHS<float>     { pure fn float_add_rhs(lhs: &float)    -> float { *lhs + self } }
    pub impl float: SubRHS<float>     { pure fn float_sub_rhs(lhs: &float)    -> float { *lhs - self } }
    pub impl float: MulRHS<float>     { pure fn float_mul_rhs(lhs: &float)    -> float { *lhs * self } }
    pub impl float: DivRHS<float>     { pure fn float_div_rhs(lhs: &float)    -> float { *lhs / self } }
    pub impl float: ModuloRHS<float>  { pure fn float_modulo_rhs(lhs: &float) -> float { *lhs % self } }

    // pub impl float: Num {
    //     pure fn add(other: &float)    -> float { other.float_add_rhs(&self) }
    //     pure fn sub(other: &float)    -> float { other.float_sub_rhs(&self) }
    //     pure fn mul(other: &float)    -> float { other.float_mul_rhs(&self) }
    //     pure fn div(other: &float)    -> float { other.float_div_rhs(&self) }
    //     pure fn modulo(other: &float) -> float { other.float_modulo_rhs(&self) }
    //     pure fn neg()                 -> float { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> float { n as float }
    // }
}

pub mod i8 {
    pub trait AddRHS<Result>         { pure fn i8_add_rhs(lhs: &i8)          -> Result; }
    pub trait SubRHS<Result>         { pure fn i8_sub_rhs(lhs: &i8)          -> Result; }
    pub trait MulRHS<Result>         { pure fn i8_mul_rhs(lhs: &i8)          -> Result; }
    pub trait DivRHS<Result>         { pure fn i8_div_rhs(lhs: &i8)          -> Result; }
    pub trait ModuloRHS<Result>      { pure fn i8_modulo_rhs(lhs: &i8)       -> Result; }
    pub trait BitAndRHS<Result>      { pure fn i8_bitand_rhs(lhs: &i8)       -> Result; }
    pub trait BitOrRHS<Result>       { pure fn i8_bitor_rhs(lhs: &i8)        -> Result; }
    pub trait BitXorRHS<Result>      { pure fn i8_bitxor_rhs(lhs: &i8)       -> Result; }
    pub trait ShlRHS<Result>         { pure fn i8_shl_rhs(lhs: &i8)          -> Result; }
    pub trait ShrRHS<Result>         { pure fn i8_shr_rhs(lhs: &i8)          -> Result; }

    pub impl i8 : AddRHS<i8>         { pure fn i8_add_rhs(lhs: &i8)          -> i8  { *lhs + self } }
    pub impl i8 : SubRHS<i8>         { pure fn i8_sub_rhs(lhs: &i8)          -> i8  { *lhs - self } }
    pub impl i8 : MulRHS<i8>         { pure fn i8_mul_rhs(lhs: &i8)          -> i8  { *lhs * self } }
    pub impl i8 : DivRHS<i8>         { pure fn i8_div_rhs(lhs: &i8)          -> i8  { *lhs / self } }
    pub impl i8 : ModuloRHS<i8>      { pure fn i8_modulo_rhs(lhs: &i8)       -> i8  { *lhs % self } }

    // pub impl i8: Num {
    //     pure fn add(other: &i8)    -> i8 { other.i8_add_rhs(&self) }
    //     pure fn sub(other: &i8)    -> i8 { other.i8_sub_rhs(&self) }
    //     pure fn mul(other: &i8)    -> i8 { other.i8_mul_rhs(&self) }
    //     pure fn div(other: &i8)    -> i8 { other.i8_div_rhs(&self) }
    //     pure fn modulo(other: &i8) -> i8 { other.i8_modulo_rhs(&self) }
    //     pure fn neg()              -> i8 { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> i8 { n as i8 }
    // }
}

pub mod i16 {
    pub trait AddRHS<Result>        { pure fn i16_add_rhs(lhs: &i16)        -> Result; }
    pub trait SubRHS<Result>        { pure fn i16_sub_rhs(lhs: &i16)        -> Result; }
    pub trait MulRHS<Result>        { pure fn i16_mul_rhs(lhs: &i16)        -> Result; }
    pub trait DivRHS<Result>        { pure fn i16_div_rhs(lhs: &i16)        -> Result; }
    pub trait ModuloRHS<Result>     { pure fn i16_modulo_rhs(lhs: &i16)     -> Result; }
    pub trait BitAndRHS<Result>     { pure fn i16_bitand_rhs(lhs: &i16)     -> Result; }
    pub trait BitOrRHS<Result>      { pure fn i16_bitor_rhs(lhs: &i16)      -> Result; }
    pub trait BitXorRHS<Result>     { pure fn i16_bitxor_rhs(lhs: &i16)     -> Result; }
    pub trait ShlRHS<Result>        { pure fn i16_shl_rhs(lhs: &i16)        -> Result; }
    pub trait ShrRHS<Result>        { pure fn i16_shr_rhs(lhs: &i16)        -> Result; }

    pub impl i16: AddRHS<i16>       { pure fn i16_add_rhs(lhs: &i16)        -> i16 { *lhs + self } }
    pub impl i16: SubRHS<i16>       { pure fn i16_sub_rhs(lhs: &i16)        -> i16 { *lhs - self } }
    pub impl i16: MulRHS<i16>       { pure fn i16_mul_rhs(lhs: &i16)        -> i16 { *lhs * self } }
    pub impl i16: DivRHS<i16>       { pure fn i16_div_rhs(lhs: &i16)        -> i16 { *lhs / self } }
    pub impl i16: ModuloRHS<i16>    { pure fn i16_modulo_rhs(lhs: &i16)     -> i16 { *lhs % self } }

    // pub impl i16: Num {
    //     pure fn add(other: &i16)    -> i16 { other.i16_add_rhs(&self) }
    //     pure fn sub(other: &i16)    -> i16 { other.i16_sub_rhs(&self) }
    //     pure fn mul(other: &i16)    -> i16 { other.i16_mul_rhs(&self) }
    //     pure fn div(other: &i16)    -> i16 { other.i16_div_rhs(&self) }
    //     pure fn modulo(other: &i16) -> i16 { other.i16_modulo_rhs(&self) }
    //     pure fn neg()               -> i16 { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> i16 { n as i16 }
    // }
}

pub mod i32 {
    pub trait AddRHS<Result>        { pure fn i32_add_rhs(lhs: &i32)        -> Result; }
    pub trait SubRHS<Result>        { pure fn i32_sub_rhs(lhs: &i32)        -> Result; }
    pub trait MulRHS<Result>        { pure fn i32_mul_rhs(lhs: &i32)        -> Result; }
    pub trait DivRHS<Result>        { pure fn i32_div_rhs(lhs: &i32)        -> Result; }
    pub trait ModuloRHS<Result>     { pure fn i32_modulo_rhs(lhs: &i32)     -> Result; }
    pub trait BitAndRHS<Result>     { pure fn i32_bitand_rhs(lhs: &i32)     -> Result; }
    pub trait BitOrRHS<Result>      { pure fn i32_bitor_rhs(lhs: &i32)      -> Result; }
    pub trait BitXorRHS<Result>     { pure fn i32_bitxor_rhs(lhs: &i32)     -> Result; }
    pub trait ShlRHS<Result>        { pure fn i32_shl_rhs(lhs: &i32)        -> Result; }
    pub trait ShrRHS<Result>        { pure fn i32_shr_rhs(lhs: &i32)        -> Result; }

    pub impl i32: AddRHS<i32>       { pure fn i32_add_rhs(lhs: &i32)        -> i32 { *lhs + self } }
    pub impl i32: SubRHS<i32>       { pure fn i32_sub_rhs(lhs: &i32)        -> i32 { *lhs - self } }
    pub impl i32: MulRHS<i32>       { pure fn i32_mul_rhs(lhs: &i32)        -> i32 { *lhs * self } }
    pub impl i32: DivRHS<i32>       { pure fn i32_div_rhs(lhs: &i32)        -> i32 { *lhs / self } }
    pub impl i32: ModuloRHS<i32>    { pure fn i32_modulo_rhs(lhs: &i32)     -> i32 { *lhs % self } }

    // pub impl i32: Num {
    //     pure fn add(other: &i32)    -> i32 { other.i32_add_rhs(&self) }
    //     pure fn sub(other: &i32)    -> i32 { other.i32_sub_rhs(&self) }
    //     pure fn mul(other: &i32)    -> i32 { other.i32_mul_rhs(&self) }
    //     pure fn div(other: &i32)    -> i32 { other.i32_div_rhs(&self) }
    //     pure fn modulo(other: &i32) -> i32 { other.i32_modulo_rhs(&self) }
    //     pure fn neg()               -> i32 { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> i32 { n as i32 }
    // }
}

pub mod i64 {
    pub trait AddRHS<Result>        { pure fn i64_add_rhs(lhs: &i64)        -> Result; }
    pub trait SubRHS<Result>        { pure fn i64_sub_rhs(lhs: &i64)        -> Result; }
    pub trait MulRHS<Result>        { pure fn i64_mul_rhs(lhs: &i64)        -> Result; }
    pub trait DivRHS<Result>        { pure fn i64_div_rhs(lhs: &i64)        -> Result; }
    pub trait ModuloRHS<Result>     { pure fn i64_modulo_rhs(lhs: &i64)     -> Result; }
    pub trait BitAndRHS<Result>     { pure fn i64_bitand_rhs(lhs: &i64)     -> Result; }
    pub trait BitOrRHS<Result>      { pure fn i64_bitor_rhs(lhs: &i64)      -> Result; }
    pub trait BitXorRHS<Result>     { pure fn i64_bitxor_rhs(lhs: &i64)     -> Result; }
    pub trait ShlRHS<Result>        { pure fn i64_shl_rhs(lhs: &i64)        -> Result; }
    pub trait ShrRHS<Result>        { pure fn i64_shr_rhs(lhs: &i64)        -> Result; }

    pub impl i64: AddRHS<i64>       { pure fn i64_add_rhs(lhs: &i64)        -> i64 { *lhs + self } }
    pub impl i64: SubRHS<i64>       { pure fn i64_sub_rhs(lhs: &i64)        -> i64 { *lhs - self } }
    pub impl i64: MulRHS<i64>       { pure fn i64_mul_rhs(lhs: &i64)        -> i64 { *lhs * self } }
    pub impl i64: DivRHS<i64>       { pure fn i64_div_rhs(lhs: &i64)        -> i64 { *lhs / self } }
    pub impl i64: ModuloRHS<i64>    { pure fn i64_modulo_rhs(lhs: &i64)     -> i64 { *lhs % self } }

    // pub impl i64: Num {
    //     pure fn add(other: &i64)    -> i64 { other.i64_add_rhs(&self) }
    //     pure fn sub(other: &i64)    -> i64 { other.i64_sub_rhs(&self) }
    //     pure fn mul(other: &i64)    -> i64 { other.i64_mul_rhs(&self) }
    //     pure fn div(other: &i64)    -> i64 { other.i64_div_rhs(&self) }
    //     pure fn modulo(other: &i64) -> i64 { other.i64_modulo_rhs(&self) }
    //     pure fn neg()               -> i64 { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> i64 { n as i64 }
    // }
}

pub mod int {
    pub trait AddRHS<Result>        { pure fn int_add_rhs(lhs: &int)        -> Result; }
    pub trait SubRHS<Result>        { pure fn int_sub_rhs(lhs: &int)        -> Result; }
    pub trait MulRHS<Result>        { pure fn int_mul_rhs(lhs: &int)        -> Result; }
    pub trait DivRHS<Result>        { pure fn int_div_rhs(lhs: &int)        -> Result; }
    pub trait ModuloRHS<Result>     { pure fn int_modulo_rhs(lhs: &int)     -> Result; }
    pub trait BitAndRHS<Result>     { pure fn int_bitand_rhs(lhs: &int)     -> Result; }
    pub trait BitOrRHS<Result>      { pure fn int_bitor_rhs(lhs: &int)      -> Result; }
    pub trait BitXorRHS<Result>     { pure fn int_bitxor_rhs(lhs: &int)     -> Result; }
    pub trait ShlRHS<Result>        { pure fn int_shl_rhs(lhs: &int)        -> Result; }
    pub trait ShrRHS<Result>        { pure fn int_shr_rhs(lhs: &int)        -> Result; }

    pub impl int: AddRHS<int>       { pure fn int_add_rhs(lhs: &int)        -> int { *lhs + self } }
    pub impl int: SubRHS<int>       { pure fn int_sub_rhs(lhs: &int)        -> int { *lhs - self } }
    pub impl int: MulRHS<int>       { pure fn int_mul_rhs(lhs: &int)        -> int { *lhs * self } }
    pub impl int: DivRHS<int>       { pure fn int_div_rhs(lhs: &int)        -> int { *lhs / self } }
    pub impl int: ModuloRHS<int>    { pure fn int_modulo_rhs(lhs: &int)     -> int { *lhs % self } }

    // pub impl int: Num {
    //     pure fn add(other: &int)    -> int { other.int_add_rhs(&self) }
    //     pure fn sub(other: &int)    -> int { other.int_sub_rhs(&self) }
    //     pure fn mul(other: &int)    -> int { other.int_mul_rhs(&self) }
    //     pure fn div(other: &int)    -> int { other.int_div_rhs(&self) }
    //     pure fn modulo(other: &int) -> int { other.int_modulo_rhs(&self) }
    //     pure fn neg()               -> int { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> int { n }
    // }
}

pub mod u8 {
    pub trait AddRHS<Result>         { pure fn u8_add_rhs(lhs: &u8)          -> Result; }
    pub trait SubRHS<Result>         { pure fn u8_sub_rhs(lhs: &u8)          -> Result; }
    pub trait MulRHS<Result>         { pure fn u8_mul_rhs(lhs: &u8)          -> Result; }
    pub trait DivRHS<Result>         { pure fn u8_div_rhs(lhs: &u8)          -> Result; }
    pub trait ModuloRHS<Result>      { pure fn u8_modulo_rhs(lhs: &u8)       -> Result; }
    pub trait BitAndRHS<Result>      { pure fn u8_bitand_rhs(lhs: &u8)       -> Result; }
    pub trait BitOrRHS<Result>       { pure fn u8_bitor_rhs(lhs: &u8)        -> Result; }
    pub trait BitXorRHS<Result>      { pure fn u8_bitxor_rhs(lhs: &u8)       -> Result; }
    pub trait ShlRHS<Result>         { pure fn u8_shl_rhs(lhs: &u8)          -> Result; }
    pub trait ShrRHS<Result>         { pure fn u8_shr_rhs(lhs: &u8)          -> Result; }

    pub impl u8: AddRHS<u8>          { pure fn u8_add_rhs(lhs: &u8)          -> u8   { *lhs + self } }
    pub impl u8: SubRHS<u8>          { pure fn u8_sub_rhs(lhs: &u8)          -> u8   { *lhs - self } }
    pub impl u8: MulRHS<u8>          { pure fn u8_mul_rhs(lhs: &u8)          -> u8   { *lhs * self } }
    pub impl u8: DivRHS<u8>          { pure fn u8_div_rhs(lhs: &u8)          -> u8   { *lhs / self } }
    pub impl u8: ModuloRHS<u8>       { pure fn u8_modulo_rhs(lhs: &u8)       -> u8   { *lhs % self } }

    // pub impl u8: Num {
    //     pure fn add(other: &u8)    -> u8 { other.u8_add_rhs(&self) }
    //     pure fn sub(other: &u8)    -> u8 { other.u8_sub_rhs(&self) }
    //     pure fn mul(other: &u8)    -> u8 { other.u8_mul_rhs(&self) }
    //     pure fn div(other: &u8)    -> u8 { other.u8_div_rhs(&self) }
    //     pure fn modulo(other: &u8) -> u8 { other.u8_modulo_rhs(&self) }
    //     pure fn neg()              -> u8 { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> u8 { n as u8 }
    // }
}

pub mod u16 {
    pub trait AddRHS<Result>        { pure fn u16_add_rhs(lhs: &u16)        -> Result; }
    pub trait SubRHS<Result>        { pure fn u16_sub_rhs(lhs: &u16)        -> Result; }
    pub trait MulRHS<Result>        { pure fn u16_mul_rhs(lhs: &u16)        -> Result; }
    pub trait DivRHS<Result>        { pure fn u16_div_rhs(lhs: &u16)        -> Result; }
    pub trait ModuloRHS<Result>     { pure fn u16_modulo_rhs(lhs: &u16)     -> Result; }
    pub trait BitAndRHS<Result>     { pure fn u16_bitand_rhs(lhs: &u16)     -> Result; }
    pub trait BitOrRHS<Result>      { pure fn u16_bitor_rhs(lhs: &u16)      -> Result; }
    pub trait BitXorRHS<Result>     { pure fn u16_bitxor_rhs(lhs: &u16)     -> Result; }
    pub trait ShlRHS<Result>        { pure fn u16_shl_rhs(lhs: &u16)        -> Result; }
    pub trait ShrRHS<Result>        { pure fn u16_shr_rhs(lhs: &u16)        -> Result; }

    pub impl u16: AddRHS<u16>       { pure fn u16_add_rhs(lhs: &u16)        -> u16  { *lhs + self } }
    pub impl u16: SubRHS<u16>       { pure fn u16_sub_rhs(lhs: &u16)        -> u16  { *lhs - self } }
    pub impl u16: MulRHS<u16>       { pure fn u16_mul_rhs(lhs: &u16)        -> u16  { *lhs * self } }
    pub impl u16: DivRHS<u16>       { pure fn u16_div_rhs(lhs: &u16)        -> u16  { *lhs / self } }
    pub impl u16: ModuloRHS<u16>    { pure fn u16_modulo_rhs(lhs: &u16)     -> u16  { *lhs % self } }

    // pub impl u16: Num {
    //     pure fn add(other: &u16)    -> u16 { other.u16_add_rhs(&self) }
    //     pure fn sub(other: &u16)    -> u16 { other.u16_sub_rhs(&self) }
    //     pure fn mul(other: &u16)    -> u16 { other.u16_mul_rhs(&self) }
    //     pure fn div(other: &u16)    -> u16 { other.u16_div_rhs(&self) }
    //     pure fn modulo(other: &u16) -> u16 { other.u16_modulo_rhs(&self) }
    //     pure fn neg()               -> u16 { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> u16 { n as u16 }
    // }
}

pub mod u32 {
    pub trait AddRHS<Result>        { pure fn u32_add_rhs(lhs: &u32)        -> Result; }
    pub trait SubRHS<Result>        { pure fn u32_sub_rhs(lhs: &u32)        -> Result; }
    pub trait MulRHS<Result>        { pure fn u32_mul_rhs(lhs: &u32)        -> Result; }
    pub trait DivRHS<Result>        { pure fn u32_div_rhs(lhs: &u32)        -> Result; }
    pub trait ModuloRHS<Result>     { pure fn u32_modulo_rhs(lhs: &u32)     -> Result; }
    pub trait BitAndRHS<Result>     { pure fn u32_bitand_rhs(lhs: &u32)     -> Result; }
    pub trait BitOrRHS<Result>      { pure fn u32_bitor_rhs(lhs: &u32)      -> Result; }
    pub trait BitXorRHS<Result>     { pure fn u32_bitxor_rhs(lhs: &u32)     -> Result; }
    pub trait ShlRHS<Result>        { pure fn u32_shl_rhs(lhs: &u32)        -> Result; }
    pub trait ShrRHS<Result>        { pure fn u32_shr_rhs(lhs: &u32)        -> Result; }

    pub impl u32: AddRHS<u32>       { pure fn u32_add_rhs(lhs: &u32)        -> u32  { *lhs + self } }
    pub impl u32: SubRHS<u32>       { pure fn u32_sub_rhs(lhs: &u32)        -> u32  { *lhs - self } }
    pub impl u32: MulRHS<u32>       { pure fn u32_mul_rhs(lhs: &u32)        -> u32  { *lhs * self } }
    pub impl u32: DivRHS<u32>       { pure fn u32_div_rhs(lhs: &u32)        -> u32  { *lhs / self } }
    pub impl u32: ModuloRHS<u32>    { pure fn u32_modulo_rhs(lhs: &u32)     -> u32  { *lhs % self } }

    // pub impl u32: Num {
    //     pure fn add(other: &u32)    -> u32 { other.u32_add_rhs(&self) }
    //     pure fn sub(other: &u32)    -> u32 { other.u32_sub_rhs(&self) }
    //     pure fn mul(other: &u32)    -> u32 { other.u32_mul_rhs(&self) }
    //     pure fn div(other: &u32)    -> u32 { other.u32_div_rhs(&self) }
    //     pure fn modulo(other: &u32) -> u32 { other.u32_modulo_rhs(&self) }
    //     pure fn neg()               -> u32 { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> u32 { n as u32 }
    // }
}

pub mod u64 {
    pub trait AddRHS<Result>        { pure fn u64_add_rhs(lhs: &u64)        -> Result; }
    pub trait SubRHS<Result>        { pure fn u64_sub_rhs(lhs: &u64)        -> Result; }
    pub trait MulRHS<Result>        { pure fn u64_mul_rhs(lhs: &u64)        -> Result; }
    pub trait DivRHS<Result>        { pure fn u64_div_rhs(lhs: &u64)        -> Result; }
    pub trait ModuloRHS<Result>     { pure fn u64_modulo_rhs(lhs: &u64)     -> Result; }
    pub trait BitAndRHS<Result>     { pure fn u64_bitand_rhs(lhs: &u64)     -> Result; }
    pub trait BitOrRHS<Result>      { pure fn u64_bitor_rhs(lhs: &u64)      -> Result; }
    pub trait BitXorRHS<Result>     { pure fn u64_bitxor_rhs(lhs: &u64)     -> Result; }
    pub trait ShlRHS<Result>        { pure fn u64_shl_rhs(lhs: &u64)        -> Result; }
    pub trait ShrRHS<Result>        { pure fn u64_shr_rhs(lhs: &u64)        -> Result; }

    pub impl u64: AddRHS<u64>       { pure fn u64_add_rhs(lhs: &u64)        -> u64  { *lhs + self } }
    pub impl u64: SubRHS<u64>       { pure fn u64_sub_rhs(lhs: &u64)        -> u64  { *lhs - self } }
    pub impl u64: MulRHS<u64>       { pure fn u64_mul_rhs(lhs: &u64)        -> u64  { *lhs * self } }
    pub impl u64: DivRHS<u64>       { pure fn u64_div_rhs(lhs: &u64)        -> u64  { *lhs / self } }
    pub impl u64: ModuloRHS<u64>    { pure fn u64_modulo_rhs(lhs: &u64)     -> u64  { *lhs % self } }

    // pub impl u64: Num {
    //     pure fn add(other: &u64)    -> u64 { other.u64_add_rhs(&self) }
    //     pure fn sub(other: &u64)    -> u64 { other.u64_sub_rhs(&self) }
    //     pure fn mul(other: &u64)    -> u64 { other.u64_mul_rhs(&self) }
    //     pure fn div(other: &u64)    -> u64 { other.u64_div_rhs(&self) }
    //     pure fn modulo(other: &u64) -> u64 { other.u64_modulo_rhs(&self) }
    //     pure fn neg()               -> u64 { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> u64 { n as u64 }
    // }
}

pub mod uint {
    pub trait AddRHS<Result>       { pure fn uint_add_rhs(lhs: &uint)      -> Result; }
    pub trait SubRHS<Result>       { pure fn uint_sub_rhs(lhs: &uint)      -> Result; }
    pub trait MulRHS<Result>       { pure fn uint_mul_rhs(lhs: &uint)      -> Result; }
    pub trait DivRHS<Result>       { pure fn uint_div_rhs(lhs: &uint)      -> Result; }
    pub trait ModuloRHS<Result>    { pure fn uint_modulo_rhs(lhs: &uint)   -> Result; }
    pub trait BitAndRHS<Result>    { pure fn uint_bitand_rhs(lhs: &uint)   -> Result; }
    pub trait BitOrRHS<Result>     { pure fn uint_bitor_rhs(lhs: &uint)    -> Result; }
    pub trait BitXorRHS<Result>    { pure fn uint_bitxor_rhs(lhs: &uint)   -> Result; }
    pub trait ShlRHS<Result>       { pure fn uint_shl_rhs(lhs: &uint)      -> Result; }
    pub trait ShrRHS<Result>       { pure fn uint_shr_rhs(lhs: &uint)      -> Result; }

    pub impl uint: AddRHS<uint>    { pure fn uint_add_rhs(lhs: &uint)      -> uint { *lhs + self } }
    pub impl uint: SubRHS<uint>    { pure fn uint_sub_rhs(lhs: &uint)      -> uint { *lhs - self } }
    pub impl uint: MulRHS<uint>    { pure fn uint_mul_rhs(lhs: &uint)      -> uint { *lhs * self } }
    pub impl uint: DivRHS<uint>    { pure fn uint_div_rhs(lhs: &uint)      -> uint { *lhs / self } }
    pub impl uint: ModuloRHS<uint> { pure fn uint_modulo_rhs(lhs: &uint)   -> uint { *lhs % self } }

    // pub impl uint: Num {
    //     pure fn add(other: &uint)    -> uint { other.uint_add_rhs(&self) }
    //     pure fn sub(other: &uint)    -> uint { other.uint_sub_rhs(&self) }
    //     pure fn mul(other: &uint)    -> uint { other.uint_mul_rhs(&self) }
    //     pure fn div(other: &uint)    -> uint { other.uint_div_rhs(&self) }
    //     pure fn modulo(other: &uint) -> uint { other.uint_modulo_rhs(&self) }
    //     pure fn neg()                -> uint { -self }

    //     pure fn to_int() -> int { self as int }
    //     static pure fn from_int(n: int) -> uint { n as uint }
    // }
}