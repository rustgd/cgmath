use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::to_unsafe_ptr;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;

use funs::common::*;
use funs::exponential::*;
use num::types::{Float, Number};
use vec::Vec2;

/**
 *  A 2 x 2 column major matrix
 *
 * # Type parameters
 *
 * * `T` - The type of the elements of the matrix. Should be a floating point type.
 *
 * # Fields
 *
 * * `x` - the first column vector of the matrix
 * * `y` - the second column vector of the matrix
 * * `z` - the third column vector of the matrix
 */
pub struct Mat2<T> { x: Vec2<T>, y: Vec2<T> }

pub impl<T:Copy Float> Mat2<T> {
    /**
     * Construct a 2 x 2 matrix
     *
     * # Arguments
     *
     * * `c0r0`, `c0r1` - the first column of the matrix
     * * `c1r0`, `c1r1` - the second column of the matrix
     *
     * ~~~
     *        c0     c1
     *     +------+------+
     *  r0 | c0r0 | c1r0 |
     *     +------+------+
     *  r1 | c0r1 | c1r1 |
     *     +------+------+
     * ~~~
     */
    #[inline(always)]
    static pure fn new(c0r0: T, c0r1: T,
                       c1r0: T, c1r1: T) -> Mat2<T> {
        Mat2::from_cols(Vec2::new(move c0r0, move c0r1),
                        Vec2::new(move c1r0, move c1r1))
    }
    
    /**
     * Construct a 2 x 2 matrix from column vectors
     *
     * # Arguments
     *
     * * `c0` - the first column vector of the matrix
     * * `c1` - the second column vector of the matrix
     *
     * ~~~
     *        c0     c1
     *     +------+------+
     *  r0 | c0.x | c1.x |
     *     +------+------+
     *  r1 | c0.y | c1.y |
     *     +------+------+
     * ~~~
     */
    #[inline(always)]
    static pure fn from_cols(c0: Vec2<T>, c1: Vec2<T>) -> Mat2<T> {
        Mat2 { x: move c0,
               y: move c1 }
    }
    
    /**
     * Construct a 2 x 2 diagonal matrix with the major diagonal set to `value`
     *
     * # Arguments
     *
     * * `value` - the value to set the major diagonal to
     *
     * ~~~
     *        c0    c1
     *     +-----+-----+
     *  r0 | val |   0 |
     *     +-----+-----+
     *  r1 |   0 | val |
     *     +-----+-----+
     * ~~~
     */
    #[inline(always)]
    static pure fn from_value(value: T) -> Mat2<T> {
        let _0 = Number::from(0);
        Mat2::new(value,    _0,
                     _0, value)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn identity() -> Mat2<T> {
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat2::new(_1, _0,
                  _0, _1)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn zero() -> Mat2<T> {
        let _0 = Number::from(0);
        Mat2::new(_0, _0,
                  _0, _0)
    }
}

pub impl<T:Copy Float> Mat2<T>: Matrix<T, Vec2<T>> {
    #[inline(always)]
    pure fn col(&self, i: uint) -> Vec2<T> { self[i] }
    
    #[inline(always)]
    pure fn row(&self, i: uint) -> Vec2<T> {
        Vec2::new(self[0][i],
                  self[1][i])
    }
    
    /**
     * Returns the multiplicative identity matrix
     * ~~~
     *       c0   c1
     *     +----+----+ 
     *  r0 |  1 |  0 |
     *     +----+----+
     *  r1 |  0 |  1 |
     *     +----+----+
     * ~~~
     */
    #[inline(always)]
    static pure fn identity() -> Mat2<T> {
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat2::new(_1, _0,
                  _0, _1)
    }
    
    /**
     * Returns the additive identity matrix
     * ~~~
     *       c0   c1
     *     +----+----+ 
     *  r0 |  0 |  0 |
     *     +----+----+
     *  r1 |  0 |  0 |
     *     +----+----+
     * ~~~
     */
    #[inline(always)]
    static pure fn zero() -> Mat2<T> {
        let _0 = Number::from(0);
        Mat2::new(_0, _0,
                  _0, _0)
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Mat2<T> {
        Mat2::from_cols(self[0].mul_t(value),
                        self[1].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(&self, vec: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self.row(0).dot(vec),
                  self.row(1).dot(vec))
    }
    
    #[inline(always)]
    pure fn add_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self[0].add_v(&other[0]),
                        self[1].add_v(&other[1]))
    }
    
    #[inline(always)]
    pure fn sub_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self[0].sub_v(&other[0]),
                        self[1].sub_v(&other[1]))
    }
    
