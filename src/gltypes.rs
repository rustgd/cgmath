pub use mat::{Mat2, Mat3, Mat4};
pub use vec::{Vec2, Vec3, Vec4};
pub use quat::Quat;

use vec::{Vector, NumericVector};
use mat::{NumericMatrix, NumericMatrix_NxN};

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


//
// Wrappers to make working with static functions cleaner
//
// For example:   let v = dvec::identity();
// as opposed to: let v: dvec4 = NumericVector::identity();
//

pub impl vec2 {
    #[inline(always)] static pure fn new(x: f32, y: f32) -> vec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: f32) -> vec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> vec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> vec2 { NumericVector::zero()     }
}

pub impl vec3 {
    #[inline(always)] static pure fn new(x: f32, y: f32, z: f32) -> vec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: f32) -> vec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> vec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> vec3 { NumericVector::zero()     }
}

pub impl vec4 {
    #[inline(always)] static pure fn new(x: f32, y: f32, z: f32, w: f32) -> vec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: f32) -> vec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> vec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> vec4 { NumericVector::zero()     }
}


pub impl dvec2 {
    #[inline(always)] static pure fn new(x: f64, y: f64) -> dvec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec2 { NumericVector::zero()     }
}

pub impl dvec3 {
    #[inline(always)] static pure fn new(x: f64, y: f64, z: f64) -> dvec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec3 { NumericVector::zero()     }
}

pub impl dvec4 {
    #[inline(always)] static pure fn new(x: f64, y: f64, z: f64, w: f64) -> dvec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec4 { NumericVector::zero()     }
}


pub impl bvec2 {
    #[inline(always)] static pure fn new(x: bool, y: bool) -> bvec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec2 { Vector::from_value(v) }
    // #[inline(always)] static pure fn identity() -> bvec2 { NumericVector::identity() }
    // #[inline(always)] static pure fn zero() -> bvec2 { NumericVector::zero()     }
}

pub impl bvec3 {
    #[inline(always)] static pure fn new(x: bool, y: bool, z: bool) -> bvec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec3 { Vector::from_value(v) }
    // #[inline(always)] static pure fn identity() -> bvec3 { NumericVector::identity() }
    // #[inline(always)] static pure fn zero() -> bvec3 { NumericVector::zero()     }
}

pub impl bvec4 {
    #[inline(always)] static pure fn new(x: bool, y: bool, z: bool, w: bool) -> bvec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec4 { Vector::from_value(v) }
    // #[inline(always)] static pure fn identity() -> bvec4 { NumericVector::identity() }
    // #[inline(always)] static pure fn zero() -> bvec4 { NumericVector::zero()     }
}


pub impl ivec2 {
    #[inline(always)] static pure fn new(x: i32, y: i32) -> ivec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero()     -> ivec2 { NumericVector::zero()     }
}

pub impl ivec3 {
    #[inline(always)] static pure fn new(x: i32, y: i32, z: i32) -> ivec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero()     -> ivec3 { NumericVector::zero()     }
}

pub impl ivec4 {
    #[inline(always)] static pure fn new(x: i32, y: i32, z: i32, w: i32) -> ivec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero()     -> ivec4 { NumericVector::zero()     }
}


pub impl uvec2 {
    #[inline(always)] static pure fn new(x: u32, y: u32) -> uvec2 { Vec2::new(x, y) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec2 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec2 { NumericVector::identity() }
    #[inline(always)] static pure fn zero()     -> uvec2 { NumericVector::zero()     }
}

pub impl uvec3 {
    #[inline(always)] static pure fn new(x: u32, y: u32, z: u32) -> uvec3 { Vec3::new(x, y, z) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec3 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec3 { NumericVector::identity() }
    #[inline(always)] static pure fn zero()     -> uvec3 { NumericVector::zero()     }
}

pub impl uvec4 {
    #[inline(always)] static pure fn new(x: u32, y: u32, z: u32, w: u32) -> uvec4 { Vec4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero()     -> uvec4 { NumericVector::zero()     }
}


// Matrix aliases

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


//
// Wrappers to make working with static functions cleaner
//
// For example:   let m = dmat::identity();
// as opposed to: let m: dmat4 = NumericMatrix_NxN::identity();
//

pub impl mat2  {
    #[inline(always)] static pure fn identity() -> mat2  { NumericMatrix_NxN::identity() }
    #[inline(always)] static pure fn zero()     -> mat2  { NumericMatrix::zero()         }
}

pub impl mat3  {
    #[inline(always)] static pure fn identity() -> mat3  { NumericMatrix_NxN::identity() }
    #[inline(always)] static pure fn zero()     -> mat3  { NumericMatrix::zero()         }
}

pub impl mat4  {
    #[inline(always)] static pure fn identity() -> mat4  { NumericMatrix_NxN::identity() }
    #[inline(always)] static pure fn zero()     -> mat4  { NumericMatrix::zero()         }
}


pub impl dmat2 {
    #[inline(always)] static pure fn identity() -> dmat2 { NumericMatrix_NxN::identity() }
    #[inline(always)] static pure fn zero()     -> dmat2 { NumericMatrix::zero()         }
}

pub impl dmat3 {
    #[inline(always)] static pure fn identity() -> dmat3 { NumericMatrix_NxN::identity() }
    #[inline(always)] static pure fn zero()     -> dmat3 { NumericMatrix::zero()         }
}

pub impl dmat4 {
    #[inline(always)] static pure fn identity() -> dmat4 { NumericMatrix_NxN::identity() }
    #[inline(always)] static pure fn zero()     -> dmat4 { NumericMatrix::zero()         }
}


// Quaternion types

// These quaternion type aliases are not actually specified in the GLSL spec
// but they follow the same nomenclature

pub type quat4  = Quat<f32>;             /// a single-precision floating-point quaternion
pub type dquat4 = Quat<f64>;             /// a double-precision floating-point quaternion

// prevents "error: expected item"
priv fn hack() {}