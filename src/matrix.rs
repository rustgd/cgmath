use cast::transmute;
use cmp::Eq;
use num::from_int;
use ptr::to_unsafe_ptr;
use vec::raw::buf_as_slice;
use std::cmp::FuzzyEq;

use funs::exp::Exp;
use math::*;
use ncast::*;
use quaternion::{Quat, ToQuat};
use vector::{Vec2, Vec3, Vec4};


// GLSL equivalent type aliases

pub type mat2 = Mat2<f32>;              /// a 2×2 single-precision floating-point matrix
pub type mat3 = Mat3<f32>;              /// a 3×3 single-precision floating-point matrix
pub type mat4 = Mat4<f32>;              /// a 4×4 single-precision floating-point matrix
pub type mat2x2 = Mat2<f32>;            /// same as a `mat2`
// pub type mat2x3 =                    /// a single-precision floating-point matrix with 2 columns and 3 rows
// pub type mat2x4 =                    /// a single-precision floating-point matrix with 2 columns and 4 rows
// pub type mat3x2 =                    /// a single-precision floating-point matrix with 3 columns and 2 rows
pub type mat3x3 = Mat3<f32>;            /// same as a `mat3`
// pub type mat3x4 =                    /// a single-precision floating-point matrix with 3 columns and 4 rows
// pub type mat4x2 =                    /// a single-precision floating-point matrix with 4 columns and 2 rows
// pub type mat4x3 =                    /// a single-precision floating-point matrix with 4 columns and 3 rows
pub type mat4x4 = Mat4<f32>;            /// same as a `mat4`

pub type dmat2 = Mat2<f64>;             /// a 2×2 double-precision floating-point matrix
pub type dmat3 = Mat3<f64>;             /// a 3×3 double-precision floating-point matrix
pub type dmat4 = Mat4<f64>;             /// a 4×4 double-precision floating-point matrix
pub type dmat2x2 = Mat2<f64>;           /// same as a `dmat2`
// pub type dmat2x3 =                   /// a double-precision floating-point matrix with 2 columns and 3 rows
// pub type dmat2x4 =                   /// a double-precision floating-point matrix with 2 columns and 4 rows
// pub type dmat3x2 =                   /// a double-precision floating-point matrix with 3 columns and 2 rows
pub type dmat3x3 = Mat3<f64>;           /// same as a `dmat3`
// pub type dmat3x4 =                   /// a double-precision floating-point matrix with 3 columns and 4 rows
// pub type dmat4x2 =                   /// a double-precision floating-point matrix with 4 columns and 2 rows
// pub type dmat4x3 =                   /// a double-precision floating-point matrix with 4 columns and 3 rows
pub type dmat4x4 = Mat4<f64>;           /// same as a `dmat4`


pub trait Matrix<T, ColV, RowV> {
    pure fn rows() -> uint;
    pure fn cols() -> uint;
    pure fn is_col_major() -> bool;
    pure fn is_square() -> bool;
    
    pure fn col(i: uint) -> ColV;
    pure fn row(i: uint) -> RowV;
    
    pure fn mul_t(value: T) -> self;
    pure fn mul_v(other: &ColV) -> ColV;
}

pub trait SquareMatrix<T> {
    pure fn add_m(other: &self) -> self;
    pure fn sub_m(other: &self) -> self;
    pure fn mul_m(other: &self) -> self;
    
    // pure fn invert(other: &self) -> self;
    pure fn transpose() -> self;
    
    pure fn is_identity() -> bool;
    pure fn is_symmetric() -> bool;
    pure fn is_diagonal() -> bool;
    pure fn is_rotated() -> bool;
}

//
//  2x2 Matrix
//
pub trait Matrix2<T> {
    pure fn to_Mat3() -> Mat3<T>;
    pure fn to_Mat4() -> Mat4<T>;
}

//
//  3x3 Matrix
//
pub trait Matrix3<T> {
    pure fn to_Mat4() -> Mat4<T>;
}

//
//  4x4 Matrix
//
pub trait Matrix4<T> {
    
}






//
//  Mat2: A 2x2, column major matrix
//
pub struct Mat2<T> { x: Vec2<T>, y: Vec2<T> }

pub mod Mat2 {
    
    #[inline(always)]
    pub pure fn new<T>(c0r0: T, c0r1: T,
                       c1r0: T, c1r1: T) -> Mat2<T> {
        Mat2::from_cols(Vec2::new(move c0r0, move c0r1),
                        Vec2::new(move c1r0, move c1r1))
    }

