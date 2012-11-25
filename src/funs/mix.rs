use num::cast::*;
use vec::{Vec2, Vec3, Vec4};

pub trait Mix {
    pure fn mix(other: &self, value: &self) -> self;
}

pub impl f32: Mix {
    #[inline(always)]
    pure fn mix(other: &f32, value: &f32) -> f32 {
        self * (1f32 - (*value)) + (*other) * (*value)
    }
}

pub impl f64: Mix {
    #[inline(always)]
    pure fn mix(other: &f64, value: &f64) -> f64 {
        self * (1f64 - (*value)) + (*other) * (*value)
    }
}

pub impl float: Mix {
    #[inline(always)]
    pure fn mix(other: &float, value: &float) -> float {
        self * (1f - (*value)) + (*other) * (*value)
    }
}



pub trait MixV<T> {
    pure fn mix(other: &self, value: &T) -> self;
    pure fn mix_v(other: &self, values: &self) -> self;
}

pub impl<T:Copy Mix> Vec2<T>: MixV<T> {
    #[inline(always)]
    pure fn mix(other: &Vec2<T>, value: &T) -> Vec2<T> {
        Vec2::new(self[0].mix(&other[0], value),
                  self[1].mix(&other[1], value))
    }
    
    #[inline(always)]
    pure fn mix_v(other: &Vec2<T>, values: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0].mix(&other[0], &values[0]),
                  self[1].mix(&other[1], &values[1]))
    }
}

pub impl<T:Copy Mix> Vec3<T>: MixV<T> {
    #[inline(always)]
    pure fn mix(other: &Vec3<T>, value: &T) -> Vec3<T> {
        Vec3::new(self[0].mix(&other[0], value),
                  self[1].mix(&other[1], value),
                  self[2].mix(&other[2], value))
    }
    
    #[inline(always)]
    pure fn mix_v(other: &Vec3<T>, values: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0].mix(&other[0], &values[0]),
                  self[1].mix(&other[1], &values[1]),
                  self[2].mix(&other[2], &values[2]))
    }
}

pub impl<T:Copy Mix> Vec4<T>: MixV<T> {
    #[inline(always)]
    pure fn mix(other: &Vec4<T>, value: &T) -> Vec4<T> {
        Vec4::new(self[0].mix(&other[0], value),
                  self[1].mix(&other[1], value),
                  self[2].mix(&other[2], value),
                  self[3].mix(&other[3], value))
    }
    
    #[inline(always)]
    pure fn mix_v(other: &Vec4<T>, values: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0].mix(&other[0], &values[0]),
                  self[1].mix(&other[1], &values[1]),
                  self[2].mix(&other[2], &values[2]),
                  self[3].mix(&other[3], &values[3]))
    }
}