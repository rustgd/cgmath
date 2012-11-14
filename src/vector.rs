use cast::transmute;
use cmp::{Eq, Ord};
use vec::raw::buf_as_slice;
use ptr::{addr_of, to_unsafe_ptr};
use std::cmp::FuzzyEq;

use funs::exp::Exp;
use ncast::*;
use math::*;

// GLSL equivalent type aliases

pub type vec2  = Vec2<f32>;         /// a two-component single-precision floating-point vector
pub type vec3  = Vec3<f32>;         /// a three-component single-precision floating-point vector
pub type vec4  = Vec4<f32>;         /// a four-component single-precision floating-point vector

pub type dvec2 = Vec2<f64>;         /// a two-component double-precision floating-point vector
pub type dvec3 = Vec3<f64>;         /// a three-component double-precision floating-point vector
pub type dvec4 = Vec4<f64>;         /// a four-component double-precision floating-point vector

pub type bvec2 = Vec2<bool>;        /// a two-component Boolean vector
pub type bvec3 = Vec3<bool>;        /// a three-component Boolean vector
pub type bvec4 = Vec4<bool>;        /// a four-component Boolean vector

pub type ivec2 = Vec2<i32>;         /// a two-component signed integer vector
pub type ivec3 = Vec3<i32>;         /// a three-component signed integer vector
pub type ivec4 = Vec4<i32>;         /// a four-component signed integer vector

pub type uvec2 = Vec2<u32>;         /// a two-component unsigned integer vector
pub type uvec3 = Vec3<u32>;         /// a three-component unsigned integer vector
pub type uvec4 = Vec4<u32>;         /// a four-component unsigned integer vector


pub trait Vector<T>: Eq, Index<uint, T>, ToPtr<T> {
    static pure fn dim() -> uint;
}

pub trait NumericVector<T>: Vector<T>, Neg<self>{
    pure fn mul_t(value: T) -> self;
    pure fn div_t(value: T) -> self;
    
    pure fn add_v(other: &self) -> self;
    pure fn sub_v(other: &self) -> self;
    
    pure fn dot(other: &self) -> T;
}

pub trait GeometricVector<T>: NumericVector<T> {
    pure fn length2() -> T;
    pure fn length() -> T;
    pure fn normalize() -> self;
    pure fn lerp(other: &self, amount: T) -> self;
}

pub trait Vector2<T>: Vector<T> {
    // static pure fn new(x: T, y: T) -> self;
    // static pure fn from_value(value: T) -> self;
}

pub trait Vector3<T>: Vector<T> {
    // static pure fn new(x: T, y: T, z: T) -> self;
    // static pure fn from_value(value: T) -> self;
    
    pure fn cross(other: &self) -> self;
}

pub trait Vector4<T>: Vector<T> {
    // pub static pure fn new(x: T, y: T, z: T, w: T) -> self;
    // pub static pure fn from_value(value: T) -> self;
}





//
//  Vec2
//
pub struct Vec2<T> { x: T, y: T }

pub mod Vec2 {
    
    #[inline(always)]
    pub pure fn new<T>(x: T, y: T) -> Vec2<T> {
        Vec2 { x: move x, y: move y }
    }
    
    #[inline(always)]
    pub pure fn from_value<T:Copy>(value: T) -> Vec2<T> {
        Vec2::new(value, value)
    }
    
    #[inline(always)]
    pub pure fn zero<T:Copy NumCast>() -> Vec2<T> {
        let _0 = cast(0);
        Vec2::new(_0, _0)
    }
    
    #[inline(always)]
    pub pure fn unit_x<T:Copy NumCast>() -> Vec2<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Vec2::new(_1, _0)
    }
    
    #[inline(always)]
    pub pure fn unit_y<T:Copy NumCast>() -> Vec2<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Vec2::new(_0, _1)
    }
    
    #[inline(always)]
    pub pure fn identity<T:Copy NumCast>() -> Vec2<T> {
        let _1 = cast(1);
        Vec2::new(_1, _1)
    }
}

pub impl<T> Vec2<T>: Vector<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 2 }
}
    
pub impl<T:Copy Num> Vec2<T>: NumericVector<T> {
    #[inline(always)]
    pure fn neg() -> Vec2<T> {
        Vec2::new(-self[0], -self[1])
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Vec2<T> {
        Vec2::new(self[0] * value,
                  self[1] * value)
    }
    
    #[inline(always)]
    pure fn div_t(value: T) -> Vec2<T> {
        Vec2::new(self[0] / value,
                  self[1] / value)
    }
    
    #[inline(always)]
    pure fn add_v(other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0] + other[0],
                  self[1] + other[1])
    }
    
    #[inline(always)]
    pure fn sub_v(other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0] - other[0],
                  self[1] - other[1])
    }
    
    #[inline(always)]
    pure fn dot(other: &Vec2<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1]
    }
}
    
