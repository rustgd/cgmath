use std::cmp::FuzzyEq;
use cmp::Eq;
use math::{ExactEq, Sqrt};
use quaternion::Quat;
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
    
    pure fn mul_f(value: T) -> self;
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
pub trait Matrix3<V3> {
    pure fn scale(vec: &V3) -> self;
    pure fn to_Mat4() -> Mat4;
    pure fn to_Quat() -> Quat;
}

//
//  4x4 Matrix
//
pub trait Matrix4<V3, V4> {
    pure fn scale(vec: &V3) -> self;
    pure fn translate(vec: &V3) -> self;
}






//
//  Mat2: A 2x2, column major matrix
//
pub struct Mat2 { data:[Vec2 * 2] }

pub const mat2_zero     :Mat2 = Mat2 { data: [ Vec2::zero,
                                               Vec2::zero ] };
pub const mat2_identity :Mat2 = Mat2 { data: [ Vec2::unit_x,
                                               Vec2::unit_y ] };

//
//  Mat2 Constructor
//
#[inline]
pub pure fn Mat2(m00:float, m01:float,
                 m10:float, m11:float) -> Mat2 {
    Mat2 { data: [ Vec2(m00, m01),
                   Vec2(m10, m11) ] }
}

//
//  Construct Mat2 from column vectors
//
#[inline]
pub pure fn Mat2_v(col0: &Vec2, col1: &Vec2) -> Mat2 {
    Mat2 { data: [ *col0, *col1 ] }
}

pub mod Mat2 {
    pub const zero     :Mat2 = Mat2 { data: [ Vec2::zero,
                                              Vec2::zero ] };
    pub const identity :Mat2 = Mat2 { data: [ Vec2::unit_x,
                                              Vec2::unit_y ] };
}

//
//  Matrix2x2 Implementation
//
pub impl Mat2: Matrix<float, Vec2> {
    #[inline]
    pure fn rows() -> uint { 2 }
    
    #[inline]
    pure fn cols() -> uint { 2 }
    
    #[inline]
    pure fn is_col_major() -> bool { true }
    
    #[inline]
    pure fn row(i: uint) -> Vec2 {
        Vec2(self[0][i],
             self[1][i])
    }
    
    #[inline]
    pure fn col(i: uint) -> Vec2 {
        self.data[i]
    }
    
    #[inline]
    pure fn mul_f(value: float) -> Mat2 {
        Mat2_v(&self[0].mul_f(value),
               &self[1].mul_f(value))
    }
    
    #[inline]
    pure fn mul_v(other: &Vec2) -> Vec2 {
        Vec2(self[0][0]*other[0] + self[1][0]*other[1],
             self[0][1]*other[0] + self[1][1]*other[1])
    }
    
    #[inline]
    pure fn add_m(other: &Mat2) -> Mat2 {
        Mat2_v(&self[0].add_v(&other[0]),
               &self[1].add_v(&other[1]))
    }
    
    #[inline]
    pure fn sub_m(other: &Mat2) -> Mat2 {
        Mat2_v(&self[0].sub_v(&other[0]),
               &self[1].sub_v(&other[1]))
    }
    
    #[inline]
    pure fn mul_m(other: &Mat2) -> Mat2 {
        Mat2(self[0][0]*other[0][0] + self[1][0]*other[0][1],
             self[0][1]*other[0][0] + self[1][1]*other[0][1],
             
             self[0][0]*other[1][0] + self[1][0]*other[1][1],
             self[0][1]*other[1][0] + self[1][1]*other[1][1])
    }
    
    // TODO - inversion is harrrd D:
    // #[inline]
    // pure fn invert(other: &Mat2) -> Mat2 {}
    
    #[inline]
    pure fn transpose() -> Mat2 {
        Mat2(self[0][0], self[1][0],
             self[0][1], self[1][1])
    }
    
    #[inline]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(&Mat2::identity)
    }
    
    #[inline]
    pure fn is_symmetric() -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[1][0].fuzzy_eq(&self[0][1])
    }
    
    #[inline]
    pure fn is_diagonal() -> bool {
        self[0][1].fuzzy_eq(&0f) &&
        self[1][0].fuzzy_eq(&0f)
    }
    
    #[inline]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat2::identity)
    }
}

pub impl Mat2: Index<uint, Vec2> {
    #[inline]
    pure fn index(i: uint) -> Vec2 {
        self.data[i]
    }
}

