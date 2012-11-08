/**
 * Exponential Functions
 */

use num::Num;

use ncast::*;
use vector::{Vec2, Vec3, Vec4};

pub trait Exp {
    pure fn pow<N:NumCast>(n: &N) -> self;
    pure fn exp()      -> self;
    pure fn log_()     -> self;
    pure fn exp2()     -> self;
    pure fn log2()     -> self;
    pure fn sqrt()     -> self;
    pure fn inv_sqrt() -> self;
}

#[inline(always)] pub pure fn pow<T:Exp, N:NumCast>(x: &T, n: &N) -> T { x.pow(n) }
#[inline(always)] pub pure fn exp<T:Exp>(x: &T)      -> T { x.exp() }
#[inline(always)] pub pure fn log_<T:Exp>(x: &T)     -> T { x.log_() }
#[inline(always)] pub pure fn exp2<T:Exp>(x: &T)     -> T { x.exp2() }
#[inline(always)] pub pure fn log2<T:Exp>(x: &T)     -> T { x.log2() }
#[inline(always)] pub pure fn sqrt<T:Exp>(x: &T)     -> T { x.sqrt() }
#[inline(always)] pub pure fn inv_sqrt<T:Exp>(x: &T) -> T { x.inv_sqrt() }

pub impl f32: Exp {
    #[inline(always)] pure fn pow<N:NumCast>(n: &N) -> f32 { f32::pow(self, n.cast()) }
    #[inline(always)] pure fn exp()      -> f32 { f32::exp(self) }
    #[inline(always)] pure fn log_()     -> f32 { f32::ln(self) }
    #[inline(always)] pure fn exp2()     -> f32 { f32::exp2(self) }
    #[inline(always)] pure fn log2()     -> f32 { f32::log2(self) }
    #[inline(always)] pure fn sqrt()     -> f32 { f32::sqrt(self) }
    #[inline(always)] pure fn inv_sqrt() -> f32 { 1f32 / self.sqrt() }  // TODO: optimise? need a wizard
}

pub impl f64: Exp {
    #[inline(always)] pure fn pow<N:NumCast>(n: &N) -> f64 { f64::pow(self, n.cast()) }
    #[inline(always)] pure fn exp()      -> f64 { f64::exp(self) }
    #[inline(always)] pure fn log_()     -> f64 { f64::ln(self) }
    #[inline(always)] pure fn exp2()     -> f64 { f64::exp2(self) }
    #[inline(always)] pure fn log2()     -> f64 { f64::log2(self) }
    #[inline(always)] pure fn sqrt()     -> f64 { f64::sqrt(self) }
    #[inline(always)] pure fn inv_sqrt() -> f64 { 1f64 / self.sqrt() }  // TODO: optimise? need a wizard
}

pub impl float: Exp {
    #[inline(always)] pure fn pow<N:NumCast>(n: &N) -> float { cast(float::pow(cast(self), n.cast())) }
    #[inline(always)] pure fn exp()      -> float { cast(float::exp(cast(self))) }
    #[inline(always)] pure fn log_()     -> float { cast(float::ln(cast(self))) }
    #[inline(always)] pure fn exp2()     -> float { cast(float::exp2(cast(self))) }
    #[inline(always)] pure fn log2()     -> float { cast(float::log2(cast(self))) }
    #[inline(always)] pure fn sqrt()     -> float { cast(float::sqrt(cast(self))) }
    #[inline(always)] pure fn inv_sqrt() -> float { 1f / self.sqrt() }  // TODO: optimise? need a wizard
}

pub impl<T:Copy Exp> Vec2<T>: Exp {
    #[inline(always)]
    pure fn pow<N:NumCast>(n: &N) -> Vec2<T> {
        Vec2::new(pow(&self[0], n),
                  pow(&self[1], n))
    }
    
    #[inline(always)]
    pure fn exp() -> Vec2<T> {
        Vec2::new(exp(&self[0]),
                  exp(&self[1]))
    }
    
    #[inline(always)]
    pure fn log_() -> Vec2<T> {
        Vec2::new(log_(&self[0]),
                  log_(&self[1]))
    }
    
