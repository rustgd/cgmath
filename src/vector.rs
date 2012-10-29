use cast::transmute;
use vec::raw::buf_as_slice;
use ptr::to_unsafe_ptr;
use cmp::Eq;
use std::cmp::FuzzyEq;
use math::*;

//
//  N-dimensional Vector
//
pub trait Vector<T> {
    static pure fn dim() -> uint;
    
    pure fn add_f(value: T) -> self;
    pure fn sub_f(value: T) -> self;
    pure fn mul_f(value: T) -> self;
    pure fn div_f(value: T) -> self;
    
    pure fn add_v(other: &self) -> self;
    pure fn sub_v(other: &self) -> self;
    
    pure fn dot(other: &self) -> T;
    
    pure fn exact_eq(other:&self) -> bool;
    
    pure fn magnitude2() -> T;
    pure fn magnitude() -> T;
    pure fn normalize() -> self;
    
    pure fn lerp(other: &self, value: T) -> self;
    pure fn min(other: &self) -> self;
    pure fn max(other: &self) -> self;
    
    static pure fn zero() -> self;
    static pure fn identity() -> self;
}

pub trait Vector3<T> {
    fn cross(other: &self) -> self;
}






//
//  Vec2
//
pub struct Vec2 { x: float, y: float }


//
//  Constructor
//
#[inline]
pub pure fn Vec2(x: float, y: float) -> Vec2 {
    Vec2 { x: x, y: y }
}

pub mod Vec2 {
    pub const zero     :Vec2 = Vec2 { x: 0f, y: 0f };
    pub const unit_x   :Vec2 = Vec2 { x: 1f, y: 0f };
    pub const unit_y   :Vec2 = Vec2 { x: 0f, y: 1f };
    pub const identity :Vec2 = Vec2 { x: 1f, y: 1f };
}

pub impl Vec2: Vector<float> {
    #[inline]
    static pure fn dim() -> uint { 2 }
    
    #[inline]
    pure fn add_f(value: float) -> Vec2 {
        Vec2(self[0] + value,
             self[1] + value)
    }
    
    #[inline]
    pure fn sub_f(value: float) -> Vec2 {
        Vec2(self[0] - value,
             self[1] - value)
    }
    
    #[inline]
    pure fn mul_f(value: float) -> Vec2 {
        Vec2(self[0] * value,
             self[1] * value)
    }
    
    #[inline]
    pure fn div_f(value: float) -> Vec2 {
        Vec2(self[0] / value,
             self[1] / value)
    }
    
    #[inline]
    pure fn add_v(other: &Vec2) -> Vec2{
        Vec2(self[0] + other[0],
             self[1] + other[1])
    }
    
    #[inline]
    pure fn sub_v(other: &Vec2) -> Vec2{
        Vec2(self[0] - other[0],
             self[1] - other[1])
    }
    
    #[inline]
    pure fn dot(other: &Vec2) -> float {
        self[0] * other[0] +
        self[1] * other[1]
    }
    
    #[inline]
    pure fn exact_eq(other: &Vec2) -> bool {
        self[0] == other[0] &&
        self[1] == other[1]
    }
    
    #[inline]
    pure fn magnitude2() -> float {
        self[0] * self[0] +
        self[1] * self[1]
    }
    
    #[inline]
    pure fn magnitude() -> float {
        self.magnitude2().sqrt()
    }
    
    #[inline]
    pure fn normalize() -> Vec2 {
        let n = 1f / self.magnitude();
        return self.mul_f(n);
    }
    
    #[inline]
    pure fn lerp(other: &Vec2, value: float) -> Vec2 {
        self.add_v(&other.sub_v(&self).mul_f(value))
    }
    
    #[inline]
    pure fn min(other: &Vec2) -> Vec2 {
        Vec2(min(&self[0], &other[0]),
             min(&self[1], &other[1]))
    }
    
    #[inline]
    pure fn max(other: &Vec2) -> Vec2 {
        Vec2(max(&self[0], &other[0]),
             max(&self[1], &other[1]))
    }
    
    #[inline] static pure fn zero()     -> Vec2 { Vec2(0f, 0f) }
    #[inline] static pure fn identity() -> Vec2 { Vec2(1f, 1f) }
}

pub impl Vec2: Index<uint, float> {
    #[inline]
    pure fn index(i: uint) -> float {
        unsafe {
            do buf_as_slice(
                transmute::<*Vec2, *float>(
                    to_unsafe_ptr(&self)), 2) |slice| { slice[i] }
        }
    }
}

pub impl Vec2: Abs {
    #[inline]
    pure fn abs() -> Vec2 {
        Vec2(abs(&self[0]),
             abs(&self[1]))
    }
}

pub impl Vec2: Neg<Vec2> {
    #[inline]
    pure fn neg() -> Vec2 {
        Vec2(-self[0], -self[1])
    }
}

pub impl Vec2: Eq {
    #[inline]
    pure fn eq(other: &Vec2) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline]
    pure fn ne(other: &Vec2) -> bool {
        !(self == *other)
    }
}

