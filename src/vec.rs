use std::cmp::FuzzyEq;
use cmp::Eq;
use ops::Neg;
use math::{Abs, min, max, Sqrt};
use to_str::ToStr;

//
//  N-dimensional Vector
//
pub trait Vector<T> {
    static pure fn dim() -> uint;
    
    pure fn index(&&index:uint) -> T;
    
    pure fn add_f(&&value:T) -> self;
    pure fn sub_f(&&value:T) -> self;
    pure fn mul_f(&&value:T) -> self;
    pure fn div_f(&&value:T) -> self;
    
    pure fn add_v(&&other: self) -> self;
    pure fn sub_v(&&other: self) -> self;
    
    pure fn dot(&&other: self) -> T;
    
    pure fn exact_eq(&&other:self) -> bool;
    
    pure fn magnitude2() -> T;
    pure fn magnitude() -> T;
    pure fn normalize() -> self;
    
    pure fn lerp(&&other:self, &&value:T) -> self;
    pure fn abs() -> self;
    pure fn min(&&other:self) -> self;
    pure fn max(&&other:self) -> self;
    
    static pure fn zero() -> self;
    static pure fn identity() -> self;
}







pub trait Vector2<T> {
    // static pure fn _new(x:float, y:float) -> self;
    
    // This is where I wish rust had properties ;)
    pure fn x() -> T;
    pure fn y() -> T;
    
    // static pure fn unit_x() -> self;
    // static pure fn unit_y() -> self;
}

pub trait Vector3<T> {
    // error: duplicate function definition
    // static pure fn _new(x:float, y:float, z:float) -> self;
    
    pure fn x() -> T;
    pure fn y() -> T;
    pure fn z() -> T;
    
    // static pure fn unit_x() -> self;
    // static pure fn unit_y() -> self;
    // static pure fn unit_z() -> self;
    
    fn cross(&&other:self) -> self;
}

pub trait Vector4<T> {
    // error: duplicate function definition
    // static pure fn _new(x:float, y:float, z:float, w:float) -> self;
    
    pure fn x() -> T;
    pure fn y() -> T;
    pure fn z() -> T;
    pure fn w() -> T;
    
    // static pure fn unit_x() -> self;
    // static pure fn unit_y() -> self;
    // static pure fn unit_z() -> self;
    // static pure fn unit_w() -> self;
}






//
//  Vec2
//
pub struct Vec2 { data:[float * 2] }

pub const vec2_zero     :Vec2 = Vec2 { data: [ 0f, 0f ] };
pub const vec2_unit_x   :Vec2 = Vec2 { data: [ 1f, 0f ] };
pub const vec2_unit_y   :Vec2 = Vec2 { data: [ 0f, 1f ] };
pub const vec2_identity :Vec2 = Vec2 { data: [ 1f, 1f ] };

//
//  Constructor
//
#[inline]
pub pure fn Vec2(x:float, y:float) -> Vec2 {
    Vec2 { data: [ x, y ] }
}

pub impl Vec2: Vector2<float> {
    // #[inline]
    // static pure fn _new(x:float, y:float) -> Vec2 {
    //     Vec2 { data: [ x, y ] }
    // }
    
    #[inline] pure fn x() -> float { self.data[0] }
    #[inline] pure fn y() -> float { self.data[1] }
    
    // #[inline] static pure fn unit_x() -> Vec2 { Vec2(1f, 0f) }
    // #[inline] static pure fn unit_y() -> Vec2 { Vec2(0f, 1f) }
    // #[inline] static pure fn unit_z() -> Vec2 { Vec2(0f, 0f) }
}

pub impl Vec2: Vector<float> {
    #[inline]
    static pure fn dim() -> uint { 2 }
    
    #[inline]
    pure fn index(&&i: uint) -> float {
        self.data[i]
    }
    
    #[inline]
    pure fn add_f(&&value:float) -> Vec2 {
        Vec2(self[0] + value,
             self[1] + value)
    }
    
    #[inline]
    pure fn sub_f(&&value:float) -> Vec2 {
        Vec2(self[0] - value,
             self[1] - value)
    }
    
    #[inline]
    pure fn mul_f(&&value:float) -> Vec2 {
        Vec2(self[0] * value,
             self[1] * value)
    }
    
    #[inline]
    pure fn div_f(&&value:float) -> Vec2 {
        Vec2(self[0] / value,
             self[1] / value)
    }
    
    #[inline]
    pure fn add_v(&&other: Vec2) -> Vec2{
        Vec2(self[0] + other[0],
             self[1] + other[1])
    }
    
    #[inline]
    pure fn sub_v(&&other: Vec2) -> Vec2{
        Vec2(self[0] - other[0],
             self[1] - other[1])
    }
    
    #[inline]
    pure fn dot(&&other: Vec2) -> float {
        self[0] * other[0] +
        self[1] * other[1]
    }
    
