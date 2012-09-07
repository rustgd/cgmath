import std::cmp::FuzzyEq;
import cmp::Ord;
import num::Num;
import float::{sqrt, abs, fmin, fmax};
import to_str::ToStr;

// TODO: Unittests! I've probably made lots of mistakes...

//
//  N-dimensional Vector
//
trait Vector<T:Num Ord FuzzyEq> {
    pure fn dim() -> uint;
    
    pure fn index(&&index:uint) -> T;
    
    pure fn neg() -> self;
    
    pure fn add_f(&&value:T) -> self;
    pure fn sub_f(&&value:T) -> self;
    pure fn mul_f(&&value:T) -> self;
    pure fn div_f(&&value:T) -> self;
    
    pure fn add_v(&&other: self) -> self;
    pure fn sub_v(&&other: self) -> self;
    
    pure fn dot(&&other: self) -> T;
    
    pure fn exact_eq(&&other:self) -> bool;
    pure fn fuzzy_eq(&&other:self) -> bool;
    pure fn eq(&&other:self) -> bool;
    
    pure fn magnitude2() -> T;
    pure fn magnitude() -> T;
    pure fn normalize() -> self;
    
    pure fn lerp(&&other:self, &&value:T) -> self;
    pure fn abs() -> self;
    // pure fn min(&&other:self) -> self;
    // pure fn max(&&other:self) -> self;
}






trait Vector2<T:Num Ord FuzzyEq> {
    // This is where I wish rust had properties ;)
    pure fn x() -> T;
    pure fn y() -> T;
}

trait Vector3<T:Num Ord FuzzyEq> {
    pure fn x() -> T;
    pure fn y() -> T;
    pure fn z() -> T;
    
    fn cross(&&other:self) -> self;
}

trait Vector4<T:Num Ord FuzzyEq> {
    pure fn x() -> T;
    pure fn y() -> T;
    pure fn z() -> T;
    pure fn w() -> T;
}






//
//  Vec2
//
struct vec2 { data:[float * 2] }

//
//  Constructor
//
#[inline(always)]
pure fn vec2(x:float, y:float) -> vec2 {
    vec2 { data: [ x, y ] }
}

//
//  Constants
//
#[inline(always)] pure fn vec2_zero()     -> vec2 { vec2 (0f, 0f) }
#[inline(always)] pure fn vec2_unit_x()   -> vec2 { vec2 (1f, 0f) }
#[inline(always)] pure fn vec2_unit_y()   -> vec2 { vec2 (0f, 1f) }
#[inline(always)] pure fn vec2_identity() -> vec2 { vec2 (1f, 1f) }

impl vec2: Vector2<float> {
    #[inline(always)] pure fn x() -> float { self.data[0] }
    #[inline(always)] pure fn y() -> float { self.data[1] }
}

impl vec2: Vector<float> {
    #[inline(always)]
    pure fn dim() -> uint { 2 }
    
    #[inline(always)]
    pure fn index(&&i: uint) -> float {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn neg() -> vec2 {
        vec2(-self[0], -self[1])
    }
    
    #[inline(always)]
    pure fn add_f(&&value:float) -> vec2 {
        vec2(self[0] + value,
             self[1] + value)
    }
    
    #[inline(always)]
    pure fn sub_f(&&value:float) -> vec2 {
        vec2(self[0] - value,
             self[1] - value)
    }
    
    #[inline(always)]
    pure fn mul_f(&&value:float) -> vec2 {
        vec2(self[0] * value,
             self[1] * value)
    }
    
    #[inline(always)]
    pure fn div_f(&&value:float) -> vec2 {
        vec2(self[0] / value,
             self[1] / value)
    }
    
    #[inline(always)]
    pure fn add_v(&&other: vec2) -> vec2{
        vec2(self[0] + other[0],
             self[1] + other[1])
    }
    
    #[inline(always)]
    pure fn sub_v(&&other: vec2) -> vec2{
        vec2(self[0] - other[0],
             self[1] - other[1])
    }
    
    #[inline(always)]
    pure fn dot(&&other: vec2) -> float {
        self[0] * other[0] +
        self[1] * other[1]
    }
    
    #[inline(always)]
    pure fn exact_eq(&&other:vec2) -> bool {
        self[0] == other[0] &&
        self[1] == other[1]
    }
    
    #[inline(always)]
    pure fn fuzzy_eq(&&other: vec2) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1])
    }
    
    #[inline(always)]
    pure fn eq(&&other:vec2) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn magnitude2() -> float {
        self[0] * self[0] +
        self[1] * self[1]
    }
    
    #[inline(always)]
    pure fn magnitude() -> float {
        sqrt(self.magnitude2())
    }
    
    #[inline(always)]
    pure fn normalize() -> vec2 {
        let n = 1f / self.magnitude();
        return self.mul_f(n);
    }
    
    #[inline(always)]
    pure fn lerp(&&other:vec2, &&value:float) -> vec2 {
        self.add_v((other.sub_v(self)).mul_f(value))
    }
    
    #[inline(always)]
    pure fn abs() -> vec2 {
        vec2(abs(self[0]),
             abs(self[1]))
    }
    
    // #[inline(always)]
    // pure fn min(&&other:vec2) -> vec2 {
    //     vec2(fmin(self[0], other[0]),
    //          fmin(self[1], other[1]))
    // }
    
    // #[inline(always)]
    // pure fn max(&&other:vec2) -> vec2 {
    //     vec2(fmin(self[0], other[0]),
    //          fmin(self[1], other[1]))
    // }
}