pub impl Vec2: FuzzyEq {
    #[inline]
    pure fn fuzzy_eq(other: &Vec2) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1])
    }
}

pub impl Vec2: ToStr {
    pure fn to_str() -> ~str {
        fmt!("Vec2[ %f, %f ]", self[0], self[1])
    }
}






//
//  Vec3
//
pub struct Vec3 { x: float, y: float, z: float }

//
//  Constructor
//
#[inline]
pub pure fn Vec3(x: float, y: float, z: float) -> Vec3 {
    Vec3 { x: x, y: y, z: z }
}

pub mod Vec3 {
    pub const zero     :Vec3 = Vec3 { x: 0f, y: 0f, z: 0f };
    pub const unit_x   :Vec3 = Vec3 { x: 1f, y: 0f, z: 0f };
    pub const unit_y   :Vec3 = Vec3 { x: 0f, y: 1f, z: 0f };
    pub const unit_z   :Vec3 = Vec3 { x: 0f, y: 0f, z: 1f };
    pub const identity :Vec3 = Vec3 { x: 1f, y: 1f, z: 1f };
}

pub impl Vec3: Vector3<float> {
    #[inline]
    fn cross(other: &Vec3) -> Vec3 {
        Vec3((self[1] * other[2]) - (self[2] * other[1]),
             (self[2] * other[0]) - (self[0] * other[2]),
             (self[0] * other[1]) - (self[1] * other[0]))
    }
}

pub impl Vec3: Vector<float> {
    #[inline]
    static pure fn dim() -> uint { 3 }
    
    #[inline]
    pure fn add_f(value: float) -> Vec3 {
        Vec3(self[0] + value,
             self[1] + value,
             self[2] + value)
    }
    
    #[inline]
    pure fn sub_f(value: float) -> Vec3 {
        Vec3(self[0] - value,
             self[1] - value,
             self[2] - value)
    }
    
    #[inline]
    pure fn mul_f(value: float) -> Vec3 {
        Vec3(self[0] * value,
             self[1] * value,
             self[2] * value)
    }
    
    #[inline]
    pure fn div_f(value: float) -> Vec3 {
        Vec3(self[0] / value,
             self[1] / value,
             self[2] / value)
    }
    
    #[inline]
    pure fn add_v(other: &Vec3) -> Vec3{
        Vec3(self[0] + other[0],
             self[1] + other[1],
             self[2] + other[2])
    }
    
    #[inline]
    pure fn sub_v(other: &Vec3) -> Vec3{
        Vec3(self[0] - other[0],
             self[1] - other[1],
             self[2] - other[2])
    }
    
    #[inline]
    pure fn dot(other: &Vec3) -> float {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2]
    }
    
    #[inline]
    pure fn exact_eq(other: &Vec3) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2]
    }
    
    #[inline]
    pure fn magnitude2() -> float {
        self[0] * self[0] +
        self[1] * self[1] +
        self[2] * self[2]
    }
    
    #[inline]
    pure fn magnitude() -> float {
        self.magnitude2().sqrt()
    }
    
    #[inline]
    pure fn normalize() -> Vec3 {
        let n = 1f / self.magnitude();
        return self.mul_f(n);
    }
    
    #[inline]
    pure fn lerp(other: &Vec3, value: float) -> Vec3 {
        self.add_v(&other.sub_v(&self).mul_f(value))
    }
    
    #[inline]
    pure fn min(other: &Vec3) -> Vec3 {
        Vec3(min(&self[0], &other[0]),
             min(&self[1], &other[1]),
             min(&self[2], &other[2]))
    }
    
    #[inline]
    pure fn max(other: &Vec3) -> Vec3 {
        Vec3(max(&self[0], &other[0]),
             max(&self[1], &other[1]),
             max(&self[2], &other[2]))
    }
    
    #[inline] static pure fn zero()     -> Vec3 { Vec3(0f, 0f, 0f) }
    #[inline] static pure fn identity() -> Vec3 { Vec3(1f, 1f, 1f) }
}

pub impl Vec3: Index<uint, float> {
    #[inline]
    pure fn index(i: uint) -> float {
        unsafe { do buf_as_slice(
            transmute::<*Vec3, *float>(
                to_unsafe_ptr(&self)), 3) |slice| { slice[i] }
        }
    }
}

pub impl Vec3: Abs {
    #[inline]
    pure fn abs() -> Vec3 {
        Vec3(abs(&self[0]),
             abs(&self[1]),
             abs(&self[2]))
    }
}

pub impl Vec3: Neg<Vec3> {
    #[inline]
    pure fn neg() -> Vec3 {
        Vec3(-self[0], -self[1], -self[2])
    }
}

pub impl Vec3: Eq {
    #[inline]
    pure fn eq(other: &Vec3) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline]
    pure fn ne(other: &Vec3) -> bool {
        !(self == *other)
    }
}

pub impl Vec3: FuzzyEq {
    #[inline]
    pure fn fuzzy_eq(other: &Vec3) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2])
    }
}

pub impl Vec3: ToStr {
    pure fn to_str() -> ~str {
        fmt!("Vec3[ %f, %f, %f ]", self[0], self[1], self[2])
    }
}






