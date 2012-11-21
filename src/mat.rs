use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::to_unsafe_ptr;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;

use dim::Dimensional;
use funs::common::*;
use funs::exp::*;
use num::cast::*;
use num::default_eq::DefaultEq;
use quat::{Quat, ToQuat};
use vec::{NumericVector, Vec2, Vec3, Vec4};

///
/// The base Matrix trait
///
pub trait Matrix<T, Col, Row>: Dimensional<T>, Eq, DefaultEq {
    pure fn rows() -> uint;
    pure fn cols() -> uint;
    pure fn is_col_major() -> bool;
    pure fn is_square() -> bool;
    
    pure fn col(i: uint) -> Col;
    pure fn row(i: uint) -> Row;
}

/// A 2 x N matrix
pub trait Matrix2<T, Col, Row>: Matrix<T, Col, Row> {
//     /// Construct the matrix from two column vectors
//     static pure fn from_cols(c0: Col, c1: Col) -> self;
}

/// A 3 x N matrix
pub trait Matrix3<T, Col, Row>: Matrix<T, Col, Row> {
//     /// Construct the matrix from three column vectors
//     static pure fn from_cols(c0: Col, c1: Col, c2: Col) -> self;
}

/// A 4 x N matrix
pub trait Matrix4<T, Col, Row>: Matrix<T, Col, Row> {
//     /// Construct the matrix from four column vectors
//     static pure fn from_cols(c0: Col, c1: Col, c2: Col, c3: Col) -> self;
}

///
/// A square matrix
///
pub trait MatrixNxN<T, ColRow>: Matrix<T, ColRow, ColRow> {
    pure fn is_symmetric() -> bool;
}

/// A 2 x 2 square matrix
pub trait Matrix2x2<T, ColRow>: MatrixNxN<T, ColRow>,
                                Matrix2<T, ColRow, ColRow> {
//     /// Construct the matrix from a column major series of elements
//     static pure fn new(c0r0: T, c0r1: T,
//                        c1r0: T, c1r1: T) -> self;
}

/// A 3 x 3 square matrix
pub trait Matrix3x3<T, ColRow>: MatrixNxN<T, ColRow>,
                                Matrix3<T, ColRow, ColRow> {
//     /// Construct the matrix from a column major series of elements
//     static pure fn new(c0r0: T, c0r1: T, c0r2: T,
//                        c1r0: T, c1r1: T, c1r2: T,
//                        c2r0: T, c2r1: T, c2r2: T) -> self;
}

/// A 4 x 4 square matrix
pub trait Matrix4x4<T, ColRow>: MatrixNxN<T, ColRow>,
                                Matrix4<T, ColRow, ColRow> {
//     /// Construct the matrix from a column major series of elements
//     static pure fn new(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
//                        c1r0: T, c1r1: T, c1r2: T, c1r3: T,
//                        c2r0: T, c2r1: T, c2r2: T, c2r3: T,
//                        c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> self;
}

///
/// A matrix with numeric elements
///
pub trait NumericMatrix<T, Col, Row>: Matrix<T, Col, Row>, Neg<self> {
    static pure fn zero() -> self;
    
    pure fn mul_t(value: T) -> self;
    pure fn mul_v(other: &Col) -> Col;
    pure fn add_m(other: &self) -> self;
    pure fn sub_m(other: &self) -> self;
}

///
/// A square matrix with numeric elements
///
pub trait NumericMatrixNxN<T, ColRow>: MatrixNxN<T, ColRow>,
                                       NumericMatrix<T, ColRow, ColRow> {
    static pure fn identity() -> self;
    
    pure fn mul_m(other: &self) -> self;
    
    pure fn det() -> T;
    pure fn trace() -> T;
    
    pure fn invert() -> Option<self>;
    pure fn transpose() -> self;
    
    pure fn is_identity() -> bool;
    pure fn is_diagonal() -> bool;
    pure fn is_rotated() -> bool;
    pure fn is_invertible() -> bool;
}

