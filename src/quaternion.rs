use cast::transmute;
use vec::raw::buf_as_slice;
use ptr::to_unsafe_ptr;
use cmp::Eq;
use std::cmp::FuzzyEq;
use math::*;
use matrix::{Mat3, Mat4};
use vector::Vec3;

//
//  Quaternion
//
pub trait Quaternion<T> {
    pure fn dim() -> uint;
    
    pure fn mul_f(value: T) -> self;
    pure fn div_f(value: T) -> self;
    
    // pure fn mul_v(other: &Vec3) -> Vec3;
    
    pure fn add_q(other: &self) -> self;
    pure fn sub_q(other: &self) -> self;
    pure fn mul_q(other: &self) -> self;
    
    pure fn exact_eq(other: &self) -> bool;
    
    pure fn conjugate() -> self;
    pure fn inverse() -> self;
    pure fn magnitude2() -> T;
    pure fn magnitude() -> T;
    
    pure fn to_Mat3() -> Mat3;
    pure fn to_Mat4() -> Mat4;
}






//
//  Quat struct definition
//
pub struct Quat { w: float, x: float, y: float, z: float }

pub const quat_zero     :Quat = Quat { w: 0f, x: 0f, y: 0f, z: 0f };
pub const quat_identity :Quat = Quat { w: 1f, x: 0f, y: 0f, z: 0f };

//
//  Quat Constructor
//
#[inline]
pub pure fn Quat(w: float, x: float, y: float, z: float) -> Quat {
    Quat { w: w, x: x, y: y, z: z }
}

//
//  Quaternion Implementation
//
pub impl Quat: Quaternion<float> {
    #[inline]
    pure fn dim() -> uint { 4 }
    
    #[inline]
    pure fn mul_f(value: float) -> Quat {
        Quat(self[0] * value,
             self[1] * value,
             self[2] * value,
             self[3] * value)
    }
    
    #[inline]
    pure fn div_f(value: float) -> Quat {
        Quat(self[0] / value,
             self[1] / value,
             self[2] / value,
             self[3] / value)
    }
    
    #[inline]
    pure fn add_q(other: &Quat) -> Quat{
        Quat(self[0] + other[0],
             self[1] + other[1],
             self[2] + other[2],
             self[3] + other[3])
    }
    
    #[inline]
    pure fn sub_q(other: &Quat) -> Quat{
        Quat(self[0] - other[0],
             self[1] - other[1],
             self[2] - other[2],
             self[3] - other[3])
    }
    
    #[inline]
    pure fn mul_q(other: &Quat) -> Quat {
        Quat(self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
             self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y, 
             self.w * other.y + self.y * other.w + self.z * other.x - self.x * other.z, 
             self.w * other.z + self.z * other.w + self.x * other.y - self.y * other.x) 
    }
    
    #[inline]
    pure fn exact_eq(other: &Quat) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2] &&
        self[3] == other[3]
    }
    
    #[inline]
    pure fn conjugate() -> Quat {
        Quat(self.w, -self.x, -self.y, -self.z)
    }
    
    #[inline]
    pure fn inverse() -> Quat {
        self.conjugate().mul_f((1f / self.magnitude2()))
    }
    
    #[inline]
    pure fn magnitude2() -> float {
        self.w * self.w +
        self.x * self.x +
        self.y * self.y +
        self.z * self.z
    }
    
    #[inline]
    pure fn magnitude() -> float {
        self.magnitude2().sqrt()
    }
    
    #[inline]
    pure fn to_Mat3() -> Mat3 {
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
        
        return Mat3(1f - yy2 - zz2,      xy2 - wz2,      xz2 + wy2,
                         xy2 + wz2, 1f - xx2 - zz2,      yz2 - wx2,
                         xz2 - wy2,      yz2 + wx2, 1f - xx2 - yy2);
    }
    
    #[inline]
    pure fn to_Mat4() -> Mat4 {
        self.to_Mat3().to_Mat4()
    }
}

pub impl Quat: Index<uint, float> {
    #[inline]
    pure fn index(i: uint) -> float {
        unsafe {
            do buf_as_slice(
                transmute::<*Quat, *float>(
                    to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl Quat: Neg<Quat> {
    #[inline]
    pure fn neg() -> Quat {
        Quat(-self[0], -self[1], -self[2], -self[3])
    }
}

pub impl Quat: Eq {
    #[inline]
    pure fn eq(other: &Quat) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline]
    pure fn ne(other: &Quat) -> bool {
        !(self == *other)
    }
}

pub impl Quat: FuzzyEq {
    #[inline]
    pure fn fuzzy_eq(other: &Quat) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
}

pub impl Quat: ToStr {
    pure fn to_str() -> ~str {
        fmt!("Quat[ %f, %f, %f, %f ]", self.w, self.x, self.y, self.z)
    }
}