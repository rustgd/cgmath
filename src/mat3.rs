use core::cast::transmute;
use core::sys::size_of;
use core::vec::raw::buf_as_slice;

use std::cmp::{FuzzyEq, FUZZY_EPSILON};
use numeric::*;
use numeric::number::Number;
use numeric::number::Number::{zero,one};

use quat::Quat;

use vec::{
    Vec3,
    Vector3,
    Vector,
    NumericVector,
    NumericVector3,
    EuclideanVector,
    vec3,
    dvec3,
};

use mat::{
    Mat4,
    Matrix,
    Matrix3,
    Matrix4,
};

/**
 *  A 3 x 3 column major matrix
 *
 * # Type parameters
 *
 * * `T` - The type of the elements of the matrix. Should be a floating point type.
 *
 * # Fields
 *
 * * `x` - the first column vector of the matrix
 * * `y` - the second column vector of the matrix
 * * `z` - the third column vector of the matrix
 */
#[deriving(Eq)]
pub struct Mat3<T> { x: Vec3<T>, y: Vec3<T>, z: Vec3<T> }

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Matrix<T, Vec3<T>> for Mat3<T> {
    #[inline(always)]
    fn col(&self, i: uint) -> Vec3<T> { self[i] }

    #[inline(always)]
    fn row(&self, i: uint) -> Vec3<T> {
        Vector3::new(self[0][i],
                     self[1][i],
                     self[2][i])
    }

    /**
     * Construct a 3 x 3 diagonal matrix with the major diagonal set to `value`
     *
     * # Arguments
     *
     * * `value` - the value to set the major diagonal to
     *
     * ~~~
     *        c0    c1    c2
     *     +-----+-----+-----+
     *  r0 | val |   0 |   0 |
     *     +-----+-----+-----+
     *  r1 |   0 | val |   0 |
     *     +-----+-----+-----+
     *  r2 |   0 |   0 | val |
     *     +-----+-----+-----+
     * ~~~
     */
    #[inline(always)]
    fn from_value(value: T) -> Mat3<T> {
        Matrix3::new(value, zero(), zero(),
                     zero(), value, zero(),
                     zero(), zero(), value)
    }

    /**
     * Returns the multiplicative identity matrix
     * ~~~
     *       c0   c1   c2
     *     +----+----+----+
     *  r0 |  1 |  0 |  0 |
     *     +----+----+----+
     *  r1 |  0 |  1 |  0 |
     *     +----+----+----+
     *  r2 |  0 |  0 |  1 |
     *     +----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn identity() -> Mat3<T> {
        Matrix3::new( one::<T>(), zero::<T>(), zero::<T>(),
                     zero::<T>(),  one::<T>(), zero::<T>(),
                     zero::<T>(), zero::<T>(),  one::<T>())
    }

    /**
     * Returns the additive identity matrix
     * ~~~
     *       c0   c1   c2
     *     +----+----+----+
     *  r0 |  0 |  0 |  0 |
     *     +----+----+----+
     *  r1 |  0 |  0 |  0 |
     *     +----+----+----+
     *  r2 |  0 |  0 |  0 |
     *     +----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn zero() -> Mat3<T> {
        Matrix3::new(zero::<T>(), zero::<T>(), zero::<T>(),
                     zero::<T>(), zero::<T>(), zero::<T>(),
                     zero::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn mul_t(&self, value: T) -> Mat3<T> {
        Matrix3::from_cols(self[0].mul_t(value),
                           self[1].mul_t(value),
                           self[2].mul_t(value))
    }

    #[inline(always)]
    fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T> {
        Vector3::new(self.row(0).dot(vec),
                     self.row(1).dot(vec),
                     self.row(2).dot(vec))
    }

    #[inline(always)]
    fn add_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Matrix3::from_cols(self[0].add_v(&other[0]),
                           self[1].add_v(&other[1]),
                           self[2].add_v(&other[2]))
    }

    #[inline(always)]
    fn sub_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Matrix3::from_cols(self[0].sub_v(&other[0]),
                           self[1].sub_v(&other[1]),
                           self[2].sub_v(&other[2]))
    }

    #[inline(always)]
    fn mul_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Matrix3::new(self.row(0).dot(&other.col(0)),
                     self.row(1).dot(&other.col(0)),
                     self.row(2).dot(&other.col(0)),

                     self.row(0).dot(&other.col(1)),
                     self.row(1).dot(&other.col(1)),
                     self.row(2).dot(&other.col(1)),

                     self.row(0).dot(&other.col(2)),
                     self.row(1).dot(&other.col(2)),
                     self.row(2).dot(&other.col(2)))
    }

    fn dot(&self, other: &Mat3<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    fn determinant(&self) -> T {
        self.col(0).dot(&self.col(1).cross(&self.col(2)))
    }

    fn trace(&self) -> T {
        self[0][0] + self[1][1] + self[2][2]
    }

    // #[inline(always)]
    fn inverse(&self) -> Option<Mat3<T>> {
        let d = self.determinant();
        if d.fuzzy_eq(&zero()) {
            None
        } else {
            let m: Mat3<T> = Matrix3::from_cols(self[1].cross(&self[2]).div_t(d),
                                                self[2].cross(&self[0]).div_t(d),
                                                self[0].cross(&self[1]).div_t(d));
            Some(m.transpose())
        }
    }

    #[inline(always)]
    fn transpose(&self) -> Mat3<T> {
        Matrix3::new(self[0][0], self[1][0], self[2][0],
                     self[0][1], self[1][1], self[2][1],
                     self[0][2], self[1][2], self[2][2])
    }
    
    #[inline(always)]
    fn col_mut(&mut self, i: uint) -> &'self mut Vec3<T> {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => fail!(fmt!("index out of bounds: expected an index from 0 to 2, but found %u", i))
        }
    }

    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        *self.col_mut(a) <-> *self.col_mut(b);
    }

    #[inline(always)]
    fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
        self.z.swap(a, b);
    }

    #[inline(always)]
    fn set(&mut self, other: &Mat3<T>) {
        (*self) = (*other);
    }

    #[inline(always)]
    fn to_identity(&mut self) {
        (*self) = Matrix::identity();
    }

    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = Matrix::zero();
    }

    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(value);
        self.col_mut(1).mul_self_t(value);
        self.col_mut(2).mul_self_t(value);
    }

    #[inline(always)]
    fn add_self_m(&mut self, other: &Mat3<T>) {
        self.col_mut(0).add_self_v(&other[0]);
        self.col_mut(1).add_self_v(&other[1]);
        self.col_mut(2).add_self_v(&other[2]);
    }

    #[inline(always)]
    fn sub_self_m(&mut self, other: &Mat3<T>) {
        self.col_mut(0).sub_self_v(&other[0]);
        self.col_mut(1).sub_self_v(&other[1]);
        self.col_mut(2).sub_self_v(&other[2]);
    }

    #[inline(always)]
    fn invert_self(&mut self) {
        match self.inverse() {
            Some(m) => (*self) = m,
            None => fail!(~"Couldn't invert the matrix!")
        }
    }

    #[inline(always)]
    fn transpose_self(&mut self) {
        *self.col_mut(0).index_mut(1) <-> *self.col_mut(1).index_mut(0);
        *self.col_mut(0).index_mut(2) <-> *self.col_mut(2).index_mut(0);

        *self.col_mut(1).index_mut(0) <-> *self.col_mut(0).index_mut(1);
        *self.col_mut(1).index_mut(2) <-> *self.col_mut(2).index_mut(1);

        *self.col_mut(2).index_mut(0) <-> *self.col_mut(0).index_mut(2);
        *self.col_mut(2).index_mut(1) <-> *self.col_mut(1).index_mut(2);
    }

    #[inline(always)]
    fn is_identity(&self) -> bool {
        self.fuzzy_eq(&Matrix::identity())
    }

    #[inline(always)]
    fn is_diagonal(&self) -> bool {
        self[0][1].fuzzy_eq(&zero()) &&
        self[0][2].fuzzy_eq(&zero()) &&

        self[1][0].fuzzy_eq(&zero()) &&
        self[1][2].fuzzy_eq(&zero()) &&

        self[2][0].fuzzy_eq(&zero()) &&
        self[2][1].fuzzy_eq(&zero())
    }

    #[inline(always)]
    fn is_rotated(&self) -> bool {
        !self.fuzzy_eq(&Matrix::identity())
    }

    #[inline(always)]
    fn is_symmetric(&self) -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[0][2].fuzzy_eq(&self[2][0]) &&

        self[1][0].fuzzy_eq(&self[0][1]) &&
        self[1][2].fuzzy_eq(&self[2][1]) &&

        self[2][0].fuzzy_eq(&self[0][2]) &&
        self[2][1].fuzzy_eq(&self[1][2])
    }

    #[inline(always)]
    fn is_invertible(&self) -> bool {
        !self.determinant().fuzzy_eq(&zero())
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { transmute(self) }
    }
}

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Matrix3<T, Vec3<T>> for Mat3<T> {
    /**
     * Construct a 3 x 3 matrix
     *
     * # Arguments
     *
     * * `c0r0`, `c0r1`, `c0r2` - the first column of the matrix
     * * `c1r0`, `c1r1`, `c1r2` - the second column of the matrix
     * * `c2r0`, `c2r1`, `c2r2` - the third column of the matrix
     *
     * ~~~
     *         c0     c1     c2
     *      +------+------+------+
     *   r0 | c0r0 | c1r0 | c2r0 |
     *      +------+------+------+
     *   r1 | c0r1 | c1r1 | c2r1 |
     *      +------+------+------+
     *   r2 | c0r2 | c1r2 | c2r2 |
     *      +------+------+------+
     * ~~~
     */
    #[inline(always)]
    fn new(c0r0:T, c0r1:T, c0r2:T,
                       c1r0:T, c1r1:T, c1r2:T,
                       c2r0:T, c2r1:T, c2r2:T) -> Mat3<T> {
        Matrix3::from_cols(Vector3::new::<T,Vec3<T>>(c0r0, c0r1, c0r2),
                           Vector3::new::<T,Vec3<T>>(c1r0, c1r1, c1r2),
                           Vector3::new::<T,Vec3<T>>(c2r0, c2r1, c2r2))
    }

    /**
     * Construct a 3 x 3 matrix from column vectors
     *
     * # Arguments
     *
     * * `c0` - the first column vector of the matrix
     * * `c1` - the second column vector of the matrix
     * * `c2` - the third column vector of the matrix
     *
     * ~~~
     *        c0     c1     c2
     *     +------+------+------+
     *  r0 | c0.x | c1.x | c2.x |
     *     +------+------+------+
     *  r1 | c0.y | c1.y | c2.y |
     *     +------+------+------+
     *  r2 | c0.z | c1.z | c2.z |
     *     +------+------+------+
     * ~~~
     */
    #[inline(always)]
    fn from_cols(c0: Vec3<T>,
                             c1: Vec3<T>,
                             c2: Vec3<T>) -> Mat3<T> {
        Mat3 { x: c0, y: c1, z: c2 }
    }

    /**
     * Construct a matrix from an angular rotation around the `x` axis
     */
    #[inline(always)]
    fn from_angle_x(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = cos(radians);
        let sin_theta = sin(radians);

        Matrix3::new( one(),     zero(),    zero(),
                     zero(),  cos_theta, sin_theta,
                     zero(), -sin_theta, cos_theta)
    }

    /**
     * Construct a matrix from an angular rotation around the `y` axis
     */
    #[inline(always)]
    fn from_angle_y(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = cos(radians);
        let sin_theta = sin(radians);

        Matrix3::new(cos_theta, zero(), -sin_theta,
                        zero(),  one(),     zero(),
                     sin_theta, zero(),  cos_theta)
    }

    /**
     * Construct a matrix from an angular rotation around the `z` axis
     */
    #[inline(always)]
    fn from_angle_z(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = cos(radians);
        let sin_theta = sin(radians);

        Matrix3::new( cos_theta, sin_theta, zero(),
                     -sin_theta, cos_theta, zero(),
                         zero(),    zero(),  one())
    }

    /**
     * Construct a matrix from Euler angles
     *
     * # Arguments
     *
     * * `theta_x` - the angular rotation around the `x` axis (pitch)
     * * `theta_y` - the angular rotation around the `y` axis (yaw)
     * * `theta_z` - the angular rotation around the `z` axis (roll)
     */
    #[inline(always)]
    fn from_angle_xyz(radians_x: T, radians_y: T, radians_z: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
        let cx = cos(radians_x);
        let sx = sin(radians_x);
        let cy = cos(radians_y);
        let sy = sin(radians_y);
        let cz = cos(radians_z);
        let sz = sin(radians_z);

        Matrix3::new(            cy*cz,             cy*sz,   -sy,
                     -cx*sz + sx*sy*cz,  cx*cz + sx*sy*sz, sx*cy,
                      sx*sz + cx*sy*cz, -sx*cz + cx*sy*sz, cx*cy)
    }

    /**
     * Construct a matrix from an axis and an angular rotation
     */
    #[inline(always)]
    fn from_angle_axis(radians: T, axis: &Vec3<T>) -> Mat3<T> {
        let c = cos(radians);
        let s = sin(radians);
        let _1_c = one::<T>() - c;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        Matrix3::new(_1_c*x*x + c,   _1_c*x*y + s*z, _1_c*x*z - s*y,
                   _1_c*x*y - s*z, _1_c*y*y + c,   _1_c*y*z + s*x,
                   _1_c*x*z + s*y, _1_c*y*z - s*x, _1_c*z*z + c)
    }

    #[inline(always)]
    fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat3<T> {
        Matrix3::from_cols(x, y, z)
    }

    #[inline(always)]
    fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> Mat3<T> {
        let dir_ = dir.normalize();
        let side = dir_.cross(&up.normalize());
        let up_  = side.cross(&dir_).normalize();

        Matrix3::from_axes(up_, side, dir_)
    }

    /**
     * Returns the the matrix with an extra row and column added
     * ~~~
     *       c0   c1   c2                 c0   c1   c2   c3
     *     +----+----+----+             +----+----+----+----+
     *  r0 |  a |  b |  c |          r0 |  a |  b |  c |  0 |
     *     +----+----+----+             +----+----+----+----+
     *  r1 |  d |  e |  f |    =>    r1 |  d |  e |  f |  0 |
     *     +----+----+----+             +----+----+----+----+
     *  r2 |  g |  h |  i |          r2 |  g |  h |  i |  0 |
     *     +----+----+----+             +----+----+----+----+
     *                               r3 |  0 |  0 |  0 |  1 |
     *                                  +----+----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn to_mat4(&self) -> Mat4<T> {
        Matrix4::new(self[0][0], self[0][1], self[0][2], zero(),
                     self[1][0], self[1][1], self[1][2], zero(),
                     self[2][0], self[2][1], self[2][2], zero(),
                         zero(),     zero(),     zero(),  one())
    }

    /**
     * Convert the matrix to a quaternion
     */
    #[inline(always)]
    fn to_quat(&self) -> Quat<T> {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf

        let mut s;
        let w, x, y, z;
        let trace = self.trace();

        let _1:   T = Number::from(1.0);
        let half: T = Number::from(0.5);

        if trace >= zero() {
            s = (_1 + trace).sqrt();
            w = half * s;
            s = half / s;
            x = (self[1][2] - self[2][1]) * s;
            y = (self[2][0] - self[0][2]) * s;
            z = (self[0][1] - self[1][0]) * s;
        } else if (self[0][0] > self[1][1]) && (self[0][0] > self[2][2]) {
            s = (half + (self[0][0] - self[1][1] - self[2][2])).sqrt();
            w = half * s;
            s = half / s;
            x = (self[0][1] - self[1][0]) * s;
            y = (self[2][0] - self[0][2]) * s;
            z = (self[1][2] - self[2][1]) * s;
        } else if self[1][1] > self[2][2] {
            s = (half + (self[1][1] - self[0][0] - self[2][2])).sqrt();
            w = half * s;
            s = half / s;
            x = (self[0][1] - self[1][0]) * s;
            y = (self[1][2] - self[2][1]) * s;
            z = (self[2][0] - self[0][2]) * s;
        } else {
            s = (half + (self[2][2] - self[0][0] - self[1][1])).sqrt();
            w = half * s;
            s = half / s;
            x = (self[2][0] - self[0][2]) * s;
            y = (self[1][2] - self[2][1]) * s;
            z = (self[0][1] - self[1][0]) * s;
        }

        Quat::new(w, x, y, z)
    }
}