    #[inline(always)]
    pub pure fn from_cols<T>(c0: Vec2<T>, c1: Vec2<T>) -> Mat2<T> {
        Mat2 { x: move c0,
               y: move c1 }
    }
    
    #[inline(always)]
    pub pure fn from_value<T:Copy NumCast>(value: T) -> Mat2<T> {
        let _0 = cast(0);
        Mat2::new(value,    _0,
                     _0, value)
    }
    
    #[inline(always)]
    pub pure fn zero<T:Copy NumCast>() -> Mat2<T> {
        let _0 = cast(0);
        Mat2::new(_0, _0,
                  _0, _0)
    }
    
    #[inline(always)]
    pub pure fn identity<T:Copy NumCast>() -> Mat2<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat2::new(_1, _0,
                  _0, _1)
    }
}

pub impl<T:Copy Num NumCast> Mat2<T>: Matrix<T, Vec2<T>, Vec2<T>> {
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
    pure fn mul_t(value: T) -> Mat2<T> {
        Mat2::from_cols(self[0].mul_t(value),
                        self[1].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self.row(0).dot(other),
                  self.row(1).dot(other))
    }
}

pub impl<T:Copy Num NumCast FuzzyEq> Mat2<T>: SquareMatrix<T> {
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
    
    #[inline(always)]
    pure fn mul_m(other: &Mat2<T>) -> Mat2<T> {
        Mat2::new(self[0][0] * other[0][0] + self[1][0] * other[0][1],
                  self[0][1] * other[0][0] + self[1][1] * other[0][1],
                  
                  self[0][0] * other[1][0] + self[1][0] * other[1][1],
                  self[0][1] * other[1][0] + self[1][1] * other[1][1])
    }
    
    // TODO - inversion is harrrd D:
    // #[inline(always)]
    // pure fn invert(other: &Mat2<T>) -> Mat2<T> {}
    
    #[inline(always)]
    pure fn transpose() -> Mat2<T> {
        Mat2::new(self[0][0], self[1][0],
                  self[0][1], self[1][1])
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(&Mat2::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric() -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[1][0].fuzzy_eq(&self[0][1])
    }
    
    #[inline(always)]
    pure fn is_diagonal() -> bool {
        let _0 = cast(0);
        self[0][1].fuzzy_eq(&_0) &&
        self[1][0].fuzzy_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat2::identity())
    }
}

pub impl<T:Copy Num NumCast FuzzyEq> Mat2<T>: Matrix2<T> {
    #[inline(always)]
    pure fn to_Mat3() -> Mat3<T> {
        Mat3::from_Mat2(&self)
    }
    
    #[inline(always)]
    pure fn to_Mat4() -> Mat4<T> {
        Mat4::from_Mat2(&self)
    }
}

pub impl<T:Copy> Mat2<T>: Index<uint, Vec2<T>> {
    #[inline(always)]
    pure fn index(i: uint) -> Vec2<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat2<T>, *Vec2<T>>(
                to_unsafe_ptr(&self)), 2) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy Neg<T>> Mat2<T>: Neg<Mat2<T>> {
    #[inline(always)]
    pure fn neg() -> Mat2<T> {
        Mat2::from_cols(-self[0], -self[1])
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy FuzzyEq> Mat2<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Mat2<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Mat2<T>) -> bool {
        !(self == *other)
    }
}

impl<T:Copy Eq> Mat2<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Mat2<T>) -> bool {
        self[0].exact_eq(&other[0]) &&
        self[1].exact_eq(&other[1])
    }
}

pub impl<T:Copy FuzzyEq> Mat2<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat2<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1])
    }
}






//
//  Mat3: A 3x3, column major matrix
//
pub struct Mat3<T> { x: Vec3<T>, y: Vec3<T>, z: Vec3<T> }

pub mod Mat3 {
    
    #[inline(always)]
    pub pure fn new<T>(c0r0:T, c0r1:T, c0r2:T,
                       c1r0:T, c1r1:T, c1r2:T,
                       c2r0:T, c2r1:T, c2r2:T) -> Mat3<T> {
        Mat3::from_cols(Vec3::new(move c0r0, move c0r1, move c0r2),
                        Vec3::new(move c1r0, move c1r1, move c1r2),
                        Vec3::new(move c2r0, move c2r1, move c2r2))
    }
    
    #[inline(always)]
    pub pure fn from_cols<T>(c0: Vec3<T>, c1: Vec3<T>, c2: Vec3<T>) -> Mat3<T> {
        Mat3 { x: move c0,
               y: move c1,
               z: move c2 }
    }
    
