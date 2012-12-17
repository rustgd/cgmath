/**
 * Common Functions
 *
 * This module corresponds to Section 8.3 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */

use numeric::funs::*;
use numeric::traits::*;

use vec::{Vec2, Vec3, Vec4};

// Approx

pub impl<T:Copy Approx> Vec2<T>: Approx {
    #[inline(always)]
    pure fn floor(&self) -> Vec2<T> {
        Vec2::new(floor(&self[0]),
                  floor(&self[1]))
    }
    
    #[inline(always)]
    pure fn trunc(&self) -> Vec2<T> {
        Vec2::new(trunc(&self[0]),
                  trunc(&self[1]))
    }
    
    #[inline(always)]
    pure fn round(&self) -> Vec2<T> {
        Vec2::new(round(&self[0]),
                  round(&self[1]))
    }
    
    // #[inline(always)]
    // pure fn ceil(&self)  -> Vec2<T> {
    //     Vec2::new(roundEven(&self[0]),
    //               roundEven(&self[1]))
    // }
    
    #[inline(always)]
    pure fn ceil(&self)  -> Vec2<T> {
        Vec2::new(ceil(&self[0]),
                  ceil(&self[1]))
    }
    
    #[inline(always)]
    pure fn fract(&self) -> Vec2<T> {
        Vec2::new(fract(&self[0]),
                  fract(&self[1]))
    }
}

pub impl<T:Copy Approx> Vec3<T>: Approx {
    #[inline(always)]
    pure fn floor(&self) -> Vec3<T> {
        Vec3::new(floor(&self[0]),
                  floor(&self[1]),
                  floor(&self[2]))
    }
    
    #[inline(always)]
    pure fn trunc(&self) -> Vec3<T> {
        Vec3::new(trunc(&self[0]),
                  trunc(&self[1]),
                  trunc(&self[2]))
    }
    
    #[inline(always)]
    pure fn round(&self) -> Vec3<T> {
        Vec3::new(round(&self[0]),
                  round(&self[1]),
                  round(&self[2]))
    }
    
    // #[inline(always)]
    // pure fn ceil(&self)  -> Vec3<T> {
    //     Vec3::new(roundEven(&self[0]),
    //               roundEven(&self[1]),
    //               roundEven(&self[2]))
    // }
    
    #[inline(always)]
    pure fn ceil(&self)  -> Vec3<T> {
        Vec3::new(ceil(&self[0]),
                  ceil(&self[1]),
                  ceil(&self[2]))
    }
    
    #[inline(always)]
    pure fn fract(&self) -> Vec3<T> {
        Vec3::new(fract(&self[0]),
                  fract(&self[1]),
                  fract(&self[2]))
    }
}

pub impl<T:Copy Approx> Vec4<T>: Approx {
    #[inline(always)]
    pure fn floor(&self) -> Vec4<T> {
        Vec4::new(floor(&self[0]),
                  floor(&self[1]),
                  floor(&self[2]),
                  floor(&self[3]))
    }
    
    #[inline(always)]
    pure fn trunc(&self) -> Vec4<T> {
        Vec4::new(trunc(&self[0]),
                  trunc(&self[1]),
                  trunc(&self[2]),
                  trunc(&self[3]))
    }
    
    #[inline(always)]
    pure fn round(&self) -> Vec4<T> {
        Vec4::new(round(&self[0]),
                  round(&self[1]),
                  round(&self[2]),
                  round(&self[3]))
    }
    
    // #[inline(always)]
    // pure fn ceil(&self)  -> Vec4<T> {
    //     Vec4::new(roundEven(&self[0]),
    //               roundEven(&self[1]),
    //               roundEven(&self[2]),
    //               roundEven(&self[3]))
    // }
    
    #[inline(always)]
    pure fn ceil(&self)  -> Vec4<T> {
        Vec4::new(ceil(&self[0]),
                  ceil(&self[1]),
                  ceil(&self[2]),
                  ceil(&self[3]))
    }
    
    #[inline(always)]
    pure fn fract(&self) -> Vec4<T> {
        Vec4::new(fract(&self[0]),
                  fract(&self[1]),
                  fract(&self[2]),
                  fract(&self[3]))
    }
}

// Extent

