// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cast::transmute;
use std::cmp::ApproxEq;
use std::num::{Zero, One};

use vec::*;
use quat::Quat;
use super::Mat4;

#[deriving(Eq)]
pub struct Mat3<T> { x: Vec3<T>, y: Vec3<T>, z: Vec3<T> }

impl<T> Mat3<T> {
    #[inline]
    pub fn col<'a>(&'a self, i: uint) -> &'a Vec3<T> {
        &'a self.as_slice()[i]
    }

    #[inline]
    pub fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut Vec3<T> {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [Vec3<T>,..3] {
        unsafe { transmute(self) }
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [Vec3<T>,..3] {
        unsafe { transmute(self) }
    }

    #[inline]
    pub fn elem<'a>(&'a self, i: uint, j: uint) -> &'a T {
        self.col(i).index(j)
    }

    #[inline]
    pub fn elem_mut<'a>(&'a mut self, i: uint, j: uint) -> &'a mut T {
        self.col_mut(i).index_mut(j)
    }
}

impl<T:Copy> Mat3<T> {
    /// Construct a 3 x 3 matrix
    ///
    /// # Arguments
    ///
    /// - `c0r0`, `c0r1`, `c0r2`: the first column of the matrix
    /// - `c1r0`, `c1r1`, `c1r2`: the second column of the matrix
    /// - `c2r0`, `c2r1`, `c2r2`: the third column of the matrix
    ///
    /// ~~~
    ///         c0     c1     c2
    ///      +------+------+------+
    ///   r0 | c0r0 | c1r0 | c2r0 |
    ///      +------+------+------+
    ///   r1 | c0r1 | c1r1 | c2r1 |
    ///      +------+------+------+
    ///   r2 | c0r2 | c1r2 | c2r2 |
    ///      +------+------+------+
    /// ~~~
    #[inline]
    pub fn new(c0r0:T, c0r1:T, c0r2:T,
               c1r0:T, c1r1:T, c1r2:T,
               c2r0:T, c2r1:T, c2r2:T) -> Mat3<T> {
        Mat3::from_cols(Vec3::new(c0r0, c0r1, c0r2),
                        Vec3::new(c1r0, c1r1, c1r2),
                        Vec3::new(c2r0, c2r1, c2r2))
    }

    /// Construct a 3 x 3 matrix from column vectors
    ///
    /// # Arguments
    ///
    /// - `c0`: the first column vector of the matrix
    /// - `c1`: the second column vector of the matrix
    /// - `c2`: the third column vector of the matrix
    ///
    /// ~~~
    ///        c0     c1     c2
    ///     +------+------+------+
    ///  r0 | c0.x | c1.x | c2.x |
    ///     +------+------+------+
    ///  r1 | c0.y | c1.y | c2.y |
    ///     +------+------+------+
    ///  r2 | c0.z | c1.z | c2.z |
    ///     +------+------+------+
    /// ~~~
    #[inline]
    pub fn from_cols(c0: Vec3<T>,
                     c1: Vec3<T>,
                     c2: Vec3<T>) -> Mat3<T> {
        Mat3 { x: c0, y: c1, z: c2 }
    }

    #[inline]
    pub fn row(&self, i: uint) -> Vec3<T> {
        Vec3::new(*self.elem(0, i),
                  *self.elem(1, i),
                  *self.elem(2, i))
    }

    #[inline]
    pub fn swap_cols(&mut self, a: uint, b: uint) {
        let tmp = *self.col(a);
        *self.col_mut(a) = *self.col(b);
        *self.col_mut(b) = tmp;
    }

    #[inline]
    pub fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
        self.z.swap(a, b);
    }

    #[inline]
    pub fn transpose(&self) -> Mat3<T> {
        Mat3::new(*self.elem(0, 0), *self.elem(1, 0), *self.elem(2, 0),
                  *self.elem(0, 1), *self.elem(1, 1), *self.elem(2, 1),
                  *self.elem(0, 2), *self.elem(1, 2), *self.elem(2, 2))
    }

    #[inline]
    pub fn transpose_self(&mut self) {
        let tmp01 = *self.elem(0, 1);
        let tmp02 = *self.elem(0, 2);
        let tmp10 = *self.elem(1, 0);
        let tmp12 = *self.elem(1, 2);
        let tmp20 = *self.elem(2, 0);
        let tmp21 = *self.elem(2, 1);

        *self.elem_mut(0, 1) = *self.elem(1, 0);
        *self.elem_mut(0, 2) = *self.elem(2, 0);
        *self.elem_mut(1, 0) = *self.elem(0, 1);
        *self.elem_mut(1, 2) = *self.elem(2, 1);
        *self.elem_mut(2, 0) = *self.elem(0, 2);
        *self.elem_mut(2, 1) = *self.elem(1, 2);

        *self.elem_mut(1, 0) = tmp01;
        *self.elem_mut(2, 0) = tmp02;
        *self.elem_mut(0, 1) = tmp10;
        *self.elem_mut(2, 1) = tmp12;
        *self.elem_mut(0, 2) = tmp20;
        *self.elem_mut(1, 2) = tmp21;
    }
}