    #[inline]
    pure fn exact_eq(&&other:Vec2) -> bool {
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
    pure fn lerp(&&other:Vec2, &&value:float) -> Vec2 {
        self.add_v((other.sub_v(self)).mul_f(value))
    }
    
    #[inline]
    pure fn abs() -> Vec2 {
        Vec2(self[0].abs(),
             self[1].abs())
    }
    
    #[inline]
    pure fn min(&&other:Vec2) -> Vec2 {
        Vec2(min(self[0], other[0]),
             min(self[1], other[1]))
    }
    
    #[inline]
    pure fn max(&&other:Vec2) -> Vec2 {
        Vec2(max(self[0], other[0]),
             max(self[1], other[1]))
    }
    
    #[inline] static pure fn zero()     -> Vec2 { Vec2(1f, 1f) }
    #[inline] static pure fn identity() -> Vec2 { Vec2(1f, 1f) }
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
    fn to_str() -> ~str {
        fmt!("Vec2[ %f, %f ]", self[0], self[1])
    }
}






//
//  Vec3
//
pub struct Vec3 { data:[float * 3] }

pub const vec3_zero     :Vec3 = Vec3 { data: [ 0f, 0f, 0f ] };
pub const vec3_unit_x   :Vec3 = Vec3 { data: [ 1f, 0f, 0f ] };
pub const vec3_unit_y   :Vec3 = Vec3 { data: [ 0f, 1f, 0f ] };
pub const vec3_unit_z   :Vec3 = Vec3 { data: [ 0f, 0f, 1f ] };
pub const vec3_identity :Vec3 = Vec3 { data: [ 1f, 1f, 1f ] };

//
//  Constructor
//
#[inline]
pub pure fn Vec3(x:float, y:float, z:float) -> Vec3 {
    Vec3 { data: [ x, y, z ] }
}

pub impl Vec3: Vector3<float> {
    // #[inline]
    // static pure fn _new(x:float, y:float, z:float) -> Vec3 {
    //     Vec2 { data: [ x, y, z ] }
    // }
    
    #[inline] pure fn x() -> float { self.data[0] }
    #[inline] pure fn y() -> float { self.data[1] }
    #[inline] pure fn z() -> float { self.data[2] }
    
    #[inline]
    fn cross(&&other:Vec3) -> Vec3 {
        Vec3((self[1] * other[2]) - (self[2] * other[1]),
             (self[2] * other[0]) - (self[0] * other[2]),
             (self[0] * other[1]) - (self[1] * other[0]))
    }
    
    // #[inline] static pure fn unit_x() -> Vec3 { Vec3(1f, 0f, 0f) }
    // #[inline] static pure fn unit_y() -> Vec3 { Vec3(0f, 1f, 0f) }
    // #[inline] static pure fn unit_z() -> Vec3 { Vec3(0f, 0f, 1f) }
}

pub impl Vec3: Vector<float> {
    #[inline]
    static pure fn dim() -> uint { 3 }
    
    #[inline]
    pure fn index(&&i: uint) -> float {
        self.data[i]
    }
    
    #[inline]
    pure fn add_f(&&value:float) -> Vec3 {
        Vec3(self[0] + value,
             self[1] + value,
             self[2] + value)
    }
    
    #[inline]
    pure fn sub_f(&&value:float) -> Vec3 {
        Vec3(self[0] - value,
             self[1] - value,
             self[2] - value)
    }
    
    #[inline]
    pure fn mul_f(&&value:float) -> Vec3 {
        Vec3(self[0] * value,
             self[1] * value,
             self[2] * value)
    }
    
    #[inline]
    pure fn div_f(&&value:float) -> Vec3 {
        Vec3(self[0] / value,
             self[1] / value,
             self[2] / value)
    }
    
    #[inline]
    pure fn add_v(&&other: Vec3) -> Vec3{
        Vec3(self[0] + other[0],
             self[1] + other[1],
             self[2] + other[2])
    }
    
    #[inline]
    pure fn sub_v(&&other: Vec3) -> Vec3{
        Vec3(self[0] - other[0],
             self[1] - other[1],
             self[2] - other[2])
    }
    
    #[inline]
    pure fn dot(&&other: Vec3) -> float {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2]
    }
    