/// A 2 x 2 square matrix with numeric elements
pub trait NumericMatrix2x2<T, ColRow2>: Matrix2x2<T, ColRow2>,
                                        NumericMatrixNxN<T, ColRow2> {
   // static pure fn from_value(value: T) -> self;
    
    pure fn to_Mat3() -> Mat3<T>;
    pure fn to_Mat4() -> Mat4<T>;
}

/// A 3 x 3 square matrix with numeric elements
pub trait NumericMatrix3x3<T, ColRow3>: Matrix3x3<T, ColRow3>,
                                        NumericMatrixNxN<T, ColRow3> {
   // static pure fn from_value(value: T) -> self;
    
    pure fn to_Mat4() -> Mat4<T>;
}

/// A 4 x 4 square matrix with numeric elements
pub trait NumericMatrix4x4<T, ColRow4>: Matrix4x4<T, ColRow4>,
                                        NumericMatrixNxN<T, ColRow4> {
   // static pure fn from_value(value: T) -> self;
}






//
//  Mat2: A 2x2, column major matrix
//
pub struct Mat2<T> { x: Vec2<T>, y: Vec2<T> }

pub impl<T:Copy NumCast> Mat2<T> {
    #[inline(always)]
    static pure fn new(c0r0: T, c0r1: T,
                       c1r0: T, c1r1: T) -> Mat2<T> {
        Mat2::from_cols(Vec2::new(move c0r0, move c0r1),
                        Vec2::new(move c1r0, move c1r1))
    }

    #[inline(always)]
    static pure fn from_cols(c0: Vec2<T>, c1: Vec2<T>) -> Mat2<T> {
        Mat2 { x: move c0,
               y: move c1 }
    }
    
    #[inline(always)]
    static pure fn from_value(value: T) -> Mat2<T> {
        let _0 = cast(0);
        Mat2::new(value,    _0,
                     _0, value)
    }
}

pub impl<T:Copy> Mat2<T>: Matrix<T, Vec2<T>, Vec2<T>> {
    #[inline(always)]
    static pure fn dim() -> uint { 2 }
    
    #[inline(always)]
    pure fn rows() -> uint { 2 }
    
    #[inline(always)]
    pure fn cols() -> uint { 2 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn is_square() -> bool { true }
    
    #[inline(always)]
    pure fn col(i: uint) -> Vec2<T> { self[i] }
    
    #[inline(always)]
    pure fn row(i: uint) -> Vec2<T> {
        Vec2::new(self[0][i],
                  self[1][i])
    }
    
    #[inline(always)]
    pure fn index(i: uint) -> Vec2<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat2<T>, *Vec2<T>>(
                to_unsafe_ptr(&self)), 2) |slice| { slice[i] }
        }
    }
    
    #[inline(always)]
    pure fn to_ptr() -> *T {
        self[0].to_ptr()
    }
}

pub impl<T:Copy DefaultEq> Mat2<T>: MatrixNxN<T, Vec2<T>> {
    #[inline(always)]
    pure fn is_symmetric() -> bool {
        self[0][1].default_eq(&self[1][0]) &&
        self[1][0].default_eq(&self[0][1])
    }
}

pub impl<T:Copy Num NumCast> Mat2<T>: NumericMatrix<T, Vec2<T>, Vec2<T>> {
    #[inline(always)]
    static pure fn zero() -> Mat2<T> {
        Mat2::from_cols(NumericVector::zero(),
                        NumericVector::zero())
    }
    
    #[inline(always)]
    pure fn neg() -> Mat2<T> {
        Mat2::from_cols(-self[0], -self[1])
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Mat2<T> {
        Mat2::from_cols(self[0].mul_t(value),
                        self[1].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self.row(0).dot(other),
                  self.row(1).dot(other))
    }
    
    #[inline(always)]
    pure fn add_m(other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self[0].add_v(&other[0]),
                        self[1].add_v(&other[1]))
    }
    
    #[inline(always)]
    pure fn sub_m(other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self[0].sub_v(&other[0]),
                        self[1].sub_v(&other[1]))
    }
}

