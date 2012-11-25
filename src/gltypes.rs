///
/// This module contains various type aliases and method wrappers to make working
/// with OpenGL cleaner and safer than working with the lmath types directly.
/// This is especially important when working with type-sensitive OpenGL functions
/// such as `glVertexAttribPointer` and `glUniformMatrix4fv`) where a simple mistake
/// such writing `Vec3::new(1, 2, 3)` or `Vec3::new(1f, 2f, 3f)` as opposed to
/// `Vec3::new(1f32, 2f32, 3f32)` could cause you an afternoon of pain.
///
/// To give you an example of how using the wrapper methods can clean up your
/// code and make debugging far easier, instead of writing:
///
/// ~~~
/// let v: Mat4<f64> = NumericMatrixNxN::identity();
/// ~~~
///
/// you can write:
///
/// ~~~
/// let v = dmat4::identity();
/// ~~~
///

use core::sys::size_of;

use mat::{NumericMatrix, NumericMatrixNxN, Mat2, Mat3, Mat4};
use vec::{Vector, NumericVector, Vec2, Vec3, Vec4};
use quat::{/*Quaternion, */Quat};


// Vector aliases

pub type vec2  = Vec2<f32>;             /// a two-component single-precision floating-point vector
pub type vec3  = Vec3<f32>;             /// a three-component single-precision floating-point vector
pub type vec4  = Vec4<f32>;             /// a four-component single-precision floating-point vector

pub type dvec2 = Vec2<f64>;             /// a two-component double-precision floating-point vector
pub type dvec3 = Vec3<f64>;             /// a three-component double-precision floating-point vector
pub type dvec4 = Vec4<f64>;             /// a four-component double-precision floating-point vector

pub type bvec2 = Vec2<bool>;            /// a two-component Boolean vector
pub type bvec3 = Vec3<bool>;            /// a three-component Boolean vector
pub type bvec4 = Vec4<bool>;            /// a four-component Boolean vector

pub type ivec2 = Vec2<i32>;             /// a two-component signed integer vector
pub type ivec3 = Vec3<i32>;             /// a three-component signed integer vector
pub type ivec4 = Vec4<i32>;             /// a four-component signed integer vector

pub type uvec2 = Vec2<u32>;             /// a two-component unsigned integer vector
pub type uvec3 = Vec3<u32>;             /// a three-component unsigned integer vector
pub type uvec4 = Vec4<u32>;             /// a four-component unsigned integer vector


// Vector method wrappers

pub impl vec2 {
    #[inline(always)] static pure fn new(x: f32, y: f32) -> vec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: f32) -> vec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> vec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> vec2 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<vec2>() }
}

pub impl vec3 {
    #[inline(always)] static pure fn new(x: f32, y: f32, z: f32) -> vec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: f32) -> vec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> vec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> vec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<vec3>() }
}

pub impl vec4 {
    #[inline(always)] static pure fn new(x: f32, y: f32, z: f32, w: f32) -> vec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: f32) -> vec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> vec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> vec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<vec4>() }
}


pub impl dvec2 {
    #[inline(always)] static pure fn new(x: f64, y: f64) -> dvec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec2 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dvec2>() }
}

pub impl dvec3 {
    #[inline(always)] static pure fn new(x: f64, y: f64, z: f64) -> dvec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dvec3>() }
}

pub impl dvec4 {
    #[inline(always)] static pure fn new(x: f64, y: f64, z: f64, w: f64) -> dvec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dvec4>() }
}


pub impl bvec2 {
    #[inline(always)] static pure fn new(x: bool, y: bool) -> bvec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec2 { Vector::from_value(v) }
    // #[inline(always)] static pure fn identity() -> bvec2 { NumericVector::identity() }
    // #[inline(always)] static pure fn zero() -> bvec2 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<bvec2>() }
}

pub impl bvec3 {
    #[inline(always)] static pure fn new(x: bool, y: bool, z: bool) -> bvec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec3 { Vector::from_value(v) }
    // #[inline(always)] static pure fn identity() -> bvec3 { NumericVector::identity() }
    // #[inline(always)] static pure fn zero() -> bvec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<bvec3>() }
}