pub impl<T:Copy Num NumCast Exp> Vec2<T>: GeometricVector<T> {
    #[inline(always)]
    pure fn length2() -> T {
        self[0] * self[0] +
        self[1] * self[1]
    }
    
    #[inline(always)]
    pure fn length() -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn normalize() -> Vec2<T> {
        let mut n: T = cast(1); 
        n /= self.length();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn lerp(other: &Vec2<T>, amount: T) -> Vec2<T> {
        self.add_v(&other.sub_v(&self).mul_t(amount))
    }
}

pub impl<T:Copy> Vec2<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Vec2<T>, *T>(
                to_unsafe_ptr(&self)), 2) |slice| { slice[i] }
        }
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy FuzzyEq> Vec2<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Vec2<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Vec2<T>) -> bool {
        !(self == *other)
    }
}

impl<T:Copy Eq> Vec2<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Vec2<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1]
    }
}

pub impl<T:Copy FuzzyEq> Vec2<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Vec2<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1])
    }
}

pub impl<T:Copy> Vec2<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr() -> *T {
        ptr::addr_of(&self[0])
    }
}






//
//  Vec3
//
pub struct Vec3<T> { x: T, y: T, z: T }

pub mod Vec3 {
    
    #[inline(always)]
    pub pure fn new<T>(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x: move x, y: move y, z: move z }
    }
    
    #[inline(always)]
    pub pure fn from_value<T:Copy>(value: T) -> Vec3<T> {
        Vec3::new(value, value, value)
    }
    
    #[inline(always)]
    pub pure fn zero<T:Copy NumCast>() -> Vec3<T> {
        let _0 = cast(0);
        Vec3::new(_0, _0, _0)
    }
    
    #[inline(always)]
    pub pure fn unit_x<T:Copy NumCast>() -> Vec3<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Vec3::new(_1, _0, _0)
    }
    
    #[inline(always)]
    pub pure fn unit_y<T:Copy NumCast>() -> Vec3<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Vec3::new(_0, _1, _0)
    }
    
    #[inline(always)]
    pub pure fn unit_z<T:Copy NumCast>() -> Vec3<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Vec3::new(_0, _0, _1)
    }
    
    #[inline(always)]
    pub pure fn identity<T:Copy NumCast>() -> Vec3<T> {
        let _1 = cast(1);
        Vec3::new(_1, _1, _1)
    }
}

pub impl<T:Copy Num> Vec3<T>: Vector3<T> {
    #[inline(always)]
    pure fn cross(other: &Vec3<T>) -> Vec3<T> {
        Vec3::new((self[1] * other[2]) - (self[2] * other[1]),
                  (self[2] * other[0]) - (self[0] * other[2]),
                  (self[0] * other[1]) - (self[1] * other[0]))
    }
}

pub impl<T> Vec3<T>: Vector<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 3 }
}

pub impl<T:Copy Num> Vec3<T>: NumericVector<T> {
    #[inline(always)]
    pure fn neg() -> Vec3<T> {
        Vec3::new(-self[0], -self[1], -self[2])
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Vec3<T> {
        Vec3::new(self[0] * value,
                  self[1] * value,
                  self[2] * value)
    }
    
    #[inline(always)]
    pure fn div_t(value: T) -> Vec3<T> {
        Vec3::new(self[0] / value,
                  self[1] / value,
                  self[2] / value)
    }
    
    #[inline(always)]
    pure fn add_v(other: &Vec3<T>) -> Vec3<T>{
        Vec3::new(self[0] + other[0],
                  self[1] + other[1],
                  self[2] + other[2])
    }
    
    #[inline(always)]
    pure fn sub_v(other: &Vec3<T>) -> Vec3<T>{
        Vec3::new(self[0] - other[0],
                  self[1] - other[1],
                  self[2] - other[2])
    }
    
    #[inline(always)]
    pure fn dot(other: &Vec3<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2]
    }
}

pub impl<T:Copy Num NumCast Exp> Vec3<T>: GeometricVector<T> {
    #[inline(always)]
    pure fn length2() -> T {
        self[0] * self[0] +
        self[1] * self[1] +
        self[2] * self[2]
    }
    