pub impl<T:Copy Num NumCast DefaultEq> Mat2<T>: NumericMatrixNxN<T, Vec2<T>> {
    #[inline(always)]
    static pure fn identity() -> Mat2<T> {
        Mat2::new(NumCast::one() , NumCast::zero(),
                  NumCast::zero(), NumCast::one())
    }
    
    #[inline(always)]
    pure fn mul_m(other: &Mat2<T>) -> Mat2<T> {
        Mat2::new(self.row(0).dot(&other.col(0)), self.row(1).dot(&other.col(0)),
                  self.row(0).dot(&other.col(1)), self.row(1).dot(&other.col(1)))
    }
    
    pure fn det() -> T {
       self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    pure fn trace() -> T {
        self[0][0] + self[1][1]
    }

    #[inline(always)]
    pure fn invert() -> Option<Mat2<T>> {
        let _0 = cast(0);
        let d = self.det();
        if d.default_eq(&_0) {
            None
        } else {
            Some(Mat2::new( self[1][1]/d, -self[0][1]/d,
                           -self[1][0]/d,  self[0][0]/d))
        }
    }
    
    #[inline(always)]
    pure fn transpose() -> Mat2<T> {
        Mat2::new(self[0][0], self[1][0],
                  self[0][1], self[1][1])
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.default_eq(&NumericMatrixNxN::identity())
    }
    
    #[inline(always)]
    pure fn is_diagonal() -> bool {
        let _0 = cast(0);
        self[0][1].default_eq(&_0) &&
        self[1][0].default_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.default_eq(&NumericMatrixNxN::identity())
    }

    #[inline(always)]
    pure fn is_invertible() -> bool {
        let _0 = cast(0);
        !self.det().default_eq(&_0)
    }
}

pub impl<T:Copy NumCast> Mat2<T>: NumericMatrix2x2<T, Vec2<T>> {
    #[inline(always)]
    pure fn to_Mat3() -> Mat3<T> {
        Mat3::from_Mat2(&self)
    }
    
    #[inline(always)]
    pure fn to_Mat4() -> Mat4<T> {
        Mat4::from_Mat2(&self)
    }
}

pub impl<T:Copy DefaultEq> Mat2<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Mat2<T>) -> bool {
        self.default_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Mat2<T>) -> bool {
        !(self == *other)
    }
}

pub impl<T:Copy FuzzyEq> Mat2<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat2<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1])
    }
}

pub impl<T:Copy DefaultEq> Mat2<T>: DefaultEq {
    #[inline(always)]
    pure fn default_eq(other: &Mat2<T>) -> bool {
        self[0].default_eq(&other[0]) &&
        self[1].default_eq(&other[1])
    }
}






//
//  Mat3: A 3x3, column major matrix
//
pub struct Mat3<T> { x: Vec3<T>, y: Vec3<T>, z: Vec3<T> }

pub impl<T:Copy NumCast> Mat3<T> {
    #[inline(always)]
    static pure fn new(c0r0:T, c0r1:T, c0r2:T,
                       c1r0:T, c1r1:T, c1r2:T,
                       c2r0:T, c2r1:T, c2r2:T) -> Mat3<T> {
        Mat3::from_cols(Vec3::new(move c0r0, move c0r1, move c0r2),
                        Vec3::new(move c1r0, move c1r1, move c1r2),
                        Vec3::new(move c2r0, move c2r1, move c2r2))
    }
    
    #[inline(always)]
    static pure fn from_cols(c0: Vec3<T>, c1: Vec3<T>, c2: Vec3<T>) -> Mat3<T> {
        Mat3 { x: move c0,
               y: move c1,
               z: move c2 }
    }
    
    #[inline(always)]
    static pure fn from_value(value: T) -> Mat3<T> {
        let _0 = cast(0);
        Mat3::new(value,    _0,    _0,
                     _0, value,    _0,
                     _0,    _0, value)
    }
    