impl<T:Copy + Num> Mat3<T> {
    /// Construct a 3 x 3 diagonal matrix with the major diagonal set to `value`
    ///
    /// # Arguments
    ///
    /// - `value`: the value to set the major diagonal to
    ///
    /// ~~~
    ///        c0    c1    c2
    ///     +-----+-----+-----+
    ///  r0 | val |   0 |   0 |
    ///     +-----+-----+-----+
    ///  r1 |   0 | val |   0 |
    ///     +-----+-----+-----+
    ///  r2 |   0 |   0 | val |
    ///     +-----+-----+-----+
    /// ~~~
    #[inline]
    pub fn from_value(value: T) -> Mat3<T> {
        Mat3::new(value, Zero::zero(), Zero::zero(),
                  Zero::zero(), value, Zero::zero(),
                  Zero::zero(), Zero::zero(), value)
    }

    /// Returns the multiplicative identity matrix
    /// ~~~
    ///       c0   c1   c2
    ///     +----+----+----+
    ///  r0 |  1 |  0 |  0 |
    ///     +----+----+----+
    ///  r1 |  0 |  1 |  0 |
    ///     +----+----+----+
    ///  r2 |  0 |  0 |  1 |
    ///     +----+----+----+
    /// ~~~
    #[inline]
    pub fn identity() -> Mat3<T> {
        Mat3::new(One::one::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), One::one::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), One::one::<T>())
    }

    /// Returns the additive identity matrix
    /// ~~~
    ///       c0   c1   c2
    ///     +----+----+----+
    ///  r0 |  0 |  0 |  0 |
    ///     +----+----+----+
    ///  r1 |  0 |  0 |  0 |
    ///     +----+----+----+
    ///  r2 |  0 |  0 |  0 |
    ///     +----+----+----+
    /// ~~~
    #[inline]
    pub fn zero() -> Mat3<T> {
        Mat3::new(Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline]
    pub fn mul_t(&self, value: T) -> Mat3<T> {
        Mat3::from_cols(self.col(0).mul_t(value),
                        self.col(1).mul_t(value),
                        self.col(2).mul_t(value))
    }

    #[inline]
    pub fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.row(0).dot(vec),
                  self.row(1).dot(vec),
                  self.row(2).dot(vec))
    }

    #[inline]
    pub fn add_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self.col(0).add_v(other.col(0)),
                        self.col(1).add_v(other.col(1)),
                        self.col(2).add_v(other.col(2)))
    }

    #[inline]
    pub fn sub_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self.col(0).sub_v(other.col(0)),
                        self.col(1).sub_v(other.col(1)),
                        self.col(2).sub_v(other.col(2)))
    }

    #[inline]
    pub fn mul_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::new(self.row(0).dot(other.col(0)),
                  self.row(1).dot(other.col(0)),
                  self.row(2).dot(other.col(0)),

                  self.row(0).dot(other.col(1)),
                  self.row(1).dot(other.col(1)),
                  self.row(2).dot(other.col(1)),

                  self.row(0).dot(other.col(2)),
                  self.row(1).dot(other.col(2)),
                  self.row(2).dot(other.col(2)))
    }

    #[inline]
    pub fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(value);
        self.col_mut(1).mul_self_t(value);
        self.col_mut(2).mul_self_t(value);
    }

    #[inline]
    pub fn add_self_m(&mut self, other: &Mat3<T>) {
        self.col_mut(0).add_self_v(other.col(0));
        self.col_mut(1).add_self_v(other.col(1));
        self.col_mut(2).add_self_v(other.col(2));
    }

    #[inline]
    pub fn sub_self_m(&mut self, other: &Mat3<T>) {
        self.col_mut(0).sub_self_v(other.col(0));
        self.col_mut(1).sub_self_v(other.col(1));
        self.col_mut(2).sub_self_v(other.col(2));
    }

    pub fn dot(&self, other: &Mat3<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    pub fn determinant(&self) -> T {
        self.col(0).dot(&self.col(1).cross(self.col(2)))
    }

    pub fn trace(&self) -> T {
        *self.elem(0, 0) +
        *self.elem(1, 1) +
        *self.elem(2, 2)
    }

    #[inline]
    pub fn to_identity(&mut self) {
        *self = Mat3::identity();
    }

    #[inline]
    pub fn to_zero(&mut self) {
        *self = Mat3::zero();
    }

    /// Returns the the matrix with an extra row and column added
    /// ~~~
    ///       c0   c1   c2                 c0   c1   c2   c3
    ///     +----+----+----+             +----+----+----+----+
    ///  r0 |  a |  b |  c |          r0 |  a |  b |  c |  0 |
    ///     +----+----+----+             +----+----+----+----+
    ///  r1 |  d |  e |  f |    =>    r1 |  d |  e |  f |  0 |
    ///     +----+----+----+             +----+----+----+----+
    ///  r2 |  g |  h |  i |          r2 |  g |  h |  i |  0 |
    ///     +----+----+----+             +----+----+----+----+
    ///                               r3 |  0 |  0 |  0 |  1 |
    ///                                  +----+----+----+----+
    /// ~~~
    #[inline]
    pub fn to_mat4(&self) -> Mat4<T> {
        Mat4::new(*self.elem(0, 0), *self.elem(0, 1), *self.elem(0, 2), Zero::zero(),
                  *self.elem(1, 0), *self.elem(1, 1), *self.elem(1, 2), Zero::zero(),
                  *self.elem(2, 0), *self.elem(2, 1), *self.elem(2, 2), Zero::zero(),
                  Zero::zero(), Zero::zero(), Zero::zero(), One::one())
    }
}

