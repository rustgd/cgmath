use core::cmp::{Eq, Ord};
use std::cmp::FuzzyEq;

use num::conv::NumConv;


pub trait Number: Eq Num NumConv Ord {
    /**
     * Construct a new number by casting a number the the static method's
     * enclosing type
     *
     * # Example
     *
     * ~~~
     * let twenty: f32 = Number::from(0x14);
     * assert twenty == 20f32;
     * ~~~
     */
    static pure fn from<T:Number>(n: T) -> self;
    
    static pure fn size_of() -> uint;
    static pure fn bits() -> uint;
    
    static pure fn zero() -> self;      /// The additive identity
    static pure fn one()  -> self;      // The multiplicative identity
}

pub impl u8: Number {
    /**
     * Construct a `u8` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> u8 { n.to_u8() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<u8>() }
    #[inline(always)] static pure fn bits() -> uint { 8 }
    
    #[inline(always)] static pure fn zero() -> u8 { 0u8 }       /// 0u8
    #[inline(always)] static pure fn one()  -> u8 { 1u8 }       //  1u8
}

pub impl u16: Number {
    /**
     * Construct a `u16` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> u16 { n.to_u16() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<u16>() }
    #[inline(always)] static pure fn bits() -> uint { 16 }
    
    #[inline(always)] static pure fn zero() -> u16 { 0u16 }     /// 0u16
    #[inline(always)] static pure fn one()  -> u16 { 1u16 }     //  1u16
}

pub impl u32: Number {
    /**
     * Construct a `u32` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> u32 { n.to_u32() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<u32>() }
    #[inline(always)] static pure fn bits() -> uint { 32 }
    
    #[inline(always)] static pure fn zero() -> u32 { 0u32 }     /// 0u32
    #[inline(always)] static pure fn one()  -> u32 { 1u32 }     //  1u32
}

pub impl u64: Number {
    /**
     * Construct a `u64` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> u64 { n.to_u64() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<u64>() }
    #[inline(always)] static pure fn bits() -> uint { 64 }
    
    #[inline(always)] static pure fn zero() -> u64 { 0u64 }     /// 0u64
    #[inline(always)] static pure fn one()  -> u64 { 1u64 }     //  1u64
}

pub impl uint: Number {
    /**
     * Construct a `uint` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> uint { n.to_uint() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<uint>() }
    #[inline(always)] static pure fn bits() -> uint { sys::size_of::<uint>() * 8 }
    
    #[inline(always)] static pure fn zero() -> uint { 0u }      /// 0u
    #[inline(always)] static pure fn one()  -> uint { 1u }      //  1u
}

pub impl i8: Number {
    /**
     * Construct an `i8` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> i8 { n.to_i8() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<i8>() }
    #[inline(always)] static pure fn bits() -> uint { 8 }
    
    #[inline(always)] static pure fn zero() -> i8 { 0i8 }       /// 0i8
    #[inline(always)] static pure fn one()  -> i8 { 1i8 }       //  1i8
}

pub impl i16: Number {
    /**
     * Construct an `i16` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> i16 { n.to_i16() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<i16>() }
    #[inline(always)] static pure fn bits() -> uint { 16 }
    
    #[inline(always)] static pure fn zero() -> i16 { 0i16 }     /// 0i16
    #[inline(always)] static pure fn one()  -> i16 { 1i16 }     //  1i16
}

pub impl i32: Number {
    /**
     * Construct an `i32` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> i32 { n.to_i32() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<i32>() }
    #[inline(always)] static pure fn bits() -> uint { 32 }
    
    #[inline(always)] static pure fn zero() -> i32 { 0i32 }     /// 0i32
    #[inline(always)] static pure fn one()  -> i32 { 1i32 }     //  1i32
}

pub impl i64: Number {
    /**
     * Construct an `i64` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> i64 { n.to_i64() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<i64>() }
    #[inline(always)] static pure fn bits() -> uint { 64 }
    
    #[inline(always)] static pure fn zero() -> i64 { 0i64 }     /// 0i64
    #[inline(always)] static pure fn one()  -> i64 { 1i64 }     //  1i64
}

pub impl int: Number {
    /**
     * Construct an `int` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> int { n.to_int_() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<int>() }
    #[inline(always)] static pure fn bits() -> uint { sys::size_of::<int>() * 8 }
    
    #[inline(always)] static pure fn zero() -> int { 0 }        /// 0
    #[inline(always)] static pure fn one()  -> int { 1 }        //  1
}

pub impl f32: Number {
    /**
     * Construct a `f32` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> f32 { n.to_f32() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<f32>() }
    #[inline(always)] static pure fn bits() -> uint { 32 }
    
    #[inline(always)] static pure fn zero() -> f32 { 0f32 }     /// 0f32
    #[inline(always)] static pure fn one()  -> f32 { 1f32 }     //  1f32
}

pub impl f64: Number {
    /**
     * Construct a `f64` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> f64 { n.to_f64() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<f64>() }
    #[inline(always)] static pure fn bits() -> uint { 64 }
    
    #[inline(always)] static pure fn zero() -> f64 { 0f64 }     /// 0f64
    #[inline(always)] static pure fn one()  -> f64 { 1f64 }     //  1f64
}

pub impl float: Number {
    /**
     * Construct a `float` from the type `T:Number`
     */
    #[inline(always)] static pure fn from<T:Number>(n: T) -> float { n.to_float() }
    
