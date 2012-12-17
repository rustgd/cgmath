/**
 * Exponential Functions
 *
 * This module corresponds to Section 8.2 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */

use numeric::funs::*;
use numeric::traits::*;

use vec::{Vector, Vec2, Vec3, Vec4};

// Exp

pub trait ExpVector<T>: Vector<T> {
    pure fn pow_t(&self, n: T) -> self;
    pure fn pow_v(&self, n: &self) -> self; 
}

pub impl<T:Copy Exp> Vec2<T>: ExpVector<T> {
    #[inline(always)]
    pure fn pow_t(&self, n: T) -> Vec2<T> {
        Vec2::new(pow(&self[0], &n),
                  pow(&self[1], &n))
    }
    
    #[inline(always)]
    pure fn pow_v(&self, n: &Vec2<T>) -> Vec2<T> {
        Vec2::new(pow(&self[0], &n[0]),
                  pow(&self[1], &n[1]))
    }
}

pub impl<T:Copy Exp> Vec2<T>: Exp {
    #[inline(always)]
    pure fn pow(&self, n: &Vec2<T>) -> Vec2<T> {
        self.pow_v(n)
    }
    
    #[inline(always)]
    pure fn exp(&self) -> Vec2<T> {
        Vec2::new(exp(&self[0]),
                  exp(&self[1]))
    }
    
    #[inline(always)]
    pure fn log_(&self) -> Vec2<T> {
        Vec2::new(log_(&self[0]),
                  log_(&self[1]))
    }
    
    #[inline(always)]
    pure fn exp2(&self) -> Vec2<T> {
        Vec2::new(exp2(&self[0]),
                  exp2(&self[1]))
    }
    
    #[inline(always)]
    pure fn log2(&self) -> Vec2<T> {
        Vec2::new(log2(&self[0]),
                  log2(&self[1]))
    }
    
    #[inline(always)]
    pure fn sqrt(&self) -> Vec2<T> {
        Vec2::new(sqrt(&self[0]),
                  sqrt(&self[1]))
    }
    
    #[inline(always)]
    pure fn inv_sqrt(&self) -> Vec2<T> {
        Vec2::new(inv_sqrt(&self[0]),
                  inv_sqrt(&self[1]))
    }
}

pub impl<T:Copy Exp> Vec3<T>: ExpVector<T> {
    #[inline(always)]
    pure fn pow_t(&self, n: T) -> Vec3<T> {
        Vec3::new(pow(&self[0], &n),
                  pow(&self[1], &n),
                  pow(&self[2], &n))
    }
    
    #[inline(always)]
    pure fn pow_v(&self, n: &Vec3<T>) -> Vec3<T> {
        Vec3::new(pow(&self[0], &n[0]),
                  pow(&self[1], &n[1]),
                  pow(&self[2], &n[2]))
    }
}

pub impl<T:Copy Exp> Vec3<T>: Exp {
    #[inline(always)]
    pure fn pow(&self, n: &Vec3<T>) -> Vec3<T> {
        self.pow_v(n)
    }
    
    #[inline(always)]
    pure fn exp(&self) -> Vec3<T> {
        Vec3::new(exp(&self[0]),
                  exp(&self[1]),
                  exp(&self[2]))
    }
    
    #[inline(always)]
    pure fn log_(&self) -> Vec3<T> {
        Vec3::new(log_(&self[0]),
                  log_(&self[1]),
                  log_(&self[2]))
    }
    
    #[inline(always)]
    pure fn exp2(&self) -> Vec3<T> {
        Vec3::new(exp2(&self[0]),
                  exp2(&self[1]),
                  exp2(&self[2]))
    }
    
    #[inline(always)]
    pure fn log2(&self) -> Vec3<T> {
        Vec3::new(log2(&self[0]),
                  log2(&self[1]),
                  log2(&self[2]))
    }
    
    #[inline(always)]
    pure fn sqrt(&self) -> Vec3<T> {
        Vec3::new(sqrt(&self[0]),
                  sqrt(&self[1]),
                  sqrt(&self[2]))
    }
    
    #[inline(always)]
    pure fn inv_sqrt(&self) -> Vec3<T> {
        Vec3::new(inv_sqrt(&self[0]),
                  inv_sqrt(&self[1]),
                  inv_sqrt(&self[2]))
    }
}

pub impl<T:Copy Exp> Vec4<T>: ExpVector<T> {
    #[inline(always)]
    pure fn pow_t(&self, n: T) -> Vec4<T> {
        Vec4::new(pow(&self[0], &n),
                  pow(&self[1], &n),
                  pow(&self[2], &n),
                  pow(&self[3], &n))
    }
    
    #[inline(always)]
    pure fn pow_v(&self, n: &Vec4<T>) -> Vec4<T> {
        Vec4::new(pow(&self[0], &n[0]),
                  pow(&self[1], &n[1]),
                  pow(&self[2], &n[2]),
                  pow(&self[3], &n[3]))
    }
}

pub impl<T:Copy Exp> Vec4<T>: Exp {
    #[inline(always)]
    pure fn pow(&self, n: &Vec4<T>) -> Vec4<T> {
        self.pow_v(n)
    }
    
    #[inline(always)]
    pure fn exp(&self) -> Vec4<T> {
        Vec4::new(exp(&self[0]),
                  exp(&self[1]),
                  exp(&self[2]),
                  exp(&self[3]))
    }
    
    #[inline(always)]
    pure fn log_(&self) -> Vec4<T> {
        Vec4::new(log_(&self[0]),
                  log_(&self[1]),
                  log_(&self[2]),
                  log_(&self[3]))
    }
    
    #[inline(always)]
    pure fn exp2(&self) -> Vec4<T> {
        Vec4::new(exp2(&self[0]),
                  exp2(&self[1]),
                  exp2(&self[2]),
                  exp2(&self[3]))
    }
    
    #[inline(always)]
    pure fn log2(&self) -> Vec4<T> {
        Vec4::new(log2(&self[0]),
                  log2(&self[1]),
                  log2(&self[2]),
                  log2(&self[3]))
    }
    
    #[inline(always)]
    pure fn sqrt(&self) -> Vec4<T> {
        Vec4::new(sqrt(&self[0]),
                  sqrt(&self[1]),
                  sqrt(&self[2]),
                  sqrt(&self[3]))
    }
    
    #[inline(always)]
    pure fn inv_sqrt(&self) -> Vec4<T> {
        Vec4::new(inv_sqrt(&self[0]),
                  inv_sqrt(&self[1]),
                  inv_sqrt(&self[2]),
                  inv_sqrt(&self[3]))
    }
}