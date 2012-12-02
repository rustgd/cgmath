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

use angle::{Angle, Radians, Degrees, Rotation, Euler};
use color::color::{RGB, RGBA, HSV, HSVA};
use mat::{Matrix, Mat2, Mat3, Mat4};
use vec::{Vector, NumericVector, Vec2, Vec3, Vec4};
use quat::{/*Quaternion, */Quat};


// Vector aliases, corresponding to Section 4.1.5 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

pub type vec2  = Vec2<f32>;             /// a two-component single-precision floating-point vector
pub type vec3  = Vec3<f32>;             /// a three-component single-precision floating-point vector
pub type vec4  = Vec4<f32>;             /// a four-component single-precision floating-point vector

pub type dvec2 = Vec2<f64>;             /// a two-component double-precision floating-point vector
pub type dvec3 = Vec3<f64>;             /// a three-component double-precision floating-point vector
pub type dvec4 = Vec4<f64>;             /// a four-component double-precision floating-point vector

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
    
    // FIXME: there's something wrong with static functions here!
    // #[inline(always)] static pure fn identity() -> mat2 { NumericMatrixNxN::identity() }
    // #[inline(always)] static pure fn zero() -> mat2 { NumericMatrix::zero() }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)] static pure fn identity() -> mat2 { Mat2::identity() }
    #[inline(always)] static pure fn zero() -> mat2 { Mat2::zero() }
    
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
    
    // FIXME: there's something wrong with static functions here!
    // #[inline(always)] static pure fn identity() -> mat3 { NumericMatrixNxN::identity() }
    // #[inline(always)] static pure fn zero() -> mat3 { NumericMatrix::zero() }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)] static pure fn identity() -> mat3 { Mat3::identity() }
    #[inline(always)] static pure fn zero() -> mat3 { Mat3::zero() }
    
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
    
    // FIXME: there's something wrong with static functions here!
    // #[inline(always)] static pure fn identity() -> mat4 { NumericMatrixNxN::identity() }
    // #[inline(always)] static pure fn zero() -> mat4 { NumericMatrix::zero() }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)] static pure fn identity() -> mat4 { Mat4::identity() }
    #[inline(always)] static pure fn zero() -> mat4 { Mat4::zero() }
    
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
    
    // FIXME: there's something wrong with static functions here!
    // #[inline(always)] static pure fn identity() -> dmat2 { NumericMatrixNxN::identity() }
    // #[inline(always)] static pure fn zero() -> dmat2 { NumericMatrix::zero() }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)] static pure fn identity() -> dmat2 { Mat2::identity() }
    #[inline(always)] static pure fn zero() -> dmat2 { Mat2::zero() }
    
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
    
    // FIXME: there's something wrong with static functions here!
    // #[inline(always)] static pure fn identity() -> dmat3 { NumericMatrixNxN::identity() }
    // #[inline(always)] static pure fn zero() -> dmat3 { NumericMatrix::zero() }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)] static pure fn identity() -> dmat3 { Mat3::identity() }
    #[inline(always)] static pure fn zero() -> dmat3 { Mat3::zero() }
    
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
    
    // FIXME: there's something wrong with static functions here!
    // #[inline(always)] static pure fn identity() -> dmat4 { NumericMatrixNxN::identity() }
    // #[inline(always)] static pure fn zero() -> dmat4 { NumericMatrix::zero() }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)] static pure fn identity() -> dmat4 { Mat4::identity() }
    #[inline(always)] static pure fn zero() -> dmat4 { Mat4::zero() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn rows() -> uint { 4 }
    #[inline(always)] static pure fn cols() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dmat4>() }
}


// Angle unit aliases. These are not present in the GLSL specification, but they
// follow roughly the same nomenclature.

pub type radians  = Radians<f32>;       /// a single-precision angle with floating-point radian units
pub type dradians = Radians<f64>;       /// a double-precision angle with floating-point radian units
pub type degrees  = Degrees<f32>;       /// a single-precision angle with floating-point degree units
pub type ddegrees = Degrees<f64>;       /// a double-precision angle with floating-point degree units
                                        //  TODO: not sure about 'ddegrees' - could be easy to mis-type


// Angle unit constructors

pub fn radians(theta: f32)  -> radians  { Radians(theta) }
pub fn dradians(theta: f64) -> dradians { Radians(theta) }
pub fn degrees(theta: f32)  -> degrees  { Degrees(theta) }
pub fn ddegrees(theta: f64) -> ddegrees { Degrees(theta) }

pub impl radians {
    #[inline(always)] static pure fn full_turn() -> radians  { Angle::full_turn() }
    #[inline(always)] static pure fn half_turn() -> radians  { Angle::half_turn() }
    #[inline(always)] static pure fn quadrant()  -> radians  { Angle::quadrant()  }
    #[inline(always)] static pure fn sextant()   -> radians  { Angle::sextant()   }
    #[inline(always)] static pure fn octant()    -> radians  { Angle::octant()    }
}

pub impl dradians {
    #[inline(always)] static pure fn full_turn() -> dradians { Angle::full_turn() }
    #[inline(always)] static pure fn half_turn() -> dradians { Angle::half_turn() }
    #[inline(always)] static pure fn quadrant()  -> dradians { Angle::quadrant()  }
    #[inline(always)] static pure fn sextant()   -> dradians { Angle::sextant()   }
    #[inline(always)] static pure fn octant()    -> dradians { Angle::octant()    }
}