impl vec2: ToStr {
    fn to_str() -> ~str {
        fmt!("vec2[ %f, %f ]", self[0], self[1])
    }
}






//
//  Vec3
//
struct vec3 { data:[float * 3] }

//
//  Constants
//
#[inline(always)] pure fn vec3_zero()     -> vec3 { vec3(0f, 0f, 0f) }
#[inline(always)] pure fn vec3_unit_x()   -> vec3 { vec3(1f, 0f, 0f) }
#[inline(always)] pure fn vec3_unit_y()   -> vec3 { vec3(0f, 1f, 0f) }
#[inline(always)] pure fn vec3_unit_z()   -> vec3 { vec3(0f, 0f, 1f) }
#[inline(always)] pure fn vec3_identity() -> vec3 { vec3(1f, 1f, 1f) }

//
//  Constructor
//
#[inline(always)]
pure fn vec3(x:float, y:float, z:float) -> vec3 {
    vec3 { data: [ x, y, z ] }
}

impl vec3: Vector3<float> {
    #[inline(always)] pure fn x() -> float { self.data[0] }
    #[inline(always)] pure fn y() -> float { self.data[1] }
    #[inline(always)] pure fn z() -> float { self.data[2] }
    
    #[inline(always)]
    fn cross(&&other:vec3) -> vec3 {
        vec3((self[1] * other[2]) - (self[2] * other[1]),
             (self[2] * other[0]) - (self[0] * other[2]),
             (self[0] * other[1]) - (self[1] * other[0]))
    }
}

impl vec3: Vector<float> {
    #[inline(always)]
    pure fn dim() -> uint { 3 }
    
    #[inline(always)]
    pure fn index(&&i: uint) -> float {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn neg() -> vec3 {
        vec3(-self[0], -self[1], -self[2])
    }
    
    #[inline(always)]
    pure fn add_f(&&value:float) -> vec3 {
        vec3(self[0] + value,
             self[1] + value,
             self[2] + value)
    }
    
    #[inline(always)]
    pure fn sub_f(&&value:float) -> vec3 {
        vec3(self[0] - value,
             self[1] - value,
             self[2] - value)
    }
    
    #[inline(always)]
    pure fn mul_f(&&value:float) -> vec3 {
        vec3(self[0] * value,
             self[1] * value,
             self[2] * value)
    }
    
    #[inline(always)]
    pure fn div_f(&&value:float) -> vec3 {
        vec3(self[0] / value,
             self[1] / value,
             self[2] / value)
    }
    
    #[inline(always)]
    pure fn add_v(&&other: vec3) -> vec3{
        vec3(self[0] + other[0],
             self[1] + other[1],
             self[2] + other[2])
    }
    
    #[inline(always)]
    pure fn sub_v(&&other: vec3) -> vec3{
        vec3(self[0] - other[0],
             self[1] - other[1],
             self[2] - other[2])
    }
    
    #[inline(always)]
    pure fn dot(&&other: vec3) -> float {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2]
    }
    
    #[inline(always)]
    pure fn exact_eq(&&other:vec3) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2]
    }
    
    #[inline(always)]
    pure fn fuzzy_eq(&&other: vec3) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2])
    }
    
    #[inline(always)]
    pure fn eq(&&other:vec3) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn magnitude2() -> float {
        self[0] * self[0] +
        self[1] * self[1] +
        self[2] * self[2]
    }
    
    #[inline(always)]
    pure fn magnitude() -> float {
        sqrt(self.magnitude2())
    }
    
    #[inline(always)]
    pure fn normalize() -> vec3 {
        let n = 1f / self.magnitude();
        return self.mul_f(n);
    }
    
    #[inline(always)]
    pure fn lerp(&&other:vec3, &&value:float) -> vec3 {
        self.add_v((other.sub_v(self)).mul_f(value))
    }
    
    #[inline(always)]
    pure fn abs() -> vec3 {
        vec3(abs(self[0]),
             abs(self[1]),
             abs(self[2]))
    }
    
    // #[inline(always)]
    // pure fn min(&&other:vec3) -> vec3 {
    //     vec3(fmin(self[0], other[0]),
    //          fmin(self[1], other[1]),
    //          fmin(self[2], other[2]))
    // }
    
    // #[inline(always)]
    // pure fn max(&&other:vec3) -> vec3 {
    //     vec3(fmin(self[0], other[0]),
    //          fmin(self[1], other[1]),
    //          fmin(self[2], other[2]))
    // }
}

