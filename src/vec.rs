use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;

use dim::Dimensional;
use funs::exponential::Exp;
use num::cast::*;
use num::default_eq::DefaultEq;

///
/// The base vector trait
///
pub trait Vector<T>: Dimensional<T>, Eq, DefaultEq {
    /// Construct the vector from a single value, copying it to each component
    static pure fn from_value(value: T) -> self;
}

/// A 2-dimensional vector
pub trait Vector2<T>: Vector<T> {
//     static pure fn new(x: T, y: T) -> self;
}

/// A 3-dimensional vector
pub trait Vector3<T>: Vector<T> {
//     static pure fn new(x: T, y: T, z: T) -> self;
}

/// A 4-dimensional vector
pub trait Vector4<T>: Vector<T> {
//     static pure fn new(x: T, y: T, z: T, w: T) -> self;
}

///
/// A vector with numeric components
///
pub trait NumericVector<T>: Vector<T>, Neg<self>{
    static pure fn identity() -> self;
    static pure fn zero() -> self;
    
    pure fn mul_t(value: T) -> self;
    pure fn div_t(value: T) -> self;
    
    pure fn add_v(other: &self) -> self;
    pure fn sub_v(other: &self) -> self;
    
    pure fn dot(other: &self) -> T;
}

/// A 2-dimensional vector with numeric components
pub trait NumericVector2<T>: NumericVector<T> {
//     static pure fn unit_x() -> self;
//     static pure fn unit_y() -> self;
}

/// A 3-dimensional vector with numeric components
pub trait NumericVector3<T>: NumericVector<T> {
//     static pure fn unit_x() -> self;
//     static pure fn unit_y() -> self;
//     static pure fn unit_z() -> self;
    pure fn cross(other: &self) -> self;
}

/// A 4-dimensional vector with numeric components
pub trait NumericVector4<T>: NumericVector<T> {
//     static pure fn unit_x() -> self;
//     static pure fn unit_y() -> self;
//     static pure fn unit_z() -> self;
//     static pure fn unit_w() -> self;
}

///
/// A vector with geometric properties
///
pub trait GeometricVector<T>: NumericVector<T> {
    pure fn length2() -> T;
    pure fn length() -> T;
    pure fn distance2(other: &self) -> T;
    pure fn distance(other: &self) -> T;
    pure fn normalize() -> self;
    pure fn lerp(other: &self, amount: T) -> self;
}





//
//  Vec2
//
pub struct Vec2<T> { x: T, y: T }

pub impl<T> Vec2<T>/*: Vector2<T>*/ {
    #[inline(always)]
    static pure fn new(x: T, y: T ) -> Vec2<T> {
        Vec2 { x: move x, y: move y }
    }
}

pub impl<T:Copy> Vec2<T>: Vector<T> {
    #[inline(always)]
    static pure fn from_value(value: T) -> Vec2<T> {
        Vec2::new(value, value)
    }
    
    #[inline(always)]
    static pure fn dim() -> uint { 2 }
    
    #[inline(always)]
    static pure fn size_of() -> uint { size_of::<Vec2<T>>() }
    
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Vec2<T>, *T>(
                to_unsafe_ptr(&self)), 2) |slice| { slice[i] }
        }
    }
    
    #[inline(always)]
    pure fn to_ptr() -> *T {
        ptr::to_unsafe_ptr(&self[0])
    }
}
    
pub impl<T:Copy Num NumCast> Vec2<T>: NumericVector<T> {
    #[inline(always)]
    static pure fn identity() -> Vec2<T> {
        Vec2::new(NumCast::one(),
                  NumCast::one())
    }
    
    #[inline(always)]
    static pure fn zero() -> Vec2<T> {
        Vec2::new(NumCast::zero(),
                  NumCast::zero())
    }
    
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
        self.dot(&self)
    }
    
    #[inline(always)]
    pure fn length() -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn distance2(other: &Vec2<T>) -> T {
        other.sub_v(&self).length2()
    }
    
    #[inline(always)]
    pure fn distance(other: &Vec2<T>) -> T {
        other.distance2(&self).sqrt()
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

pub impl<T:Copy DefaultEq> Vec2<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Vec2<T>) -> bool {
        self.default_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(&self, other: &Vec2<T>) -> bool {
        !(self == other)
    }
}

pub impl<T:Copy FuzzyEq> Vec2<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Vec2<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1])
    }
}

pub impl<T:Copy DefaultEq> Vec2<T>: DefaultEq {
    #[inline(always)]
    pure fn default_eq(other: &Vec2<T>) -> bool {
        self[0].default_eq(&other[0]) &&
        self[1].default_eq(&other[1])
    }
}






//
//  Vec3
//
pub struct Vec3<T> { x: T, y: T, z: T }