    #[inline(always)]
    pure fn length() -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn normalize() -> Vec3<T> {
        let mut n: T = cast(1);
        n /= self.length();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn lerp(other: &Vec3<T>, amount: T) -> Vec3<T> {
        self.add_v(&other.sub_v(&self).mul_t(amount))
    }
}

pub impl<T:Copy> Vec3<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Vec3<T>, *T>(
                to_unsafe_ptr(&self)), 3) |slice| { slice[i] }
        }
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy FuzzyEq> Vec3<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Vec3<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Vec3<T>) -> bool {
        !(self == *other)
    }
}

pub impl<T:Copy Eq> Vec3<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Vec3<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2]
    }
}

pub impl<T:Copy FuzzyEq> Vec3<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Vec3<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2])
    }
}

pub impl<T:Copy> Vec3<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr() -> *T {
        addr_of(&self[0])
    }
}






//
//  Vec4
//
pub struct Vec4<T> { x: T, y: T, z: T, w: T }

pub mod Vec4 {
    
    #[inline(always)]
    pub pure fn new<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 { x: move x, y: move y, z: move z, w: move w }
    }
    
    #[inline(always)]
    pub pure fn from_value<T:Copy>(value: T) -> Vec4<T> {
        Vec4::new(value, value, value, value)
    }
    
    #[inline(always)]
    pub pure fn zero<T:Copy NumCast>() -> Vec4<T> {
        let _0 = cast(0);
        Vec4::new(_0, _0, _0, _0)
    }
    
    #[inline(always)]
    pub pure fn unit_x<T:Copy NumCast>() -> Vec4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Vec4::new(_1, _0, _0, _0)
    }
    
    #[inline(always)]
    pub pure fn unit_y<T:Copy NumCast>() -> Vec4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Vec4::new(_0, _1, _0, _0)
    }
    
    #[inline(always)]
    pub pure fn unit_z<T:Copy NumCast>() -> Vec4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Vec4::new(_0, _0, _1, _0)
    }
    
    #[inline(always)]
    pub pure fn unit_w<T:Copy NumCast>() -> Vec4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Vec4::new(_0, _0, _0, _1)
    }
    
    #[inline(always)]
    pub pure fn identity<T:Copy NumCast>() -> Vec4<T> {
        let _1 = cast(1);
        Vec4::new(_1, _1, _1, _1)
    }
}

pub impl<T> Vec4<T>: Vector<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 4 }
}

pub impl<T:Copy Num> Vec4<T>: NumericVector<T> {
    #[inline(always)]
    pure fn neg() -> Vec4<T> {
        Vec4::new(-self[0], -self[1], -self[2], -self[3])
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Vec4<T> {
        Vec4::new(self[0] * value,
                  self[1] * value,
                  self[2] * value,
                  self[3] * value)
    }
    
    #[inline(always)]
    pure fn div_t(value: T) -> Vec4<T> {
        Vec4::new(self[0] / value,
                  self[1] / value,
                  self[2] / value,
                  self[3] / value)
    }
    
    #[inline(always)]
    pure fn add_v(other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0] + other[0],
                  self[1] + other[1],
                  self[2] + other[2],
                  self[3] + other[3])
    }
    
    #[inline(always)]
    pure fn sub_v(other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0] - other[0],
                  self[1] - other[1],
                  self[2] - other[2],
                  self[3] - other[3])
    }
    
    #[inline(always)]
    pure fn dot(other: &Vec4<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2] +
        self[3] * other[3]
    }
}

pub impl<T:Copy Num NumCast Exp> Vec4<T>: GeometricVector<T> {
    #[inline(always)]
    pure fn length2() -> T {
        self[0] * self[0] +
        self[1] * self[1] +
        self[2] * self[2] +
        self[3] * self[3]
    }
    
    #[inline(always)]
    pure fn length() -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn normalize() -> Vec4<T> {
        let mut n: T = cast(1);
        n /= self.length();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn lerp(other: &Vec4<T>, amount: T) -> Vec4<T> {
        self.add_v(&other.sub_v(&self).mul_t(amount))
    }
}

pub impl<T:Copy> Vec4<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Vec4<T>, *T>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy FuzzyEq> Vec4<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Vec4<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Vec4<T>) -> bool {
        !(self == *other)
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy Eq> Vec4<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Vec4<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2] &&
        self[3] == other[3]
    }
}

pub impl<T:Copy FuzzyEq> Vec4<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Vec4<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
}

pub impl<T:Copy> Vec4<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr() -> *T {
        addr_of(&self[0])
    }
}