    #[inline(always)]
    pub pure fn from_value<T:Copy NumCast>(value: T) -> Mat3<T> {
        let _0 = cast(0);
        Mat3::new(value,    _0,    _0,
                     _0, value,    _0,
                     _0,    _0, value)
    }
    
    #[inline(always)]
    pub pure fn from_Mat2<T:Copy NumCast>(m: &Mat2<T>) -> Mat3<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat3::new(m[0][0], m[0][1], _0,
                  m[1][0], m[1][1], _0,
                       _0,      _0, _1)
    }
    
    #[inline(always)]
    pub pure fn zero<T:Copy NumCast>() -> Mat3<T> {
        let _0 = cast(0);
        Mat3::new(_0, _0, _0,
                  _0, _0, _0,
                  _0, _0, _0)
    }
    
    #[inline(always)]
    pub pure fn identity<T:Copy NumCast>() -> Mat3<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat3::new(_1, _0, _0,
                  _0, _1, _0,
                  _0, _0, _1)
    }
}

pub impl<T:Copy Num NumCast> Mat3<T>: Matrix<T, Vec3<T>, Vec3<T>> {
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
}

pub impl<T:Copy Num NumCast FuzzyEq> Mat3<T>: SquareMatrix<T> {
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
    
    #[inline(always)]
    pure fn mul_m(other: &Mat3<T>) -> Mat3<T> {
        Mat3::new(self[0][0] * other[0][0] + self[1][0] * other[0][1] + self[2][0] * other[0][2],
                  self[0][1] * other[0][0] + self[1][1] * other[0][1] + self[2][1] * other[0][2],
                  self[0][2] * other[0][0] + self[1][2] * other[0][1] + self[2][2] * other[0][2],
                  
                  self[0][0] * other[1][0] + self[1][0] * other[1][1] + self[2][0] * other[1][2],
                  self[0][1] * other[1][0] + self[1][1] * other[1][1] + self[2][1] * other[1][2],
                  self[0][2] * other[1][0] + self[1][2] * other[1][1] + self[2][2] * other[1][2],
                  
                  self[0][0] * other[2][0] + self[1][0] * other[2][1] + self[2][0] * other[2][2],
                  self[0][1] * other[2][0] + self[1][1] * other[2][1] + self[2][1] * other[2][2],
                  self[0][2] * other[2][0] + self[1][2] * other[2][1] + self[2][2] * other[2][2])
    }
    
    // TODO - inversion is harrrd D:
    // #[inline(always)]
    // pure fn invert(other: &Mat3) -> Mat3 {}
    
    #[inline(always)]
    pure fn transpose() -> Mat3<T> {
        Mat3::new(self[0][0], self[1][0], self[2][0],
                  self[0][1], self[1][1], self[2][1],
                  self[0][2], self[1][2], self[2][2])
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(&Mat3::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric() -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[0][2].fuzzy_eq(&self[2][0]) &&
        
        self[1][0].fuzzy_eq(&self[0][1]) &&
        self[1][2].fuzzy_eq(&self[2][1]) &&
        
        self[2][0].fuzzy_eq(&self[0][2]) &&
        self[2][1].fuzzy_eq(&self[1][2])
    }
    
    #[inline(always)]
    pure fn is_diagonal() -> bool {
        let _0 = cast(0);
        self[0][1].fuzzy_eq(&_0) &&
        self[0][2].fuzzy_eq(&_0) &&
        
        self[1][0].fuzzy_eq(&_0) &&
        self[1][2].fuzzy_eq(&_0) &&
        
        self[2][0].fuzzy_eq(&_0) &&
        self[2][1].fuzzy_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat3::identity())
    }
}

pub impl<T:Copy Num NumCast FuzzyEq> Mat3<T>: Matrix3<T> {
    #[inline(always)]
    pure fn to_Mat4() -> Mat4<T> {
        Mat4::from_Mat3(&self)
    }
}