impl<T:Copy + Num> Neg<Mat3<T>> for Mat3<T> {
    #[inline]
    pub fn neg(&self) -> Mat3<T> {
        Mat3::from_cols(-self.col(0), -self.col(1), -self.col(2))
    }
}

impl<T:Copy + Real> Mat3<T> {
    /// Construct a matrix from an angular rotation around the `x` axis
    pub fn from_angle_x(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat3::new(One::one(), Zero::zero(), Zero::zero(),
                  Zero::zero(), cos_theta, sin_theta,
                  Zero::zero(), -sin_theta, cos_theta)
    }

    /// Construct a matrix from an angular rotation around the `y` axis
    pub fn from_angle_y(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat3::new(cos_theta, Zero::zero(), -sin_theta,
                  Zero::zero(), One::one(), Zero::zero(),
                  sin_theta, Zero::zero(), cos_theta)
    }

    /// Construct a matrix from an angular rotation around the `z` axis
    pub fn from_angle_z(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat3::new(cos_theta, sin_theta, Zero::zero(),
                  -sin_theta, cos_theta, Zero::zero(),
                  Zero::zero(), Zero::zero(), One::one())
    }

    /// Construct a matrix from Euler angles
    ///
    /// # Arguments
    ///
    /// - `theta_x`: the angular rotation around the `x` axis (pitch)
    /// - `theta_y`: the angular rotation around the `y` axis (yaw)
    /// - `theta_z`: the angular rotation around the `z` axis (roll)
    pub fn from_angle_xyz(radians_x: T, radians_y: T, radians_z: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
        let cx = radians_x.cos();
        let sx = radians_x.sin();
        let cy = radians_y.cos();
        let sy = radians_y.sin();
        let cz = radians_z.cos();
        let sz = radians_z.sin();

        Mat3::new(cy*cz, cy*sz, -sy,
                  -cx*sz + sx*sy*cz, cx*cz + sx*sy*sz, sx*cy,
                  sx*sz + cx*sy*cz, -sx*cz + cx*sy*sz, cx*cy)
    }

    /// Construct a matrix from an axis and an angular rotation
    pub fn from_angle_axis(radians: T, axis: &Vec3<T>) -> Mat3<T> {
        let c = radians.cos();
        let s = radians.sin();
        let _1_c = One::one::<T>() - c;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        Mat3::new(_1_c*x*x + c, _1_c*x*y + s*z, _1_c*x*z - s*y,
                  _1_c*x*y - s*z, _1_c*y*y + c, _1_c*y*z + s*x,
                  _1_c*x*z + s*y, _1_c*y*z - s*x, _1_c*z*z + c)
    }

    #[inline]
    pub fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat3<T> {
        Mat3::from_cols(x, y, z)
    }