pub impl Mat2: Neg<Mat2> {
    #[inline]
    pure fn neg() -> Mat2 {
        Mat2_v(&-self[0], &-self[1])
    }
}

pub impl Mat2: Eq {
    #[inline]
    pure fn eq(other: &Mat2) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline]
    pure fn ne(other: &Mat2) -> bool {
        !(self == *other)
    }
}

impl Mat2: ExactEq {
    #[inline]
    pure fn exact_eq(other: &Mat2) -> bool {
        self[0].exact_eq(&other[0]) &&
        self[1].exact_eq(&other[1])
    }
}

pub impl Mat2: FuzzyEq {
    #[inline]
    pure fn fuzzy_eq(other: &Mat2) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1])
    }
}






//
//  Mat3: A 3x3, column major matrix
//
pub struct Mat3 { data:[Vec3 * 3] }

//
//  Mat3 Constructor
//
#[inline]
pub pure fn Mat3(m00:float, m01:float, m02:float,
                 m10:float, m11:float, m12:float,
                 m20:float, m21:float, m22:float) -> Mat3 {
    Mat3 { data: [ Vec3(m00, m01, m02),
                   Vec3(m10, m11, m12),
                   Vec3(m20, m21, m22) ] }
}

//
//  Construct Mat3 from column vectors
//
#[inline]
pub pure fn Mat3_v(col0: &Vec3, col1: &Vec3, col2: &Vec3) -> Mat3 {
    Mat3 { data: [ *col0, *col1, *col2 ] }
}

pub mod Mat3 {
    pub const zero     :Mat3 = Mat3 { data: [ Vec3::zero,
                                              Vec3::zero,
                                              Vec3::zero ] };
    pub const identity :Mat3 = Mat3 { data: [ Vec3::unit_x,
                                              Vec3::unit_y,
                                              Vec3::unit_z ] };
}

//
//  Matrix3x3 Implementation
//
pub impl Mat3: Matrix<float, Vec3> {
    #[inline]
    pure fn rows() -> uint { 3 }
    
    #[inline]
    pure fn cols() -> uint { 3 }
    
    #[inline]
    pure fn is_col_major() -> bool { true }
    
    #[inline]
    pure fn row(i: uint) -> Vec3 {
        Vec3(self[0][i],
             self[1][i],
             self[2][i])
    }
    
    #[inline]
    pure fn col(i: uint) -> Vec3 {
        self.data[i]
    }
    
    #[inline]
    pure fn mul_f(value: float) -> Mat3 {
        Mat3_v(&self[0].mul_f(value),
               &self[1].mul_f(value),
               &self[2].mul_f(value))
    }
    
    #[inline]
    pure fn mul_v(other: &Vec3) -> Vec3 {
        Vec3(self[0][0]*other[0] + self[1][0]*other[1] + self[2][0]*other[2],
             self[0][1]*other[0] + self[1][1]*other[1] + self[2][1]*other[2],
             self[0][2]*other[0] + self[1][2]*other[1] + self[2][2]*other[2])
    }
    
    #[inline]
    pure fn add_m(other: &Mat3) -> Mat3 {
        Mat3_v(&self[0].add_v(&other[0]),
               &self[1].add_v(&other[1]),
               &self[2].add_v(&other[2]))
    }
    
    #[inline]
    pure fn sub_m(other: &Mat3) -> Mat3 {
        Mat3_v(&self[0].sub_v(&other[0]),
               &self[1].sub_v(&other[1]),
               &self[2].sub_v(&other[2]))
    }
    
    #[inline]
    pure fn mul_m(other: &Mat3) -> Mat3 {
        Mat3(self[0][0]*other[0][0] + self[1][0]*other[0][1] + self[2][0]*other[0][2],
             self[0][1]*other[0][0] + self[1][1]*other[0][1] + self[2][1]*other[0][2],
             self[0][2]*other[0][0] + self[1][2]*other[0][1] + self[2][2]*other[0][2],
            
             self[0][0]*other[1][0] + self[1][0]*other[1][1] + self[2][0]*other[1][2],
             self[0][1]*other[1][0] + self[1][1]*other[1][1] + self[2][1]*other[1][2],
             self[0][2]*other[1][0] + self[1][2]*other[1][1] + self[2][2]*other[1][2],
            
             self[0][0]*other[2][0] + self[1][0]*other[2][1] + self[2][0]*other[2][2],
             self[0][1]*other[2][0] + self[1][1]*other[2][1] + self[2][1]*other[2][2],
             self[0][2]*other[2][0] + self[1][2]*other[2][1] + self[2][2]*other[2][2])
    }
    