impl<T:Copy> Index<uint, Vec3<T>> for Mat3<T> {
    #[inline(always)]
    fn index(&self, i: &uint) -> Vec3<T> {
        unsafe { do buf_as_slice(transmute(self), 3) |slice| { slice[*i] } }
    }
}

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Neg<Mat3<T>> for Mat3<T> {
    #[inline(always)]
    fn neg(&self) -> Mat3<T> {
        Matrix3::from_cols(-self[0], -self[1], -self[2])
    }
}

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> FuzzyEq<T> for Mat3<T> {
    #[inline(always)]
    fn fuzzy_eq(&self, other: &Mat3<T>) -> bool {
        self.fuzzy_eq_eps(other, &Number::from(FUZZY_EPSILON))
    }

    #[inline(always)]
    fn fuzzy_eq_eps(&self, other: &Mat3<T>, epsilon: &T) -> bool {
        self[0].fuzzy_eq_eps(&other[0], epsilon) &&
        self[1].fuzzy_eq_eps(&other[1], epsilon) &&
        self[2].fuzzy_eq_eps(&other[2], epsilon)
    }
}

// GLSL-style type aliases, corresponding to Section 4.1.6 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

/// a 3×3 single-precision floating-point matrix
pub type mat3 = Mat3<f32>;
/// a 3×3 double-precision floating-point matrix
pub type dmat3 = Mat3<f64>;

