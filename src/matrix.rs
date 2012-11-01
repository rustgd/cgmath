use std::cmp::FuzzyEq;
use cmp::Eq;
use num::from_int;

use math::*;
use num_util::*;
use quaternion::{Quat, ToQuat};
use vector::{Vec2, Vec3, Vec4};

//
//  NxN Matrix
//
pub trait Matrix<T, V> {
    pure fn rows() -> uint;
    pure fn cols() -> uint;
    pure fn is_col_major() -> bool;
    
    pure fn row(i: uint) -> V;
    pure fn col(i: uint) -> V;
    
    pure fn mul_t(value: T) -> self;
    pure fn mul_v(other: &V) -> V;
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
//  3x3 Matrix
//
pub trait Matrix3<T> {
    pure fn scale(vec: &Vec3<T>) -> self;
    pure fn to_Mat4() -> Mat4<T>;
}

//
//  4x4 Matrix
//
pub trait Matrix4<T> {
    pure fn scale(vec: &Vec3<T>) -> self;
    pure fn translate(vec: &Vec3<T>) -> self;
}






//
//  Mat2: A 2x2, column major matrix
//
pub struct Mat2<T> { data:[Vec2<T> * 2] }

pub mod Mat2 {
    
    #[inline(always)]
    pub pure fn new<T:Copy>(m00: T, m01: T,
                            m10: T, m11: T) -> Mat2<T> {
        Mat2::from_cols(&Vec2::new(m00, m01),
                        &Vec2::new(m10, m11))
    }

    #[inline(always)]
    pub pure fn from_cols<T:Copy>(col0: &Vec2<T>, col1: &Vec2<T>) -> Mat2<T> {
        Mat2 { data: [ *col0, *col1 ] }
    }
    
    #[inline(always)]
    pub pure fn zero<T:Copy Num>() -> Mat2<T> {
        Mat2 { data: [ Vec2::zero(),
                       Vec2::zero() ] }
    }
    
    #[inline(always)]
    pub pure fn identity<T:Copy Num>() -> Mat2<T> {
        Mat2 { data: [ Vec2::unit_x(),
                       Vec2::unit_y() ] }
    }
}

pub impl<T:Copy Num Sqrt FuzzyEq> Mat2<T>: Matrix<T, Vec2<T>> {
    #[inline(always)]
    pure fn rows() -> uint { 2 }
    