    #[inline(always)] static pure fn size_of() -> uint { sys::size_of::<float>() }
    #[inline(always)] static pure fn bits() -> uint { sys::size_of::<float>() * 8 }
    
    #[inline(always)] static pure fn zero() -> float { 0f }     /// 0f
    #[inline(always)] static pure fn one()  -> float { 1f }     //  1f
}


pub trait UnSigned: Number {}

pub impl u8:   UnSigned {}
pub impl u16:  UnSigned {}
pub impl u32:  UnSigned {}
pub impl u64:  UnSigned {}
pub impl uint: UnSigned {}


pub trait Signed: Number {
    pure fn is_positive(&self)    -> bool;
    pure fn is_negative(&self)    -> bool;
    pure fn is_nonpositive(&self) -> bool;
    pure fn is_nonnegative(&self) -> bool;
}

pub impl i8: Signed {
    #[inline(always)] pure fn is_positive(&self)    -> bool { (*self) >  0 }
    #[inline(always)] pure fn is_negative(&self)    -> bool { (*self) <  0 }
    #[inline(always)] pure fn is_nonpositive(&self) -> bool { (*self) <= 0 }
    #[inline(always)] pure fn is_nonnegative(&self) -> bool { (*self) >= 0 }
}

pub impl i16: Signed {
    #[inline(always)] pure fn is_positive(&self)    -> bool { (*self) >  0 }
    #[inline(always)] pure fn is_negative(&self)    -> bool { (*self) <  0 }
    #[inline(always)] pure fn is_nonpositive(&self) -> bool { (*self) <= 0 }
    #[inline(always)] pure fn is_nonnegative(&self) -> bool { (*self) >= 0 }
}

pub impl i32: Signed {
    #[inline(always)] pure fn is_positive(&self)    -> bool { (*self) >  0 }
    #[inline(always)] pure fn is_negative(&self)    -> bool { (*self) <  0 }
    #[inline(always)] pure fn is_nonpositive(&self) -> bool { (*self) <= 0 }
    #[inline(always)] pure fn is_nonnegative(&self) -> bool { (*self) >= 0 }
}

pub impl i64: Signed {
    #[inline(always)] pure fn is_positive(&self)    -> bool { (*self) >  0 }
    #[inline(always)] pure fn is_negative(&self)    -> bool { (*self) <  0 }
    #[inline(always)] pure fn is_nonpositive(&self) -> bool { (*self) <= 0 }
    #[inline(always)] pure fn is_nonnegative(&self) -> bool { (*self) >= 0 }
}

pub impl int: Signed {
    #[inline(always)] pure fn is_positive(&self)    -> bool { (*self) >  0 }
    #[inline(always)] pure fn is_negative(&self)    -> bool { (*self) <  0 }
    #[inline(always)] pure fn is_nonpositive(&self) -> bool { (*self) <= 0 }
    #[inline(always)] pure fn is_nonnegative(&self) -> bool { (*self) >= 0 }
}

