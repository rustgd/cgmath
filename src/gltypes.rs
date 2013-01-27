/***
 * This module contains various type aliases and method wrappers to make working
 * with OpenGL cleaner and safer than working with the lmath types directly.
 * This is especially important when working with type-sensitive OpenGL functions
 * such as `glVertexAttribPointer` and `glUniformMatrix4fv`) where a simple mistake
 * such writing `Vec3::new(1, 2, 3)` or `Vec3::new(1f, 2f, 3f)` as opposed to
 * `Vec3::new(1f32, 2f32, 3f32)` could cause you an afternoon of pain.
 *
 * To give you an example of how using the wrapper methods can clean up your
 * code and make debugging far easier, instead of writing:
 *
 * ~~~
 * let v: Mat4<f64> = NumericMatrixNxN::identity();
 * ~~~
 *
 * you can write:
 *
 * ~~~
 * let v = dmat4::identity();
 * ~~~
 */

use core::sys::size_of;

use vec::{
    Vector,
    NumericVector,
    NumericVector2,
    NumericVector3,
    NumericVector4,
    Vec2,
    Vec3,
    Vec4,
};

use mat::{Matrix, Mat2, Mat3, Mat4};
use quat::Quat;

use numeric::*;


// Vector aliases, corresponding to Section 4.1.5 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

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
    
    #[inline(always)] static pure fn unit_x() -> vec2 { NumericVector2::unit_x() }
    #[inline(always)] static pure fn unit_y() -> vec2 { NumericVector2::unit_y() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<vec2>() }
}

pub impl vec3 {
    #[inline(always)] static pure fn new(x: f32, y: f32, z: f32) -> vec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: f32) -> vec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> vec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> vec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> vec3 { NumericVector3::unit_x() }
    #[inline(always)] static pure fn unit_y() -> vec3 { NumericVector3::unit_y() }
    #[inline(always)] static pure fn unit_z() -> vec3 { NumericVector3::unit_z() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<vec3>() }
}

pub impl vec4 {
    #[inline(always)] static pure fn new(x: f32, y: f32, z: f32, w: f32) -> vec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: f32) -> vec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> vec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> vec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> vec4 { NumericVector4::unit_x() }
    #[inline(always)] static pure fn unit_y() -> vec4 { NumericVector4::unit_y() }
    #[inline(always)] static pure fn unit_z() -> vec4 { NumericVector4::unit_z() }
    #[inline(always)] static pure fn unit_w() -> vec4 { NumericVector4::unit_w() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<vec4>() }
}


pub impl dvec2 {
    #[inline(always)] static pure fn new(x: f64, y: f64) -> dvec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec2 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> dvec2 { NumericVector2::unit_x() }
    #[inline(always)] static pure fn unit_y() -> dvec2 { NumericVector2::unit_y() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dvec2>() }
}

pub impl dvec3 {
    #[inline(always)] static pure fn new(x: f64, y: f64, z: f64) -> dvec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> dvec3 { NumericVector3::unit_x() }
    #[inline(always)] static pure fn unit_y() -> dvec3 { NumericVector3::unit_y() }
    #[inline(always)] static pure fn unit_z() -> dvec3 { NumericVector3::unit_z() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dvec3>() }
}

pub impl dvec4 {
    #[inline(always)] static pure fn new(x: f64, y: f64, z: f64, w: f64) -> dvec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> dvec4 { NumericVector4::unit_x() }
    #[inline(always)] static pure fn unit_y() -> dvec4 { NumericVector4::unit_y() }
    #[inline(always)] static pure fn unit_z() -> dvec4 { NumericVector4::unit_z() }
    #[inline(always)] static pure fn unit_w() -> dvec4 { NumericVector4::unit_w() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dvec4>() }
}
 
 
pub impl bvec2 {
    #[inline(always)] static pure fn new(x: bool, y: bool) -> bvec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec2 { Vector::from_value(v) }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<bvec2>() }
}
 
pub impl bvec3 {
    #[inline(always)] static pure fn new(x: bool, y: bool, z: bool) -> bvec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec3 { Vector::from_value(v) }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<bvec3>() }
}
 
pub impl bvec4 {
    #[inline(always)] static pure fn new(x: bool, y: bool, z: bool, w: bool) -> bvec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec4 { Vector::from_value(v) }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<bvec4>() }
}