    // TODO - inversion is harrrd D:
    // #[inline]
    // pure fn invert(other: &Mat3) -> Mat3 {}
    
    #[inline]
    pure fn transpose() -> Mat3 {
        Mat3(self[0][0], self[1][0], self[2][0],
             self[0][1], self[1][1], self[2][1],
             self[0][2], self[1][2], self[2][2])
    }
    
    #[inline]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(&Mat3::identity)
    }
    
    #[inline]
    pure fn is_symmetric() -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[0][2].fuzzy_eq(&self[2][0]) &&
        
        self[1][0].fuzzy_eq(&self[0][1]) &&
        self[1][2].fuzzy_eq(&self[2][1]) &&
        
        self[2][0].fuzzy_eq(&self[0][2]) &&
        self[2][1].fuzzy_eq(&self[1][2])
    }
    
    #[inline]
    pure fn is_diagonal() -> bool {
        self[0][1].fuzzy_eq(&0f) &&
        self[0][2].fuzzy_eq(&0f) &&
        
        self[1][0].fuzzy_eq(&0f) &&
        self[1][2].fuzzy_eq(&0f) &&
        
        self[2][0].fuzzy_eq(&0f) &&
        self[2][1].fuzzy_eq(&0f)
    }
    
    #[inline]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat3::identity)
    }
}

pub impl Mat3: Matrix3<Vec3> {
    #[inline]
    pure fn scale(vec: &Vec3) -> Mat3 {
        self.mul_m(&Mat3(vec.x,    0f,    0f,
                            0f, vec.y,    0f,
                            0f,    0f, vec.z))
    }
    
    #[inline]
    pure fn to_Mat4() -> Mat4 {
        Mat4(self[0][0], self[0][1], self[0][2],  0f,
             self[1][0], self[1][1], self[1][2],  0f,
             self[2][0], self[2][1], self[2][2],  0f,
                     0f,         0f,         0f,  1f)
    }
    
    pure fn to_Quat() -> Quat {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf
        
        let mut s:float;
        let w:float, x:float, y:float, z:float;
        let trace:float = self[0][0] + self[1][1] + self[2][2];
        
        if trace >= 0f {
            s = (trace + 1f).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = self[1][2] - self[2][1] * s;
            y = self[2][0] - self[0][2] * s;
            z = self[0][1] - self[1][0] * s;
        } else if (self[0][0] > self[1][1]) && (self[0][0] > self[2][2]) {
            s = (1f + self[0][0] - self[1][1] - self[2][2]).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = self[0][1] - self[1][0] * s;
            y = self[2][0] - self[0][2] * s;
            z = self[1][2] - self[2][1] * s;
        } else if self[1][1] > self[2][2] {
            s = (1f + self[1][1] - self[0][0] - self[2][2]).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = self[0][1] - self[1][0] * s;
            y = self[1][2] - self[2][1] * s;
            z = self[2][0] - self[0][2] * s;
        } else {
            s = (1f + self[2][2] - self[0][0] - self[1][1]).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = self[2][0] - self[0][2] * s;
            y = self[1][2] - self[2][1] * s;
            z = self[0][1] - self[1][0] * s;
        }
        return Quat(w, x, y, z);
    }
}

pub impl Mat3: Index<uint, Vec3> {
    #[inline]
    pure fn index(i: uint) -> Vec3 {
        self.data[i]
    }
}

pub impl Mat3: Neg<Mat3> {
    #[inline]
    pure fn neg() -> Mat3 {
        Mat3_v(&-self[0], &-self[1], &-self[2])
    }
}

pub impl Mat3: Eq {
    #[inline]
    pure fn eq(other: &Mat3) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline]
    pure fn ne(other: &Mat3) -> bool {
        !(self == *other)
    }
}

impl Mat3: ExactEq {
    #[inline]
    pure fn exact_eq(other: &Mat3) -> bool {
        self[0].exact_eq(&other[0]) &&
        self[1].exact_eq(&other[1]) &&
        self[2].exact_eq(&other[2])
    }
}

pub impl Mat3: FuzzyEq {
    #[inline]
    pure fn fuzzy_eq(other: &Mat3) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2])
    }
}






//
//  Mat4: A 4x4, column major matrix
//
pub struct Mat4 { data:[Vec4 * 4] }