//
//  Vec4
//
pub struct Vec4 { x: float, y: float, z: float, w: float }

pub mod Vec4 {
    pub const zero     :Vec4 = Vec4 { x: 0f, y: 0f, z: 0f, w: 0f };
    pub const unit_x   :Vec4 = Vec4 { x: 1f, y: 0f, z: 0f, w: 0f };
    pub const unit_y   :Vec4 = Vec4 { x: 0f, y: 1f, z: 0f, w: 0f };
    pub const unit_z   :Vec4 = Vec4 { x: 0f, y: 0f, z: 1f, w: 0f };
    pub const unit_w   :Vec4 = Vec4 { x: 0f, y: 0f, z: 0f, w: 1f };
    pub const identity :Vec4 = Vec4 { x: 1f, y: 1f, z: 1f, w: 1f };
}

//
//  Constructor
//
#[inline]
pub pure fn Vec4(x: float, y: float, z: float, w: float) -> Vec4 {
    Vec4 { x: x, y: y, z: z, w: w }
}

pub impl Vec4: Vector<float> {
    #[inline]
    static pure fn dim() -> uint { 4 }
    
    #[inline]
    pure fn add_f(value: float) -> Vec4 {
        Vec4(self[0] + value,
             self[1] + value,
             self[2] + value,
             self[3] + value)
    }
    
    #[inline]
    pure fn sub_f(value: float) -> Vec4 {
        Vec4(self[0] - value,
             self[1] - value,
             self[2] - value,
             self[3] - value)
    }
    
    #[inline]
    pure fn mul_f(value: float) -> Vec4 {
        Vec4(self[0] * value,
             self[1] * value,
             self[2] * value,
             self[3] * value)
    }
    
    #[inline]
    pure fn div_f(value: float) -> Vec4 {
        Vec4(self[0] / value,
             self[1] / value,
             self[2] / value,
             self[3] / value)
    }
    
    #[inline]
    pure fn add_v(other: &Vec4) -> Vec4{
        Vec4(self[0] + other[0],
             self[1] + other[1],
             self[2] + other[2],
             self[3] + other[3])
    }
    
    #[inline]
    pure fn sub_v(other: &Vec4) -> Vec4{
        Vec4(self[0] - other[0],
             self[1] - other[1],
             self[2] - other[2],
             self[3] - other[3])
    }
    
    #[inline]
    pure fn dot(other: &Vec4) -> float {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2] +
        self[3] * other[3]
    }
    
    #[inline]
    pure fn exact_eq(other: &Vec4) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2] &&
        self[3] == other[3]
    }
    
    #[inline]
    pure fn magnitude2() -> float {
        self[0] * self[0] +
        self[1] * self[1] +
        self[2] * self[2] +
        self[3] * self[3]
    }
    
    #[inline]
    pure fn magnitude() -> float {
        self.magnitude2().sqrt()
    }
    
    #[inline]
    pure fn normalize() -> Vec4 {
        let n = 1f / self.magnitude();
        return self.mul_f(n);
    }
    
    #[inline]
    pure fn lerp(other: &Vec4, value: float) -> Vec4 {
        self.add_v(&other.sub_v(&self).mul_f(value))
    }
    
    #[inline]
    pure fn min(other: &Vec4) -> Vec4 {
        Vec4(min(&self[0], &other[0]),
             min(&self[1], &other[1]),
             min(&self[2], &other[2]),
             min(&self[3], &other[3]))
    }
    
    #[inline]
    pure fn max(other: &Vec4) -> Vec4 {
        Vec4(max(&self[0], &other[0]),
             max(&self[1], &other[1]),
             max(&self[2], &other[2]),
             max(&self[3], &other[3]))
    }
    
    #[inline] static pure fn zero()     -> Vec4 { Vec4(0f, 0f, 0f, 0f) }
    #[inline] static pure fn identity() -> Vec4 { Vec4(1f, 1f, 1f, 1f) }
}

pub impl Vec4: Index<uint, float> {
    #[inline]
    pure fn index(i: uint) -> float {
        unsafe {
            do buf_as_slice(
                transmute::<*Vec4, *float>(
                    to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl Vec4: Abs {
    #[inline]
    pure fn abs() -> Vec4 {
        Vec4(abs(&self[0]),
             abs(&self[1]),
             abs(&self[2]),
             abs(&self[3]))
    }
}

pub impl Vec4: Neg<Vec4> {
    #[inline]
    pure fn neg() -> Vec4 {
        Vec4(-self[0], -self[1], -self[2], -self[3])
    }
}

pub impl Vec4: Eq {
    #[inline]
    pure fn eq(other: &Vec4) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline]
    pure fn ne(other: &Vec4) -> bool {
        !(self == *other)
    }
}

pub impl Vec4: FuzzyEq {
    #[inline]
    pure fn fuzzy_eq(other: &Vec4) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
}

pub impl Vec4: ToStr {
    pure fn to_str() -> ~str {
        fmt!("Vec4[ %f, %f, %f, %f ]", self[0], self[1], self[2], self[3])
    }
}