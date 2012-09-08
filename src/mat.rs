import std::cmp::FuzzyEq;
import cmp::Ord;
import num::Num;
// import to_str::ToStr;
import quat::quat;
import vec::*;

// TODO: Unittests! I've probably made lots of mistakes...

//
//  NxN Matrix
//
trait Matrix<T:Num Ord FuzzyEq, V:Vector<T>> {
    pure fn rows() -> uint;
    pure fn cols() -> uint;
    pure fn is_col_major() -> bool;
    
    pure fn index(&&index:uint) -> V;
    pure fn row(&&i:uint) -> V;
    pure fn col(&&i:uint) -> V;
    
    pure fn neg() -> self;
    
    pure fn mul_f(&&value:T) -> self;
    pure fn mul_v(&&other:V) -> V;
    pure fn add_m(&&other:self) -> self;
    pure fn sub_m(&&other:self) -> self;
    pure fn mul_m(&&other:self) -> self;
    
    // pure fn invert(&&other:self) -> self;
    pure fn transpose() -> self;
    
    pure fn exact_eq(&&other:self) -> bool;
    pure fn fuzzy_eq(&&other:self) -> bool;
    pure fn eq(&&other:self) -> bool;
    
    pure fn is_identity() -> bool;
    pure fn is_symmetric() -> bool;
    pure fn is_diagonal() -> bool;
    pure fn is_rotated() -> bool;
}

//
//  3x3 Matrix
//
trait Matrix3<T:Num Ord FuzzyEq, V:Vector<T>> {
    pure fn scale(&&vec:V) -> self;
    pure fn to_mat4() -> mat4;
    pure fn to_quat() -> quat;
}

//
//  4x4 Matrix
//
trait Matrix4<T:Num Ord FuzzyEq, V:Vector<T>> {
    pure fn scale(&&vec:vec3) -> self;      // I don't like the use of `vec3` here
    pure fn translate(&&vec:vec3) -> self;
}






//
//  Mat2: A 2x2, column major matrix
//
struct mat2 { data:[vec2 * 2] }

//
//  Mat2 Constructor
//
#[inline(always)]
pure fn mat2(m00:float, m01:float,
             m10:float, m11:float) -> mat2 {
    mat2 { data: [ vec2(m00, m01),
                   vec2(m10, m11) ] }
}

//
//  Construct Mat2 from column vectors
//
#[inline(always)]
pure fn mat2_v(col0:vec2, col1:vec2) -> mat2 {
    mat2 { data: [ col0, col1 ] }
}

#[inline(always)]
pure fn mat2_zero() -> mat2 {
    mat2(0f, 0f,
         0f, 0f)
}

#[inline(always)]
pure fn mat2_identity() -> mat2 {
    mat2(1f, 0f,
         0f, 1f)
}

//
//  Matrix2x2 Implementation
//
impl mat2: Matrix<float, vec2> {
    #[inline(always)]
    pure fn rows() -> uint { 2 }
    