    #[inline]
    pure fn exact_eq(&&other:Vec3) -> bool {
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
    pure fn lerp(&&other:Vec3, &&value:float) -> Vec3 {
        self.add_v((other.sub_v(self)).mul_f(value))
    }
    
    #[inline]
    pure fn abs() -> Vec3 {
        Vec3(self[0].abs(),
             self[1].abs(),
             self[2].abs())
    }
    
    #[inline]
    pure fn min(&&other:Vec3) -> Vec3 {
        Vec3(min(self[0], other[0]),
             min(self[1], other[1]),
             min(self[2], other[2]))
    }
    
    #[inline]
    pure fn max(&&other:Vec3) -> Vec3 {
        Vec3(max(self[0], other[0]),
             max(self[1], other[1]),
             max(self[2], other[2]))
    }
    
    #[inline] static pure fn zero()     -> Vec3 { Vec3(1f, 1f, 1f) }
    #[inline] static pure fn identity() -> Vec3 { Vec3(1f, 1f, 1f) }
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
    fn to_str() -> ~str {
        fmt!("Vec3[ %f, %f, %f ]", self[0], self[1], self[2])
    }
}






//
//  Vec4
//
pub struct Vec4 { data:[float * 4] }

pub const vec4_zero     :Vec4 = Vec4 { data: [ 0f, 0f, 0f, 0f ] };
pub const vec4_unit_x   :Vec4 = Vec4 { data: [ 1f, 0f, 0f, 0f ] };
pub const vec4_unit_y   :Vec4 = Vec4 { data: [ 0f, 1f, 0f, 0f ] };
pub const vec4_unit_z   :Vec4 = Vec4 { data: [ 0f, 0f, 1f, 0f ] };
pub const vec4_unit_w   :Vec4 = Vec4 { data: [ 0f, 0f, 0f, 1f ] };
pub const vec4_identity :Vec4 = Vec4 { data: [ 1f, 1f, 1f, 1f ] };

//
//  Constructor
//
#[inline]
pub pure fn Vec4(x:float, y:float, z:float, w:float) -> Vec4 {
    Vec4 { data: [ x, y, z, w ] }
}

pub impl Vec4: Vector4<float> {
    // #[inline]
    // static pure fn _new(x:float, y:float, z:float, w:float) -> Vec3 {
    //     Vec2 { data: [ x, y, z, w ] }
    // }
    
    #[inline] pure fn x() -> float { self.data[0] }
    #[inline] pure fn y() -> float { self.data[1] }
    #[inline] pure fn z() -> float { self.data[2] }
    #[inline] pure fn w() -> float { self.data[3] }
    
    // #[inline] static pure fn unit_x() -> Vec4 { Vec4(1f, 0f, 0f, 0f) }
    // #[inline] static pure fn unit_y() -> Vec4 { Vec4(0f, 1f, 0f, 0f) }
    // #[inline] static pure fn unit_z() -> Vec4 { Vec4(0f, 0f, 1f, 0f) }
    // #[inline] static pure fn unit_w() -> Vec4 { Vec4(0f, 0f, 0f, 1f) }
}

pub impl Vec4: Vector<float> {
    #[inline]
    static pure fn dim() -> uint { 4 }
    
    #[inline]
    pure fn index(&&i: uint) -> float {
        self.data[i]
    }
    
    #[inline]
    pure fn add_f(&&value:float) -> Vec4 {
        Vec4(self[0] + value,
             self[1] + value,
             self[2] + value,
             self[3] + value)
    }
    
    #[inline]
    pure fn sub_f(&&value:float) -> Vec4 {
        Vec4(self[0] - value,
             self[1] - value,
             self[2] - value,
             self[3] - value)
    }
    
    #[inline]
    pure fn mul_f(&&value:float) -> Vec4 {
        Vec4(self[0] * value,
             self[1] * value,
             self[2] * value,
             self[3] * value)
    }
    
    #[inline]
    pure fn div_f(&&value:float) -> Vec4 {
        Vec4(self[0] / value,
             self[1] / value,
             self[2] / value,
             self[3] / value)
    }
    
    #[inline]
    pure fn add_v(&&other: Vec4) -> Vec4{
        Vec4(self[0] + other[0],
             self[1] + other[1],
             self[2] + other[2],
             self[3] + other[3])
    }
    
    #[inline]
    pure fn sub_v(&&other: Vec4) -> Vec4{
        Vec4(self[0] - other[0],
             self[1] - other[1],
             self[2] - other[2],
             self[3] - other[3])
    }
    
    #[inline]
    pure fn dot(&&other:Vec4) -> float {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2] +
        self[3] * other[3]
    }
    
    #[inline]
    pure fn exact_eq(&&other:Vec4) -> bool {
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
    pure fn lerp(&&other:Vec4, &&value:float) -> Vec4 {
        self.add_v((other.sub_v(self)).mul_f(value))
    }
    
    #[inline]
    pure fn abs() -> Vec4 {
        Vec4(self[0].abs(),
             self[1].abs(),
             self[2].abs(),
             self[3].abs())
    }
    
    #[inline]
    pure fn min(&&other:Vec4) -> Vec4 {
        Vec4(min(self[0], other[0]),
             min(self[1], other[1]),
             min(self[2], other[2]),
             min(self[3], other[3]))
    }
    
    #[inline]
    pure fn max(&&other:Vec4) -> Vec4 {
        Vec4(max(self[0], other[0]),
             max(self[1], other[1]),
             max(self[2], other[2]),
             max(self[3], other[3]))
    }
    
    #[inline] static pure fn zero()     -> Vec4 { Vec4(1f, 1f, 1f, 1f) }
    #[inline] static pure fn identity() -> Vec4 { Vec4(1f, 1f, 1f, 1f) }
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
    fn to_str() -> ~str {
        fmt!("Vec4[ %f, %f, %f, %f ]", self[0], self[1], self[2], self[3])
    }
}