impl vec3: ToStr {
    fn to_str() -> ~str {
        fmt!("vec3[ %f, %f, %f ]", self[0], self[1], self[2])
    }
}






//
//  Vec4
//
struct vec4 { data:[float * 4] }

//
//  Constants
//
#[inline(always)] pure fn vec4_zero()     -> vec4 { vec4(0f, 0f, 0f, 0f) }
#[inline(always)] pure fn vec4_unit_x()   -> vec4 { vec4(1f, 0f, 0f, 0f) }
#[inline(always)] pure fn vec4_unit_y()   -> vec4 { vec4(0f, 1f, 0f, 0f) }
#[inline(always)] pure fn vec4_unit_z()   -> vec4 { vec4(0f, 0f, 1f, 0f) }
#[inline(always)] pure fn vec4_unit_w()   -> vec4 { vec4(0f, 0f, 0f, 1f) }
#[inline(always)] pure fn vec4_identity() -> vec4 { vec4(1f, 1f, 1f, 1f) }

//
//  Constructor
//
#[inline(always)]
pure fn vec4(x:float, y:float, z:float, w:float) -> vec4 {
    vec4 { data: [ x, y, z, w ] }
}

impl vec4: Vector4<float> {
    #[inline(always)] pure fn x() -> float { self.data[0] }
    #[inline(always)] pure fn y() -> float { self.data[1] }
    #[inline(always)] pure fn z() -> float { self.data[2] }
    #[inline(always)] pure fn w() -> float { self.data[3] }
}

impl vec4: Vector<float> {
    #[inline(always)]
    pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    pure fn index(&&i: uint) -> float {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn neg() -> vec4 {
        vec4(-self[0], -self[1], -self[2], -self[3])
    }
    
    #[inline(always)]
    pure fn add_f(&&value:float) -> vec4 {
        vec4(self[0] + value,
             self[1] + value,
             self[2] + value,
             self[3] + value)
    }
    
    #[inline(always)]
    pure fn sub_f(&&value:float) -> vec4 {
        vec4(self[0] - value,
             self[1] - value,
             self[2] - value,
             self[3] - value)
    }
    
    #[inline(always)]
    pure fn mul_f(&&value:float) -> vec4 {
        vec4(self[0] * value,
             self[1] * value,
             self[2] * value,
             self[3] * value)
    }
    
    #[inline(always)]
    pure fn div_f(&&value:float) -> vec4 {
        vec4(self[0] / value,
             self[1] / value,
             self[2] / value,
             self[3] / value)
    }
    
    #[inline(always)]
    pure fn add_v(&&other: vec4) -> vec4{
        vec4(self[0] + other[0],
             self[1] + other[1],
             self[2] + other[2],
             self[3] + other[3])
    }
    
    #[inline(always)]
    pure fn sub_v(&&other: vec4) -> vec4{
        vec4(self[0] - other[0],
             self[1] - other[1],
             self[2] - other[2],
             self[3] - other[3])
    }
    
    #[inline(always)]
    pure fn dot(&&other:vec4) -> float {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2] +
        self[3] * other[3]
    }
    
    #[inline(always)]
    pure fn exact_eq(&&other:vec4) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2] &&
        self[3] == other[3]
    }
    
    #[inline(always)]
    pure fn fuzzy_eq(&&other: vec4) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
    
    #[inline(always)]
    pure fn eq(&&other:vec4) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn magnitude2() -> float {
        self[0] * self[0] +
        self[1] * self[1] +
        self[2] * self[2] +
        self[3] * self[3]
    }
    
    #[inline(always)]
    pure fn magnitude() -> float {
        sqrt(self.magnitude2())
    }
    
    #[inline(always)]
    pure fn normalize() -> vec4 {
        let n = 1f / self.magnitude();
        return self.mul_f(n);
    }
    
    #[inline(always)]
    pure fn lerp(&&other:vec4, &&value:float) -> vec4 {
        self.add_v((other.sub_v(self)).mul_f(value))
    }
    
    #[inline(always)]
    pure fn abs() -> vec4 {
        vec4(abs(self[0]),
             abs(self[1]),
             abs(self[2]),
             abs(self[3]))
    }
    
    // #[inline(always)]
    // pure fn min(&&other:vec4) -> vec4 {
    //     vec4(fmin(self[0], other[0]),
    //          fmin(self[1], other[1]),
    //          fmin(self[2], other[2]),
    //          fmin(self[3], other[3]))
    // }
    
    // #[inline(always)]
    // pure fn max(&&other:vec4) -> vec4 {
    //     vec4(fmin(self[0], other[0]),
    //          fmin(self[1], other[1]),
    //          fmin(self[2], other[2]),
    //          fmin(self[3], other[3]))
    // }
}

impl vec4: ToStr {
    fn to_str() -> ~str {
        fmt!("vec4[ %f, %f, %f, %f ]", self[0], self[1], self[2], self[3])
    }
}