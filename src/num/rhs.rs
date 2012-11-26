/// FIXME:
/// These RHS operator traits would make overloading the RHS of operators (described in
/// http://smallcultfollowing.com/babysteps/blog/2012/10/04/refining-traits-slash-impls/)
/// much easier. For example, if I wanted to do something like: `int * Degrees<int>`, all
/// I'd have to do is implement Int_MulRHS<Degrees<int>>. Unfortunately because the Num
/// implementations for the base types are automatically imported from core, trying to
/// actually use this causes some ugly 'multiple applicable method' errors.

use core::Num;

// Floats

pub trait F32_AddRHS<Result>            { pure fn f32_add_rhs(lhs: &f32)        -> Result; }
pub trait F32_SubRHS<Result>            { pure fn f32_sub_rhs(lhs: &f32)        -> Result; }
pub trait F32_MulRHS<Result>            { pure fn f32_mul_rhs(lhs: &f32)        -> Result; }
pub trait F32_DivRHS<Result>            { pure fn f32_div_rhs(lhs: &f32)        -> Result; }
pub trait F32_ModuloRHS<Result>         { pure fn f32_modulo_rhs(lhs: &f32)     -> Result; }
pub trait F32_BitAndRHS<Result>         { pure fn f32_bitand_rhs(lhs: &f32)     -> Result; }
pub trait F32_BitOrRHS<Result>          { pure fn f32_bitor_rhs(lhs: &f32 )     -> Result; }
pub trait F32_BitXorRHS<Result>         { pure fn f32_bitxor_rhs(lhs: &f32)     -> Result; }
pub trait F32_ShlRHS<Result>            { pure fn f32_shl_rhs(lhs: &f32)        -> Result; }
pub trait F32_ShrRHS<Result>            { pure fn f32_shr_rhs(lhs: &f32)        -> Result; }

pub trait F64_AddRHS<Result>            { pure fn f64_add_rhs(lhs: &f64)        -> Result; }
pub trait F64_SubRHS<Result>            { pure fn f64_sub_rhs(lhs: &f64)        -> Result; }
pub trait F64_MulRHS<Result>            { pure fn f64_mul_rhs(lhs: &f64)        -> Result; }
pub trait F64_DivRHS<Result>            { pure fn f64_div_rhs(lhs: &f64)        -> Result; }
pub trait F64_ModuloRHS<Result>         { pure fn f64_modulo_rhs(lhs: &f64)     -> Result; }
pub trait F64_BitAndRHS<Result>         { pure fn f64_bitand_rhs(lhs: &f64)     -> Result; }
pub trait F64_BitOrRHS<Result>          { pure fn f64_bitor_rhs(lhs: &f64)      -> Result; }
pub trait F64_BitXorRHS<Result>         { pure fn f64_bitxor_rhs(lhs: &f64)     -> Result; }
pub trait F64_ShlRHS<Result>            { pure fn f64_shl_rhs(lhs: &f64)        -> Result; }
pub trait F64_ShrRHS<Result>            { pure fn f64_shr_rhs(lhs: &f64)        -> Result; }

pub trait Float_AddRHS<Result>          { pure fn float_add_rhs(lhs: &float)    -> Result; }
pub trait Float_SubRHS<Result>          { pure fn float_sub_rhs(lhs: &float)    -> Result; }
pub trait Float_MulRHS<Result>          { pure fn float_mul_rhs(lhs: &float)    -> Result; }
pub trait Float_DivRHS<Result>          { pure fn float_div_rhs(lhs: &float)    -> Result; }
pub trait Float_ModuloRHS<Result>       { pure fn float_modulo_rhs(lhs: &float) -> Result; }
pub trait Float_BitAndRHS<Result>       { pure fn float_bitand_rhs(lhs: &float) -> Result; }
pub trait Float_BitOrRHS<Result>        { pure fn float_bitor_rhs(lhs: &float)  -> Result; }
pub trait Float_BitXorRHS<Result>       { pure fn float_bitxor_rhs(lhs: &float) -> Result; }
pub trait Float_ShlRHS<Result>          { pure fn float_shl_rhs(lhs: &float)    -> Result; }
pub trait Float_ShrRHS<Result>          { pure fn float_shr_rhs(lhs: &float)    -> Result; }

