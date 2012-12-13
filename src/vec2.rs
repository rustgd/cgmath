use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::to_unsafe_ptr;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;

use angle::Radians;
use funs::exponential::Exp;
use funs::triganomic::{InvTrig, atan2};
use num::types::Number;

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
    
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Vec2<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T:Copy> Vec2<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 2) |slice| { slice[i] } }
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

pub impl<T:Copy Number> Vec2<T>: NumericVector2<T> {
    #[inline(always)]
    pure fn perp_dot(&self, other: &Vec2<T>) ->T {
        (self[0] * other[1]) - (self[1] * other[0])
    }
}

pub impl<T:Copy Number Exp InvTrig> Vec2<T>: EuclideanVector<T> {
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
    pure fn angle(&self, other: &Vec2<T>) -> Radians<T> {
        atan2(&self.perp_dot(other), &self.dot(other))
    }
    
    #[inline(always)]
    pure fn normalize(&self) -> Vec2<T> {
        let mut n: T = Number::from(1); 
        n /= self.length();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn normalize_to(&self, length: T) -> Vec2<T> {
        let mut n: T = length / self.length();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn lerp(&self, other: &Vec2<T>, amount: T) -> Vec2<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }
}

pub impl<T:Copy Number Exp InvTrig> Vec2<T>: MutableEuclideanVector<&self/T> {
    #[inline(always)]
    fn normalize_self(&mut self) {
        let mut n: T = Number::from(1); 
        n /= self.length();
        self.mul_self_t(&n);
    }
    
    #[inline(always)]
    fn normalize_self_to(&mut self, length: &T) {
        let mut n: T = length / self.length();
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