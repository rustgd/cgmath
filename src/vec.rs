use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;

use dim::{Dimensional, ToPtr};
use funs::exponential::Exp;
use num::default_eq::DefaultEq;
use num::kinds::Number;

/**
 * The base generic vector trait
 */
pub trait Vector<T>: Dimensional<T>, ToPtr<T>, Eq, DefaultEq {
    /// Construct the vector from a single value, copying it to each component
    static pure fn from_value(value: T) -> self;
}

pub trait MutableVector<T>: Vector<T> {
    /**
     * Get a mutable reference to the component at `i`
     */
    fn index_mut(&mut self, i: uint) -> &self/mut T;
    
    /**
     * Swap two components of the vector in place
     */
    fn swap(&mut self, a: uint, b: uint);
}

/**
 * A generic 2-dimensional vector
 */
pub trait Vector2<T>: Vector<T> {
    // static pure fn new(x: T, y: T) -> self;
}

/**
 * A generic 3-dimensional vector
 */
pub trait Vector3<T>: Vector<T> {
    // static pure fn new(x: T, y: T, z: T) -> self;
}

/**
 * A generic 4-dimensional vector
 */
pub trait Vector4<T>: Vector<T> {
//     static pure fn new(x: T, y: T, z: T, w: T) -> self;
}

/**
 * A vector with numeric components
 */
pub trait NumericVector<T>: Vector<T>, Neg<self> {
    /**
     * Returns a vector with each component set to one
     */
    static pure fn identity() -> self;
    
    /**
     * Returns a vector with each component set to zero
     */
    static pure fn zero() -> self;
    
    /**
     * Returns the scalar multiplication of the vector and `value`
     */
    pure fn mul_t(&self, value: T) -> self;
    
    /**
     * Returns the scalar quotient of the vector and `value`
     */
    pure fn div_t(&self, value: T) -> self;
    
    /**
     * Returns the sum of the vector with `other`
     */
    pure fn add_v(&self, other: &self) -> self;
    
    /**
     * Add the vector `other`
     */
    // fn add_v_self(&mut self, other: &self);
    
    /**
     * Returns the difference between the vector and `other`
     */
    pure fn sub_v(&self, other: &self) -> self;
    
    /**
     * Returns the dot product of the vector and `other`
     */
    pure fn dot(&self, other: &self) -> T;
}

/**
 * A 2-dimensional vector with numeric components
 */
pub trait NumericVector2<T>: NumericVector<T> {
    // static pure fn unit_x() -> self;
    // static pure fn unit_y() -> self;
}

/**
 * A 3-dimensional vector with numeric components
 */
pub trait NumericVector3<T>: NumericVector<T> {
    // static pure fn unit_x() -> self;
    // static pure fn unit_y() -> self;
    // static pure fn unit_z() -> self;
    /**
     * Returns the cross product of the vector and `other`
     */
    pure fn cross(&self, other: &self) -> self;
}

/**
 * A 4-dimensional vector with numeric components
 */
pub trait NumericVector4<T>: NumericVector<T> {
    // static pure fn unit_x() -> self;
    // static pure fn unit_y() -> self;
    // static pure fn unit_z() -> self;
    // static pure fn unit_w() -> self;
}

/**
 * A vector with geometric properties
 */
pub trait GeometricVector<T>: NumericVector<T> {
    /**
     * Returns the squared length of the vector
     */
    pure fn length2(&self) -> T;
    
    /**
     * Returns the length of the vector
     */
    pure fn length(&self) -> T;
    
    /**
     * Returns the squared distance between the vector and `other`.
     */
    pure fn distance2(&self, other: &self) -> T;
    
    /**
     * Returns the distance between the vector and `other`
     */
    pure fn distance(&self, other: &self) -> T;
    
    /**
     * Returns the normalized vector
     */
    pure fn normalize(&self) -> self;
    
    /**
     * Linearly intoperlate between the vector and `other`
     */
    pure fn lerp(&self, other: &self, amount: T) -> self;
}





/**
 *  Vec2
 */
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
}

pub impl<T> Vec2<T>: Dimensional<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 2 }
    
    #[inline(always)]
    static pure fn size_of() -> uint { size_of::<Vec2<T>>() }
}

pub impl<T:Copy> Vec2<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 2) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> Vec2<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Vec2<T>, *T>(
                to_unsafe_ptr(&*self)
            )
        }
    }
}