pub impl f32: Signed {
    #[inline(always)] pure fn is_positive(&self)    -> bool { f32::is_positive(*self) }
    #[inline(always)] pure fn is_negative(&self)    -> bool { f32::is_negative(*self) }
    #[inline(always)] pure fn is_nonpositive(&self) -> bool { f32::is_nonpositive(*self) }
    #[inline(always)] pure fn is_nonnegative(&self) -> bool { f32::is_nonnegative(*self) }
}

pub impl f64: Signed {
    #[inline(always)] pure fn is_positive(&self)    -> bool { f64::is_positive(*self) }
    #[inline(always)] pure fn is_negative(&self)    -> bool { f64::is_negative(*self) }
    #[inline(always)] pure fn is_nonpositive(&self) -> bool { f64::is_nonpositive(*self) }
    #[inline(always)] pure fn is_nonnegative(&self) -> bool { f64::is_nonnegative(*self) }
}

pub impl float: Signed {
    #[inline(always)] pure fn is_positive(&self)    -> bool { core::float::is_positive(*self) }
    #[inline(always)] pure fn is_negative(&self)    -> bool { core::float::is_negative(*self) }
    #[inline(always)] pure fn is_nonpositive(&self) -> bool { core::float::is_nonpositive(*self) }
    #[inline(always)] pure fn is_nonnegative(&self) -> bool { core::float::is_nonnegative(*self) }
}


pub trait Integer: Number {}

pub impl u8:   Integer {}
pub impl u16:  Integer {}
pub impl u32:  Integer {}
pub impl u64:  Integer {}
pub impl uint: Integer {}

pub impl i8:   Integer {}
pub impl i16:  Integer {}
pub impl i32:  Integer {}
pub impl i64:  Integer {}
pub impl int:  Integer {}


pub trait Float: Number FuzzyEq {
    pure fn to_float() -> float;
    static pure fn from_float(n: float) -> self;
    
    static pure fn NaN()            -> self;
    static pure fn infinity()       -> self;
    static pure fn neg_infinity()   -> self;
    
    pure fn is_NaN(&self)           -> bool;
    pure fn is_infinite(&self)      -> bool;
    pure fn is_finite(&self)        -> bool;
    
    static pure fn two_pi()         -> self;    /// 2 × π
    static pure fn pi()             -> self;    /// π
    static pure fn frac_pi_2()      -> self;    /// π / 2
    static pure fn frac_pi_3()      -> self;    /// π / 3
    static pure fn frac_pi_4()      -> self;    /// π / 4
    static pure fn frac_pi_6()      -> self;    /// π / 6
    static pure fn frac_pi_8()      -> self;    /// π / 8
    static pure fn frac_1_pi()      -> self;    /// 1 / π
    static pure fn frac_2_pi()      -> self;    /// 2 / π
    static pure fn frac_2_sqrtpi()  -> self;    /// 2 / sqrt(π)
    static pure fn sqrt2()          -> self;    /// sqrt(2)
    static pure fn frac_1_sqrt2()   -> self;    /// 1 / sqrt(2)
    static pure fn e()              -> self;    /// Euler's number
    static pure fn log2_e()         -> self;    /// log2(e)
    static pure fn log10_e()        -> self;    /// log10(e)
    static pure fn ln_2()           -> self;    /// ln(2)
    static pure fn ln_10()          -> self;    //  ln(10)
}

pub impl f32: Float {
    #[inline(always)] pure fn to_float() -> float { self as float }
    #[inline(always)] static pure fn from_float(n: float) -> f32 { n as f32 }
    
    #[inline(always)] static pure fn NaN()              -> f32 { 0_f32 / 0_f32 }
    #[inline(always)] static pure fn infinity()         -> f32 { 1_f32 / 0_f32 }
    #[inline(always)] static pure fn neg_infinity()     -> f32 {-1_f32 / 0_f32 }
    
    #[inline(always)] pure fn is_NaN(&self)             -> bool { self != self }
    #[inline(always)] pure fn is_infinite(&self)        -> bool { (*self) == Float::infinity() || (*self) == Float::neg_infinity() }
    #[inline(always)] pure fn is_finite(&self)          -> bool { !(self.is_NaN() || self.is_infinite()) }
    
