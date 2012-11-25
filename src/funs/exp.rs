/**
 * Exponential Functions
 */
use num::cast::*;
use vec::{Vec2, Vec3, Vec4};

pub trait Exp {
    pure fn pow(n: &self) -> self;
    pure fn exp()      -> self;
    pure fn log_()     -> self;
    pure fn exp2()     -> self;
    pure fn log2()     -> self;
    pure fn sqrt()     -> self;
    pure fn inv_sqrt() -> self;
}

#[inline(always)] pub pure fn pow<T:Exp>(x: &T, n: &T)  -> T { x.pow(n) }
#[inline(always)] pub pure fn exp<T:Exp>(x: &T)         -> T { x.exp()  }
#[inline(always)] pub pure fn log_<T:Exp>(x: &T)        -> T { x.log_() }
#[inline(always)] pub pure fn exp2<T:Exp>(x: &T)        -> T { x.exp2() }
#[inline(always)] pub pure fn log2<T:Exp>(x: &T)        -> T { x.log2() }
#[inline(always)] pub pure fn sqrt<T:Exp>(x: &T)        -> T { x.sqrt() }
#[inline(always)] pub pure fn inv_sqrt<T:Exp>(x: &T)    -> T { x.inv_sqrt() }

pub impl f32: Exp {
    #[inline(always)] pure fn pow(n: &f32)      -> f32 { cast(cmath::c_float_utils::pow(self, n.cast())) }
    #[inline(always)] pure fn exp()             -> f32 { cast(cmath::c_float_utils::exp(self)) }
    #[inline(always)] pure fn log_()            -> f32 { cast(cmath::c_float_utils::ln(self)) }
    #[inline(always)] pure fn exp2()            -> f32 { cast(cmath::c_float_utils::exp2(self)) }
    #[inline(always)] pure fn log2()            -> f32 { cast(cmath::c_float_utils::log2(self)) }
    #[inline(always)] pure fn sqrt()            -> f32 { cast(cmath::c_float_utils::sqrt(self)) }
    #[inline(always)] pure fn inv_sqrt()        -> f32 { 1f32 / self.sqrt() }  // TODO: optimise? need a wizard
}

pub impl f64: Exp {
    #[inline(always)] pure fn pow(n: &f64)      -> f64 { cast(cmath::c_double_utils::pow(self, n.cast())) }
    #[inline(always)] pure fn exp()             -> f64 { cast(cmath::c_double_utils::exp(self)) }
    #[inline(always)] pure fn log_()            -> f64 { cast(cmath::c_double_utils::ln(self)) }
    #[inline(always)] pure fn exp2()            -> f64 { cast(cmath::c_double_utils::exp2(self)) }
    #[inline(always)] pure fn log2()            -> f64 { cast(cmath::c_double_utils::log2(self)) }
    #[inline(always)] pure fn sqrt()            -> f64 { cast(cmath::c_double_utils::sqrt(self)) }
    #[inline(always)] pure fn inv_sqrt()        -> f64 { 1f64 / self.sqrt() }  // TODO: optimise? need a wizard
}

pub impl float: Exp {
    #[inline(always)] pure fn pow(n: &float)    -> float { cast(cmath::c_float_utils::pow(cast(self), n.cast())) }
    #[inline(always)] pure fn exp()             -> float { cast(cmath::c_float_utils::exp(cast(self))) }
    #[inline(always)] pure fn log_()            -> float { cast(cmath::c_float_utils::ln(cast(self))) }
    #[inline(always)] pure fn exp2()            -> float { cast(cmath::c_float_utils::exp2(cast(self))) }
    #[inline(always)] pure fn log2()            -> float { cast(cmath::c_float_utils::log2(cast(self))) }
    #[inline(always)] pure fn sqrt()            -> float { cast(cmath::c_float_utils::sqrt(cast(self))) }
    #[inline(always)] pure fn inv_sqrt()        -> float { 1f / self.sqrt() }  // TODO: optimise? need a wizard
}

pub impl<T:Copy Exp> Vec2<T>: Exp {
    #[inline(always)]
    pure fn pow(n: &Vec2<T>) -> Vec2<T> {
        Vec2::new(pow(&self[0], &n[0]),
                  pow(&self[1], &n[1]))
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
    pure fn pow(n: &Vec3<T>) -> Vec3<T> {
        Vec3::new(pow(&self[0], &n[0]),
                  pow(&self[1], &n[1]),
                  pow(&self[2], &n[2]))
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
    pure fn pow(n: &Vec4<T>) -> Vec4<T> {
        Vec4::new(pow(&self[0], &n[0]),
                  pow(&self[1], &n[1]),
                  pow(&self[2], &n[2]),
                  pow(&self[3], &n[3]))
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