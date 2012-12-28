/**
 * Common Functions
 *
 * This module corresponds to Section 8.3 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */

use numeric::funs::*;
use numeric::traits::*;

use vec::{Vector, Vec2, Vec3, Vec4};

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
    // pure fn round_even(&self)  -> Vec2<T> {
    //     Vec2::new(round_even(&self[0]),
    //               round_even(&self[1]))
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
    // pure fn round_even(&self)  -> Vec3<T> {
    //     Vec3::new(round_even(&self[0]),
    //               round_even(&self[1]),
    //               round_even(&self[2]))
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
    // pure fn round_even(&self)  -> Vec4<T> {
    //     Vec4::new(round_even(&self[0]),
    //               round_even(&self[1]),
    //               round_even(&self[2]),
    //               round_even(&self[3]))
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

pub trait ExtentVector<T>: Vector<T> {
    pure fn min_t(&self, value: T) -> self;
    pure fn max_t(&self, value: T) -> self;
    pure fn clamp_t(&self, mn: T, mx: T) -> self;
    
    pure fn min_v(&self, other: &self) -> self;
    pure fn max_v(&self, other: &self) -> self;
    pure fn clamp_v(&self, mn: &self, mx: &self) -> self;
}

pub impl<T:Copy Extent> Vec2<T>: ExtentVector<T> {
    #[inline(always)]
    pure fn min_t(&self, value: T) -> Vec2<T> {
        Vec2::new(min(&self[0], &value),
                  min(&self[1], &value))
    }
    
    #[inline(always)]
    pure fn max_t(&self, value: T) -> Vec2<T> {
        Vec2::new(max(&self[0], &value),
                  max(&self[1], &value))
    }
    
    #[inline(always)]
    pure fn clamp_t(&self, mn: T, mx: T) -> Vec2<T> {
        Vec2::new(self[0].clamp(&mn, &mx),
                  self[1].clamp(&mn, &mx))
    }
    
    #[inline(always)]
    pure fn min_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]))
    }
    
    #[inline(always)]
    pure fn max_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]))
    }
    
    #[inline(always)]
    pure fn clamp_v(&self, mn: &Vec2<T>, mx: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]))
    }
}

pub impl<T:Copy Extent> Vec3<T>: ExtentVector<T> {
    #[inline(always)]
    pure fn min_t(&self, value: T) -> Vec3<T> {
        Vec3::new(min(&self[0], &value),
                  min(&self[1], &value),
                  min(&self[2], &value))
    }
    
    #[inline(always)]
    pure fn max_t(&self, value: T) -> Vec3<T> {
        Vec3::new(max(&self[0], &value),
                  max(&self[1], &value),
                  max(&self[2], &value))
    }
    
    #[inline(always)]
    pure fn clamp_t(&self, mn: T, mx: T) -> Vec3<T> {
        Vec3::new(self[0].clamp(&mn, &mx),
                  self[1].clamp(&mn, &mx),
                  self[2].clamp(&mn, &mx))
    }
    
    #[inline(always)]
    pure fn min_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]),
                  min(&self[2], &other[2]))
    }
    
    #[inline(always)]
    pure fn max_v(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]),
                  max(&self[2], &other[2]))
    }
    
    #[inline(always)]
    pure fn clamp_v(&self, mn: &Vec3<T>, mx: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]),
                  self[2].clamp(&mn[2], &mx[2]))
    }
}

pub impl<T:Copy Extent> Vec4<T>: ExtentVector<T> {
    #[inline(always)]
    pure fn min_t(&self, value: T) -> Vec4<T> {
        Vec4::new(min(&self[0], &value),
                  min(&self[1], &value),
                  min(&self[2], &value),
                  min(&self[3], &value))
    }
    
    #[inline(always)]
    pure fn max_t(&self, value: T) -> Vec4<T> {
        Vec4::new(max(&self[0], &value),
                  max(&self[1], &value),
                  max(&self[2], &value),
                  max(&self[3], &value))
    }
    
    #[inline(always)]
    pure fn clamp_t(&self, mn: T, mx: T) -> Vec4<T> {
        Vec4::new(self[0].clamp(&mn, &mx),
                  self[1].clamp(&mn, &mx),
                  self[2].clamp(&mn, &mx),
                  self[3].clamp(&mn, &mx))
    }
    
    #[inline(always)]
    pure fn min_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(min(&self[0], &other[0]),
                  min(&self[1], &other[1]),
                  min(&self[2], &other[2]),
                  min(&self[3], &other[3]))
    }
    
    #[inline(always)]
    pure fn max_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(max(&self[0], &other[0]),
                  max(&self[1], &other[1]),
                  max(&self[2], &other[2]),
                  max(&self[3], &other[3]))
    }
    
    #[inline(always)]
    pure fn clamp_v(&self, mn: &Vec4<T>, mx: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].clamp(&mn[0], &mx[0]),
                  self[1].clamp(&mn[1], &mx[1]),
                  self[2].clamp(&mn[2], &mx[2]),
                  self[3].clamp(&mn[3], &mx[3]))
    }
}