pub impl f32: F32_AddRHS<f32>           { pure fn f32_add_rhs(lhs: &f32)        -> f32 { *lhs + self } }
pub impl f32: F32_SubRHS<f32>           { pure fn f32_sub_rhs(lhs: &f32)        -> f32 { *lhs - self } }
pub impl f32: F32_MulRHS<f32>           { pure fn f32_mul_rhs(lhs: &f32)        -> f32 { *lhs * self } }
pub impl f32: F32_DivRHS<f32>           { pure fn f32_div_rhs(lhs: &f32)        -> f32 { *lhs / self } }
pub impl f32: F32_ModuloRHS<f32>        { pure fn f32_modulo_rhs(lhs: &f32)     -> f32 { *lhs % self } }

pub impl f64: F64_AddRHS<f64>           { pure fn f64_add_rhs(lhs: &f64)        -> f64 { *lhs + self } }
pub impl f64: F64_SubRHS<f64>           { pure fn f64_sub_rhs(lhs: &f64)        -> f64 { *lhs - self } }
pub impl f64: F64_MulRHS<f64>           { pure fn f64_mul_rhs(lhs: &f64)        -> f64 { *lhs * self } }
pub impl f64: F64_DivRHS<f64>           { pure fn f64_div_rhs(lhs: &f64)        -> f64 { *lhs / self } }
pub impl f64: F64_ModuloRHS<f64>        { pure fn f64_modulo_rhs(lhs: &f64)     -> f64 { *lhs % self } }

pub impl float: Float_AddRHS<float>     { pure fn float_add_rhs(lhs: &float)    -> float { *lhs + self } }
pub impl float: Float_SubRHS<float>     { pure fn float_sub_rhs(lhs: &float)    -> float { *lhs - self } }
pub impl float: Float_MulRHS<float>     { pure fn float_mul_rhs(lhs: &float)    -> float { *lhs * self } }
pub impl float: Float_DivRHS<float>     { pure fn float_div_rhs(lhs: &float)    -> float { *lhs / self } }
pub impl float: Float_ModuloRHS<float>  { pure fn float_modulo_rhs(lhs: &float) -> float { *lhs % self } }

/*

FIXME: see module description above

pub impl f32: Num {
    pure fn add(other: &f32)    -> f32 { other.f32_add_rhs(&self) }
    pure fn sub(other: &f32)    -> f32 { other.f32_sub_rhs(&self) }
    pure fn mul(other: &f32)    -> f32 { other.f32_mul_rhs(&self) }
    pure fn div(other: &f32)    -> f32 { other.f32_div_rhs(&self) }
    pure fn modulo(other: &f32) -> f32 { other.f32_modulo_rhs(&self) }
    pure fn neg()               -> f32 { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> f32 { n as f32 }
}

pub impl f64: Num {
    pure fn add(other: &f64)    -> f64 { other.f64_add_rhs(&self) }
    pure fn sub(other: &f64)    -> f64 { other.f64_sub_rhs(&self) }
    pure fn mul(other: &f64)    -> f64 { other.f64_mul_rhs(&self) }
    pure fn div(other: &f64)    -> f64 { other.f64_div_rhs(&self) }
    pure fn modulo(other: &f64) -> f64 { other.f64_modulo_rhs(&self) }
    pure fn neg()               -> f64 { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> f64 { n as f64 }
}

pub impl float: Num {
    pure fn add(other: &float)    -> float { other.float_add_rhs(&self) }
    pure fn sub(other: &float)    -> float { other.float_sub_rhs(&self) }
    pure fn mul(other: &float)    -> float { other.float_mul_rhs(&self) }
    pure fn div(other: &float)    -> float { other.float_div_rhs(&self) }
    pure fn modulo(other: &float) -> float { other.float_modulo_rhs(&self) }
    pure fn neg()                 -> float { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> float { n as float }
}
*/

// Integers

