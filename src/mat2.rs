use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::util::swap;
use core::vec::raw::buf_as_slice;

use std::cmp::{FuzzyEq, FUZZY_EPSILON};
use numeric::*;
use numeric::number::Number;
use numeric::number::Number::{zero,one};

use vec::{
    Vec2,
    Vector2,
    MutableVector,
    NumericVector,
    MutableNumericVector,
    vec2,
    dvec2,
};

use mat::{
    Mat3,
    Mat4,
    Matrix,
    Matrix2,
    Matrix3,
    Matrix4,
    MutableMatrix,
};

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
#[deriving(Eq)]
pub struct Mat2<T> { x: Vec2<T>, y: Vec2<T> }

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Matrix<T, Vec2<T>> for Mat2<T> {
    #[inline(always)]
    fn col(&self, i: uint) -> Vec2<T> { self[i] }

    #[inline(always)]
    fn row(&self, i: uint) -> Vec2<T> {
        Vector2::new(self[0][i],
                     self[1][i])
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
    fn from_value(value: T) -> Mat2<T> {
        Matrix2::new(value, zero(),
                     zero(), value)
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
    fn identity() -> Mat2<T> {
        Matrix2::new( one::<T>(), zero::<T>(),
                     zero::<T>(),  one::<T>())
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
    fn zero() -> Mat2<T> {
        Matrix2::new(zero::<T>(), zero::<T>(),
                     zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn mul_t(&self, value: T) -> Mat2<T> {
        Matrix2::from_cols(self[0].mul_t(value),
                           self[1].mul_t(value))
    }

    #[inline(always)]
    fn mul_v(&self, vec: &Vec2<T>) -> Vec2<T> {
        Vector2::new(self.row(0).dot(vec),
                     self.row(1).dot(vec))
    }

    #[inline(always)]
    fn add_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Matrix2::from_cols(self[0].add_v(&other[0]),
                           self[1].add_v(&other[1]))
    }

    #[inline(always)]
    fn sub_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Matrix2::from_cols(self[0].sub_v(&other[0]),
                           self[1].sub_v(&other[1]))
    }

    #[inline(always)]
    fn mul_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Matrix2::new(self.row(0).dot(&other.col(0)), self.row(1).dot(&other.col(0)),
                     self.row(0).dot(&other.col(1)), self.row(1).dot(&other.col(1)))
    }

    fn dot(&self, other: &Mat2<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    fn determinant(&self) -> T {
       self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    fn trace(&self) -> T {
        self[0][0] + self[1][1]
    }

    #[inline(always)]
    fn inverse(&self) -> Option<Mat2<T>> {
        let d = self.determinant();
        if d.fuzzy_eq(&zero()) {
            None
        } else {
            Some(Matrix2::new( self[1][1]/d, -self[0][1]/d,
                              -self[1][0]/d,  self[0][0]/d))
        }
    }

    #[inline(always)]
    fn transpose(&self) -> Mat2<T> {
        Matrix2::new(self[0][0], self[1][0],
                     self[0][1], self[1][1])
    }

    #[inline(always)]
    fn is_identity(&self) -> bool {
        self.fuzzy_eq(&Matrix::identity())
    }

    #[inline(always)]
    fn is_diagonal(&self) -> bool {
        self[0][1].fuzzy_eq(&zero()) &&
        self[1][0].fuzzy_eq(&zero())
    }

    #[inline(always)]
    fn is_rotated(&self) -> bool {
        !self.fuzzy_eq(&Matrix::identity())
    }

    #[inline(always)]
    fn is_symmetric(&self) -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[1][0].fuzzy_eq(&self[0][1])
    }

    #[inline(always)]
    fn is_invertible(&self) -> bool {
        !self.determinant().fuzzy_eq(&zero())
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Mat2<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> MutableMatrix<T, Vec2<T> > for Mat2<T> {
    #[inline(always)]
    fn col_mut(&mut self, i: uint) -> &'self mut Vec2<T> {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => fail!(fmt!("index out of bounds: expected an index from 0 to 1, but found %u", i))
        }
    }

    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        swap(self.col_mut(a),
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
        (*self) = Matrix::identity();
    }

    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = Matrix::zero();
    }

    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        self.x.mul_self_t(value);
        self.y.mul_self_t(value);
    }

    #[inline(always)]
    fn add_self_m(&mut self, other: &Mat2<T>) {
        self.x.add_self_v(&other[0]);
        self.y.add_self_v(&other[1]);
    }

    #[inline(always)]
    fn sub_self_m(&mut self, other: &Mat2<T>) {
        self.x.sub_self_v(&other[0]);
        self.y.sub_self_v(&other[1]);
    }

    #[inline(always)]
    fn invert_self(&mut self) {
        match self.inverse() {
            Some(m) => (*self) = m,
            None => fail!(~"Couldn't invert the matrix!")
        }
    }

    #[inline(always)]
    fn transpose_self(&mut self) {
        swap(self.x.index_mut(1), self.y.index_mut(0));
        swap(self.y.index_mut(0), self.x.index_mut(1));
    }
}

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Matrix2<T, Vec2<T>> for Mat2<T> {
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
    fn new(c0r0: T, c0r1: T,
                       c1r0: T, c1r1: T) -> Mat2<T> {
        Matrix2::from_cols(Vector2::new::<T,Vec2<T>>(c0r0, c0r1),
                           Vector2::new::<T,Vec2<T>>(c1r0, c1r1))
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
    fn from_cols(c0: Vec2<T>,
                             c1: Vec2<T>) -> Mat2<T> {
        Mat2 { x: c0, y: c1 }
    }

    #[inline(always)]
    fn from_angle(radians: T) -> Mat2<T> {
        let cos_theta = cos(radians);
        let sin_theta = sin(radians);

        Matrix2::new(cos_theta, -sin_theta,
                     sin_theta,  cos_theta)
    }

    /**
     * Returns the the matrix with an extra row and column added
     * ~~~
     *       c0   c1                 c0   c1   c2
     *     +----+----+             +----+----+----+
     *  r0 |  a |  b |          r0 |  a |  b |  0 |
     *     +----+----+             +----+----+----+
     *  r1 |  c |  d |    =>    r1 |  c |  d |  0 |
     *     +----+----+             +----+----+----+
     *                          r2 |  0 |  0 |  1 |
     *                             +----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn to_mat3(&self) -> Mat3<T> {
        Matrix3::new(self[0][0], self[0][1], zero(),
                     self[1][0], self[1][1], zero(),
                         zero(),     zero(),  one())
    }

    /**
     * Returns the the matrix with an extra two rows and columns added
     * ~~~
     *       c0   c1                 c0   c1   c2   c3
     *     +----+----+             +----+----+----+----+
     *  r0 |  a |  b |          r0 |  a |  b |  0 |  0 |
     *     +----+----+             +----+----+----+----+
     *  r1 |  c |  d |    =>    r1 |  c |  d |  0 |  0 |
     *     +----+----+             +----+----+----+----+
     *                          r2 |  0 |  0 |  1 |  0 |
     *                             +----+----+----+----+
     *                          r3 |  0 |  0 |  0 |  1 |
     *                             +----+----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn to_mat4(&self) -> Mat4<T> {
        Matrix4::new(self[0][0], self[0][1], zero(), zero(),
                     self[1][0], self[1][1], zero(), zero(),
                         zero(),     zero(),  one(), zero(),
                         zero(),     zero(), zero(),  one())
    }
}

impl<T:Copy> Index<uint, Vec2<T>> for Mat2<T> {
    #[inline(always)]
    fn index(&self, i: &uint) -> Vec2<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat2<T>, *Vec2<T>>(
                to_unsafe_ptr(self)), 2) |slice| { slice[*i] }
        }
    }
}

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Neg<Mat2<T>> for Mat2<T> {
    #[inline(always)]
    fn neg(&self) -> Mat2<T> {
        Matrix2::from_cols(-self[0], -self[1])
    }
}

impl<T:Copy + Float + FuzzyEq<T>> FuzzyEq<T> for Mat2<T> {
    #[inline(always)]
    fn fuzzy_eq(&self, other: &Mat2<T>) -> bool {
        self.fuzzy_eq_eps(other, &Number::from(FUZZY_EPSILON))
    }

    #[inline(always)]
    fn fuzzy_eq_eps(&self, other: &Mat2<T>, epsilon: &T) -> bool {
        self[0].fuzzy_eq_eps(&other[0], epsilon) &&
        self[1].fuzzy_eq_eps(&other[1], epsilon)
    }
}

// GLSL-style type aliases, corresponding to Section 4.1.6 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

pub type mat2 = Mat2<f32>;      // a 2×2 single-precision floating-point matrix
pub type dmat2 = Mat2<f64>;     // a 2×2 double-precision floating-point matrix

// Static method wrappers for GLSL-style types

pub impl mat2 {
    #[inline(always)] fn new(c0r0: f32, c0r1: f32, c1r0: f32, c1r1: f32)
        -> mat2 { Matrix2::new(c0r0, c0r1, c1r0, c1r1) }
    #[inline(always)] fn from_cols(c0: vec2, c1: vec2)
        -> mat2 { Matrix2::from_cols(c0, c1) }
    #[inline(always)] fn from_value(v: f32) -> mat2 { Matrix::from_value(v) }

    #[inline(always)] fn identity() -> mat2 { Matrix::identity() }
    #[inline(always)] fn zero() -> mat2 { Matrix::zero() }

    #[inline(always)] fn from_angle(radians: f32) -> mat2 { Matrix2::from_angle(radians) }

    #[inline(always)] fn dim() -> uint { 2 }
    #[inline(always)] fn rows() -> uint { 2 }
    #[inline(always)] fn cols() -> uint { 2 }
    #[inline(always)] fn size_of() -> uint { size_of::<mat2>() }
}

pub impl dmat2 {
    #[inline(always)] fn new(c0r0: f64, c0r1: f64, c1r0: f64, c1r1: f64)
        -> dmat2 { Matrix2::new(c0r0, c0r1, c1r0, c1r1) }
    #[inline(always)] fn from_cols(c0: dvec2, c1: dvec2)
        -> dmat2 { Matrix2::from_cols(c0, c1) }
    #[inline(always)] fn from_value(v: f64) -> dmat2 { Matrix::from_value(v) }

    #[inline(always)] fn identity() -> dmat2 { Matrix::identity() }
    #[inline(always)] fn zero() -> dmat2 { Matrix::zero() }

    #[inline(always)] fn from_angle(radians: f64) -> dmat2 { Matrix2::from_angle(radians) }

    #[inline(always)] fn dim() -> uint { 2 }
    #[inline(always)] fn rows() -> uint { 2 }
    #[inline(always)] fn cols() -> uint { 2 }
    #[inline(always)] fn size_of() -> uint { size_of::<dmat2>() }
}