pub impl bvec4 {
    #[inline(always)] static pure fn new(x: bool, y: bool, z: bool, w: bool) -> bvec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec4 { Vector::from_value(v) }
    // #[inline(always)] static pure fn identity() -> bvec4 { NumericVector::identity() }
    // #[inline(always)] static pure fn zero() -> bvec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<bvec4>() }
}


pub impl ivec2 {
    #[inline(always)] static pure fn new(x: i32, y: i32) -> ivec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> ivec2 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<ivec2>() }
}

pub impl ivec3 {
    #[inline(always)] static pure fn new(x: i32, y: i32, z: i32) -> ivec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> ivec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<ivec3>() }
}

pub impl ivec4 {
    #[inline(always)] static pure fn new(x: i32, y: i32, z: i32, w: i32) -> ivec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> ivec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<ivec4>() }
}


pub impl uvec2 {
    #[inline(always)] static pure fn new(x: u32, y: u32) -> uvec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> uvec2 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<uvec2>() }
}

pub impl uvec3 {
    #[inline(always)] static pure fn new(x: u32, y: u32, z: u32) -> uvec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> uvec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<uvec3>() }
}

pub impl uvec4 {
    #[inline(always)] static pure fn new(x: u32, y: u32, z: u32, w: u32) -> uvec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> uvec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<uvec4>() }
}


// Matrix aliases

pub type mat2 = mat2x2;                 /// a 2×2 single-precision floating-point matrix
pub type mat3 = mat3x3;                 /// a 3×3 single-precision floating-point matrix
pub type mat4 = mat4x4;                 /// a 4×4 single-precision floating-point matrix
pub type mat2x2 = Mat2<f32>;            /// same as a `mat2`
// pub type mat2x3 =                    /// a single-precision floating-point matrix with 2 columns and 3 rows
// pub type mat2x4 =                    /// a single-precision floating-point matrix with 2 columns and 4 rows
// pub type mat3x2 =                    /// a single-precision floating-point matrix with 3 columns and 2 rows
pub type mat3x3 = Mat3<f32>;            /// same as a `mat3`
// pub type mat3x4 =                    /// a single-precision floating-point matrix with 3 columns and 4 rows
// pub type mat4x2 =                    /// a single-precision floating-point matrix with 4 columns and 2 rows
// pub type mat4x3 =                    /// a single-precision floating-point matrix with 4 columns and 3 rows
pub type mat4x4 = Mat4<f32>;            /// same as a `mat4`

pub type dmat2 = dmat2x2;               /// a 2×2 double-precision floating-point matrix
pub type dmat3 = dmat3x3;               /// a 3×3 double-precision floating-point matrix
pub type dmat4 = dmat4x4;               /// a 4×4 double-precision floating-point matrix
pub type dmat2x2 = Mat2<f64>;           /// same as a `dmat2`
// pub type dmat2x3 =                   /// a double-precision floating-point matrix with 2 columns and 3 rows
// pub type dmat2x4 =                   /// a double-precision floating-point matrix with 2 columns and 4 rows
// pub type dmat3x2 =                   /// a double-precision floating-point matrix with 3 columns and 2 rows
pub type dmat3x3 = Mat3<f64>;           /// same as a `dmat3`
// pub type dmat3x4 =                   /// a double-precision floating-point matrix with 3 columns and 4 rows
// pub type dmat4x2 =                   /// a double-precision floating-point matrix with 4 columns and 2 rows
// pub type dmat4x3 =                   /// a double-precision floating-point matrix with 4 columns and 3 rows
pub type dmat4x4 = Mat4<f64>;           /// same as a `dmat4`


// Matrix method wrappers