pub impl<T:Copy> Vec2<T>: MutableVector<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &self/mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 1, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        util::swap(self.index_mut(a),
                   self.index_mut(b));
    }
}
    
pub impl<T:Copy Number> Vec2<T>: NumericVector<T> {
    #[inline(always)]
    static pure fn identity() -> Vec2<T> {
        Vec2::new(Number::one(),
                  Number::one())
    }
    
    #[inline(always)]
    static pure fn zero() -> Vec2<T> {
        Vec2::new(Number::zero(),
                  Number::zero())
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Vec2<T> {
        Vec2::new(self[0] * value,
                  self[1] * value)
    }
    
    #[inline(always)]
    pure fn div_t(&self, value: T) -> Vec2<T> {
        Vec2::new(self[0] / value,
                  self[1] / value)
    }
    
    #[inline(always)]
    pure fn add_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0] + other[0],
                  self[1] + other[1])
    }
    
    #[inline(always)]
    pure fn sub_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0] - other[0],
                  self[1] - other[1])
    }
    
    #[inline(always)]
    pure fn dot(&self, other: &Vec2<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1]
    }
}

pub impl<T:Copy Number> Vec2<T>: Neg<Vec2<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Vec2<T> {
        Vec2::new(-self[0], -self[1])
    }
}
    
pub impl<T:Copy Number Exp> Vec2<T>: GeometricVector<T> {
    #[inline(always)]
    pure fn length2(&self) -> T {
        self.dot(self)
    }
    
    #[inline(always)]
    pure fn length(&self) -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn distance2(&self, other: &Vec2<T>) -> T {
        other.sub_v(self).length2()
    }
    
    #[inline(always)]
    pure fn distance(&self, other: &Vec2<T>) -> T {
        other.distance2(self).sqrt()
    }
    
    #[inline(always)]
    pure fn normalize(&self) -> Vec2<T> {
        let mut n: T = Number::from(1); 
        n /= self.length();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn lerp(&self, other: &Vec2<T>, amount: T) -> Vec2<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
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
    pure fn default_eq(&self, other: &Vec2<T>) -> bool {
        self[0].default_eq(&other[0]) &&
        self[1].default_eq(&other[1])
    }
}






/**
 *  Vec3
 */
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
}

pub impl<T> Vec3<T>: Dimensional<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 3 }
    
    #[inline(always)]
    static pure fn size_of() -> uint { size_of::<Vec3<T>>() }
}

pub impl<T:Copy> Vec3<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 3) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> Vec3<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Vec3<T>, *T>(
                to_unsafe_ptr(&*self)
            )
        }
    }
}

pub impl<T:Copy> Vec3<T>: MutableVector<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &self/mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 2, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        util::swap(self.index_mut(a),
                   self.index_mut(b));
    }
}

pub impl<T:Copy Number> Vec3<T>: NumericVector<T> {
    #[inline(always)]
    static pure fn identity() -> Vec3<T> {
        Vec3::new(Number::one(),
                  Number::one(),
                  Number::one())
    }
    
    #[inline(always)]
    static pure fn zero() -> Vec3<T> {
        Vec3::new(Number::zero(),
                  Number::zero(),
                  Number::zero())
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Vec3<T> {
        Vec3::new(self[0] * value,
                  self[1] * value,
                  self[2] * value)
    }
    
    #[inline(always)]
    pure fn div_t(&self, value: T) -> Vec3<T> {
        Vec3::new(self[0] / value,
                  self[1] / value,
                  self[2] / value)
    }
    
    #[inline(always)]
    pure fn add_v(&self, other: &Vec3<T>) -> Vec3<T>{
        Vec3::new(self[0] + other[0],
                  self[1] + other[1],
                  self[2] + other[2])
    }
    
    #[inline(always)]
    pure fn sub_v(&self, other: &Vec3<T>) -> Vec3<T>{
        Vec3::new(self[0] - other[0],
                  self[1] - other[1],
                  self[2] - other[2])
    }
    
    #[inline(always)]
    pure fn dot(&self, other: &Vec3<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2]
    }
}

pub impl<T:Copy Number> Vec3<T>: Neg<Vec3<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Vec3<T> {
        Vec3::new(-self[0], -self[1], -self[2])
    }
}