    #[inline(always)]
    static pure fn from_Mat2(m: &Mat2<T>) -> Mat3<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat3::new(m[0][0], m[0][1], _0,
                  m[1][0], m[1][1], _0,
                       _0,      _0, _1)
    }
}

pub impl<T:Copy> Mat3<T>: Matrix<T, Vec3<T>, Vec3<T>> {
    #[inline(always)]
    static pure fn dim() -> uint { 3 }
    
    #[inline(always)]
    pure fn rows() -> uint { 3 }
    
    #[inline(always)]
    pure fn cols() -> uint { 3 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn is_square() -> bool { true }
    
    #[inline(always)]
    pure fn col(i: uint) -> Vec3<T> { self[i] }
    
    #[inline(always)]
    pure fn row(i: uint) -> Vec3<T> {
        Vec3::new(self[0][i],
                  self[1][i],
                  self[2][i])
    }
    
    #[inline(always)]
    pure fn index(i: uint) -> Vec3<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat3<T>, *Vec3<T>>(
                to_unsafe_ptr(&self)), 3) |slice| { slice[i] }
        }
    }
    
    #[inline(always)]
    pure fn to_ptr() -> *T {
        self[0].to_ptr()
    }
}

pub impl<T:Copy DefaultEq> Mat3<T>: MatrixNxN<T, Vec3<T>> {
    #[inline(always)]
    pure fn is_symmetric() -> bool {
        self[0][1].default_eq(&self[1][0]) &&
        self[0][2].default_eq(&self[2][0]) &&
        
        self[1][0].default_eq(&self[0][1]) &&
        self[1][2].default_eq(&self[2][1]) &&
        
        self[2][0].default_eq(&self[0][2]) &&
        self[2][1].default_eq(&self[1][2])
    }
}

pub impl<T:Copy Num NumCast> Mat3<T>: NumericMatrix<T, Vec3<T>, Vec3<T>> {
    #[inline(always)]
    static pure fn zero() -> Mat3<T> {
        Mat3::from_cols(NumericVector::zero(),
                        NumericVector::zero(),
                        NumericVector::zero())
    }
    
    #[inline(always)]
    pure fn neg() -> Mat3<T> {
        Mat3::from_cols(-self[0], -self[1], -self[2])
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Mat3<T> {
        Mat3::from_cols(self[0].mul_t(value),
                        self[1].mul_t(value),
                        self[2].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.row(0).dot(other),
                  self.row(1).dot(other),
                  self.row(2).dot(other))
    }
    
    #[inline(always)]
    pure fn add_m(other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self[0].add_v(&other[0]),
                        self[1].add_v(&other[1]),
                        self[2].add_v(&other[2]))
    }
    
    #[inline(always)]
    pure fn sub_m(other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self[0].sub_v(&other[0]),
                        self[1].sub_v(&other[1]),
                        self[2].sub_v(&other[2]))
    }
}

pub impl<T:Copy Num NumCast DefaultEq> Mat3<T>: NumericMatrixNxN<T, Vec3<T>> {
    #[inline(always)]
    static pure fn identity() -> Mat3<T> {
        Mat3::new(NumCast::one() , NumCast::zero(), NumCast::zero(),
                  NumCast::zero(), NumCast::one() , NumCast::zero(),
                  NumCast::zero(), NumCast::zero(), NumCast::one())
    }
    
    #[inline(always)]
    pure fn mul_m(other: &Mat3<T>) -> Mat3<T> {
        Mat3::new(self.row(0).dot(&other.col(0)), self.row(1).dot(&other.col(0)), self.row(2).dot(&other.col(0)),
                  self.row(0).dot(&other.col(1)), self.row(1).dot(&other.col(1)), self.row(2).dot(&other.col(1)),
                  self.row(0).dot(&other.col(2)), self.row(1).dot(&other.col(2)), self.row(2).dot(&other.col(2)))
    }
    
    pure fn det() -> T {
        self.col(0).dot(&self.col(1).cross(&self.col(2)))
    }