    #[inline(always)]
    pure fn cols() -> uint { 2 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn index(&&i: uint) -> vec2 {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn row(&&i:uint) -> vec2 {
        vec2(self[0][i],
             self[1][i])
    }
    
    #[inline(always)]
    pure fn col(&&i:uint) -> vec2 {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn neg() -> mat2 {
        mat2_v(-self[0], -self[1])
    }
    
    #[inline(always)]
    pure fn mul_f(&&value:float) -> mat2 {
        mat2_v(self[0].mul_f(value),
               self[1].mul_f(value))
    }
    
    #[inline(always)]
    pure fn mul_v(&&other:vec2) -> vec2 {
        vec2(self[0][0]*other[0] + self[1][0]*other[1],
             self[0][1]*other[0] + self[1][1]*other[1])
    }
    
    #[inline(always)]
    pure fn add_m(&&other:mat2) -> mat2 {
        mat2_v(self[0].add_v(other[0]),
               self[1].add_v(other[1]))
    }
    
    #[inline(always)]
    pure fn sub_m(&&other:mat2) -> mat2 {
        mat2_v(self[0].sub_v(other[0]),
               self[1].sub_v(other[1]))
    }
    
    #[inline(always)]
    pure fn mul_m(&&other:mat2) -> mat2 {
        mat2(self[0][0]*other[0][0] + self[1][0]*other[0][1],
             self[0][1]*other[0][0] + self[1][1]*other[0][1],
             
             self[0][0]*other[1][0] + self[1][0]*other[1][1],
             self[0][1]*other[1][0] + self[1][1]*other[1][1])
    }
    
    // TODO - inversion is harrrd D:
    // #[inline(always)]
    // pure fn invert(&&other:mat2) -> mat2 {}
    
    #[inline(always)]
    pure fn transpose() -> mat2 {
        mat2(self[0][0], self[1][0],
             self[0][1], self[1][1])
    }
    
    #[inline(always)]
    pure fn exact_eq(&&other:mat2) -> bool {
        self[0].exact_eq(other[0]) &&
        self[1].exact_eq(other[1])
    }
    
    #[inline(always)]
    pure fn fuzzy_eq(&&other:mat2) -> bool {
        self[0].fuzzy_eq(other[0]) &&
        self[1].fuzzy_eq(other[1])
    }
    
    #[inline(always)]
    pure fn eq(&&other:mat2) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(mat2_identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric() -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[1][0].fuzzy_eq(&self[0][1])
    }
    
    #[inline(always)]
    pure fn is_diagonal() -> bool {
        self[0][1].fuzzy_eq(&0f) &&
        self[1][0].fuzzy_eq(&0f)
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(mat2_identity())
    }
}






//
//  Mat3: A 3x3, column major matrix
//
struct mat3 { data:[vec3 * 3] }

//
//  Mat3 Constructor
//
#[inline(always)]
pure fn mat3(m00:float, m01:float, m02:float,
             m10:float, m11:float, m12:float,
             m20:float, m21:float, m22:float) -> mat3 {
    mat3 { data: [ vec3(m00, m01, m02),
                   vec3(m10, m11, m12),
                   vec3(m20, m21, m22) ] }
}

//
//  Construct Mat3 from column vectors
//
#[inline(always)]
pure fn mat3_v(col0:vec3, col1:vec3, col2:vec3) -> mat3 {
    mat3 { data: [ col0, col1, col2 ] }
}

#[inline(always)]
pure fn mat3_zero() -> mat3 {
    mat3 (0f, 0f, 0f,
          0f, 0f, 0f,
          0f, 0f, 0f)
}

#[inline(always)]
pure fn mat3_identity() -> mat3 {
    mat3 (1f, 0f, 0f,
          0f, 1f, 0f,
          0f, 0f, 1f)
}

//
//  Matrix3x3 Implementation
//
impl mat3: Matrix<float, vec3> {
    #[inline(always)]
    pure fn rows() -> uint { 3 }
    
    #[inline(always)]
    pure fn cols() -> uint { 3 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn index(&&i: uint) -> vec3 {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn row(&&i:uint) -> vec3 {
        vec3(self[0][i],
             self[1][i],
             self[2][i])
    }
    
    #[inline(always)]
    pure fn col(&&i:uint) -> vec3 {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn neg() -> mat3 {
        mat3_v(-self[0], -self[1], -self[2])
    }
    
    #[inline(always)]
    pure fn mul_f(&&value:float) -> mat3 {
        mat3_v(self[0].mul_f(value),
               self[1].mul_f(value),
               self[2].mul_f(value))
    }
    
    #[inline(always)]
    pure fn mul_v(&&other:vec3) -> vec3 {
        vec3(self[0][0]*other[0] + self[1][0]*other[1] + self[2][0]*other[2],
             self[0][1]*other[0] + self[1][1]*other[1] + self[2][1]*other[2],
             self[0][2]*other[0] + self[1][2]*other[1] + self[2][2]*other[2])
    }
    
    #[inline(always)]
    pure fn add_m(&&other:mat3) -> mat3 {
        mat3_v(self[0].add_v(other[0]),
               self[1].add_v(other[1]),
               self[2].add_v(other[2]))
    }
    
    #[inline(always)]
    pure fn sub_m(&&other:mat3) -> mat3 {
        mat3_v(self[0].sub_v(other[0]),
               self[1].sub_v(other[1]),
               self[2].sub_v(other[2]))
    }
    
    #[inline(always)]
    pure fn mul_m(&&other:mat3) -> mat3 {
        mat3(self[0][0]*other[0][0] + self[1][0]*other[0][1] + self[2][0]*other[0][2],
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
    // #[inline(always)]
    // pure fn invert(&&other:mat3) -> mat3 {}
    
    #[inline(always)]
    pure fn transpose() -> mat3 {
        mat3(self[0][0], self[1][0], self[2][0],
             self[0][1], self[1][1], self[2][1],
             self[0][2], self[1][2], self[2][2])
    }
    
    #[inline(always)]
    pure fn exact_eq(&&other:mat3) -> bool {
        self[0].exact_eq(other[0]) &&
        self[1].exact_eq(other[1]) &&
        self[2].exact_eq(other[2])
    }
    
    #[inline(always)]
    pure fn fuzzy_eq(&&other:mat3) -> bool {
        self[0].fuzzy_eq(other[0]) &&
        self[1].fuzzy_eq(other[1]) &&
        self[2].fuzzy_eq(other[2])
    }
    
    #[inline(always)]
    pure fn eq(&&other:mat3) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(mat3_identity())
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
        self[0][1].fuzzy_eq(&0f) &&
        self[0][2].fuzzy_eq(&0f) &&
        
        self[1][0].fuzzy_eq(&0f) &&
        self[1][2].fuzzy_eq(&0f) &&
        
        self[2][0].fuzzy_eq(&0f) &&
        self[2][1].fuzzy_eq(&0f)
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(mat3_identity())
    }
}

impl mat3: Matrix3<float, vec3> {
    #[inline(always)]
    pure fn scale(&&vec:vec3) -> mat3 {
        self.mul_m(mat3(vec.x(),      0f,      0f,
                             0f, vec.y(),      0f,
                             0f,      0f, vec.z()))
    }
    
    #[inline(always)]
    pure fn to_mat4() -> mat4 {
        mat4(self[0][0], self[0][1], self[0][2],  0f,
             self[1][0], self[1][1], self[1][2],  0f,
             self[2][0], self[2][1], self[2][2],  0f,
                     0f,         0f,         0f,  1f)
    }
    
    pure fn to_quat() -> quat {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/quatut.pdf
        
        let mut s:float;
        let w:float, x:float, y:float, z:float;
        let trace:float = self[0][0] + self[1][1] + self[2][2];
        
        if trace >= 0f {
            s = sqrt(trace + 1f);
            w = 0.5 * s;
            s = 0.5 / s;
            x = self[1][2] - self[2][1] * s;
            y = self[2][0] - self[0][2] * s;
            z = self[0][1] - self[1][0] * s;
        } else if (self[0][0] > self[1][1]) && (self[0][0] > self[2][2]) {
            s = sqrt(1f + self[0][0] - self[1][1] - self[2][2]);
            w = 0.5 * s;
            s = 0.5 / s;
            x = self[0][1] - self[1][0] * s;
            y = self[2][0] - self[0][2] * s;
            z = self[1][2] - self[2][1] * s;
        } else if self[1][1] > self[2][2] {
            s = sqrt(1f + self[1][1] - self[0][0] - self[2][2]);
            w = 0.5 * s;
            s = 0.5 / s;
            x = self[0][1] - self[1][0] * s;
            y = self[1][2] - self[2][1] * s;
            z = self[2][0] - self[0][2] * s;
        } else {
            s = sqrt(1f + self[2][2] - self[0][0] - self[1][1]);
            w = 0.5 * s;
            s = 0.5 / s;
            x = self[2][0] - self[0][2] * s;
            y = self[1][2] - self[2][1] * s;
            z = self[0][1] - self[1][0] * s;
        }
        return quat(w, x, y, z);
    }
}






//
//  Mat4: A 4x4, column major matrix
//
struct mat4 { data:[vec4 * 4] }

//
//  Mat4 Constructor
//
#[inline(always)]
pure fn mat4(m00:float, m01:float, m02:float, m03:float,
             m10:float, m11:float, m12:float, m13:float,
             m20:float, m21:float, m22:float, m23:float,
             m30:float, m31:float, m32:float, m33:float) -> mat4 {
    mat4 { data: [ vec4(m00, m01, m02, m03),
                   vec4(m10, m11, m12, m13),
                   vec4(m20, m21, m22, m23),
                   vec4(m30, m31, m32, m33) ] }
}

//
//  Construct Mat4 from column vectors
//
#[inline(always)]
pure fn mat4_v(col0:vec4, col1:vec4, col2:vec4, col3:vec4) -> mat4 {
    mat4 { data: [ col0, col1, col2, col3 ] }
}

#[inline(always)]
pure fn mat4_zero() -> mat4 {
    mat4 (0f, 0f, 0f, 0f,
          0f, 0f, 0f, 0f,
          0f, 0f, 0f, 0f,
          0f, 0f, 0f, 0f)
}

#[inline(always)]
pure fn mat4_identity() -> mat4 {
    mat4 (1f, 0f, 0f, 0f,
          0f, 1f, 0f, 0f,
          0f, 0f, 1f, 0f,
          0f, 0f, 0f, 1f)
}

//
//  Matrix4x4 Implementation
//
impl mat4: Matrix<float, vec4> {
    #[inline(always)]
    pure fn rows() -> uint { 4 }
    
    #[inline(always)]
    pure fn cols() -> uint { 4 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn index(&&i: uint) -> vec4 {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn row(&&i:uint) -> vec4 {
        vec4(self[0][i],
             self[1][i],
             self[2][i],
             self[3][i])
    }
    
    #[inline(always)]
    pure fn col(&&i:uint) -> vec4 {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn neg() -> mat4 {
        mat4_v(-self[0], -self[1], -self[2], -self[3])
    }
    
    #[inline(always)]
    pure fn mul_f(&&value:float) -> mat4 {
        mat4_v(self[0].mul_f(value),
               self[1].mul_f(value),
               self[2].mul_f(value),
               self[3].mul_f(value))
    }
    
    #[inline(always)]
    pure fn mul_v(&&other:vec4) -> vec4 {
        vec4(self[0][0]*other[0] + self[1][0]*other[1] + self[2][0]*other[2] + self[3][0]*other[3],
             self[0][1]*other[0] + self[1][1]*other[1] + self[2][1]*other[2] + self[3][1]*other[3],
             self[0][2]*other[0] + self[1][2]*other[1] + self[2][2]*other[2] + self[3][2]*other[3],
             self[0][3]*other[0] + self[1][3]*other[1] + self[2][3]*other[2] + self[3][3]*other[3])
    }
    
    #[inline(always)]
    pure fn add_m(&&other:mat4) -> mat4 {
        mat4_v(self[0].add_v(other[0]),
               self[1].add_v(other[1]),
               self[2].add_v(other[2]),
               self[3].add_v(other[3]))
    }
    
    #[inline(always)]
    pure fn sub_m(&&other:mat4) -> mat4 {
        mat4_v(self[0].sub_v(other[0]),
               self[1].sub_v(other[1]),
               self[2].sub_v(other[2]),
               self[3].sub_v(other[3]))
    }
    
    #[inline(always)]
    pure fn mul_m(&&other:mat4) -> mat4 {
        mat4(self[0][0]*other[0][0] + self[1][0]*other[0][1] + self[2][0]*other[0][2] + self[3][0]*other[0][3],
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
    // #[inline(always)]
    // pure fn invert(&&other:mat4) -> mat4 {}
    
    #[inline(always)]
    pure fn transpose() -> mat4 {
        mat4(self[0][0], self[1][0], self[2][0], self[3][0],
             self[0][1], self[1][1], self[2][1], self[3][1],
             self[0][2], self[1][2], self[2][2], self[3][2],
             self[0][3], self[1][3], self[2][3], self[3][3])
    }
    
    #[inline(always)]
    pure fn exact_eq(&&other:mat4) -> bool {
        self[0].exact_eq(other[0]) &&
        self[1].exact_eq(other[1]) &&
        self[2].exact_eq(other[2]) &&
        self[3].exact_eq(other[3])
    }
    
    #[inline(always)]
    pure fn fuzzy_eq(&&other:mat4) -> bool {
        self[0].fuzzy_eq(other[0]) &&
        self[1].fuzzy_eq(other[1]) &&
        self[2].fuzzy_eq(other[2]) &&
        self[3].fuzzy_eq(other[3])
    }
    
    #[inline(always)]
    pure fn eq(&&other:mat4) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(mat4_identity())
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
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(mat4_identity())
    }
}

impl mat4: Matrix4<float, vec4> {
    #[inline(always)]
    pure fn scale(&&vec:vec3) -> mat4 {
        self.mul_m(mat4(vec.x(),       0f,      0f, 0f,
                              0f, vec.y(),      0f, 0f,
                              0f,      0f, vec.z(), 0f,
                              0f,      0f,      0f, 1f))
    }
    
    #[inline(always)]
    pure fn translate(&&vec:vec3) -> mat4 {
        mat4_v(self[0],
               self[1],
               self[2],
               vec4(self[3][0] + vec.x(),
                    self[3][1] + vec.y(),
                    self[3][2] + vec.z(),
                    self[3][3]))
    }
}