//
//  Mat4 Constructor
//
#[inline]
pub pure fn Mat4(m00:float, m01:float, m02:float, m03:float,
                 m10:float, m11:float, m12:float, m13:float,
                 m20:float, m21:float, m22:float, m23:float,
                 m30:float, m31:float, m32:float, m33:float) -> Mat4 {
    Mat4 { data: [ Vec4(m00, m01, m02, m03),
                   Vec4(m10, m11, m12, m13),
                   Vec4(m20, m21, m22, m23),
                   Vec4(m30, m31, m32, m33) ] }
}

//
//  Construct Mat4 from column vectors
//
#[inline]
pub pure fn Mat4_v(col0: &Vec4, col1: &Vec4, col2: &Vec4, col3: &Vec4) -> Mat4 {
    Mat4 { data: [ *col0, *col1, *col2, *col3 ] }
}

pub mod Mat4 {
    pub const zero     :Mat4 = Mat4 { data: [ Vec4::zero,
                                              Vec4::zero,
                                              Vec4::zero,
                                              Vec4::zero ] };
    pub const identity :Mat4 = Mat4 { data: [ Vec4::unit_x,
                                              Vec4::unit_y,
                                              Vec4::unit_z,
                                              Vec4::unit_w ] };
}

//
//  Matrix4x4 Implementation
//
pub impl Mat4: Matrix<float, Vec4> {
    #[inline]
    pure fn rows() -> uint { 4 }
    
    #[inline]
    pure fn cols() -> uint { 4 }
    
    #[inline]
    pure fn is_col_major() -> bool { true }
    
    #[inline]
    pure fn row(i: uint) -> Vec4 {
        Vec4(self[0][i],
             self[1][i],
             self[2][i],
             self[3][i])
    }
    
    #[inline]
    pure fn col(i: uint) -> Vec4 {
        self.data[i]
    }
    
    #[inline]
    pure fn mul_f(value: float) -> Mat4 {
        Mat4_v(&self[0].mul_f(value),
               &self[1].mul_f(value),
               &self[2].mul_f(value),
               &self[3].mul_f(value))
    }
    
    #[inline]
    pure fn mul_v(other: &Vec4) -> Vec4 {
        Vec4(self[0][0]*other[0] + self[1][0]*other[1] + self[2][0]*other[2] + self[3][0]*other[3],
             self[0][1]*other[0] + self[1][1]*other[1] + self[2][1]*other[2] + self[3][1]*other[3],
             self[0][2]*other[0] + self[1][2]*other[1] + self[2][2]*other[2] + self[3][2]*other[3],
             self[0][3]*other[0] + self[1][3]*other[1] + self[2][3]*other[2] + self[3][3]*other[3])
    }
    
    #[inline]
    pure fn add_m(other: &Mat4) -> Mat4 {
        Mat4_v(&self[0].add_v(&other[0]),
               &self[1].add_v(&other[1]),
               &self[2].add_v(&other[2]),
               &self[3].add_v(&other[3]))
    }
    
    #[inline]
    pure fn sub_m(other: &Mat4) -> Mat4 {
        Mat4_v(&self[0].sub_v(&other[0]),
               &self[1].sub_v(&other[1]),
               &self[2].sub_v(&other[2]),
               &self[3].sub_v(&other[3]))
    }
    