    pure fn trace() -> T {
        self[0][0] + self[1][1] + self[2][2]
    }

    // #[inline(always)]
    pure fn invert() -> Option<Mat3<T>> {
        let d = self.det();
        let _0 = cast(0);
        if d.default_eq(&_0) {
            None
        } else {
            Some(Mat3::from_cols(self[1].cross(&self[2]).div_t(d),
                                 self[2].cross(&self[0]).div_t(d),
                                 self[0].cross(&self[1]).div_t(d))
            .transpose())
        }
    }
    
    #[inline(always)]
    pure fn transpose() -> Mat3<T> {
        Mat3::new(self[0][0], self[1][0], self[2][0],
                  self[0][1], self[1][1], self[2][1],
                  self[0][2], self[1][2], self[2][2])
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.default_eq(&NumericMatrixNxN::identity())
    }
    
    #[inline(always)]
    pure fn is_diagonal() -> bool {
        let _0 = cast(0);
        self[0][1].default_eq(&_0) &&
        self[0][2].default_eq(&_0) &&
        
        self[1][0].default_eq(&_0) &&
        self[1][2].default_eq(&_0) &&
        
        self[2][0].default_eq(&_0) &&
        self[2][1].default_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.default_eq(&NumericMatrixNxN::identity())
    }

    #[inline(always)]
    pure fn is_invertible() -> bool {
        let _0 = cast(0);
        !self.det().default_eq(&_0)
    }
}

pub impl<T:Copy NumCast> Mat3<T>: NumericMatrix3x3<T, Vec3<T>> {
    #[inline(always)]
    pure fn to_Mat4() -> Mat4<T> {
        Mat4::from_Mat3(&self)
    }
}

pub impl<T:Copy Num NumCast Ord DefaultEq> Mat3<T>: ToQuat<T> {
    pure fn to_Quat() -> Quat<T> {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf
        
        let mut s: float;
        let w: float, x: float, y: float, z: float;
        let trace: float = cast(self.trace());
        
        if trace >= cast(0) {
            s = (trace + 1f).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = (self[1][2] - self[2][1]).cast::<float>() * s;
            y = (self[2][0] - self[0][2]).cast::<float>() * s;
            z = (self[0][1] - self[1][0]).cast::<float>() * s;
        } else if (self[0][0] > self[1][1]) && (self[0][0] > self[2][2]) {
            s = (1f + (self[0][0] - self[1][1] - self[2][2]).cast::<float>()).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = (self[0][1] - self[1][0]).cast::<float>() * s;
            y = (self[2][0] - self[0][2]).cast::<float>() * s;
            z = (self[1][2] - self[2][1]).cast::<float>() * s;
        } else if self[1][1] > self[2][2] {
            s = (1f + (self[1][1] - self[0][0] - self[2][2]).cast::<float>()).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = (self[0][1] - self[1][0]).cast::<float>() * s;
            y = (self[1][2] - self[2][1]).cast::<float>() * s;
            z = (self[2][0] - self[0][2]).cast::<float>() * s;
        } else {
            s = (1f + (self[2][2] - self[0][0] - self[1][1]).cast::<float>()).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = (self[2][0] - self[0][2]).cast::<float>() * s;
            y = (self[1][2] - self[2][1]).cast::<float>() * s;
            z = (self[0][1] - self[1][0]).cast::<float>() * s;
        }
        
        Quat::new(cast(w), cast(x), cast(y), cast(z))
    }
}

pub impl<T:Copy DefaultEq> Mat3<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Mat3<T>) -> bool {
        self.default_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Mat3<T>) -> bool {
        !(self == *other)
    }
}

pub impl<T:Copy FuzzyEq> Mat3<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat3<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2])
    }
}

pub impl<T:Copy DefaultEq> Mat3<T>: DefaultEq {
    #[inline(always)]
    pure fn default_eq(other: &Mat3<T>) -> bool {
        self[0].default_eq(&other[0]) &&
        self[1].default_eq(&other[1]) &&
        self[2].default_eq(&other[2])
    }
}