pub impl ivec2 {
    #[inline(always)] static pure fn new(x: i32, y: i32) -> ivec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> ivec2 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> ivec2 { NumericVector2::unit_x() }
    #[inline(always)] static pure fn unit_y() -> ivec2 { NumericVector2::unit_y() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<ivec2>() }
}

pub impl ivec3 {
    #[inline(always)] static pure fn new(x: i32, y: i32, z: i32) -> ivec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> ivec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> ivec3 { NumericVector3::unit_x() }
    #[inline(always)] static pure fn unit_y() -> ivec3 { NumericVector3::unit_y() }
    #[inline(always)] static pure fn unit_z() -> ivec3 { NumericVector3::unit_z() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<ivec3>() }
}

pub impl ivec4 {
    #[inline(always)] static pure fn new(x: i32, y: i32, z: i32, w: i32) -> ivec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> ivec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> ivec4 { NumericVector4::unit_x() }
    #[inline(always)] static pure fn unit_y() -> ivec4 { NumericVector4::unit_y() }
    #[inline(always)] static pure fn unit_z() -> ivec4 { NumericVector4::unit_z() }
    #[inline(always)] static pure fn unit_w() -> ivec4 { NumericVector4::unit_w() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<ivec4>() }
}


pub impl uvec2 {
    #[inline(always)] static pure fn new(x: u32, y: u32) -> uvec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> uvec2 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> uvec2 { NumericVector2::unit_x() }
    #[inline(always)] static pure fn unit_y() -> uvec2 { NumericVector2::unit_y() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<uvec2>() }
}

pub impl uvec3 {
    #[inline(always)] static pure fn new(x: u32, y: u32, z: u32) -> uvec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> uvec3 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> uvec3 { NumericVector3::unit_x() }
    #[inline(always)] static pure fn unit_y() -> uvec3 { NumericVector3::unit_y() }
    #[inline(always)] static pure fn unit_z() -> uvec3 { NumericVector3::unit_z() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<uvec3>() }
}

pub impl uvec4 {
    #[inline(always)] static pure fn new(x: u32, y: u32, z: u32, w: u32) -> uvec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> uvec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> uvec4 { NumericVector4::unit_x() }
    #[inline(always)] static pure fn unit_y() -> uvec4 { NumericVector4::unit_y() }
    #[inline(always)] static pure fn unit_z() -> uvec4 { NumericVector4::unit_z() }
    #[inline(always)] static pure fn unit_w() -> uvec4 { NumericVector4::unit_w() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<uvec4>() }
}


// Matrix aliases, corresponding to Section 4.1.6 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

pub type mat2 = Mat2<f32>;              /// a 2×2 single-precision floating-point matrix
pub type mat3 = Mat3<f32>;              /// a 3×3 single-precision floating-point matrix
pub type mat4 = Mat4<f32>;              /// a 4×4 single-precision floating-point matrix

pub type dmat2 = Mat2<f64>;             /// a 2×2 double-precision floating-point matrix
pub type dmat3 = Mat3<f64>;             /// a 3×3 double-precision floating-point matrix
pub type dmat4 = Mat4<f64>;             /// a 4×4 double-precision floating-point matrix


// Matrix method wrappers

pub impl mat2 {
    #[inline(always)] static pure fn new(c0r0: f32, c0r1: f32, c1r0: f32, c1r1: f32)
        -> mat2 { Mat2::new(c0r0, c0r1, c1r0, c1r1) }
    #[inline(always)] static pure fn from_cols(c0: vec2, c1: vec2)
        -> mat2 { Mat2::from_cols(move c0, move c1) }
    #[inline(always)] static pure fn from_value(v: f32) -> mat2 { Mat2::from_value(v) }
    
    #[inline(always)] static pure fn identity() -> mat2 { Matrix::identity() }
    #[inline(always)] static pure fn zero() -> mat2 { Matrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn rows() -> uint { 2 }
    #[inline(always)] static pure fn cols() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<mat2>() }
}

pub impl mat3 {
    #[inline(always)] static pure fn new(c0r0: f32, c0r1: f32, c0r2: f32, c1r0: f32, c1r1: f32, c1r2: f32, c2r0: f32, c2r1: f32, c2r2: f32)
        -> mat3 { Mat3::new(c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2) }
    #[inline(always)] static pure fn from_cols(c0: vec3, c1: vec3, c2: vec3)
        -> mat3 { Mat3::from_cols(move c0, move c1, move c2) }
    #[inline(always)] static pure fn from_value(v: f32) -> mat3 { Mat3::from_value(v) }
    
