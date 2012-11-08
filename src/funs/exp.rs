/**
 * Exponential Functions
 */

use num::Num;

use ncast::*;

pub trait Exp {
    pure fn pow(n: self) -> self;
    pure fn exp()        -> self;
    pure fn log_()       -> self;
    pure fn exp2()       -> self;
    pure fn log2()       -> self;
    pure fn sqrt()       -> self;
    pure fn inv_sqrt()   -> self;
}

#[inline(always)] pub pure fn pow<T:Exp>(x: T, n: T) -> T { x.pow(move n) }
#[inline(always)] pub pure fn exp<T:Exp>(x: T)       -> T { x.exp() }
#[inline(always)] pub pure fn log_<T:Exp>(x: T)      -> T { x.log_() }
#[inline(always)] pub pure fn exp2<T:Exp>(x: T)      -> T { x.exp2() }
#[inline(always)] pub pure fn log2<T:Exp>(x: T)      -> T { x.log2() }
#[inline(always)] pub pure fn sqrt<T:Exp>(x: T)      -> T { x.sqrt() }
#[inline(always)] pub pure fn inv_sqrt<T:Exp>(x: T)  -> T { x.inv_sqrt() }

pub impl f32: Exp {
    #[inline(always)] pure fn pow(n: f32)   -> f32 { f32::pow(self, n) }
    #[inline(always)] pure fn exp()         -> f32 { f32::exp(self) }
    #[inline(always)] pure fn log_()        -> f32 { f32::ln(self) }
    #[inline(always)] pure fn exp2()        -> f32 { f32::exp2(self) }
    #[inline(always)] pure fn log2()        -> f32 { f32::log2(self) }
    #[inline(always)] pure fn sqrt()        -> f32 { f32::sqrt(self) }
    #[inline(always)] pure fn inv_sqrt()    -> f32 { 1f32 / self.sqrt() }  // TODO: optimise? need a wizard
}

pub impl f64: Exp {
    #[inline(always)] pure fn pow(n: f64)   -> f64 { f64::pow(self, n) }
    #[inline(always)] pure fn exp()         -> f64 { f64::exp(self) }
    #[inline(always)] pure fn log_()        -> f64 { f64::ln(self) }
    #[inline(always)] pure fn exp2()        -> f64 { f64::exp2(self) }
    #[inline(always)] pure fn log2()        -> f64 { f64::log2(self) }
    #[inline(always)] pure fn sqrt()        -> f64 { f64::sqrt(self) }
    #[inline(always)] pure fn inv_sqrt()    -> f64 { 1f64 / self.sqrt() }  // TODO: optimise? need a wizard
}

pub impl float: Exp {
    #[inline(always)] pure fn pow(n: float) -> float { cast(float::pow(cast(self), cast(n))) }
    #[inline(always)] pure fn exp()         -> float { cast(float::exp(cast(self))) }
    #[inline(always)] pure fn log_()        -> float { cast(float::ln(cast(self))) }
    #[inline(always)] pure fn exp2()        -> float { cast(float::exp2(cast(self))) }
    #[inline(always)] pure fn log2()        -> float { cast(float::log2(cast(self))) }
    #[inline(always)] pure fn sqrt()        -> float { cast(float::sqrt(cast(self))) }
    #[inline(always)] pure fn inv_sqrt()    -> float { 1f / self.sqrt() }  // TODO: optimise? need a wizard
}