//
//  Mat4: A 4x4, column major matrix
//
pub struct Mat4<T> { x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T> }

pub impl<T:Copy NumCast> Mat4<T> {
    #[inline(always)]
    static pure fn new(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
                       c1r0: T, c1r1: T, c1r2: T, c1r3: T,
                       c2r0: T, c2r1: T, c2r2: T, c2r3: T,
                       c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> Mat4<T>  {
        Mat4::from_cols(Vec4::new(move c0r0, move c0r1, move c0r2, move c0r3),
                        Vec4::new(move c1r0, move c1r1, move c1r2, move c1r3),
                        Vec4::new(move c2r0, move c2r1, move c2r2, move c2r3),
                        Vec4::new(move c3r0, move c3r1, move c3r2, move c3r3))
    }
    
    #[inline(always)]
    static pure fn from_cols(c0: Vec4<T>, c1: Vec4<T>, c2: Vec4<T>, c3: Vec4<T>) -> Mat4<T> {
        Mat4 { x: move c0,
               y: move c1,
               z: move c2,
               w: move c3 }
    }
    
    #[inline(always)]
    static pure fn from_value(value: T) -> Mat4<T> {
        let _0 = cast(0);
        Mat4::new(value,    _0,    _0,    _0,
                     _0, value,    _0,    _0,
                     _0,    _0, value,    _0,
                     _0,    _0,    _0, value)
    }
    
    #[inline(always)]
    static pure fn from_Mat2(m: &Mat2<T>) -> Mat4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat4::new(m[0][0], m[0][1], _0, _0,
                  m[1][0], m[1][1], _0, _0,
                       _0,      _0, _1, _0,
                       _0,      _0, _0, _1)
    }
    
    #[inline(always)]
    static pure fn from_Mat3(m: &Mat3<T>) -> Mat4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat4::new(m[0][0], m[0][1], m[0][2], _0,
                  m[1][0], m[1][1], m[1][2], _0,
                  m[2][0], m[2][1], m[2][2], _0,
                       _0,      _0,      _0, _1)
    }
}

pub impl<T:Copy> Mat4<T>: Matrix<T, Vec4<T>, Vec4<T>> {
    #[inline(always)]
    static pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    pure fn rows() -> uint { 4 }
    
    #[inline(always)]
    pure fn cols() -> uint { 4 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn is_square() -> bool { true }
    
    #[inline(always)]
    pure fn col(i: uint) -> Vec4<T> { self[i] }
    
    #[inline(always)]
    pure fn row(i: uint) -> Vec4<T> {
        Vec4::new(self[0][i],
                  self[1][i],
                  self[2][i],
                  self[3][i])
    }
    
    #[inline(always)]
    pure fn index(i: uint) -> Vec4<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat4<T>, *Vec4<T>>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
    
    #[inline(always)]
    pure fn to_ptr() -> *T {
        self[0].to_ptr()
    }
}

pub impl<T:Copy DefaultEq> Mat4<T>: MatrixNxN<T, Vec4<T>> {
    #[inline(always)]
    pure fn is_symmetric() -> bool {
        self[0][1].default_eq(&self[1][0]) &&
        self[0][2].default_eq(&self[2][0]) &&
        self[0][3].default_eq(&self[3][0]) &&
        
        self[1][0].default_eq(&self[0][1]) &&
        self[1][2].default_eq(&self[2][1]) &&
        self[1][3].default_eq(&self[3][1]) &&
        
        self[2][0].default_eq(&self[0][2]) &&
        self[2][1].default_eq(&self[1][2]) &&
        self[2][3].default_eq(&self[3][2]) &&
        
        self[3][0].default_eq(&self[0][3]) &&
        self[3][1].default_eq(&self[1][3]) &&
        self[3][2].default_eq(&self[2][3])
    }
}

pub impl<T:Copy Num NumCast> Mat4<T>: NumericMatrix<T, Vec4<T>, Vec4<T>> {
    #[inline(always)]
    static pure fn zero() -> Mat4<T> {
        Mat4::from_cols(NumericVector::zero(),
                        NumericVector::zero(),
                        NumericVector::zero(),
                        NumericVector::zero())
    }
    