pub impl<T:Copy Extent> Vec2<T>: Extent {
    #[inline(always)]
    pure fn min(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]))
    }
    
    #[inline(always)]
    pure fn max(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]))
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &Vec2<T>, mx: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]))
    }
}

pub impl<T:Copy Extent> Vec3<T>: Extent {
    #[inline(always)]
    pure fn min(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]),
                  min(&self[2], &other[2]))
    }
    
    #[inline(always)]
    pure fn max(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]),
                  max(&self[2], &other[2]))
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &Vec3<T>, mx: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]),
                  self[2].clamp(&mn[2], &mx[2]))
    }
}

pub impl<T:Copy Extent> Vec4<T>: Extent {
    #[inline(always)]
    pure fn min(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]),
                  min(&self[2], &other[2]),
                  min(&self[3], &other[3]))
    }
    
    #[inline(always)]
    pure fn max(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]),
                  max(&self[2], &other[2]),
                  max(&self[3], &other[3]))
    }
    
    #[inline(always)]
    pure fn clamp(&self, mn: &Vec4<T>, mx: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]),
                  self[2].clamp(&mn[2], &mx[2]),
                  self[3].clamp(&mn[3], &mx[3]))
    }
}

// Mix

pub impl<T:Copy Mix> Vec2<T>: Mix {
    #[inline(always)]
    pure fn mix(&self, other: &Vec2<T>, value: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]))
    }
    
    #[inline(always)]
    pure fn smooth_step(&self, edge0: &Vec2<T>, edge1: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]))
    }
    
    #[inline(always)]
    pure fn step(&self, edge: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]))
    }
}

pub impl<T:Copy Mix> Vec3<T>: Mix {
    #[inline(always)]
    pure fn mix(&self, other: &Vec3<T>, value: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]),
                  self[2].mix(&other[2], &value[2]))
    }
    
    #[inline(always)]
    pure fn smooth_step(&self, edge0: &Vec3<T>, edge1: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]),
                  self[2].smooth_step(&edge0[2], &edge1[2]))
    }
    
    #[inline(always)]
    pure fn step(&self, edge: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]),
                  self[2].step(&edge[2]))
    }
}

pub impl<T:Copy Mix> Vec4<T>: Mix {
    #[inline(always)]
    pure fn mix(&self, other: &Vec4<T>, value: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]),
                  self[2].mix(&other[2], &value[2]),
                  self[3].mix(&other[3], &value[3]))
    }
    
    #[inline(always)]
    pure fn smooth_step(&self, edge0: &Vec4<T>, edge1: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]),
                  self[2].smooth_step(&edge0[2], &edge1[2]),
                  self[3].smooth_step(&edge0[3], &edge1[3]))
    }
    
    #[inline(always)]
    pure fn step(&self, edge: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]),
                  self[2].step(&edge[2]),
                  self[3].step(&edge[3]))
    }
}

// Sign

pub impl<T:Copy Sign> Vec2<T>: Sign {
    #[inline(always)]
    pure fn abs(&self) -> Vec2<T> {
        Vec2::new(abs(&self[0]),
                  abs(&self[1]))
    }
    
    #[inline(always)]
    pure fn sign(&self) -> Vec2<T> {
        Vec2::new(sign(&self[0]),
                  sign(&self[1]))
    }
}

pub impl<T:Copy Sign> Vec3<T>: Sign {
    #[inline(always)]
    pure fn abs(&self) -> Vec3<T> {
        Vec3::new(abs(&self[0]),
                  abs(&self[1]),
                  abs(&self[2]))
    }
    
    #[inline(always)]
    pure fn sign(&self) -> Vec3<T> {
        Vec3::new(sign(&self[0]),
                  sign(&self[1]),
                  sign(&self[2]))
    }
}

pub impl<T:Copy Sign> Vec4<T>: Sign {
    #[inline(always)]
    pure fn abs(&self) -> Vec4<T> {
        Vec4::new(abs(&self[0]),
                  abs(&self[1]),
                  abs(&self[2]),
                  abs(&self[3]))
    }
    
    #[inline(always)]
    pure fn sign(&self) -> Vec4<T> {
        Vec4::new(sign(&self[0]),
                  sign(&self[1]),
                  sign(&self[2]),
                  sign(&self[3]))
    }
}
