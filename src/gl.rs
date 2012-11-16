pub use mat::{Mat2, Mat3, Mat4};
pub use vec::{Vec2, Vec3, Vec4};
pub use quat::Quat;


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


// These quaternion type aliases are not actually specified in the GLSL spec
// but they follow the same nomenclature

pub type quat4  = Quat<f32>;             /// a single-precision floating-point quaternion
pub type dquat4 = Quat<f64>;             /// a double-precision floating-point quaternion