pub impl<T:Copy Number> Vec3<T>: NumericVector3<T> {
    #[inline(always)]
    pure fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new((self[1] * other[2]) - (self[2] * other[1]),
                  (self[2] * other[0]) - (self[0] * other[2]),
                  (self[0] * other[1]) - (self[1] * other[0]))
    }
}

pub impl<T:Copy Number Exp> Vec3<T>: GeometricVector<T> {
    #[inline(always)]
    pure fn length2(&self) -> T {
        self.dot(self)
    }
    
    #[inline(always)]
    pure fn length(&self) -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn distance2(&self, other: &Vec3<T>) -> T {
        other.sub_v(self).length2()
    }
    
    #[inline(always)]
    pure fn distance(&self, other: &Vec3<T>) -> T {
        other.distance2(self).sqrt()
    }
    
    #[inline(always)]
    pure fn normalize(&self) -> Vec3<T> {
        let mut n: T = Number::from(1);
        n /= self.length();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn lerp(&self, other: &Vec3<T>, amount: T) -> Vec3<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
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
    pure fn default_eq(&self, other: &Vec3<T>) -> bool {
        self[0].default_eq(&other[0]) &&
        self[1].default_eq(&other[1]) &&
        self[2].default_eq(&other[2])
    }
}






/**
 *  Vec4
 */
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
}

pub impl<T> Vec4<T>: Dimensional<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    static pure fn size_of() -> uint { size_of::<Vec4<T>>() }
}

pub impl<T:Copy> Vec4<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 4) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> Vec4<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Vec4<T>, *T>(
                to_unsafe_ptr(&*self)
            )
        }
    }
}

pub impl<T:Copy> Vec4<T>: MutableVector<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &self/mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 3, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        util::swap(self.index_mut(a),
                   self.index_mut(b));
    }
}

pub impl<T:Copy Number> Vec4<T>: NumericVector<T> {
    #[inline(always)]
    static pure fn identity() -> Vec4<T> {
        Vec4::new(Number::one(),
                  Number::one(),
                  Number::one(),
                  Number::one())
    }
    
    #[inline(always)]
    static pure fn zero() -> Vec4<T> {
        Vec4::new(Number::zero(),
                  Number::zero(),
                  Number::zero(),
                  Number::zero())
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Vec4<T> {
        Vec4::new(self[0] * value,
                  self[1] * value,
                  self[2] * value,
                  self[3] * value)
    }
    
    #[inline(always)]
    pure fn div_t(&self, value: T) -> Vec4<T> {
        Vec4::new(self[0] / value,
                  self[1] / value,
                  self[2] / value,
                  self[3] / value)
    }
    
    #[inline(always)]
    pure fn add_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0] + other[0],
                  self[1] + other[1],
                  self[2] + other[2],
                  self[3] + other[3])
    }
    
    #[inline(always)]
    pure fn sub_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0] - other[0],
                  self[1] - other[1],
                  self[2] - other[2],
                  self[3] - other[3])
    }
    
    #[inline(always)]
    pure fn dot(&self, other: &Vec4<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2] +
        self[3] * other[3]
    }
}

pub impl<T:Copy Number> Vec4<T>: Neg<Vec4<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Vec4<T> {
        Vec4::new(-self[0], -self[1], -self[2], -self[3])
    }
}

pub impl<T:Copy Number Exp> Vec4<T>: GeometricVector<T> {
    #[inline(always)]
    pure fn length2(&self) -> T {
        self.dot(self)
    }
    
    #[inline(always)]
    pure fn length(&self) -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn distance2(&self, other: &Vec4<T>) -> T {
        other.sub_v(self).length2()
    }
    
    #[inline(always)]
    pure fn distance(&self, other: &Vec4<T>) -> T {
        other.distance2(self).sqrt()
    }
    
    #[inline(always)]
    pure fn normalize(&self) -> Vec4<T> {
        let mut n: T = Number::from(1);
        n /= self.length();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn lerp(&self, other: &Vec4<T>, amount: T) -> Vec4<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
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
    pure fn default_eq(&self, other: &Vec4<T>) -> bool {
        self[0].default_eq(&other[0]) &&
        self[1].default_eq(&other[1]) &&
        self[2].default_eq(&other[2]) &&
        self[3].default_eq(&other[3])
    }
}