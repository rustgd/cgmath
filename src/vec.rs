use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;

use dim::{Dimensional, ToPtr};
use funs::exponential::Exp;
use num::kinds::Number;

/**
 * The base generic vector trait.
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 */
pub trait Vector<T>: Dimensional<T> ToPtr<T> Eq {
    /**
     * Construct the vector from a single value, copying it to each component
     */
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
    // static pure fn new(x: T, y: T, z: T, w: T) -> self;
}

/**
 * A vector with numeric components
 */
pub trait NumericVector<T>: Vector<T> Neg<self> {
    /**
     * The standard basis vector
     *
     * # Return value
     *
     * A vector with each component set to one
     */
    static pure fn identity() -> self;
    
    /**
     * The null vector
     *
     * # Return value
     *
     * A vector with each component set to zero
     */
    static pure fn zero() -> self;
    
    /**
     * # Return value
     *
     * The scalar multiplication of the vector and `value`
     */
    pure fn mul_t(&self, value: T) -> self;
    
    /**
     * # Return value
     *
     * The scalar division of the vector and `value`
     */
    pure fn div_t(&self, value: T) -> self;
    
    /**
     * # Return value
     *
     * The sum of the vector and `other`
     */
    pure fn add_v(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The difference between the vector and `other`
     */
    pure fn sub_v(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The dot product of the vector and `other`
     */
    pure fn dot(&self, other: &self) -> T;
}

/**
 * A mutable vector with numeric components
 */
pub trait MutableNumericVector<T>: MutableVector<&self/T> NumericVector<T> {
    /**
     * Negate the vector
     */
    fn neg_self(&mut self);
    
    /**
     * Multiply the vector by a scalar
     */
    fn mul_self_t(&mut self, value: T);
    
    /**
     * Divide the vector by a scalar
     */
    fn div_self_t(&mut self, value: T);
    
    /**
     * Set to the sum of the vector and `other`
     */
    fn add_self_v(&mut self, other: &self);
    
    /**
     * Set to the difference between the vector and `other`
     */
    fn sub_self_v(&mut self, other: &self);
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
     * # Return value
     *
     * The cross product of the vector and `other`
     */
    pure fn cross(&self, other: &self) -> self;
}

/**
 * A mutable 3-dimensional vector with numeric components
 */
pub trait MutableNumericVector3<T>: MutableNumericVector<&self/T> {
    /**
     * Set to the cross product of the vector and `other`
     */
    fn cross_self(&mut self, other: &self);
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
 * A Euclidean (or Affine) vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This should be a floating point type.
 */
pub trait EuclideanVector<T>: NumericVector<T> {
    /**
     * # Return value
     *
     * The squared length of the vector. This is useful for comparisons where
     * the exact length does not need to be calculated.
     */
    pure fn length2(&self) -> T;
    
    /**
     * # Return value
     *
     * The length of the vector
     *
     * # Performance notes
     *
     * For instances where the exact length of the vector does not need to be
     * known, for example for quaternion-quaternion length comparisons,
     * it is advisable to use the `length2` method instead.
     */
    pure fn length(&self) -> T;
    
    /**
     * # Return value
     *
     * The squared distance between the vector and `other`.
     */
    pure fn distance2(&self, other: &self) -> T;
    
    /**
     * # Return value
     *
     * The distance between the vector and `other`
     */
    pure fn distance(&self, other: &self) -> T;
    
    /**
     * # Return value
     *
     * The normalized vector
     */
    pure fn normalize(&self) -> self;
    
    /**
     * Linearly intoperlate between the vector and `other`
     *
     * # Return value
     *
     * The intoperlated vector
     */
    pure fn lerp(&self, other: &self, amount: T) -> self;
}

/**
 * A mutable Euclidean (or Affine) vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This should be a floating point type.
 */
pub trait MutableEuclideanVector<T>: MutableNumericVector<&self/T>
                                     EuclideanVector<T> {
    /**
     * Normalize the vector
     */
    fn normalize_self(&mut self);
    
    /**
     * Linearly intoperlate the vector towards `other`
     */
    fn lerp_self(&mut self, other: &self, amount: T);
}





/**
 * A 2-dimensional vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 *
 * # Fields
 *
 * * `x` - the first component of the vector
 * * `y` - the second component of the vector
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
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 2) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> Vec2<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Vec2<T>, *T>(
                to_unsafe_ptr(self)
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

pub impl<T:Copy Number> Vec2<T>: MutableNumericVector<&self/T> {
    #[inline(always)]
    fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index_mut(0);
        *self.index_mut(1) = -*self.index_mut(1);
    }
    
    #[inline(always)]
    fn mul_self_t(&mut self, value: &T) {
        *self.index_mut(0) *= (*value);
        *self.index_mut(1) *= (*value);
    }
    
    #[inline(always)]
    fn div_self_t(&mut self, value: &T) {
        *self.index_mut(0) /= (*value);
        *self.index_mut(1) /= (*value);
    }
    
    #[inline(always)]
    fn add_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) += other[0];
        *self.index_mut(1) += other[1];
    }
    
    #[inline(always)]
    fn sub_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) -= other[0];
        *self.index_mut(1) -= other[1];
    }
}

pub impl<T:Copy Number Exp> Vec2<T>: EuclideanVector<T> {
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

pub impl<T:Copy Number Exp> Vec2<T>: MutableEuclideanVector<&self/T> {
    #[inline(always)]
    fn normalize_self(&mut self) {
        let mut n: T = Number::from(1); 
        n /= self.length();
        self.mul_self_t(&n);
    }
    
    fn lerp_self(&mut self, other: &Vec2<T>, amount: &T) {
        self.add_self_v(&other.sub_v(&*self).mul_t(*amount));
    }
}

pub impl<T:Copy Eq> Vec2<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Vec2<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1]
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






/**
 * A 3-dimensional vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 *
 * # Fields
 *
 * * `x` - the first component of the vector
 * * `y` - the second component of the vector
 * * `z` - the third component of the vector
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
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 3) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> Vec3<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Vec3<T>, *T>(
                to_unsafe_ptr(self)
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

pub impl<T:Copy Number> Vec3<T>: MutableNumericVector<&self/T> {
    #[inline(always)]
    fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index_mut(0);
        *self.index_mut(1) = -*self.index_mut(1);
        *self.index_mut(2) = -*self.index_mut(2);
    }
    
    #[inline(always)]
    fn mul_self_t(&mut self, value: &T) {
        *self.index_mut(0) *= (*value);
        *self.index_mut(1) *= (*value);
        *self.index_mut(2) *= (*value);
    }
    
    #[inline(always)]
    fn div_self_t(&mut self, value: &T) {
        *self.index_mut(0) /= (*value);
        *self.index_mut(1) /= (*value);
        *self.index_mut(2) /= (*value);
    }
    
    #[inline(always)]
    fn add_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) += other[0];
        *self.index_mut(1) += other[1];
        *self.index_mut(2) += other[2];
    }
    
    #[inline(always)]
    fn sub_self_v(&mut self, other: &Vec3<T>) {
        *self.index_mut(0) -= other[0];
        *self.index_mut(1) -= other[1];
        *self.index_mut(2) -= other[2];
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

pub impl<T:Copy Number> Vec3<T>: MutableNumericVector3<&self/T> {
    #[inline(always)]
    fn cross_self(&mut self, other: &Vec3<T>) {
        *self = self.cross(other);
    }
}

pub impl<T:Copy Number Exp> Vec3<T>: EuclideanVector<T> {
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

pub impl<T:Copy Number Exp> Vec3<T>: MutableEuclideanVector<&self/T> {
    #[inline(always)]
    fn normalize_self(&mut self) {
        let mut n: T = Number::from(1); 
        n /= self.length();
        self.mul_self_t(&n);
    }
    
    fn lerp_self(&mut self, other: &Vec3<T>, amount: &T) {
        self.add_self_v(&other.sub_v(&*self).mul_t(*amount));
    }
}

pub impl<T:Copy Eq> Vec3<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Vec3<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2]
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






/**
 * A 4-dimensional vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 *
 * # Fields
 *
 * * `x` - the first component of the vector
 * * `y` - the second component of the vector
 * * `z` - the third component of the vector
 * * `w` - the fourth component of the vector
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
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 4) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> Vec4<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Vec4<T>, *T>(
                to_unsafe_ptr(self)
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

pub impl<T:Copy Number> Vec4<T>: MutableNumericVector<&self/T> {
    #[inline(always)]
    fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index_mut(0);
        *self.index_mut(1) = -*self.index_mut(1);
        *self.index_mut(2) = -*self.index_mut(2);
        *self.index_mut(3) = -*self.index_mut(3);
    }
    
    #[inline(always)]
    fn mul_self_t(&mut self, value: &T) {
        *self.index_mut(0) *= (*value);
        *self.index_mut(1) *= (*value);
        *self.index_mut(2) *= (*value);
        *self.index_mut(3) *= (*value);
    }
    
    #[inline(always)]
    fn div_self_t(&mut self, value: &T) {
        *self.index_mut(0) /= (*value);
        *self.index_mut(1) /= (*value);
        *self.index_mut(2) /= (*value);
        *self.index_mut(3) /= (*value);
    }
    
    #[inline(always)]
    fn add_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) += other[0];
        *self.index_mut(1) += other[1];
        *self.index_mut(2) += other[2];
        *self.index_mut(3) += other[3];
    }
    
    #[inline(always)]
    fn sub_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) -= other[0];
        *self.index_mut(1) -= other[1];
        *self.index_mut(2) -= other[2];
        *self.index_mut(3) -= other[3];
    }
}

pub impl<T:Copy Number Exp> Vec4<T>: EuclideanVector<T> {
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

pub impl<T:Copy Number Exp> Vec4<T>: MutableEuclideanVector<&self/T> {
    #[inline(always)]
    fn normalize_self(&mut self) {
        let mut n: T = Number::from(1); 
        n /= self.length();
        self.mul_self_t(&n);
    }
    
    fn lerp_self(&mut self, other: &Vec4<T>, amount: &T) {
        self.add_self_v(&other.sub_v(&*self).mul_t(*amount));
    }
}

pub impl<T:Copy Eq> Vec4<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Vec4<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2] &&
        self[3] == other[3]
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