    #[inline(always)]
    pure fn neg() -> Mat4<T> {
        Mat4::from_cols(-self[0], -self[1], -self[2], -self[3])
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Mat4<T> {
        Mat4::from_cols(self[0].mul_t(value),
                        self[1].mul_t(value),
                        self[2].mul_t(value),
                        self[3].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self.row(0).dot(other),
                  self.row(1).dot(other),
                  self.row(2).dot(other),
                  self.row(3).dot(other))
    }
    
    #[inline(always)]
    pure fn add_m(other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self[0].add_v(&other[0]),
                        self[1].add_v(&other[1]),
                        self[2].add_v(&other[2]),
                        self[3].add_v(&other[3]))
    }
    
    #[inline(always)]
    pure fn sub_m(other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self[0].sub_v(&other[0]),
                        self[1].sub_v(&other[1]),
                        self[2].sub_v(&other[2]),
                        self[3].sub_v(&other[3]))
    }
}

pub impl<T:Copy Num NumCast DefaultEq Signed Ord> Mat4<T>: NumericMatrixNxN<T, Vec4<T>> {
    #[inline(always)]
    static pure fn identity() -> Mat4<T> {
        Mat4::new(NumCast::one() , NumCast::zero(), NumCast::zero(), NumCast::zero(),
                  NumCast::zero(), NumCast::one() , NumCast::zero(), NumCast::zero(),
                  NumCast::zero(), NumCast::zero(), NumCast::one() , NumCast::zero(),
                  NumCast::zero(), NumCast::zero(), NumCast::zero(), NumCast::one())
    }
    
    #[inline(always)]
    pure fn mul_m(other: &Mat4<T>) -> Mat4<T> {
        // Surprisingly when building with optimisation turned on this is actually
        // faster than writing out the matrix multiplication in expanded form.
        // If you don't believe me, see ./test/performance/matrix_mul.rs
        Mat4::new(self.row(0).dot(&other.col(0)), self.row(1).dot(&other.col(0)), self.row(2).dot(&other.col(0)), self.row(3).dot(&other.col(0)),
                  self.row(0).dot(&other.col(1)), self.row(1).dot(&other.col(1)), self.row(2).dot(&other.col(1)), self.row(3).dot(&other.col(1)),
                  self.row(0).dot(&other.col(2)), self.row(1).dot(&other.col(2)), self.row(2).dot(&other.col(2)), self.row(3).dot(&other.col(2)),
                  self.row(0).dot(&other.col(3)), self.row(1).dot(&other.col(3)), self.row(2).dot(&other.col(3)), self.row(3).dot(&other.col(3)))
    }
    
    pure fn det() -> T {
        self[0][0]*Mat3::new(self[1][1], self[2][1], self[3][1],
                             self[1][2], self[2][2], self[3][2],
                             self[1][3], self[2][3], self[3][3]).det() -
        self[1][0]*Mat3::new(self[0][1], self[2][1], self[3][1],
                             self[0][2], self[2][2], self[3][2],
                             self[0][3], self[2][3], self[3][3]).det() +
        self[2][0]*Mat3::new(self[0][1], self[1][1], self[3][1],
                             self[0][2], self[1][2], self[3][2],
                             self[0][3], self[1][3], self[3][3]).det() -
        self[3][0]*Mat3::new(self[0][1], self[1][1], self[2][1],
                             self[0][2], self[1][2], self[2][2],
                             self[0][3], self[1][3], self[2][3]).det()
    }

    pure fn trace() -> T {
        self[0][0] + self[1][1] + self[2][2] + self[3][3]
    }