pub impl degrees {
    #[inline(always)] static pure fn full_turn() -> degrees  { Angle::full_turn() }
    #[inline(always)] static pure fn half_turn() -> degrees  { Angle::half_turn() }
    #[inline(always)] static pure fn quadrant()  -> degrees  { Angle::quadrant()  }
    #[inline(always)] static pure fn sextant()   -> degrees  { Angle::sextant()   }
    #[inline(always)] static pure fn octant()    -> degrees  { Angle::octant()    }
}

pub impl ddegrees {
    #[inline(always)] static pure fn full_turn() -> ddegrees { Angle::full_turn() }
    #[inline(always)] static pure fn half_turn() -> ddegrees { Angle::half_turn() }
    #[inline(always)] static pure fn quadrant()  -> ddegrees { Angle::quadrant()  }
    #[inline(always)] static pure fn sextant()   -> ddegrees { Angle::sextant()   }
    #[inline(always)] static pure fn octant()    -> ddegrees { Angle::octant()    }
}


// Axis rotation aliases. These are not present in the GLSL specification, but
// they follow roughly the same nomenclature.

pub type rotation  = Rotation<f32>;       /// a single-precision floating-point axis rotation
pub type drotation = Rotation<f64>;       /// a double-precision floating-point axis rotation

pub impl rotation {
    #[inline(always)] static pure fn new(theta: radians, axis: vec3) -> rotation { Rotation::new(move theta, move axis) }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<rotation>() }
}

pub impl drotation {
    #[inline(always)] static pure fn new(theta: dradians, axis: dvec3) -> drotation { Rotation::new(move theta, move axis) }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<drotation>() }
}


// Axis rotation aliases. These are not present in the GLSL specification, but
// they follow roughly the same nomenclature.

pub type euler  = Euler<f32>;       /// single-precision floating-point euler angles (pitch/yaw/roll)
pub type deuler = Euler<f64>;       /// double-precision floating-point euler angles (pitch/yaw/roll)


// Quaternion aliases. These are not present in the GLSL specification, but
// they follow roughly the same nomenclature.

pub type quat4  = Quat<f32>;             /// a single-precision floating-point quaternion
pub type dquat4 = Quat<f64>;             /// a double-precision floating-point quaternion

// Color type aliases. Prefixing the colors with the type letter looked a little
// strange, so in this case I opted for a suffix. It actually loosely follows the
// nomanclature defined in [this article](http://www.opengl.org/wiki/Image_Formats#Color_formats)
// on the OpenGL wiki.

pub type rgb      = rgbf;           /// same as a `rgb32f`
pub type rgba     = rgbaf;          /// same as a `rgba32f`

pub type rgbf     = rgb32f;         /// same as a `rgb32f`
pub type rgb32f   = RGB<f32>;       /// a 32-bit floating-point RGB color with component values ranging from 0f32 to 1f32
pub type rgb64f   = RGB<f64>;       /// a 64-bit floating-point RGB color with component values ranging from 0f64 to 1f64

pub type rgbaf    = rgba32f;        /// same as a `rgba32f`
pub type rgba32f  = RGBA<f32>;      /// a 32-bit floating-point RGBA color with component values ranging from 0.0 to 1.0
pub type rgba64f  = RGBA<f64>;      /// a 64-bit floating-point RGBA color with component values ranging from 0.0 to 1.0

pub type rgbu     = rgb8u;          /// same as a `rgb8u`
pub type rgb8u    = RGB<u8>;        /// an 8-bit unsigned-integer RGB color with component values ranging from 0x00 to 0xFF
pub type rgb16u   = RGB<u16>;       /// a 16-bit unsigned-integer RGB color with component values ranging from 0x0000 to 0xFFFF
pub type rgb32u   = RGB<u32>;       /// a 32-bit unsigned-integer RGB color with component values ranging from 0x0000_0000 to 0xFFFF_FFFF
pub type rgb64u   = RGB<u64>;       /// a 64-bit unsigned-integer RGB color with component values ranging from 0x0000_0000 to 0xFFFF_FFFF

pub type rgbau    = rgba8u;         /// same as a `rgba8u`
pub type rgba8u   = RGBA<u8>;       /// an 8-bit unsigned-integer RGB color with component values ranging from 0x00 to 0xFF
pub type rgba16u  = RGBA<u16>;      /// a 16-bit unsigned-integer RGB color with component values ranging from 0x0000 to 0xFFFF
pub type rgba32u  = RGBA<u32>;      /// a 32-bit unsigned-integer RGB color with component values ranging from 0x0000_0000 to 0xFFFF_FFFF
pub type rgba64u  = RGBA<u64>;      /// a 364bit unsigned-integer RGB color with component values ranging from 0x0000_0000 to 0xFFFF_FFFF

pub type hsv      = hsvaf;          /// same as a `hsv32f`
pub type hsva     = hsvaf;          /// same as a `hsva32f`

pub type hsvf     = hsv32f;         /// same as a `hsv32f`
pub type hsv32f   = HSV<f32>;       /// TODO: documentation
pub type hsv64f   = HSV<f64>;       /// TODO: documentation

pub type hsvaf    = hsva32f;        /// same as a `hsva32f`
pub type hsva32f  = HSVA<f32>;      /// TODO: documentation
pub type hsva64f  = HSVA<f64>;      /// TODO: documentation

// TODO: Color method wrappers

// prevents "error: expected item"
priv fn hack() {}