pub trait I8_AddRHS<Result>         { pure fn i8_add_rhs(lhs: &i8)          -> Result; }
pub trait I8_SubRHS<Result>         { pure fn i8_sub_rhs(lhs: &i8)          -> Result; }
pub trait I8_MulRHS<Result>         { pure fn i8_mul_rhs(lhs: &i8)          -> Result; }
pub trait I8_DivRHS<Result>         { pure fn i8_div_rhs(lhs: &i8)          -> Result; }
pub trait I8_ModuloRHS<Result>      { pure fn i8_modulo_rhs(lhs: &i8)       -> Result; }
pub trait I8_BitAndRHS<Result>      { pure fn i8_bitand_rhs(lhs: &i8)       -> Result; }
pub trait I8_BitOrRHS<Result>       { pure fn i8_bitor_rhs(lhs: &i8)        -> Result; }
pub trait I8_BitXorRHS<Result>      { pure fn i8_bitxor_rhs(lhs: &i8)       -> Result; }
pub trait I8_ShlRHS<Result>         { pure fn i8_shl_rhs(lhs: &i8)          -> Result; }
pub trait I8_ShrRHS<Result>         { pure fn i8_shr_rhs(lhs: &i8)          -> Result; }

pub trait I16_AddRHS<Result>        { pure fn i16_add_rhs(lhs: &i16)        -> Result; }
pub trait I16_SubRHS<Result>        { pure fn i16_sub_rhs(lhs: &i16)        -> Result; }
pub trait I16_MulRHS<Result>        { pure fn i16_mul_rhs(lhs: &i16)        -> Result; }
pub trait I16_DivRHS<Result>        { pure fn i16_div_rhs(lhs: &i16)        -> Result; }
pub trait I16_ModuloRHS<Result>     { pure fn i16_modulo_rhs(lhs: &i16)     -> Result; }
pub trait I16_BitAndRHS<Result>     { pure fn i16_bitand_rhs(lhs: &i16)     -> Result; }
pub trait I16_BitOrRHS<Result>      { pure fn i16_bitor_rhs(lhs: &i16)      -> Result; }
pub trait I16_BitXorRHS<Result>     { pure fn i16_bitxor_rhs(lhs: &i16)     -> Result; }
pub trait I16_ShlRHS<Result>        { pure fn i16_shl_rhs(lhs: &i16)        -> Result; }
pub trait I16_ShrRHS<Result>        { pure fn i16_shr_rhs(lhs: &i16)        -> Result; }

pub trait I32_AddRHS<Result>        { pure fn i32_add_rhs(lhs: &i32)        -> Result; }
pub trait I32_SubRHS<Result>        { pure fn i32_sub_rhs(lhs: &i32)        -> Result; }
pub trait I32_MulRHS<Result>        { pure fn i32_mul_rhs(lhs: &i32)        -> Result; }
pub trait I32_DivRHS<Result>        { pure fn i32_div_rhs(lhs: &i32)        -> Result; }
pub trait I32_ModuloRHS<Result>     { pure fn i32_modulo_rhs(lhs: &i32)     -> Result; }
pub trait I32_BitAndRHS<Result>     { pure fn i32_bitand_rhs(lhs: &i32)     -> Result; }
pub trait I32_BitOrRHS<Result>      { pure fn i32_bitor_rhs(lhs: &i32)      -> Result; }
pub trait I32_BitXorRHS<Result>     { pure fn i32_bitxor_rhs(lhs: &i32)     -> Result; }
pub trait I32_ShlRHS<Result>        { pure fn i32_shl_rhs(lhs: &i32)        -> Result; }
pub trait I32_ShrRHS<Result>        { pure fn i32_shr_rhs(lhs: &i32)        -> Result; }

pub trait I64_AddRHS<Result>        { pure fn i64_add_rhs(lhs: &i64)        -> Result; }
pub trait I64_SubRHS<Result>        { pure fn i64_sub_rhs(lhs: &i64)        -> Result; }
pub trait I64_MulRHS<Result>        { pure fn i64_mul_rhs(lhs: &i64)        -> Result; }
pub trait I64_DivRHS<Result>        { pure fn i64_div_rhs(lhs: &i64)        -> Result; }
pub trait I64_ModuloRHS<Result>     { pure fn i64_modulo_rhs(lhs: &i64)     -> Result; }
pub trait I64_BitAndRHS<Result>     { pure fn i64_bitand_rhs(lhs: &i64)     -> Result; }
pub trait I64_BitOrRHS<Result>      { pure fn i64_bitor_rhs(lhs: &i64)      -> Result; }
pub trait I64_BitXorRHS<Result>     { pure fn i64_bitxor_rhs(lhs: &i64)     -> Result; }
pub trait I64_ShlRHS<Result>        { pure fn i64_shl_rhs(lhs: &i64)        -> Result; }
pub trait I64_ShrRHS<Result>        { pure fn i64_shr_rhs(lhs: &i64)        -> Result; }