    #[inline(always)] static pure fn two_pi()           -> f32 { 6.28318530717958647692528676655900576__f32 }   /// 2 × π
    #[inline(always)] static pure fn pi()               -> f32 { 3.14159265358979323846264338327950288__f32 }   /// π
    #[inline(always)] static pure fn frac_pi_2()        -> f32 { 1.57079632679489661923132169163975144__f32 }   /// π / 2
    #[inline(always)] static pure fn frac_pi_3()        -> f32 { 1.04719755119659774615421446109316763__f32 }   /// π / 3
    #[inline(always)] static pure fn frac_pi_4()        -> f32 { 0.785398163397448309615660845819875721_f32 }   /// π / 4
    #[inline(always)] static pure fn frac_pi_6()        -> f32 { 0.52359877559829887307710723054658381__f32 }   /// π / 6
    #[inline(always)] static pure fn frac_pi_8()        -> f32 { 0.39269908169872415480783042290993786__f32 }   /// π / 8
    #[inline(always)] static pure fn frac_1_pi()        -> f32 { 0.318309886183790671537767526745028724_f32 }   /// 1 / π
    #[inline(always)] static pure fn frac_2_pi()        -> f32 { 0.636619772367581343075535053490057448_f32 }   /// 2 / π
    #[inline(always)] static pure fn frac_2_sqrtpi()    -> f32 { 1.12837916709551257389615890312154517__f32 }   /// 2 / sqrt(π)
    #[inline(always)] static pure fn sqrt2()            -> f32 { 1.41421356237309504880168872420969808__f32 }   /// sqrt(2)
    #[inline(always)] static pure fn frac_1_sqrt2()     -> f32 { 0.707106781186547524400844362104849039_f32 }   /// 1 / sqrt(2)
    #[inline(always)] static pure fn e()                -> f32 { 2.71828182845904523536028747135266250__f32 }   /// Euler's number
    #[inline(always)] static pure fn log2_e()           -> f32 { 1.44269504088896340735992468100189214__f32 }   /// log2(e)
    #[inline(always)] static pure fn log10_e()          -> f32 { 0.434294481903251827651128918916605082_f32 }   /// log10(e)
    #[inline(always)] static pure fn ln_2()             -> f32 { 0.693147180559945309417232121458176568_f32 }   /// ln(2)
    #[inline(always)] static pure fn ln_10()            -> f32 { 2.30258509299404568401799145468436421__f32 }   //  ln(10)
}

pub impl f64: Float {
    #[inline(always)] pure fn to_float() -> float { self as float }
    #[inline(always)] static pure fn from_float(n: float) -> f64 { n as f64 }
    
    #[inline(always)] static pure fn NaN()              -> f64 { 0_f64 / 0_f64 }
    #[inline(always)] static pure fn infinity()         -> f64 { 1_f64 / 0_f64 }
    #[inline(always)] static pure fn neg_infinity()     -> f64 {-1_f64 / 0_f64 }
    
    #[inline(always)] pure fn is_NaN(&self)             -> bool { self != self }
    #[inline(always)] pure fn is_infinite(&self)        -> bool { (*self) == Float::infinity() || (*self) == Float::neg_infinity() }
    #[inline(always)] pure fn is_finite(&self)          -> bool { !(self.is_NaN() || self.is_infinite()) }
    
    #[inline(always)] static pure fn two_pi()           -> f64 { 6.28318530717958647692528676655900576__f64 }   /// 2 × π
    #[inline(always)] static pure fn pi()               -> f64 { 3.14159265358979323846264338327950288__f64 }   /// π
    #[inline(always)] static pure fn frac_pi_2()        -> f64 { 1.57079632679489661923132169163975144__f64 }   /// π / 2
    #[inline(always)] static pure fn frac_pi_3()        -> f64 { 1.04719755119659774615421446109316763__f64 }   /// π / 3
    #[inline(always)] static pure fn frac_pi_4()        -> f64 { 0.785398163397448309615660845819875721_f64 }   /// π / 4
    #[inline(always)] static pure fn frac_pi_6()        -> f64 { 0.52359877559829887307710723054658381__f64 }   /// π / 6
    #[inline(always)] static pure fn frac_pi_8()        -> f64 { 0.39269908169872415480783042290993786__f64 }   /// π / 8
    #[inline(always)] static pure fn frac_1_pi()        -> f64 { 0.318309886183790671537767526745028724_f64 }   /// 1 / π
    #[inline(always)] static pure fn frac_2_pi()        -> f64 { 0.636619772367581343075535053490057448_f64 }   /// 2 / π
    #[inline(always)] static pure fn frac_2_sqrtpi()    -> f64 { 1.12837916709551257389615890312154517__f64 }   /// 2 / sqrt(π)
    #[inline(always)] static pure fn sqrt2()            -> f64 { 1.41421356237309504880168872420969808__f64 }   /// sqrt(2)
    #[inline(always)] static pure fn frac_1_sqrt2()     -> f64 { 0.707106781186547524400844362104849039_f64 }   /// 1 / sqrt(2)
    #[inline(always)] static pure fn e()                -> f64 { 2.71828182845904523536028747135266250__f64 }   /// Euler's number
    #[inline(always)] static pure fn log2_e()           -> f64 { 1.44269504088896340735992468100189214__f64 }   /// log2(e)
    #[inline(always)] static pure fn log10_e()          -> f64 { 0.434294481903251827651128918916605082_f64 }   /// log10(e)
    #[inline(always)] static pure fn ln_2()             -> f64 { 0.693147180559945309417232121458176568_f64 }   /// ln(2)
    #[inline(always)] static pure fn ln_10()            -> f64 { 2.30258509299404568401799145468436421__f64 }   //  ln(10)
}