    #[inline(always)] static pure fn identity() -> mat3 { Matrix::identity() }
    #[inline(always)] static pure fn zero() -> mat3 { Matrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn rows() -> uint { 3 }
    #[inline(always)] static pure fn cols() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<mat3>() }
}

pub impl mat4 {
    #[inline(always)] static pure fn new(c0r0: f32, c0r1: f32, c0r2: f32, c0r3: f32, c1r0: f32, c1r1: f32, c1r2: f32, c1r3: f32, c2r0: f32, c2r1: f32, c2r2: f32, c2r3: f32, c3r0: f32, c3r1: f32, c3r2: f32, c3r3: f32)
        -> mat4 { Mat4::new(c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1, c3r2, c3r3) }
    #[inline(always)] static pure fn from_cols(c0: vec4, c1: vec4, c2: vec4, c3: vec4)
        -> mat4 { Mat4::from_cols(move c0, move c1, move c2, move c3) }
    #[inline(always)] static pure fn from_value(v: f32) -> mat4 { Mat4::from_value(v) }
    
    #[inline(always)] static pure fn identity() -> mat4 { Matrix::identity() }
    #[inline(always)] static pure fn zero() -> mat4 { Matrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn rows() -> uint { 4 }
    #[inline(always)] static pure fn cols() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<mat4>() }
}


pub impl dmat2 {
    #[inline(always)] static pure fn new(c0r0: f64, c0r1: f64, c1r0: f64, c1r1: f64)
        -> dmat2 { Mat2::new(c0r0, c0r1, c1r0, c1r1) }
    #[inline(always)] static pure fn from_cols(c0: dvec2, c1: dvec2)
        -> dmat2 { Mat2::from_cols(move c0, move c1) }
    #[inline(always)] static pure fn from_value(v: f64) -> dmat2 { Mat2::from_value(v) }
    
    #[inline(always)] static pure fn identity() -> dmat2 { Matrix::identity() }
    #[inline(always)] static pure fn zero() -> dmat2 { Matrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 2 }
    #[inline(always)] static pure fn rows() -> uint { 2 }
    #[inline(always)] static pure fn cols() -> uint { 2 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dmat2>() }
}

pub impl dmat3 {
    #[inline(always)] static pure fn new(c0r0: f64, c0r1: f64, c0r2: f64, c1r0: f64, c1r1: f64, c1r2: f64, c2r0: f64, c2r1: f64, c2r2: f64)
        -> dmat3 { Mat3::new(c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2) }
    #[inline(always)] static pure fn from_cols(c0: dvec3, c1: dvec3, c2: dvec3)
        -> dmat3 { Mat3::from_cols(move c0, move c1, move c2) }
    #[inline(always)] static pure fn from_value(v: f64) -> dmat3 { Mat3::from_value(v) }
    
    #[inline(always)] static pure fn identity() -> dmat3 { Matrix::identity() }
    #[inline(always)] static pure fn zero() -> dmat3 { Matrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 3 }
    #[inline(always)] static pure fn rows() -> uint { 3 }
    #[inline(always)] static pure fn cols() -> uint { 3 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dmat3>() }
}

pub impl dmat4 {
    #[inline(always)] static pure fn new(c0r0: f64, c0r1: f64, c0r2: f64, c0r3: f64, c1r0: f64, c1r1: f64, c1r2: f64, c1r3: f64, c2r0: f64, c2r1: f64, c2r2: f64, c2r3: f64, c3r0: f64, c3r1: f64, c3r2: f64, c3r3: f64)
        -> dmat4 { Mat4::new(c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1, c3r2, c3r3) }
    #[inline(always)] static pure fn from_cols(c0: dvec4, c1: dvec4, c2: dvec4, c3: dvec4)
        -> dmat4 { Mat4::from_cols(move c0, move c1, move c2, move c3) }
    #[inline(always)] static pure fn from_value(v: f64) -> dmat4 { Mat4::from_value(v) }
    
    #[inline(always)] static pure fn identity() -> dmat4 { Matrix::identity() }
    #[inline(always)] static pure fn zero() -> dmat4 { Matrix::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn rows() -> uint { 4 }
    #[inline(always)] static pure fn cols() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dmat4>() }
}


// Quaternion aliases. These are not present in the GLSL specification, but
// they follow roughly the same nomenclature.

pub type quat4  = Quat<f32>;             /// a single-precision floating-point quaternion
pub type dquat4 = Quat<f64>;             /// a double-precision floating-point quaternion




// prevents "error: expected item"
priv fn hack() {}