pub trait Int_AddRHS<Result>        { pure fn int_add_rhs(lhs: &int)        -> Result; }
pub trait Int_SubRHS<Result>        { pure fn int_sub_rhs(lhs: &int)        -> Result; }
pub trait Int_MulRHS<Result>        { pure fn int_mul_rhs(lhs: &int)        -> Result; }
pub trait Int_DivRHS<Result>        { pure fn int_div_rhs(lhs: &int)        -> Result; }
pub trait Int_ModuloRHS<Result>     { pure fn int_modulo_rhs(lhs: &int)     -> Result; }
pub trait Int_BitAndRHS<Result>     { pure fn int_bitand_rhs(lhs: &int)     -> Result; }
pub trait Int_BitOrRHS<Result>      { pure fn int_bitor_rhs(lhs: &int)      -> Result; }
pub trait Int_BitXorRHS<Result>     { pure fn int_bitxor_rhs(lhs: &int)     -> Result; }
pub trait Int_ShlRHS<Result>        { pure fn int_shl_rhs(lhs: &int)        -> Result; }
pub trait Int_ShrRHS<Result>        { pure fn int_shr_rhs(lhs: &int)        -> Result; }

pub impl i8 : I8_AddRHS<i8>         { pure fn i8_add_rhs(lhs: &i8)          -> i8  { *lhs + self } }
pub impl i8 : I8_SubRHS<i8>         { pure fn i8_sub_rhs(lhs: &i8)          -> i8  { *lhs - self } }
pub impl i8 : I8_MulRHS<i8>         { pure fn i8_mul_rhs(lhs: &i8)          -> i8  { *lhs * self } }
pub impl i8 : I8_DivRHS<i8>         { pure fn i8_div_rhs(lhs: &i8)          -> i8  { *lhs / self } }
pub impl i8 : I8_ModuloRHS<i8>      { pure fn i8_modulo_rhs(lhs: &i8)       -> i8  { *lhs % self } }

pub impl i16: I16_AddRHS<i16>       { pure fn i16_add_rhs(lhs: &i16)        -> i16 { *lhs + self } }
pub impl i16: I16_SubRHS<i16>       { pure fn i16_sub_rhs(lhs: &i16)        -> i16 { *lhs - self } }
pub impl i16: I16_MulRHS<i16>       { pure fn i16_mul_rhs(lhs: &i16)        -> i16 { *lhs * self } }
pub impl i16: I16_DivRHS<i16>       { pure fn i16_div_rhs(lhs: &i16)        -> i16 { *lhs / self } }
pub impl i16: I16_ModuloRHS<i16>    { pure fn i16_modulo_rhs(lhs: &i16)     -> i16 { *lhs % self } }

pub impl i32: I32_AddRHS<i32>       { pure fn i32_add_rhs(lhs: &i32)        -> i32 { *lhs + self } }
pub impl i32: I32_SubRHS<i32>       { pure fn i32_sub_rhs(lhs: &i32)        -> i32 { *lhs - self } }
pub impl i32: I32_MulRHS<i32>       { pure fn i32_mul_rhs(lhs: &i32)        -> i32 { *lhs * self } }
pub impl i32: I32_DivRHS<i32>       { pure fn i32_div_rhs(lhs: &i32)        -> i32 { *lhs / self } }
pub impl i32: I32_ModuloRHS<i32>    { pure fn i32_modulo_rhs(lhs: &i32)     -> i32 { *lhs % self } }