pub impl mat2 {
    #[inline(always)] static pure fn new(c0r0: f32, c0r1: f32, c1r0: f32, c1r1: f32)
        -> mat2 { mat2x2::new(c0r0, c0r1, c1r0, c1r1) }
    #[inline(always)] static pure fn from_cols(c0: vec2, c1: vec2)
        -> mat2 { mat2x2::from_cols(move c0, move c1) }
    #[inline(always)] static pure fn from_value(v: f32) -> mat2 { mat2x2::from_value(v) }
    #[inline(always)] static pure fn identity() -> mat2 { mat2x2::identity() }
    #[inline(always)] static pure fn zero() -> mat2 { mat2x2::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { mat2x2::dim() }
    #[inline(always)] static pure fn rows() -> uint { mat2x2::rows() }
    #[inline(always)] static pure fn cols() -> uint { mat2x2::cols() }
    #[inline(always)] static pure fn size_of() -> uint { mat2x2::size_of() }
}

pub impl mat3 {
    #[inline(always)] static pure fn new(c0r0: f32, c0r1: f32, c0r2: f32, c1r0: f32, c1r1: f32, c1r2: f32, c2r0: f32, c2r1: f32, c2r2: f32)
        -> mat3 { mat3x3::new(c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2) }
    #[inline(always)] static pure fn from_cols(c0: vec3, c1: vec3, c2: vec3)
        -> mat3 { mat3x3::from_cols(move c0, move c1, move c2) }
    #[inline(always)] static pure fn from_value(v: f32) -> mat3 { mat3x3::from_value(v) }
    #[inline(always)] static pure fn identity() -> mat3 { mat3x3::identity() }
    #[inline(always)] static pure fn zero() -> mat3 { mat3x3::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { mat3x3::dim() }
    #[inline(always)] static pure fn rows() -> uint { mat3x3::rows() }
    #[inline(always)] static pure fn cols() -> uint { mat3x3::cols() }
    #[inline(always)] static pure fn size_of() -> uint { mat3x3::size_of() }
}

pub impl mat4 {
    #[inline(always)] static pure fn new(c0r0: f32, c0r1: f32, c0r2: f32, c0r3: f32, c1r0: f32, c1r1: f32, c1r2: f32, c1r3: f32, c2r0: f32, c2r1: f32, c2r2: f32, c2r3: f32, c3r0: f32, c3r1: f32, c3r2: f32, c3r3: f32)
        -> mat4 { mat4x4::new(c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1, c3r2, c3r3) }
    #[inline(always)] static pure fn from_cols(c0: vec4, c1: vec4, c2: vec4, c3: vec4)
        -> mat4 { mat4x4::from_cols(move c0, move c1, move c2, move c3) }
    #[inline(always)] static pure fn from_value(v: f32) -> mat4 { mat4x4::from_value(v) }
    #[inline(always)] static pure fn identity() -> mat4 { mat4x4::identity() }
    #[inline(always)] static pure fn zero() -> mat4 { mat4x4::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { mat4x4::dim() }
    #[inline(always)] static pure fn rows() -> uint { mat4x4::rows() }
    #[inline(always)] static pure fn cols() -> uint { mat4x4::cols() }
    #[inline(always)] static pure fn size_of() -> uint { mat4x4::size_of() }
}

pub impl mat2x2 {
    #[inline(always)] static pure fn new(c0r0: f32, c0r1: f32, c1r0: f32, c1r1: f32)
        -> mat2x2 { Mat2::new(c0r0, c0r1, c1r0, c1r1) }
    #[inline(always)] static pure fn from_cols(c0: vec2, c1: vec2)
        -> mat2x2 { Mat2::from_cols(move c0, move c1) }
    #[inline(always)] static pure fn from_value(v: f32) -> mat2x2 { Mat2::from_value(v) }
    #[inline(always)] static pure fn identity() -> mat2x2 { NumericMatrixNxN::identity() }
    #[inline(always)] static pure fn zero() -> mat2x2 { NumericMatrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn rows() -> uint { 2 }
    #[inline(always)] static pure fn cols() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<mat2x2>() }
}