    #[inline]
    pure fn mul_m(other: &Mat4) -> Mat4 {
        Mat4(self[0][0]*other[0][0] + self[1][0]*other[0][1] + self[2][0]*other[0][2] + self[3][0]*other[0][3],
             self[0][1]*other[0][0] + self[1][1]*other[0][1] + self[2][1]*other[0][2] + self[3][1]*other[0][3],
             self[0][2]*other[0][0] + self[1][2]*other[0][1] + self[2][2]*other[0][2] + self[3][2]*other[0][3],
             self[0][3]*other[0][0] + self[1][3]*other[0][1] + self[2][3]*other[0][2] + self[3][3]*other[0][3],
            
             self[0][0]*other[1][0] + self[1][0]*other[1][1] + self[2][0]*other[1][2] + self[3][0]*other[1][3],
             self[0][1]*other[1][0] + self[1][1]*other[1][1] + self[2][1]*other[1][2] + self[3][1]*other[1][3],
             self[0][2]*other[1][0] + self[1][2]*other[1][1] + self[2][2]*other[1][2] + self[3][2]*other[1][3],
             self[0][3]*other[1][0] + self[1][3]*other[1][1] + self[2][3]*other[1][2] + self[3][3]*other[1][3],
            
             self[0][0]*other[2][0] + self[1][0]*other[2][1] + self[2][0]*other[2][2] + self[3][0]*other[2][3],
             self[0][1]*other[2][0] + self[1][1]*other[2][1] + self[2][1]*other[2][2] + self[3][1]*other[2][3],
             self[0][2]*other[2][0] + self[1][2]*other[2][1] + self[2][2]*other[2][2] + self[3][2]*other[2][3],
             self[0][3]*other[2][0] + self[1][3]*other[2][1] + self[2][3]*other[2][2] + self[3][3]*other[2][3],
            
             self[0][0]*other[3][0] + self[1][0]*other[3][1] + self[2][0]*other[3][2] + self[3][0]*other[3][3],
             self[0][1]*other[3][0] + self[1][1]*other[3][1] + self[2][1]*other[3][2] + self[3][1]*other[3][3],
             self[0][2]*other[3][0] + self[1][2]*other[3][1] + self[2][2]*other[3][2] + self[3][2]*other[3][3],
             self[0][3]*other[3][0] + self[1][3]*other[3][1] + self[2][3]*other[3][2] + self[3][3]*other[3][3])
    }
    
    // TODO - inversion is harrrd D:
    // #[inline]
    // pure fn invert(other: &Mat4) -> Mat4 {}
    
    #[inline]
    pure fn transpose() -> Mat4 {
        Mat4(self[0][0], self[1][0], self[2][0], self[3][0],
             self[0][1], self[1][1], self[2][1], self[3][1],
             self[0][2], self[1][2], self[2][2], self[3][2],
             self[0][3], self[1][3], self[2][3], self[3][3])
    }
    
    #[inline]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(&Mat4::identity)
    }
    
    #[inline]
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
    
    #[inline]
    pure fn is_diagonal() -> bool {
        self[0][1].fuzzy_eq(&0f) &&
        self[0][2].fuzzy_eq(&0f) &&
        self[0][3].fuzzy_eq(&0f) &&
        
        self[1][0].fuzzy_eq(&0f) &&
        self[1][2].fuzzy_eq(&0f) &&
        self[1][3].fuzzy_eq(&0f) &&
        
        self[2][0].fuzzy_eq(&0f) &&
        self[2][1].fuzzy_eq(&0f) &&
        self[2][3].fuzzy_eq(&0f) &&
        
        self[3][0].fuzzy_eq(&0f) &&
        self[3][1].fuzzy_eq(&0f) &&
        self[3][2].fuzzy_eq(&0f)
    }
    
    #[inline]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat4::identity)
    }
}

pub impl Mat4: Matrix4<Vec3, Vec4> {
    #[inline]
    pure fn scale(vec: &Vec3) -> Mat4 {
        self.mul_m(&Mat4(vec.x,    0f,    0f, 0f,
                            0f, vec.y,    0f, 0f,
                            0f,    0f, vec.z, 0f,
                            0f,    0f,    0f, 1f))
    }
    
    #[inline]
    pure fn translate(vec: &Vec3) -> Mat4 {
        Mat4_v(&self[0],
               &self[1],
               &self[2],
               &Vec4(self[3][0] + vec.x,
                     self[3][1] + vec.y,
                     self[3][2] + vec.z,
                     self[3][3]))
    }
}

pub impl Mat4: Index<uint, Vec4> {
    #[inline]
    pure fn index(i: uint) -> Vec4 {
        self.data[i]
    }
}

pub impl Mat4: Neg<Mat4> {
    #[inline]
    pure fn neg() -> Mat4 {
        Mat4_v(&-self[0], &-self[1], &-self[2], &-self[3])
    }
}

pub impl Mat4: Eq {
    #[inline]
    pure fn eq(other: &Mat4) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline]
    pure fn ne(other: &Mat4) -> bool {
        !(self == *other)
    }
}

impl Mat4: ExactEq {
    #[inline]
    pure fn exact_eq(other: &Mat4) -> bool {
        self[0].exact_eq(&other[0]) &&
        self[1].exact_eq(&other[1]) &&
        self[2].exact_eq(&other[2]) &&
        self[3].exact_eq(&other[3])
    }
}

pub impl Mat4: FuzzyEq {
    #[inline]
    pure fn fuzzy_eq(other: &Mat4) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
}