pub impl i64: I64_AddRHS<i64>       { pure fn i64_add_rhs(lhs: &i64)        -> i64 { *lhs + self } }
pub impl i64: I64_SubRHS<i64>       { pure fn i64_sub_rhs(lhs: &i64)        -> i64 { *lhs - self } }
pub impl i64: I64_MulRHS<i64>       { pure fn i64_mul_rhs(lhs: &i64)        -> i64 { *lhs * self } }
pub impl i64: I64_DivRHS<i64>       { pure fn i64_div_rhs(lhs: &i64)        -> i64 { *lhs / self } }
pub impl i64: I64_ModuloRHS<i64>    { pure fn i64_modulo_rhs(lhs: &i64)     -> i64 { *lhs % self } }

pub impl int: Int_AddRHS<int>       { pure fn int_add_rhs(lhs: &int)        -> int { *lhs + self } }
pub impl int: Int_SubRHS<int>       { pure fn int_sub_rhs(lhs: &int)        -> int { *lhs - self } }
pub impl int: Int_MulRHS<int>       { pure fn int_mul_rhs(lhs: &int)        -> int { *lhs * self } }
pub impl int: Int_DivRHS<int>       { pure fn int_div_rhs(lhs: &int)        -> int { *lhs / self } }
pub impl int: Int_ModuloRHS<int>    { pure fn int_modulo_rhs(lhs: &int)     -> int { *lhs % self } }

/*

FIXME: see module description above

pub impl i8: Num {
    pure fn add(other: &i8)    -> i8 { other.i8_add_rhs(&self) }
    pure fn sub(other: &i8)    -> i8 { other.i8_sub_rhs(&self) }
    pure fn mul(other: &i8)    -> i8 { other.i8_mul_rhs(&self) }
    pure fn div(other: &i8)    -> i8 { other.i8_div_rhs(&self) }
    pure fn modulo(other: &i8) -> i8 { other.i8_modulo_rhs(&self) }
    pure fn neg()              -> i8 { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> i8 { n as i8 }
}

pub impl i16: Num {
    pure fn add(other: &i16)    -> i16 { other.i16_add_rhs(&self) }
    pure fn sub(other: &i16)    -> i16 { other.i16_sub_rhs(&self) }
    pure fn mul(other: &i16)    -> i16 { other.i16_mul_rhs(&self) }
    pure fn div(other: &i16)    -> i16 { other.i16_div_rhs(&self) }
    pure fn modulo(other: &i16) -> i16 { other.i16_modulo_rhs(&self) }
    pure fn neg()               -> i16 { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> i16 { n as i16 }
}

pub impl i32: Num {
    pure fn add(other: &i32)    -> i32 { other.i32_add_rhs(&self) }
    pure fn sub(other: &i32)    -> i32 { other.i32_sub_rhs(&self) }
    pure fn mul(other: &i32)    -> i32 { other.i32_mul_rhs(&self) }
    pure fn div(other: &i32)    -> i32 { other.i32_div_rhs(&self) }
    pure fn modulo(other: &i32) -> i32 { other.i32_modulo_rhs(&self) }
    pure fn neg()               -> i32 { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> i32 { n as i32 }
}

pub impl i64: Num {
    pure fn add(other: &i64)    -> i64 { other.i64_add_rhs(&self) }
    pure fn sub(other: &i64)    -> i64 { other.i64_sub_rhs(&self) }
    pure fn mul(other: &i64)    -> i64 { other.i64_mul_rhs(&self) }
    pure fn div(other: &i64)    -> i64 { other.i64_div_rhs(&self) }
    pure fn modulo(other: &i64) -> i64 { other.i64_modulo_rhs(&self) }
    pure fn neg()               -> i64 { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> i64 { n as i64 }
}

pub impl int: Num {
    pure fn add(other: &int)    -> int { other.int_add_rhs(&self) }
    pure fn sub(other: &int)    -> int { other.int_sub_rhs(&self) }
    pure fn mul(other: &int)    -> int { other.int_mul_rhs(&self) }
    pure fn div(other: &int)    -> int { other.int_div_rhs(&self) }
    pure fn modulo(other: &int) -> int { other.int_modulo_rhs(&self) }
    pure fn neg()               -> int { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> int { n }
}
*/

// Unsigned integers