pub impl<T:Copy Num NumCast Ord> Mat3<T>: ToQuat<T> {
    pure fn to_Quat() -> Quat<T> {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf
        
        let mut s: float;
        let w: float, x: float, y: float, z: float;
        let trace: float = cast(self[0][0] + self[1][1] + self[2][2]);
        
        if trace >= from_int(0) {
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

pub impl<T:Copy> Mat3<T>: Index<uint, Vec3<T>> {
    #[inline(always)]
    pure fn index(i: uint) -> Vec3<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat3<T>, *Vec3<T>>(
                to_unsafe_ptr(&self)), 3) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy Neg<T>> Mat3<T>: Neg<Mat3<T>> {
    #[inline(always)]
    pure fn neg() -> Mat3<T> {
        Mat3::from_cols(-self[0], -self[1], -self[2])
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy FuzzyEq> Mat3<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Mat3<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Mat3<T>) -> bool {
        !(self == *other)
    }
}

pub impl<T:Copy Eq> Mat3<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Mat3<T>) -> bool {
        self[0].exact_eq(&other[0]) &&
        self[1].exact_eq(&other[1]) &&
        self[2].exact_eq(&other[2])
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

pub impl<T:Copy> Mat3<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr() -> *T {
        self[0].to_ptr()
    }
}






//
//  Mat4: A 4x4, column major matrix
//
pub struct Mat4<T> { x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T> }

pub mod Mat4 {
    
    #[inline(always)]
    pub pure fn new<T>(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
                       c1r0: T, c1r1: T, c1r2: T, c1r3: T,
                       c2r0: T, c2r1: T, c2r2: T, c2r3: T,
                       c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> Mat4<T>  {
        Mat4::from_cols(Vec4::new(move c0r0, move c0r1, move c0r2, move c0r3),
                        Vec4::new(move c1r0, move c1r1, move c1r2, move c1r3),
                        Vec4::new(move c2r0, move c2r1, move c2r2, move c2r3),
                        Vec4::new(move c3r0, move c3r1, move c3r2, move c3r3))
    }
    
    #[inline(always)]
    pub pure fn from_cols<T>(c0: Vec4<T>, c1: Vec4<T>, c2: Vec4<T>, c3: Vec4<T>) -> Mat4<T> {
        Mat4 { x: move c0,
               y: move c1,
               z: move c2,
               w: move c3 }
    }
    
    #[inline(always)]
    pub pure fn from_value<T:Copy NumCast>(value: T) -> Mat4<T> {
        let _0 = cast(0);
        Mat4::new(value,    _0,    _0,    _0,
                     _0, value,    _0,    _0,
                     _0,    _0, value,    _0,
                     _0,    _0,    _0, value)
    }
    
    #[inline(always)]
    pub pure fn from_Mat2<T:Copy NumCast>(m: &Mat2<T>) -> Mat4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat4::new(m[0][0], m[0][1], _0, _0,
                  m[1][0], m[1][1], _0, _0,
                       _0,      _0, _1, _0,
                       _0,      _0, _0, _1)
    }
    
    #[inline(always)]
    pub pure fn from_Mat3<T:Copy NumCast>(m: &Mat3<T>) -> Mat4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat4::new(m[0][0], m[0][1], m[0][2], _0,
                  m[1][0], m[1][1], m[1][2], _0,
                  m[2][0], m[2][1], m[2][2], _0,
                       _0,      _0,      _0, _1)
    }
    
    #[inline(always)]
    pub pure fn zero<T:Copy NumCast>() -> Mat4<T> {
        let _0 = cast(0);
        Mat4::new(_0, _0, _0, _0,
                  _0, _0, _0, _0,
                  _0, _0, _0, _0,
                  _0, _0, _0, _0)
    }
    
    #[inline(always)]
    pub pure fn identity<T:Copy NumCast>() -> Mat4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat4::new(_1, _0, _0, _0,
                  _0, _1, _0, _0,
                  _0, _0, _1, _0,
                  _0, _0, _0, _1)
    }
}

pub impl<T:Copy Num NumCast> Mat4<T>: Matrix<T, Vec4<T>, Vec4<T>> {
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
}

pub impl<T:Copy Num NumCast FuzzyEq> Mat4<T>: SquareMatrix<T> {
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
    