pub impl mat3x3 {
    #[inline(always)] static pure fn new(c0r0: f32, c0r1: f32, c0r2: f32, c1r0: f32, c1r1: f32, c1r2: f32, c2r0: f32, c2r1: f32, c2r2: f32)
        -> mat3x3 { Mat3::new(c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2) }
    #[inline(always)] static pure fn from_cols(c0: vec3, c1: vec3, c2: vec3)
        -> mat3x3 { Mat3::from_cols(move c0, move c1, move c2) }
    #[inline(always)] static pure fn from_value(v: f32) -> mat3x3 { Mat3::from_value(v) }
    #[inline(always)] static pure fn identity() -> mat3x3 { NumericMatrixNxN::identity() }
    #[inline(always)] static pure fn zero() -> mat3x3 { NumericMatrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn rows() -> uint { 3 }
    #[inline(always)] static pure fn cols() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<mat3x3>() }
}

pub impl mat4x4 {
    #[inline(always)] static pure fn new(c0r0: f32, c0r1: f32, c0r2: f32, c0r3: f32, c1r0: f32, c1r1: f32, c1r2: f32, c1r3: f32, c2r0: f32, c2r1: f32, c2r2: f32, c2r3: f32, c3r0: f32, c3r1: f32, c3r2: f32, c3r3: f32)
        -> mat4x4 { Mat4::new(c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1, c3r2, c3r3) }
    #[inline(always)] static pure fn from_cols(c0: vec4, c1: vec4, c2: vec4, c3: vec4)
        -> mat4x4 { Mat4::from_cols(move c0, move c1, move c2, move c3) }
    #[inline(always)] static pure fn from_value(v: f32) -> mat4x4 { Mat4::from_value(v) }
    #[inline(always)] static pure fn identity() -> mat4x4 { NumericMatrixNxN::identity() }
    #[inline(always)] static pure fn zero() -> mat4x4 { NumericMatrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn rows() -> uint { 4 }
    #[inline(always)] static pure fn cols() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<mat4x4>() }
}


pub impl dmat2 {
    #[inline(always)] static pure fn new(c0r0: f64, c0r1: f64, c1r0: f64, c1r1: f64)
        -> dmat2 { dmat2x2::new(c0r0, c0r1, c1r0, c1r1) }
    #[inline(always)] static pure fn from_cols(c0: dvec2, c1: dvec2)
        -> dmat2 { dmat2x2::from_cols(move c0, move c1) }
    #[inline(always)] static pure fn from_value(v: f64) -> dmat2 { dmat2x2::from_value(v) }
    #[inline(always)] static pure fn identity() -> dmat2 { dmat2x2::identity() }
    #[inline(always)] static pure fn zero() -> dmat2 { NumericMatrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { dmat2x2::dim() }
    #[inline(always)] static pure fn rows() -> uint { dmat2x2::rows() }
    #[inline(always)] static pure fn cols() -> uint { dmat2x2::cols() }
    #[inline(always)] static pure fn size_of() -> uint { dmat2x2::size_of() }
}

pub impl dmat3 {
    #[inline(always)] static pure fn new(c0r0: f64, c0r1: f64, c0r2: f64, c1r0: f64, c1r1: f64, c1r2: f64, c2r0: f64, c2r1: f64, c2r2: f64)
        -> dmat3 { dmat3x3::new(c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2) }
    #[inline(always)] static pure fn from_cols(c0: dvec3, c1: dvec3, c2: dvec3)
        -> dmat3 { dmat3x3::from_cols(move c0, move c1, move c2) }
    #[inline(always)] static pure fn identity() -> dmat3 { dmat3x3::identity() }
    #[inline(always)] static pure fn zero() -> dmat3 { dmat3x3::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { dmat3x3::dim() }
    #[inline(always)] static pure fn rows() -> uint { dmat3x3::rows() }
    #[inline(always)] static pure fn cols() -> uint { dmat3x3::cols() }
    #[inline(always)] static pure fn size_of() -> uint { dmat3x3::size_of() }
}