pub trait U8_AddRHS<Result>         { pure fn u8_add_rhs(lhs: &u8)          -> Result; }
pub trait U8_SubRHS<Result>         { pure fn u8_sub_rhs(lhs: &u8)          -> Result; }
pub trait U8_MulRHS<Result>         { pure fn u8_mul_rhs(lhs: &u8)          -> Result; }
pub trait U8_DivRHS<Result>         { pure fn u8_div_rhs(lhs: &u8)          -> Result; }
pub trait U8_ModuloRHS<Result>      { pure fn u8_modulo_rhs(lhs: &u8)       -> Result; }
pub trait U8_BitAndRHS<Result>      { pure fn u8_bitand_rhs(lhs: &u8)       -> Result; }
pub trait U8_BitOrRHS<Result>       { pure fn u8_bitor_rhs(lhs: &u8)        -> Result; }
pub trait U8_BitXorRHS<Result>      { pure fn u8_bitxor_rhs(lhs: &u8)       -> Result; }
pub trait U8_ShlRHS<Result>         { pure fn u8_shl_rhs(lhs: &u8)          -> Result; }
pub trait U8_ShrRHS<Result>         { pure fn u8_shr_rhs(lhs: &u8)          -> Result; }

pub trait U16_AddRHS<Result>        { pure fn u16_add_rhs(lhs: &u16)        -> Result; }
pub trait U16_SubRHS<Result>        { pure fn u16_sub_rhs(lhs: &u16)        -> Result; }
pub trait U16_MulRHS<Result>        { pure fn u16_mul_rhs(lhs: &u16)        -> Result; }
pub trait U16_DivRHS<Result>        { pure fn u16_div_rhs(lhs: &u16)        -> Result; }
pub trait U16_ModuloRHS<Result>     { pure fn u16_modulo_rhs(lhs: &u16)     -> Result; }
pub trait U16_BitAndRHS<Result>     { pure fn u16_bitand_rhs(lhs: &u16)     -> Result; }
pub trait U16_BitOrRHS<Result>      { pure fn u16_bitor_rhs(lhs: &u16)      -> Result; }
pub trait U16_BitXorRHS<Result>     { pure fn u16_bitxor_rhs(lhs: &u16)     -> Result; }
pub trait U16_ShlRHS<Result>        { pure fn u16_shl_rhs(lhs: &u16)        -> Result; }
pub trait U16_ShrRHS<Result>        { pure fn u16_shr_rhs(lhs: &u16)        -> Result; }

pub trait U32_AddRHS<Result>        { pure fn u32_add_rhs(lhs: &u32)        -> Result; }
pub trait U32_SubRHS<Result>        { pure fn u32_sub_rhs(lhs: &u32)        -> Result; }
pub trait U32_MulRHS<Result>        { pure fn u32_mul_rhs(lhs: &u32)        -> Result; }
pub trait U32_DivRHS<Result>        { pure fn u32_div_rhs(lhs: &u32)        -> Result; }
pub trait U32_ModuloRHS<Result>     { pure fn u32_modulo_rhs(lhs: &u32)     -> Result; }
pub trait U32_BitAndRHS<Result>     { pure fn u32_bitand_rhs(lhs: &u32)     -> Result; }
pub trait U32_BitOrRHS<Result>      { pure fn u32_bitor_rhs(lhs: &u32)      -> Result; }
pub trait U32_BitXorRHS<Result>     { pure fn u32_bitxor_rhs(lhs: &u32)     -> Result; }
pub trait U32_ShlRHS<Result>        { pure fn u32_shl_rhs(lhs: &u32)        -> Result; }
pub trait U32_ShrRHS<Result>        { pure fn u32_shr_rhs(lhs: &u32)        -> Result; }

pub trait U64_AddRHS<Result>        { pure fn u64_add_rhs(lhs: &u64)        -> Result; }
pub trait U64_SubRHS<Result>        { pure fn u64_sub_rhs(lhs: &u64)        -> Result; }
pub trait U64_MulRHS<Result>        { pure fn u64_mul_rhs(lhs: &u64)        -> Result; }
pub trait U64_DivRHS<Result>        { pure fn u64_div_rhs(lhs: &u64)        -> Result; }
pub trait U64_ModuloRHS<Result>     { pure fn u64_modulo_rhs(lhs: &u64)     -> Result; }
pub trait U64_BitAndRHS<Result>     { pure fn u64_bitand_rhs(lhs: &u64)     -> Result; }
pub trait U64_BitOrRHS<Result>      { pure fn u64_bitor_rhs(lhs: &u64)      -> Result; }
pub trait U64_BitXorRHS<Result>     { pure fn u64_bitxor_rhs(lhs: &u64)     -> Result; }
pub trait U64_ShlRHS<Result>        { pure fn u64_shl_rhs(lhs: &u64)        -> Result; }
pub trait U64_ShrRHS<Result>        { pure fn u64_shr_rhs(lhs: &u64)        -> Result; }