    pub fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> Mat3<T> {
        let dir_ = dir.normalize();
        let side = dir_.cross(&up.normalize());
        let up_  = side.cross(&dir_).normalize();

        Mat3::from_axes(up_, side, dir_)
    }

    /// Convert the matrix to a quaternion
    pub fn to_quat(&self) -> Quat<T> {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf

        let mut s;
        let w; let x; let y; let z;
        let trace = self.trace();

        // FIXME: We don't have any numeric conversions in std yet :P
        let half = One::one::<T>() / (One::one::<T>() + One::one::<T>());

        cond! (
            (trace >= Zero::zero()) {
                s = (One::one::<T>() + trace).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.elem(1, 2) - *self.elem(2, 1)) * s;
                y = (*self.elem(2, 0) - *self.elem(0, 2)) * s;
                z = (*self.elem(0, 1) - *self.elem(1, 0)) * s;
            }
            ((*self.elem(0, 0) > *self.elem(1, 1))
            && (*self.elem(0, 0) > *self.elem(2, 2))) {
                s = (half + (*self.elem(0, 0) - *self.elem(1, 1) - *self.elem(2, 2))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.elem(0, 1) - *self.elem(1, 0)) * s;
                y = (*self.elem(2, 0) - *self.elem(0, 2)) * s;
                z = (*self.elem(1, 2) - *self.elem(2, 1)) * s;
            }
            (*self.elem(1, 1) > *self.elem(2, 2)) {
                s = (half + (*self.elem(1, 1) - *self.elem(0, 0) - *self.elem(2, 2))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.elem(0, 1) - *self.elem(1, 0)) * s;
                y = (*self.elem(1, 2) - *self.elem(2, 1)) * s;
                z = (*self.elem(2, 0) - *self.elem(0, 2)) * s;
            }
            _ {
                s = (half + (*self.elem(2, 2) - *self.elem(0, 0) - *self.elem(1, 1))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.elem(2, 0) - *self.elem(0, 2)) * s;
                y = (*self.elem(1, 2) - *self.elem(2, 1)) * s;
                z = (*self.elem(0, 1) - *self.elem(1, 0)) * s;
            }
        )
        Quat::new(w, x, y, z)
    }
}

impl<T:Copy + Real + ApproxEq<T>> Mat3<T> {
    pub fn inverse(&self) -> Option<Mat3<T>> {
        let d = self.determinant();
        if d.approx_eq(&Zero::zero()) {
            None
        } else {
            Some(Mat3::from_cols(self.col(1).cross(self.col(2)).div_t(d),
                                 self.col(2).cross(self.col(0)).div_t(d),
                                 self.col(0).cross(self.col(1)).div_t(d)).transpose())
        }
    }

    #[inline]
    pub fn invert_self(&mut self) {
        *self = self.inverse().expect("Couldn't invert the matrix!");
    }

    #[inline]
    pub fn is_identity(&self) -> bool {
        self.approx_eq(&Mat3::identity())
    }

    #[inline]
    pub fn is_diagonal(&self) -> bool {
        self.elem(0, 1).approx_eq(&Zero::zero()) &&
        self.elem(0, 2).approx_eq(&Zero::zero()) &&

        self.elem(1, 0).approx_eq(&Zero::zero()) &&
        self.elem(1, 2).approx_eq(&Zero::zero()) &&

        self.elem(2, 0).approx_eq(&Zero::zero()) &&
        self.elem(2, 1).approx_eq(&Zero::zero())
    }

    #[inline]
    pub fn is_rotated(&self) -> bool {
        !self.approx_eq(&Mat3::identity())
    }

    #[inline]
    pub fn is_symmetric(&self) -> bool {
        self.elem(0, 1).approx_eq(self.elem(1, 0)) &&
        self.elem(0, 2).approx_eq(self.elem(2, 0)) &&

        self.elem(1, 0).approx_eq(self.elem(0, 1)) &&
        self.elem(1, 2).approx_eq(self.elem(2, 1)) &&

        self.elem(2, 0).approx_eq(self.elem(0, 2)) &&
        self.elem(2, 1).approx_eq(self.elem(1, 2))
    }

    #[inline]
    pub fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&Zero::zero())
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Mat3<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &Mat3<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &Mat3<T>, epsilon: &T) -> bool {
        self.col(0).approx_eq_eps(other.col(0), epsilon) &&
        self.col(1).approx_eq_eps(other.col(1), epsilon) &&
        self.col(2).approx_eq_eps(other.col(2), epsilon)
    }
}