    #[inline(always)]
    pure fn mul_m(other: &Mat4<T>) -> Mat4<T> {
        Mat4::new(self[0][0] * other[0][0] + self[1][0] * other[0][1] + self[2][0] * other[0][2] + self[3][0] * other[0][3],
                  self[0][1] * other[0][0] + self[1][1] * other[0][1] + self[2][1] * other[0][2] + self[3][1] * other[0][3],
                  self[0][2] * other[0][0] + self[1][2] * other[0][1] + self[2][2] * other[0][2] + self[3][2] * other[0][3],
                  self[0][3] * other[0][0] + self[1][3] * other[0][1] + self[2][3] * other[0][2] + self[3][3] * other[0][3],
                  
                  self[0][0] * other[1][0] + self[1][0] * other[1][1] + self[2][0] * other[1][2] + self[3][0] * other[1][3],
                  self[0][1] * other[1][0] + self[1][1] * other[1][1] + self[2][1] * other[1][2] + self[3][1] * other[1][3],
                  self[0][2] * other[1][0] + self[1][2] * other[1][1] + self[2][2] * other[1][2] + self[3][2] * other[1][3],
                  self[0][3] * other[1][0] + self[1][3] * other[1][1] + self[2][3] * other[1][2] + self[3][3] * other[1][3],
                  
                  self[0][0] * other[2][0] + self[1][0] * other[2][1] + self[2][0] * other[2][2] + self[3][0] * other[2][3],
                  self[0][1] * other[2][0] + self[1][1] * other[2][1] + self[2][1] * other[2][2] + self[3][1] * other[2][3],
                  self[0][2] * other[2][0] + self[1][2] * other[2][1] + self[2][2] * other[2][2] + self[3][2] * other[2][3],
                  self[0][3] * other[2][0] + self[1][3] * other[2][1] + self[2][3] * other[2][2] + self[3][3] * other[2][3],
                  
                  self[0][0] * other[3][0] + self[1][0] * other[3][1] + self[2][0] * other[3][2] + self[3][0] * other[3][3],
                  self[0][1] * other[3][0] + self[1][1] * other[3][1] + self[2][1] * other[3][2] + self[3][1] * other[3][3],
                  self[0][2] * other[3][0] + self[1][2] * other[3][1] + self[2][2] * other[3][2] + self[3][2] * other[3][3],
                  self[0][3] * other[3][0] + self[1][3] * other[3][1] + self[2][3] * other[3][2] + self[3][3] * other[3][3])
    }
    
    // TODO - inversion is harrrd D:
    // #[inline(always)]
    // pure fn invert(other: &Mat4<T>) -> Mat4<T> {}
    
    #[inline(always)]
    pure fn transpose() -> Mat4<T> {
        Mat4::new(self[0][0], self[1][0], self[2][0], self[3][0],
                  self[0][1], self[1][1], self[2][1], self[3][1],
                  self[0][2], self[1][2], self[2][2], self[3][2],
                  self[0][3], self[1][3], self[2][3], self[3][3])
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(&Mat4::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric() -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[0][2].fuzzy_eq(&self[2][0]) &&
        self[0][3].fuzzy_eq(&self[3][0]) &&
        
        self[1][0].fuzzy_eq(&self[0][1]) &&
        self[1][2].fuzzy_eq(&self[2][1]) &&
        self[1][3].fuzzy_eq(&self[3][1]) &&
        
        self[2][0].fuzzy_eq(&self[0][2]) &&
        self[2][1].fuzzy_eq(&self[1][2]) &&
        self[2][3].fuzzy_eq(&self[3][2]) &&
        
        self[3][0].fuzzy_eq(&self[0][3]) &&
        self[3][1].fuzzy_eq(&self[1][3]) &&
        self[3][2].fuzzy_eq(&self[2][3])
    }
    
    #[inline(always)]
    pure fn is_diagonal() -> bool {
        let _0 = cast(0);
        self[0][1].fuzzy_eq(&_0) &&
        self[0][2].fuzzy_eq(&_0) &&
        self[0][3].fuzzy_eq(&_0) &&
        
        self[1][0].fuzzy_eq(&_0) &&
        self[1][2].fuzzy_eq(&_0) &&
        self[1][3].fuzzy_eq(&_0) &&
        
        self[2][0].fuzzy_eq(&_0) &&
        self[2][1].fuzzy_eq(&_0) &&
        self[2][3].fuzzy_eq(&_0) &&
        
        self[3][0].fuzzy_eq(&_0) &&
        self[3][1].fuzzy_eq(&_0) &&
        self[3][2].fuzzy_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat4::identity())
    }
}

pub impl<T> Mat4<T>: Matrix4<T> {
    
}

pub impl<T:Copy> Mat4<T>: Index<uint, Vec4<T>> {
    #[inline(always)]
    pure fn index(i: uint) -> Vec4<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat4<T>, *Vec4<T>>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy Neg<T>> Mat4<T>: Neg<Mat4<T>> {
    #[inline(always)]
    pure fn neg() -> Mat4<T> {
        Mat4::from_cols(-self[0], -self[1], -self[2], -self[3])
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy FuzzyEq> Mat4<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Mat4<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Mat4<T>) -> bool {
        !(self == *other)
    }
}

pub impl<T:Copy Eq> Mat4<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Mat4<T>) -> bool {
        self[0].exact_eq(&other[0]) &&
        self[1].exact_eq(&other[1]) &&
        self[2].exact_eq(&other[2]) &&
        self[3].exact_eq(&other[3])
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

pub impl<T:Copy> Mat4<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr() -> *T {
        self[0].to_ptr()
    }
}