pub impl dmat4 {
    #[inline(always)] static pure fn new(c0r0: f64, c0r1: f64, c0r2: f64, c0r3: f64, c1r0: f64, c1r1: f64, c1r2: f64, c1r3: f64, c2r0: f64, c2r1: f64, c2r2: f64, c2r3: f64, c3r0: f64, c3r1: f64, c3r2: f64, c3r3: f64)
        -> dmat4 { dmat4x4::new(c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1, c3r2, c3r3) }
    #[inline(always)] static pure fn from_cols(c0: dvec4, c1: dvec4, c2: dvec4, c3: dvec4)
        -> dmat4 { dmat4x4::from_cols(move c0, move c1, move c2, move c3) }
    #[inline(always)] static pure fn identity() -> dmat4 { dmat4x4::identity() }
    #[inline(always)] static pure fn zero() -> dmat4 { dmat4x4::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { dmat4x4::dim() }
    #[inline(always)] static pure fn rows() -> uint { dmat4x4::rows() }
    #[inline(always)] static pure fn cols() -> uint { dmat4x4::cols() }
    #[inline(always)] static pure fn size_of() -> uint { dmat4x4::size_of() }
}

pub impl dmat2x2 {
    #[inline(always)] static pure fn new(c0r0: f64, c0r1: f64, c1r0: f64, c1r1: f64)
        -> dmat2x2 { Mat2::new(c0r0, c0r1, c1r0, c1r1) }
    #[inline(always)] static pure fn from_cols(c0: dvec2, c1: dvec2)
        -> dmat2x2 { Mat2::from_cols(move c0, move c1) }
    #[inline(always)] static pure fn from_value(v: f64) -> dmat2x2 { Mat2::from_value(v) }
    #[inline(always)] static pure fn identity() -> dmat2x2 { NumericMatrixNxN::identity() }
    #[inline(always)] static pure fn zero() -> dmat2x2 { NumericMatrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn rows() -> uint { 2 }
    #[inline(always)] static pure fn cols() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dmat2x2>() }
}

pub impl dmat3x3 {
    #[inline(always)] static pure fn new(c0r0: f64, c0r1: f64, c0r2: f64, c1r0: f64, c1r1: f64, c1r2: f64, c2r0: f64, c2r1: f64, c2r2: f64)
        -> dmat3x3 { Mat3::new(c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2) }
    #[inline(always)] static pure fn from_cols(c0: dvec3, c1: dvec3, c2: dvec3)
        -> dmat3x3 { Mat3::from_cols(move c0, move c1, move c2) }
    #[inline(always)] static pure fn from_value(v: f64) -> dmat3x3 { Mat3::from_value(v) }
    #[inline(always)] static pure fn identity() -> dmat3x3 { NumericMatrixNxN::identity() }
    #[inline(always)] static pure fn zero() -> dmat3x3 { NumericMatrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn rows() -> uint { 3 }
    #[inline(always)] static pure fn cols() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dmat3x3>() }
}

pub impl dmat4x4 {
    #[inline(always)] static pure fn new(c0r0: f64, c0r1: f64, c0r2: f64, c0r3: f64, c1r0: f64, c1r1: f64, c1r2: f64, c1r3: f64, c2r0: f64, c2r1: f64, c2r2: f64, c2r3: f64, c3r0: f64, c3r1: f64, c3r2: f64, c3r3: f64)
        -> dmat4x4 { Mat4::new(c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1, c3r2, c3r3) }
    #[inline(always)] static pure fn from_cols(c0: dvec4, c1: dvec4, c2: dvec4, c3: dvec4)
        -> dmat4x4 { Mat4::from_cols(move c0, move c1, move c2, move c3) }
    #[inline(always)] static pure fn from_value(v: f64) -> dmat4x4 { Mat4::from_value(v) }
    #[inline(always)] static pure fn identity() -> dmat4x4 { NumericMatrixNxN::identity() }
    #[inline(always)] static pure fn zero() -> dmat4x4 { NumericMatrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn rows() -> uint { 4 }
    #[inline(always)] static pure fn cols() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dmat4x4>() }
}


// Quaternion types

// These quaternion type aliases are not actually specified in the GLSL spec
// but they follow the same nomenclature

pub type quat4  = Quat<f32>;             /// a single-precision floating-point quaternion
pub type dquat4 = Quat<f64>;             /// a double-precision floating-point quaternion

// prevents "error: expected item"
priv fn hack() {}