    #[inline(always)]
    pure fn mul_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::new(self.row(0).dot(&other.col(0)), self.row(1).dot(&other.col(0)),
                  self.row(0).dot(&other.col(1)), self.row(1).dot(&other.col(1)))
    }

    pure fn dot(&self, other: &Mat2<T>) -> T {
        other.transpose().mul_m(self).trace()
    }
    
    pure fn determinant(&self) -> T {
       self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    pure fn trace(&self) -> T {
        self[0][0] + self[1][1]
    }

    #[inline(always)]
    pure fn inverse(&self) -> Option<Mat2<T>> {
        let d = self.determinant();
        if d.fuzzy_eq(&Number::from(0)) {
            None
        } else {
            Some(Mat2::new( self[1][1]/d, -self[0][1]/d,
                           -self[1][0]/d,  self[0][0]/d))
        }
    }
    
    #[inline(always)]
    pure fn transpose(&self) -> Mat2<T> {
        Mat2::new(self[0][0], self[1][0],
                  self[0][1], self[1][1])
    }
    
    #[inline(always)]
    pure fn is_identity(&self) -> bool {
        // self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        self.fuzzy_eq(&Mat2::identity())
    }
    
    #[inline(always)]
    pure fn is_diagonal(&self) -> bool {
        let _0 = Number::from(0);
        self[0][1].fuzzy_eq(&_0) &&
        self[1][0].fuzzy_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated(&self) -> bool {
        // !self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        !self.fuzzy_eq(&Mat2::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric(&self) -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[1][0].fuzzy_eq(&self[0][1])
    }

    #[inline(always)]
    pure fn is_invertible(&self) -> bool {
        !self.determinant().fuzzy_eq(&Number::from(0))
    }
    
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Mat2<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T:Copy Float Sign> Mat2<T>: MutableMatrix<T, Vec2<T>> {
    #[inline(always)]
    fn col_mut(&mut self, i: uint) -> &self/mut Vec2<T> {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 1, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        util::swap(self.col_mut(a),
                   self.col_mut(b));
    }
    
    #[inline(always)]
    fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
    }
    
    #[inline(always)]
    fn set(&mut self, other: &Mat2<T>) {
        (*self) = (*other);
    }
    
    #[inline(always)]
    fn to_identity(&mut self) {
        (*self) = Mat2::identity();
    }
    
    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = Mat2::zero();
    }
    
    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(&value);
        self.col_mut(1).mul_self_t(&value);
    }
    
    #[inline(always)]
    fn add_self_m(&mut self, other: &Mat2<T>) {
        self.col_mut(0).add_self_v(&other[0]);
        self.col_mut(1).add_self_v(&other[1]);
    }
    
    #[inline(always)]
    fn sub_self_m(&mut self, other: &Mat2<T>) {
        self.col_mut(0).sub_self_v(&other[0]);
        self.col_mut(1).sub_self_v(&other[1]);
    }
    
    #[inline(always)]
    fn invert_self(&mut self) {
        match self.inverse() {
            Some(m) => (*self) = m,
            None => fail(~"Couldn't invert the matrix!")
        }
    }
    
    #[inline(always)]
    fn transpose_self(&mut self) {
        util::swap(self.col_mut(0).index_mut(1), self.col_mut(1).index_mut(0));
        util::swap(self.col_mut(1).index_mut(0), self.col_mut(0).index_mut(1));
    }
}

pub impl<T:Copy Float> Mat2<T>: Matrix2<T, Vec2<T>> {
    #[inline(always)]
    pure fn to_mat3(&self) -> Mat3<T> {
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat3::new(self[0][0], self[0][1], _0,
                  self[1][0], self[1][1], _0,
                          _0,         _0, _1)
    }
    
    #[inline(always)]
    pure fn to_mat4(&self) -> Mat4<T> {
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat4::new(self[0][0], self[0][1], _0, _0,
                  self[1][0], self[1][1], _0, _0,
                          _0,         _0, _1, _0,
                          _0,         _0, _0, _1)
    }
}

pub impl<T:Copy> Mat2<T>: Index<uint, Vec2<T>> {
    #[inline(always)]
    pure fn index(&self, i: uint) -> Vec2<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat2<T>, *Vec2<T>>(
                to_unsafe_ptr(self)), 2) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy Float> Mat2<T>: Neg<Mat2<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Mat2<T> {
        Mat2::from_cols(-self[0], -self[1])
    }
}

pub impl<T:Copy Float> Mat2<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Mat2<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1]
    }
    
    #[inline(always)]
    pure fn ne(&self, other: &Mat2<T>) -> bool {
        !(self == other)
    }
}

pub impl<T:Copy Float> Mat2<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat2<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1])
    }
}