pub impl<T> Vec3<T>/*: Vector3<T>*/ {
    #[inline(always)]
    static pure fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x: move x, y: move y, z: move z }
    }
}

pub impl<T:Copy> Vec3<T>: Vector<T> {
    #[inline(always)]
    static pure fn from_value(value: T) -> Vec3<T> {
        Vec3::new(value, value, value)
    }
    
    #[inline(always)]
    static pure fn dim() -> uint { 3 }
    
    #[inline(always)]
    static pure fn size_of() -> uint { size_of::<Vec3<T>>() }
    
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Vec3<T>, *T>(
                to_unsafe_ptr(&self)), 3) |slice| { slice[i] }
        }
    }
    
    #[inline(always)]
    pure fn to_ptr() -> *T {
        to_unsafe_ptr(&self[0])
    }
}

pub impl<T:Copy Num NumCast> Vec3<T>: NumericVector<T> {
    #[inline(always)]
    static pure fn identity() -> Vec3<T> {
        Vec3::new(NumCast::one(),
                  NumCast::one(),
                  NumCast::one())
    }
    
    #[inline(always)]
    static pure fn zero() -> Vec3<T> {
        Vec3::new(NumCast::zero(),
                  NumCast::zero(),
                  NumCast::zero())
    }
    
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

pub impl<T:Copy Num> Vec3<T>: NumericVector3<T> {
    #[inline(always)]
    pure fn cross(other: &Vec3<T>) -> Vec3<T> {
        Vec3::new((self[1] * other[2]) - (self[2] * other[1]),
                  (self[2] * other[0]) - (self[0] * other[2]),
                  (self[0] * other[1]) - (self[1] * other[0]))
    }
}

pub impl<T:Copy Num NumCast Exp> Vec3<T>: GeometricVector<T> {
    #[inline(always)]
    pure fn length2() -> T {
        self.dot(&self)
    }
    
    #[inline(always)]
    pure fn length() -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn distance2(other: &Vec3<T>) -> T {
        other.sub_v(&self).length2()
    }
    
    #[inline(always)]
    pure fn distance(other: &Vec3<T>) -> T {
        other.distance2(&self).sqrt()
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

pub impl<T:Copy DefaultEq> Vec3<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Vec3<T>) -> bool {
        self.default_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(&self, other: &Vec3<T>) -> bool {
        !(self == other)
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

pub impl<T:Copy DefaultEq> Vec3<T>: DefaultEq {
    #[inline(always)]
    pure fn default_eq(other: &Vec3<T>) -> bool {
        self[0].default_eq(&other[0]) &&
        self[1].default_eq(&other[1]) &&
        self[2].default_eq(&other[2])
    }
}






//
//  Vec4
//
pub struct Vec4<T> { x: T, y: T, z: T, w: T }

pub impl<T> Vec4<T>/*: Vector4<T>*/ {
    #[inline(always)]
    static pure fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 { x: move x, y: move y, z: move z, w: move w }
    }
}

pub impl<T:Copy> Vec4<T>: Vector<T> {
    #[inline(always)]
    static pure fn from_value(value: T) -> Vec4<T> {
        Vec4::new(value, value, value, value)
    }
    
    #[inline(always)]
    static pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    static pure fn size_of() -> uint { size_of::<Vec4<T>>() }
    
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Vec4<T>, *T>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
    
    #[inline(always)]
    pure fn to_ptr() -> *T {
        to_unsafe_ptr(&self[0])
    }
}

pub impl<T:Copy Num NumCast> Vec4<T>: NumericVector<T> {
    #[inline(always)]
    static pure fn identity() -> Vec4<T> {
        Vec4::new(NumCast::one(),
                  NumCast::one(),
                  NumCast::one(),
                  NumCast::one())
    }
    
    #[inline(always)]
    static pure fn zero() -> Vec4<T> {
        Vec4::new(NumCast::zero(),
                  NumCast::zero(),
                  NumCast::zero(),
                  NumCast::zero())
    }
    
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
        self.dot(&self)
    }
    
    #[inline(always)]
    pure fn length() -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn distance2(other: &Vec4<T>) -> T {
        other.sub_v(&self).length2()
    }
    
    #[inline(always)]
    pure fn distance(other: &Vec4<T>) -> T {
        other.distance2(&self).sqrt()
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

pub impl<T:Copy DefaultEq> Vec4<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Vec4<T>) -> bool {
        self.default_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(&self, other: &Vec4<T>) -> bool {
        !(self == other)
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

pub impl<T:Copy DefaultEq> Vec4<T>: DefaultEq {
    #[inline(always)]
    pure fn default_eq(other: &Vec4<T>) -> bool {
        self[0].default_eq(&other[0]) &&
        self[1].default_eq(&other[1]) &&
        self[2].default_eq(&other[2]) &&
        self[3].default_eq(&other[3])
    }
}