use cast::transmute;
use cmp::Eq;
use num::from_int;
use ptr::to_unsafe_ptr;
use vec::raw::buf_as_slice;
use std::cmp::FuzzyEq;

use funs::exp::Exp;
use math::*;
use matrix::{Mat3, Mat4};
use ncast::*;
use vector::Vec3;


// These quaternion type aliases are not actually specified in the GLSL spec
// but they follow the same nomenclature

pub type quat4  = Quat<f32>;         /// a single-precision floating-point quaternion
pub type dquat4 = Quat<f64>;         /// a double-precision floating-point quaternion

//
//  Quaternion
//
pub trait Quaternion<T> {
    pure fn dim() -> uint;
    
    pure fn mul_t(value: T) -> self;
    pure fn div_t(value: T) -> self;
    
    // pure fn mul_v(other: &Vec3) -> Vec3;
    
    pure fn add_q(other: &self) -> self;
    pure fn sub_q(other: &self) -> self;
    pure fn mul_q(other: &self) -> self;
    
    pure fn conjugate() -> self;
    pure fn inverse() -> self;
    pure fn length2() -> T;
    pure fn length() -> T;
    
    pure fn to_Mat3() -> Mat3<T>;
    pure fn to_Mat4() -> Mat4<T>;
}

pub trait ToQuat<T> {
    pure fn to_Quat() -> Quat<T>;
}






pub struct Quat<T> { w: T, x: T, y: T, z: T }

pub mod Quat {
    
    #[inline(always)]
    pub pure fn new<T>(w: T, x: T, y: T, z: T) -> Quat<T> {
        Quat { w: move w, x: move x, y: move y, z: move z }
    }
    
    #[inline(always)]
    pub pure fn from_sv<T:Copy>(s: T, v: Vec3<T>) -> Quat<T> {
        Quat::new(s, v.x, v.y, v.z)
    }
    
    #[inline(always)]
    pub pure fn zero<T:Copy NumCast>() -> Quat<T> {
        let _0 = cast(0);
        Quat::new(_0, _0, _0, _0)
    }
    
    #[inline(always)]
    pub pure fn identity<T:Copy NumCast>() -> Quat<T> {
        let _0 = cast(0);
        Quat::new(cast(1), _0, _0, _0)
    }
}

pub impl<T:Copy Num NumCast Exp FuzzyEq> Quat<T>: Quaternion<T> {
    #[inline(always)]
    pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Quat<T> {
        Quat::new(self[0] * value,
                  self[1] * value,
                  self[2] * value,
                  self[3] * value)
    }
    
    #[inline(always)]
    pure fn div_t(value: T) -> Quat<T> {
        Quat::new(self[0] / value,
                  self[1] / value,
                  self[2] / value,
                  self[3] / value)
    }
    
    #[inline(always)]
    pure fn add_q(other: &Quat<T>) -> Quat<T> {
        Quat::new(self[0] + other[0],
                  self[1] + other[1],
                  self[2] + other[2],
                  self[3] + other[3])
    }
    
    #[inline(always)]
    pure fn sub_q(other: &Quat<T>) -> Quat<T> {
        Quat::new(self[0] - other[0],
                  self[1] - other[1],
                  self[2] - other[2],
                  self[3] - other[3])
    }
    
    #[inline(always)]
    pure fn mul_q(other: &Quat<T>) -> Quat<T> {
        Quat::new(self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
                  self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y, 
                  self.w * other.y + self.y * other.w + self.z * other.x - self.x * other.z, 
                  self.w * other.z + self.z * other.w + self.x * other.y - self.y * other.x) 
    }
    
    #[inline(always)]
    pure fn conjugate() -> Quat<T> {
        Quat::new(self.w, -self.x, -self.y, -self.z)
    }
    
    #[inline(always)]
    pure fn inverse() -> Quat<T> {
        let mut n: T = from_int(1);
        n /= self.length2();
        self.conjugate().mul_t(n)
    }
    
    #[inline(always)]
    pure fn length2() -> T {
        self.w * self.w +
        self.x * self.x +
        self.y * self.y +
        self.z * self.z
    }
    
    #[inline(always)]
    pure fn length() -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn to_Mat3() -> Mat3<T> {
        let x2 = self.x + self.x;
        let y2 = self.y + self.y;
        let z2 = self.z + self.z;
        
        let xx2 = x2 * self.x;
        let xy2 = x2 * self.y;
        let xz2 = x2 * self.z;
        
        let yy2 = y2 * self.y;
        let yz2 = y2 * self.z;
        let zz2 = z2 * self.z;
        
        let wy2 = y2 * self.w;
        let wz2 = z2 * self.w;
        let wx2 = x2 * self.w;
        
        let _1: T = from_int(1);
        
        Mat3::new(_1 - yy2 - zz2,      xy2 - wz2,      xz2 + wy2,
                       xy2 + wz2, _1 - xx2 - zz2,      yz2 - wx2,
                       xz2 - wy2,      yz2 + wx2, _1 - xx2 - yy2)
    }
    
    #[inline(always)]
    pure fn to_Mat4() -> Mat4<T> {
        self.to_Mat3().to_Mat4()
    }
}

pub impl<T:Copy> Quat<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Quat<T>, *T>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy Neg<T>> Quat<T>: Neg<Quat<T>> {
    #[inline(always)]
    pure fn neg() -> Quat<T> {
        Quat::new(-self[0], -self[1], -self[2], -self[3])
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy FuzzyEq> Quat<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Quat<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Quat<T>) -> bool {
        !(self == *other)
    }
}

pub impl<T:Copy Eq> Quat<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Quat<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2] &&
        self[3] == other[3]
    }
}

pub impl<T:Copy FuzzyEq> Quat<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Quat<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
}

pub impl<T:Copy> Quat<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr() -> *T {
        to_unsafe_ptr(&self[0])
    }
}