// Static method wrappers for GLSL-style types

pub impl mat3 {
    #[inline(always)] fn new(c0r0: f32, c0r1: f32, c0r2: f32, c1r0: f32, c1r1: f32, c1r2: f32, c2r0: f32, c2r1: f32, c2r2: f32)
        -> mat3 { Matrix3::new(c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2) }
    #[inline(always)] fn from_cols(c0: vec3, c1: vec3, c2: vec3)
        -> mat3 { Matrix3::from_cols(c0, c1, c2) }
    #[inline(always)] fn from_value(v: f32) -> mat3 { Matrix::from_value(v) }

    #[inline(always)] fn identity() -> mat3 { Matrix::identity() }
    #[inline(always)] fn zero() -> mat3 { Matrix::zero() }

    #[inline(always)] fn from_angle_x(radians: f32) -> mat3 { Matrix3::from_angle_x(radians) }
    #[inline(always)] fn from_angle_y(radians: f32) -> mat3 { Matrix3::from_angle_y(radians) }
    #[inline(always)] fn from_angle_z(radians: f32) -> mat3 { Matrix3::from_angle_z(radians) }
    #[inline(always)] fn from_angle_xyz(radians_x: f32, radians_y: f32, radians_z: f32) -> mat3 { Matrix3::from_angle_xyz(radians_x, radians_y, radians_z) }
    #[inline(always)] fn from_angle_axis(radians: f32, axis: &vec3) -> mat3 { Matrix3::from_angle_axis(radians, axis) }
    #[inline(always)] fn from_axes(x: vec3, y: vec3, z: vec3) -> mat3 { Matrix3::from_axes(x, y, z) }
    #[inline(always)] fn look_at(dir: &vec3, up: &vec3) -> mat3 { Matrix3::look_at(dir, up) }

    #[inline(always)] fn dim() -> uint { 3 }
    #[inline(always)] fn rows() -> uint { 3 }
    #[inline(always)] fn cols() -> uint { 3 }
    #[inline(always)] fn size_of() -> uint { size_of::<mat3>() }
}


pub impl dmat3 {
    #[inline(always)] fn new(c0r0: f64, c0r1: f64, c0r2: f64, c1r0: f64, c1r1: f64, c1r2: f64, c2r0: f64, c2r1: f64, c2r2: f64)
        -> dmat3 { Matrix3::new(c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2) }
    #[inline(always)] fn from_cols(c0: dvec3, c1: dvec3, c2: dvec3)
        -> dmat3 { Matrix3::from_cols(c0, c1, c2) }
    #[inline(always)] fn from_value(v: f64) -> dmat3 { Matrix::from_value(v) }

    #[inline(always)] fn identity() -> dmat3 { Matrix::identity() }
    #[inline(always)] fn zero() -> dmat3 { Matrix::zero() }

    #[inline(always)] fn dim() -> uint { 3 }
    #[inline(always)] fn rows() -> uint { 3 }
    #[inline(always)] fn cols() -> uint { 3 }
    #[inline(always)] fn size_of() -> uint { size_of::<dmat3>() }
}