pub trait UInt_AddRHS<Result>       { pure fn uint_add_rhs(lhs: &uint)      -> Result; }
pub trait UInt_SubRHS<Result>       { pure fn uint_sub_rhs(lhs: &uint)      -> Result; }
pub trait UInt_MulRHS<Result>       { pure fn uint_mul_rhs(lhs: &uint)      -> Result; }
pub trait UInt_DivRHS<Result>       { pure fn uint_div_rhs(lhs: &uint)      -> Result; }
pub trait UInt_ModuloRHS<Result>    { pure fn uint_modulo_rhs(lhs: &uint)   -> Result; }
pub trait UInt_BitAndRHS<Result>    { pure fn uint_bitand_rhs(lhs: &uint)   -> Result; }
pub trait UInt_BitOrRHS<Result>     { pure fn uint_bitor_rhs(lhs: &uint)    -> Result; }
pub trait UInt_BitXorRHS<Result>    { pure fn uint_bitxor_rhs(lhs: &uint)   -> Result; }
pub trait UInt_ShlRHS<Result>       { pure fn uint_shl_rhs(lhs: &uint)      -> Result; }
pub trait UInt_ShrRHS<Result>       { pure fn uint_shr_rhs(lhs: &uint)      -> Result; }

pub impl u8: U8_AddRHS<u8>          { pure fn u8_add_rhs(lhs: &u8)          -> u8   { *lhs + self } }
pub impl u8: U8_SubRHS<u8>          { pure fn u8_sub_rhs(lhs: &u8)          -> u8   { *lhs - self } }
pub impl u8: U8_MulRHS<u8>          { pure fn u8_mul_rhs(lhs: &u8)          -> u8   { *lhs * self } }
pub impl u8: U8_DivRHS<u8>          { pure fn u8_div_rhs(lhs: &u8)          -> u8   { *lhs / self } }
pub impl u8: U8_ModuloRHS<u8>       { pure fn u8_modulo_rhs(lhs: &u8)       -> u8   { *lhs % self } }

pub impl u16: U16_AddRHS<u16>       { pure fn u16_add_rhs(lhs: &u16)        -> u16  { *lhs + self } }
pub impl u16: U16_SubRHS<u16>       { pure fn u16_sub_rhs(lhs: &u16)        -> u16  { *lhs - self } }
pub impl u16: U16_MulRHS<u16>       { pure fn u16_mul_rhs(lhs: &u16)        -> u16  { *lhs * self } }
pub impl u16: U16_DivRHS<u16>       { pure fn u16_div_rhs(lhs: &u16)        -> u16  { *lhs / self } }
pub impl u16: U16_ModuloRHS<u16>    { pure fn u16_modulo_rhs(lhs: &u16)     -> u16  { *lhs % self } }

pub impl u32: U32_AddRHS<u32>       { pure fn u32_add_rhs(lhs: &u32)        -> u32  { *lhs + self } }
pub impl u32: U32_SubRHS<u32>       { pure fn u32_sub_rhs(lhs: &u32)        -> u32  { *lhs - self } }
pub impl u32: U32_MulRHS<u32>       { pure fn u32_mul_rhs(lhs: &u32)        -> u32  { *lhs * self } }
pub impl u32: U32_DivRHS<u32>       { pure fn u32_div_rhs(lhs: &u32)        -> u32  { *lhs / self } }
pub impl u32: U32_ModuloRHS<u32>    { pure fn u32_modulo_rhs(lhs: &u32)     -> u32  { *lhs % self } }

