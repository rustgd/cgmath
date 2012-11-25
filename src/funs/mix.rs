use num::cast::*;
use vec::{Vec2, Vec3, Vec4};

use extent::{Clamp, clamp};

pub trait Mix {
    pure fn mix(other: &self, value: &self) -> self;
    pure fn smooth_step(edge0: &self, edge1: &self) -> self;
    pure fn step(edge: &self) -> self;
}

#[inline(always)] pub pure fn mix<T:Mix>(a: &T, b: &T, value: &T) -> T { a.mix(b, value) }
#[inline(always)] pub pure fn smooth_step<T:Mix>(x: &T, edge0: &T, edge1: &T) -> T { x.smooth_step(edge0, edge1) }
#[inline(always)] pub pure fn step<T:Mix>(x: &T, edge: &T) -> T { x.step(edge) }

pub impl f32: Mix {
    #[inline(always)]
    pure fn mix(other: &f32, value: &f32) -> f32 {
        self * (1.0 - (*value)) + (*other) * (*value)
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &f32, edge1: &f32) -> f32 {
        let t = clamp(&((self - *edge0) / (*edge1 - *edge0)), &0.0, &1.0);
        return t * t * (3.0 - 2.0 * t);
    }
    
    #[inline(always)]
    pure fn step(edge: &f32) -> f32 {
        if self < *edge { 0.0 } else { 1.0 }
    }
}

pub impl f64: Mix {
    #[inline(always)]
    pure fn mix(other: &f64, value: &f64) -> f64 {
        self * (1.0 - (*value)) + (*other) * (*value)
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &f64, edge1: &f64) -> f64 {
        let t = clamp(&((self - *edge0) / (*edge1 - *edge0)), &0.0, &1.0);
        return t * t * (3.0 - 2.0 * t);
    }
    
    #[inline(always)]
    pure fn step(edge: &f64) -> f64 {
        if self < *edge { 0.0 } else { 1.0 }
    }
}

pub impl float: Mix {
    #[inline(always)]
    pure fn mix(other: &float, value: &float) -> float {
        self * (1.0 - (*value)) + (*other) * (*value)
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &float, edge1: &float) -> float {
        let t = clamp(&((self - *edge0) / (*edge1 - *edge0)), &0.0, &1.0);
        return t * t * (3.0 - 2.0 * t);
    }
    
    #[inline(always)]
    pure fn step(edge: &float) -> float {
        if self < *edge { 0.0 } else { 1.0 }
    }
}

pub impl<T:Copy Mix> Vec2<T>: Mix {
    #[inline(always)]
    pure fn mix(other: &Vec2<T>, value: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]))
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &Vec2<T>, edge1: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]))
    }
    
    #[inline(always)]
    pure fn step(edge: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]))
    }
}

pub impl<T:Copy Mix> Vec3<T>: Mix {
    #[inline(always)]
    pure fn mix(other: &Vec3<T>, value: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]),
                  self[2].mix(&other[2], &value[2]))
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &Vec3<T>, edge1: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]),
                  self[2].smooth_step(&edge0[2], &edge1[2]))
    }
    
    #[inline(always)]
    pure fn step(edge: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]),
                  self[2].step(&edge[2]))
    }
}

pub impl<T:Copy Mix> Vec4<T>: Mix {
    #[inline(always)]
    pure fn mix(other: &Vec4<T>, value: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].mix(&other[0], &value[0]),
                  self[1].mix(&other[1], &value[1]),
                  self[2].mix(&other[2], &value[2]),
                  self[3].mix(&other[3], &value[3]))
    }
    
    #[inline(always)]
    pure fn smooth_step(edge0: &Vec4<T>, edge1: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].smooth_step(&edge0[0], &edge1[0]),
                  self[1].smooth_step(&edge0[1], &edge1[1]),
                  self[2].smooth_step(&edge0[2], &edge1[2]),
                  self[3].smooth_step(&edge0[3], &edge1[3]))
    }
    
    #[inline(always)]
    pure fn step(edge: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].step(&edge[0]),
                  self[1].step(&edge[1]),
                  self[2].step(&edge[2]),
                  self[3].step(&edge[3]))
    }
}