    #[inline(always)]
    pure fn exp2() -> Vec2<T> {
        Vec2::new(exp2(&self[0]),
                  exp2(&self[1]))
    }
    
    #[inline(always)]
    pure fn log2() -> Vec2<T> {
        Vec2::new(log2(&self[0]),
                  log2(&self[1]))
    }
    
    #[inline(always)]
    pure fn sqrt() -> Vec2<T> {
        Vec2::new(sqrt(&self[0]),
                  sqrt(&self[1]))
    }
    
    #[inline(always)]
    pure fn inv_sqrt() -> Vec2<T> {
        Vec2::new(inv_sqrt(&self[0]),
                  inv_sqrt(&self[1]))
    }
}

pub impl<T:Copy Exp> Vec3<T>: Exp {
    #[inline(always)]
    pure fn pow<N:NumCast>(n: &N) -> Vec3<T> {
        Vec3::new(pow(&self[0], n),
                  pow(&self[1], n),
                  pow(&self[2], n))
    }
    
    #[inline(always)]
    pure fn exp() -> Vec3<T> {
        Vec3::new(exp(&self[0]),
                  exp(&self[1]),
                  exp(&self[2]))
    }
    
    #[inline(always)]
    pure fn log_() -> Vec3<T> {
        Vec3::new(log_(&self[0]),
                  log_(&self[1]),
                  log_(&self[2]))
    }
    
    #[inline(always)]
    pure fn exp2() -> Vec3<T> {
        Vec3::new(exp2(&self[0]),
                  exp2(&self[1]),
                  exp2(&self[2]))
    }
    
    #[inline(always)]
    pure fn log2() -> Vec3<T> {
        Vec3::new(log2(&self[0]),
                  log2(&self[1]),
                  log2(&self[2]))
    }
    
    #[inline(always)]
    pure fn sqrt() -> Vec3<T> {
        Vec3::new(sqrt(&self[0]),
                  sqrt(&self[1]),
                  sqrt(&self[2]))
    }
    
    #[inline(always)]
    pure fn inv_sqrt() -> Vec3<T> {
        Vec3::new(inv_sqrt(&self[0]),
                  inv_sqrt(&self[1]),
                  inv_sqrt(&self[2]))
    }
}

pub impl<T:Copy Exp> Vec4<T>: Exp {
    #[inline(always)]
    pure fn pow<N:NumCast>(n: &N) -> Vec4<T> {
        Vec4::new(pow(&self[0], n),
                  pow(&self[1], n),
                  pow(&self[2], n),
                  pow(&self[3], n))
    }
    
    #[inline(always)]
    pure fn exp() -> Vec4<T> {
        Vec4::new(exp(&self[0]),
                  exp(&self[1]),
                  exp(&self[2]),
                  exp(&self[3]))
    }
    
    #[inline(always)]
    pure fn log_() -> Vec4<T> {
        Vec4::new(log_(&self[0]),
                  log_(&self[1]),
                  log_(&self[2]),
                  log_(&self[3]))
    }
    
    #[inline(always)]
    pure fn exp2() -> Vec4<T> {
        Vec4::new(exp2(&self[0]),
                  exp2(&self[1]),
                  exp2(&self[2]),
                  exp2(&self[3]))
    }
    
    #[inline(always)]
    pure fn log2() -> Vec4<T> {
        Vec4::new(log2(&self[0]),
                  log2(&self[1]),
                  log2(&self[2]),
                  log2(&self[3]))
    }
    
    #[inline(always)]
    pure fn sqrt() -> Vec4<T> {
        Vec4::new(sqrt(&self[0]),
                  sqrt(&self[1]),
                  sqrt(&self[2]),
                  sqrt(&self[3]))
    }
    
    #[inline(always)]
    pure fn inv_sqrt() -> Vec4<T> {
        Vec4::new(inv_sqrt(&self[0]),
                  inv_sqrt(&self[1]),
                  inv_sqrt(&self[2]),
                  inv_sqrt(&self[3]))
    }
}