// Mix

pub trait MixVector<T>: Vector<T> {
    pure fn mix_t(&self, other: T, value: T) -> self;
    pure fn smooth_step_t(&self, edge0: T, edge1: T) -> self;
    pure fn step_t(&self, edge: T) -> self;
    
    pure fn mix_v(&self, other: &self, value: &self) -> self;
    pure fn smooth_step_v(&self, edge0: &self, edge1: &self) -> self;
    pure fn step_v(&self, edge: &self) -> self;
}

pub impl<T:Copy Mix> Vec2<T>: MixVector<T> {
    #[inline(always)]
    pure fn mix_t(&self, other: T, value: T) -> Vec2<T> {
        Vec2::new(self[0].mix(&other, &value),
                  self[1].mix(&other, &value))
    }
    
    #[inline(always)]
    pure fn smooth_step_t(&self, edge0: T, edge1: T) -> Vec2<T> {
        Vec2::new(self[0].smooth_step(&edge0, &edge1),
                  self[1].smooth_step(&edge0, &edge1))
    }
    
    #[inline(always)]
    pure fn step_t(&self, edge: T) -> Vec2<T> {
        Vec2::new(self[0].step(&edge),
                  self[1].step(&edge))
    }
    
    #[inline(always)]
    pure fn mix_v(&self, other: &Vec2<T>, value: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]))
    }
    
    #[inline(always)]
    pure fn smooth_step_v(&self, edge0: &Vec2<T>, edge1: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]))
    }
    
    #[inline(always)]
    pure fn step_v(&self, edge: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]))
    }
}

pub impl<T:Copy Mix> Vec3<T>: MixVector<T> {
    #[inline(always)]
    pure fn mix_t(&self, other: T, value: T) -> Vec3<T> {
        Vec3::new(self[0].mix(&other, &value),
                  self[1].mix(&other, &value),
                  self[2].mix(&other, &value))
    }
    
    #[inline(always)]
    pure fn smooth_step_t(&self, edge0: T, edge1: T) -> Vec3<T> {
        Vec3::new(self[0].smooth_step(&edge0, &edge1),
                  self[1].smooth_step(&edge0, &edge1),
                  self[2].smooth_step(&edge0, &edge1))
    }
    
    #[inline(always)]
    pure fn step_t(&self, edge: T) -> Vec3<T> {
        Vec3::new(self[0].step(&edge),
                  self[1].step(&edge),
                  self[2].step(&edge))
    }
    
    #[inline(always)]
    pure fn mix_v(&self, other: &Vec3<T>, value: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]),
                  self[2].mix(&other[2], &value[2]))
    }
    
    #[inline(always)]
    pure fn smooth_step_v(&self, edge0: &Vec3<T>, edge1: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]),
                  self[2].smooth_step(&edge0[2], &edge1[2]))
    }
    
    #[inline(always)]
    pure fn step_v(&self, edge: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]),
                  self[2].step(&edge[2]))
    }
}

pub impl<T:Copy Mix> Vec4<T>: MixVector<T> {
    #[inline(always)]
    pure fn mix_t(&self, other: T, value: T) -> Vec4<T> {
        Vec4::new(self[0].mix(&other, &value),
                  self[1].mix(&other, &value),
                  self[2].mix(&other, &value),
                  self[3].mix(&other, &value))
    }
    
    #[inline(always)]
    pure fn smooth_step_t(&self, edge0: T, edge1: T) -> Vec4<T> {
        Vec4::new(self[0].smooth_step(&edge0, &edge1),
                  self[1].smooth_step(&edge0, &edge1),
                  self[2].smooth_step(&edge0, &edge1),
                  self[3].smooth_step(&edge0, &edge1))
    }
    
    #[inline(always)]
    pure fn step_t(&self, edge: T) -> Vec4<T> {
        Vec4::new(self[0].step(&edge),
                  self[1].step(&edge),
                  self[2].step(&edge),
                  self[3].step(&edge))
    }
    
    #[inline(always)]
    pure fn mix_v(&self, other: &Vec4<T>, value: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]),
                  self[2].mix(&other[2], &value[2]),
                  self[3].mix(&other[3], &value[3]))
    }
    
    #[inline(always)]
    pure fn smooth_step_v(&self, edge0: &Vec4<T>, edge1: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]),
                  self[2].smooth_step(&edge0[2], &edge1[2]),
                  self[3].smooth_step(&edge0[3], &edge1[3]))
    }
    
    #[inline(always)]
    pure fn step_v(&self, edge: &Vec4<T>) -> Vec4<T> {
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