pub impl u64: U64_AddRHS<u64>       { pure fn u64_add_rhs(lhs: &u64)        -> u64  { *lhs + self } }
pub impl u64: U64_SubRHS<u64>       { pure fn u64_sub_rhs(lhs: &u64)        -> u64  { *lhs - self } }
pub impl u64: U64_MulRHS<u64>       { pure fn u64_mul_rhs(lhs: &u64)        -> u64  { *lhs * self } }
pub impl u64: U64_DivRHS<u64>       { pure fn u64_div_rhs(lhs: &u64)        -> u64  { *lhs / self } }
pub impl u64: U64_ModuloRHS<u64>    { pure fn u64_modulo_rhs(lhs: &u64)     -> u64  { *lhs % self } }

pub impl uint: UInt_AddRHS<uint>    { pure fn uint_add_rhs(lhs: &uint)      -> uint { *lhs + self } }
pub impl uint: UInt_SubRHS<uint>    { pure fn uint_sub_rhs(lhs: &uint)      -> uint { *lhs - self } }
pub impl uint: UInt_MulRHS<uint>    { pure fn uint_mul_rhs(lhs: &uint)      -> uint { *lhs * self } }
pub impl uint: UInt_DivRHS<uint>    { pure fn uint_div_rhs(lhs: &uint)      -> uint { *lhs / self } }
pub impl uint: UInt_ModuloRHS<uint> { pure fn uint_modulo_rhs(lhs: &uint)   -> uint { *lhs % self } }

/*

FIXME: see module description above

pub impl u8: Num {
    pure fn add(other: &u8)    -> u8 { other.u8_add_rhs(&self) }
    pure fn sub(other: &u8)    -> u8 { other.u8_sub_rhs(&self) }
    pure fn mul(other: &u8)    -> u8 { other.u8_mul_rhs(&self) }
    pure fn div(other: &u8)    -> u8 { other.u8_div_rhs(&self) }
    pure fn modulo(other: &u8) -> u8 { other.u8_modulo_rhs(&self) }
    pure fn neg()              -> u8 { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> u8 { n as u8 }
}

pub impl u16: Num {
    pure fn add(other: &u16)    -> u16 { other.u16_add_rhs(&self) }
    pure fn sub(other: &u16)    -> u16 { other.u16_sub_rhs(&self) }
    pure fn mul(other: &u16)    -> u16 { other.u16_mul_rhs(&self) }
    pure fn div(other: &u16)    -> u16 { other.u16_div_rhs(&self) }
    pure fn modulo(other: &u16) -> u16 { other.u16_modulo_rhs(&self) }
    pure fn neg()               -> u16 { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> u16 { n as u16 }
}

pub impl u32: Num {
    pure fn add(other: &u32)    -> u32 { other.u32_add_rhs(&self) }
    pure fn sub(other: &u32)    -> u32 { other.u32_sub_rhs(&self) }
    pure fn mul(other: &u32)    -> u32 { other.u32_mul_rhs(&self) }
    pure fn div(other: &u32)    -> u32 { other.u32_div_rhs(&self) }
    pure fn modulo(other: &u32) -> u32 { other.u32_modulo_rhs(&self) }
    pure fn neg()               -> u32 { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> u32 { n as u32 }
}

pub impl u64: Num {
    pure fn add(other: &u64)    -> u64 { other.u64_add_rhs(&self) }
    pure fn sub(other: &u64)    -> u64 { other.u64_sub_rhs(&self) }
    pure fn mul(other: &u64)    -> u64 { other.u64_mul_rhs(&self) }
    pure fn div(other: &u64)    -> u64 { other.u64_div_rhs(&self) }
    pure fn modulo(other: &u64) -> u64 { other.u64_modulo_rhs(&self) }
    pure fn neg()               -> u64 { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> u64 { n as u64 }
}

pub impl uint: Num {
    pure fn add(other: &uint)    -> uint { other.uint_add_rhs(&self) }
    pure fn sub(other: &uint)    -> uint { other.uint_sub_rhs(&self) }
    pure fn mul(other: &uint)    -> uint { other.uint_mul_rhs(&self) }
    pure fn div(other: &uint)    -> uint { other.uint_div_rhs(&self) }
    pure fn modulo(other: &uint) -> uint { other.uint_modulo_rhs(&self) }
    pure fn neg()                -> uint { -self }

    pure fn to_int() -> int { self as int }
    static pure fn from_int(n: int) -> uint { n as uint }
}
*/