pub impl float: Float {
    #[inline(always)] pure fn to_float() -> float { self }
    #[inline(always)] static pure fn from_float(n: float) -> float { n }
    
    #[inline(always)] static pure fn NaN()              -> float { 0_f / 0_f }
    #[inline(always)] static pure fn infinity()         -> float { 1_f / 0_f }
    #[inline(always)] static pure fn neg_infinity()     -> float {-1_f / 0_f }
    
    #[inline(always)] pure fn is_NaN(&self)             -> bool { self != self }
    #[inline(always)] pure fn is_infinite(&self)        -> bool { (*self) == Float::infinity() || (*self) == Float::neg_infinity() }
    #[inline(always)] pure fn is_finite(&self)          -> bool { !(self.is_NaN() || self.is_infinite()) }
    
    #[inline(always)] static pure fn two_pi()           -> float { 6.28318530717958647692528676655900576__f }   /// 2 × π
    #[inline(always)] static pure fn pi()               -> float { 3.14159265358979323846264338327950288__f }   /// π
    #[inline(always)] static pure fn frac_pi_2()        -> float { 1.57079632679489661923132169163975144__f }   /// π / 2
    #[inline(always)] static pure fn frac_pi_3()        -> float { 1.04719755119659774615421446109316763__f }   /// π / 3
    #[inline(always)] static pure fn frac_pi_4()        -> float { 0.785398163397448309615660845819875721_f }   /// π / 4
    #[inline(always)] static pure fn frac_pi_6()        -> float { 0.52359877559829887307710723054658381__f }   /// π / 6
    #[inline(always)] static pure fn frac_pi_8()        -> float { 0.39269908169872415480783042290993786__f }   /// π / 8
    #[inline(always)] static pure fn frac_1_pi()        -> float { 0.318309886183790671537767526745028724_f }   /// 1 / π
    #[inline(always)] static pure fn frac_2_pi()        -> float { 0.636619772367581343075535053490057448_f }   /// 2 / π
    #[inline(always)] static pure fn frac_2_sqrtpi()    -> float { 1.12837916709551257389615890312154517__f }   /// 2 / sqrt(π)
    #[inline(always)] static pure fn sqrt2()            -> float { 1.41421356237309504880168872420969808__f }   /// sqrt(2)
    #[inline(always)] static pure fn frac_1_sqrt2()     -> float { 0.707106781186547524400844362104849039_f }   /// 1 / sqrt(2)
    #[inline(always)] static pure fn e()                -> float { 2.71828182845904523536028747135266250__f }   /// Euler's number
    #[inline(always)] static pure fn log2_e()           -> float { 1.44269504088896340735992468100189214__f }   /// log2(e)
    #[inline(always)] static pure fn log10_e()          -> float { 0.434294481903251827651128918916605082_f }   /// log10(e)
    #[inline(always)] static pure fn ln_2()             -> float { 0.693147180559945309417232121458176568_f }   /// ln(2)
    #[inline(always)] static pure fn ln_10()            -> float { 2.30258509299404568401799145468436421__f }   //  ln(10)
}