    pure fn invert() -> Option<Mat4<T>> {
        let d = self.det();
        let _0 = cast(0);
        if d.default_eq(&_0) {
            None
        } else {

            // Gauss Jordan Elimination with partial pivoting

            let mut a = copy self;
            let mut inv: Mat4<T> = NumericMatrixNxN::identity();

            // Find largest pivot column j among rows j..3
            for uint::range(0, 4) |j| {
                let mut i1 = j;
                for uint::range(j + 1, 4) |i| {
                    if abs(&a[i][j]) > abs(&a[i1][j]) {
                        i1 = i;
                    }
                }

                // Swap rows i1 and j in a and inv to
                // put pivot on diagonal
                let c = [mut a.x, a.y, a.z, a.w];
                c[i1] <-> c[j];
                a = Mat4::from_cols(c[0], c[1], c[2], c[3]);
                let c = [mut inv.x, inv.y, inv.z, inv.w];
                c[i1] <-> c[j];
                inv = Mat4::from_cols(c[0], c[1], c[2], c[3]);

                // Scale row j to have a unit diagonal
                let c = [mut inv.x, inv.y, inv.z, inv.w];
                c[j] = c[j].div_t(a[j][j]);
                inv = Mat4::from_cols(c[0], c[1], c[2], c[3]);
                let c = [mut a.x, a.y, a.z, a.w];
                c[j] = c[j].div_t(a[j][j]);
                a = Mat4::from_cols(c[0], c[1], c[2], c[3]);

                // Eliminate off-diagonal elems in col j of a,
                // doing identical ops to inv
                for uint::range(0, 4) |i| {
                    if i != j {
                        let c = [mut inv.x, inv.y, inv.z, inv.w];
                        c[i] = c[i].sub_v(&c[j].mul_t(a[i][j]));
                        inv = Mat4::from_cols(c[0], c[1], c[2], c[3]);

                        let c = [mut a.x, a.y, a.z, a.w];
                        c[i] = c[i].sub_v(&c[j].mul_t(a[i][j]));
                        a = Mat4::from_cols(c[0], c[1], c[2], c[3]); 
                    }
                }
            }
            Some(inv)
        }
    }
    
    #[inline(always)]
    pure fn transpose() -> Mat4<T> {
        Mat4::new(self[0][0], self[1][0], self[2][0], self[3][0],
                  self[0][1], self[1][1], self[2][1], self[3][1],
                  self[0][2], self[1][2], self[2][2], self[3][2],
                  self[0][3], self[1][3], self[2][3], self[3][3])
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.default_eq(&NumericMatrixNxN::identity())
    }
    
    #[inline(always)]
    pure fn is_diagonal() -> bool {
        let _0 = cast(0);
        self[0][1].default_eq(&_0) &&
        self[0][2].default_eq(&_0) &&
        self[0][3].default_eq(&_0) &&
        
        self[1][0].default_eq(&_0) &&
        self[1][2].default_eq(&_0) &&
        self[1][3].default_eq(&_0) &&
        
        self[2][0].default_eq(&_0) &&
        self[2][1].default_eq(&_0) &&
        self[2][3].default_eq(&_0) &&
        
        self[3][0].default_eq(&_0) &&
        self[3][1].default_eq(&_0) &&
        self[3][2].default_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.default_eq(&NumericMatrixNxN::identity())
    }

    #[inline(always)]
    pure fn is_invertible() -> bool {
        let _0 = cast(0);
        !self.det().default_eq(&_0)
    }
}

pub impl<T> Mat4<T>: NumericMatrix4x4<T, Vec4<T>> {
    
}

pub impl<T:Copy DefaultEq> Mat4<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Mat4<T>) -> bool {
        self.default_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Mat4<T>) -> bool {
        !(self == *other)
    }
}

pub impl<T:Copy FuzzyEq> Mat4<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat4<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
}

pub impl<T:Copy DefaultEq> Mat4<T>: DefaultEq {
    #[inline(always)]
    pure fn default_eq(other: &Mat4<T>) -> bool {
        self[0].default_eq(&other[0]) &&
        self[1].default_eq(&other[1]) &&
        self[2].default_eq(&other[2]) &&
        self[3].default_eq(&other[3])
    }
}