    #[inline(always)]
    pure fn cols() -> uint { 2 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn row(i: uint) -> Vec2<T> {
        Vec2::new(self[0][i],
                  self[1][i])
    }
    
    #[inline(always)]
    pure fn col(i: uint) -> Vec2<T> {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Mat2<T> {
        Mat2::from_cols(&self[0].mul_t(value),
                        &self[1].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0][0] * other[0] + self[1][0] * other[1],
                  self[0][1] * other[0] + self[1][1] * other[1])
    }
    
    #[inline(always)]
    pure fn add_m(other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(&self[0].add_v(&other[0]),
                        &self[1].add_v(&other[1]))
    }
    
    #[inline(always)]
    pure fn sub_m(other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(&self[0].sub_v(&other[0]),
                        &self[1].sub_v(&other[1]))
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
        self[0][1].fuzzy_eq(&from_int(0)) &&
        self[1][0].fuzzy_eq(&from_int(0))
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat2::identity())
    }
}

pub impl<T:Copy> Mat2<T>: Index<uint, Vec2<T>> {
    #[inline(always)]
    pure fn index(i: uint) -> Vec2<T> {
        self.data[i]
    }
}

pub impl<T:Copy Neg<T>> Mat2<T>: Neg<Mat2<T>> {
    #[inline(always)]
    pure fn neg() -> Mat2<T> {
        Mat2::from_cols(&-self[0], &-self[1])
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
pub struct Mat3<T> { data:[Vec3<T> * 3] }

pub mod Mat3 {
    
    #[inline(always)]
    pub pure fn new<T:Copy>(m00:T, m01:T, m02:T,
                            m10:T, m11:T, m12:T,
                            m20:T, m21:T, m22:T) -> Mat3<T> {
        Mat3::from_cols(&Vec3::new(m00, m01, m02),
                        &Vec3::new(m10, m11, m12),
                        &Vec3::new(m20, m21, m22))
    }
    
    #[inline(always)]
    pub pure fn from_cols<T:Copy>(col0: &Vec3<T>, col1: &Vec3<T>, col2: &Vec3<T>) -> Mat3<T> {
        Mat3 { data: [ *col0, *col1, *col2 ] }
    }
    
    #[inline(always)]
    pub pure fn zero<T:Num>() -> Mat3<T> {
        Mat3 { data: [ Vec3::zero(),
                       Vec3::zero(),
                       Vec3::zero() ] }
    }
    
    #[inline(always)]
    pub pure fn identity<T:Num>() -> Mat3<T> {
        Mat3 { data: [ Vec3::unit_x(),
                       Vec3::unit_y(),
                       Vec3::unit_z() ] }
    }
}

pub impl<T:Copy Num Sqrt FuzzyEq> Mat3<T>: Matrix<T, Vec3<T>> {
    #[inline(always)]
    pure fn rows() -> uint { 3 }
    
    #[inline(always)]
    pure fn cols() -> uint { 3 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn row(i: uint) -> Vec3<T> {
        Vec3::new(self[0][i],
                  self[1][i],
                  self[2][i])
    }
    
    #[inline(always)]
    pure fn col(i: uint) -> Vec3<T> {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Mat3<T> {
        Mat3::from_cols(&self[0].mul_t(value),
                        &self[1].mul_t(value),
                        &self[2].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0][0] * other[0] + self[1][0] * other[1] + self[2][0] * other[2],
                  self[0][1] * other[0] + self[1][1] * other[1] + self[2][1] * other[2],
                  self[0][2] * other[0] + self[1][2] * other[1] + self[2][2] * other[2])
    }
    
    #[inline(always)]
    pure fn add_m(other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(&self[0].add_v(&other[0]),
                        &self[1].add_v(&other[1]),
                        &self[2].add_v(&other[2]))
    }
    
    #[inline(always)]
    pure fn sub_m(other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(&self[0].sub_v(&other[0]),
                        &self[1].sub_v(&other[1]),
                        &self[2].sub_v(&other[2]))
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
        self[0][1].fuzzy_eq(&from_int(0)) &&
        self[0][2].fuzzy_eq(&from_int(0)) &&
        
        self[1][0].fuzzy_eq(&from_int(0)) &&
        self[1][2].fuzzy_eq(&from_int(0)) &&
        
        self[2][0].fuzzy_eq(&from_int(0)) &&
        self[2][1].fuzzy_eq(&from_int(0))
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat3::identity())
    }
}

pub impl<T:Copy Num Sqrt FuzzyEq> Mat3<T>: Matrix3<T> {
    #[inline(always)]
    pure fn scale(vec: &Vec3<T>) -> Mat3<T> {
        self.mul_m(&Mat3::new(      vec.x, from_int(0), from_int(0),
                              from_int(0),       vec.y, from_int(0),
                              from_int(0), from_int(0),      vec.z))
    }
    
    #[inline(always)]
    pure fn to_Mat4() -> Mat4<T> {
        Mat4::new( self[0][0],  self[0][1],   self[0][2], from_int(0),
                   self[1][0],  self[1][1],   self[1][2], from_int(0),
                   self[2][0],  self[2][1],   self[2][2], from_int(0),
                  from_int(0), from_int(0),  from_int(0), from_int(1))
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
        self.data[i]
    }
}

pub impl<T:Copy Neg<T>> Mat3<T>: Neg<Mat3<T>> {
    #[inline(always)]
    pure fn neg() -> Mat3<T> {
        Mat3::from_cols(&-self[0], &-self[1], &-self[2])
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
pub struct Mat4<T> { data:[Vec4<T> * 4] }

pub mod Mat4 {
    
    #[inline(always)]
    pub pure fn new<T:Copy>(m00: T, m01: T, m02: T, m03: T,
                            m10: T, m11: T, m12: T, m13: T,
                            m20: T, m21: T, m22: T, m23: T,
                            m30: T, m31: T, m32: T, m33: T) -> Mat4<T>  {
        Mat4::from_cols(&Vec4::new(m00, m01, m02, m03),
                        &Vec4::new(m10, m11, m12, m13),
                        &Vec4::new(m20, m21, m22, m23),
                        &Vec4::new(m30, m31, m32, m33))
    }
    
    #[inline(always)]
    pub pure fn from_cols<T:Copy>(col0: &Vec4<T>, col1: &Vec4<T>, col2: &Vec4<T>, col3: &Vec4<T>) -> Mat4<T> {
        Mat4 { data: [ *col0, *col1, *col2, *col3 ] }
    }
    
    #[inline(always)]
    pub pure fn zero<T:Num>() -> Mat4<T> {
        Mat4 { data: [ Vec4::zero(),
                       Vec4::zero(),
                       Vec4::zero(),
                       Vec4::zero() ] }
    }
    
    #[inline(always)]
    pub pure fn identity<T:Num>() -> Mat4<T> {
        Mat4 { data: [ Vec4::unit_x(),
                       Vec4::unit_y(),
                       Vec4::unit_z(),
                       Vec4::unit_w() ] }
    }
}

pub impl<T:Copy Num Sqrt FuzzyEq> Mat4<T>: Matrix<T, Vec4<T>> {
    #[inline(always)]
    pure fn rows() -> uint { 4 }
    
    #[inline(always)]
    pure fn cols() -> uint { 4 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn row(i: uint) -> Vec4<T> {
        Vec4::new(self[0][i],
                  self[1][i],
                  self[2][i],
                  self[3][i])
    }
    
    #[inline(always)]
    pure fn col(i: uint) -> Vec4<T> {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Mat4<T> {
        Mat4::from_cols(&self[0].mul_t(value),
                        &self[1].mul_t(value),
                        &self[2].mul_t(value),
                        &self[3].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0][0] * other[0] + self[1][0] * other[1] + self[2][0] * other[2] + self[3][0] * other[3],
                  self[0][1] * other[0] + self[1][1] * other[1] + self[2][1] * other[2] + self[3][1] * other[3],
                  self[0][2] * other[0] + self[1][2] * other[1] + self[2][2] * other[2] + self[3][2] * other[3],
                  self[0][3] * other[0] + self[1][3] * other[1] + self[2][3] * other[2] + self[3][3] * other[3])
    }
    
    #[inline(always)]
    pure fn add_m(other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(&self[0].add_v(&other[0]),
                        &self[1].add_v(&other[1]),
                        &self[2].add_v(&other[2]),
                        &self[3].add_v(&other[3]))
    }
    
    #[inline(always)]
    pure fn sub_m(other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(&self[0].sub_v(&other[0]),
                        &self[1].sub_v(&other[1]),
                        &self[2].sub_v(&other[2]),
                        &self[3].sub_v(&other[3]))
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
        self[0][1].fuzzy_eq(&from_int(0)) &&
        self[0][2].fuzzy_eq(&from_int(0)) &&
        self[0][3].fuzzy_eq(&from_int(0)) &&
        
        self[1][0].fuzzy_eq(&from_int(0)) &&
        self[1][2].fuzzy_eq(&from_int(0)) &&
        self[1][3].fuzzy_eq(&from_int(0)) &&
        
        self[2][0].fuzzy_eq(&from_int(0)) &&
        self[2][1].fuzzy_eq(&from_int(0)) &&
        self[2][3].fuzzy_eq(&from_int(0)) &&
        
        self[3][0].fuzzy_eq(&from_int(0)) &&
        self[3][1].fuzzy_eq(&from_int(0)) &&
        self[3][2].fuzzy_eq(&from_int(0))
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat4::identity())
    }
}

pub impl<T:Copy Num Sqrt FuzzyEq> Mat4<T>: Matrix4<T> {
    #[inline(always)]
    pure fn scale(vec: &Vec3<T>) -> Mat4<T> {
        self.mul_m(&Mat4::new(      vec.x, from_int(0), from_int(0), from_int(0),
                              from_int(0),       vec.y, from_int(0), from_int(0),
                              from_int(0), from_int(0),       vec.z, from_int(0),
                              from_int(0), from_int(0), from_int(0), from_int(1)))
    }
    
    #[inline(always)]
    pure fn translate(vec: &Vec3<T>) -> Mat4<T> {
        Mat4::from_cols(&self[0],
                        &self[1],
                        &self[2],
                        &Vec4::new(self[3][0] + vec.x,
                                   self[3][1] + vec.y,
                                   self[3][2] + vec.z,
                                   self[3][3]))
    }
}

pub impl<T:Copy> Mat4<T>: Index<uint, Vec4<T>> {
    #[inline(always)]
    pure fn index(i: uint) -> Vec4<T> {
        self.data[i]
    }
}

pub impl<T:Copy Neg<T>> Mat4<T>: Neg<Mat4<T>> {
    #[inline(always)]
    pure fn neg() -> Mat4<T> {
        Mat4::from_cols(&-self[0], &-self[1], &-self[2], &-self[3])
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