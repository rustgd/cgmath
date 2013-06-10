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
use std::uint;

use vec::*;
use quat::Quat;

#[deriving(Eq)]
pub struct Mat2<T> { x: Vec2<T>, y: Vec2<T> }

impl<T> Mat2<T> {
    #[inline(always)]
    pub fn col<'a>(&'a self, i: uint) -> &'a Vec2<T> {
        &'a self.as_slice()[i]
    }

    #[inline(always)]
    pub fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut Vec2<T> {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline(always)]
    pub fn as_slice<'a>(&'a self) -> &'a [Vec2<T>,..2] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [Vec2<T>,..2] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn elem<'a>(&'a self, i: uint, j: uint) -> &'a T {
        self.col(i).index(j)
    }

    #[inline(always)]
    pub fn elem_mut<'a>(&'a mut self, i: uint, j: uint) -> &'a mut T {
        self.col_mut(i).index_mut(j)
    }
}

impl<T:Copy> Mat2<T> {
    /// Construct a 2 x 2 matrix
    ///
    /// # Arguments
    ///
    /// - `c0r0`, `c0r1`: the first column of the matrix
    /// - `c1r0`, `c1r1`: the second column of the matrix
    ///
    /// ~~~
    ///        c0     c1
    ///     +------+------+
    ///  r0 | c0r0 | c1r0 |
    ///     +------+------+
    ///  r1 | c0r1 | c1r1 |
    ///     +------+------+
    /// ~~~
    #[inline(always)]
    pub fn new(c0r0: T, c0r1: T,
               c1r0: T, c1r1: T) -> Mat2<T> {
        Mat2::from_cols(Vec2::new(c0r0, c0r1),
                        Vec2::new(c1r0, c1r1))
    }

    /// Construct a 2 x 2 matrix from column vectors
    ///
    /// # Arguments
    ///
    /// - `c0`: the first column vector of the matrix
    /// - `c1`: the second column vector of the matrix
    ///
    /// ~~~
    ///        c0     c1
    ///     +------+------+
    ///  r0 | c0.x | c1.x |
    ///     +------+------+
    ///  r1 | c0.y | c1.y |
    ///     +------+------+
    /// ~~~
    #[inline(always)]
    pub fn from_cols(c0: Vec2<T>,
                     c1: Vec2<T>) -> Mat2<T> {
        Mat2 { x: c0, y: c1 }
    }

    #[inline(always)]
    pub fn row(&self, i: uint) -> Vec2<T> {
        Vec2::new(*self.elem(0, i),
                  *self.elem(1, i))
    }

    #[inline(always)]
    pub fn swap_cols(&mut self, a: uint, b: uint) {
        let tmp = *self.col(a);
        *self.col_mut(a) = *self.col(b);
        *self.col_mut(b) = tmp;
    }

    #[inline(always)]
    pub fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
    }

    #[inline(always)]
    pub fn transpose(&self) -> Mat2<T> {
        Mat2::new(*self.elem(0, 0), *self.elem(1, 0),
                  *self.elem(0, 1), *self.elem(1, 1))
    }

    #[inline(always)]
    pub fn transpose_self(&mut self) {
        let tmp01 = *self.elem(0, 1);
        let tmp10 = *self.elem(1, 0);

        *self.elem_mut(0, 1) = *self.elem(1, 0);
        *self.elem_mut(1, 0) = *self.elem(0, 1);

        *self.elem_mut(1, 0) = tmp01;
        *self.elem_mut(0, 1) = tmp10;
    }
}

impl<T:Copy + Num> Mat2<T> {
    /// Construct a 2 x 2 diagonal matrix with the major diagonal set to `value`.
    /// ~~~
    ///        c0    c1
    ///     +-----+-----+
    ///  r0 | val |   0 |
    ///     +-----+-----+
    ///  r1 |   0 | val |
    ///     +-----+-----+
    /// ~~~
    #[inline(always)]
    pub fn from_value(value: T) -> Mat2<T> {
        Mat2::new(value, Zero::zero(),
                  Zero::zero(), value)
    }

    /// Returns the multiplicative identity matrix
    /// ~~~
    ///       c0   c1
    ///     +----+----+
    ///  r0 |  1 |  0 |
    ///     +----+----+
    ///  r1 |  0 |  1 |
    ///     +----+----+
    /// ~~~
    #[inline(always)]
    pub fn identity() -> Mat2<T> {
        Mat2::new(One::one::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), One::one::<T>())
    }

    /// Returns the additive identity matrix
    /// ~~~
    ///       c0   c1
    ///     +----+----+
    ///  r0 |  0 |  0 |
    ///     +----+----+
    ///  r1 |  0 |  0 |
    ///     +----+----+
    /// ~~~
    #[inline(always)]
    pub fn zero() -> Mat2<T> {
        Mat2::new(Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn mul_t(&self, value: T) -> Mat2<T> {
        Mat2::from_cols(self.col(0).mul_t(value),
                        self.col(1).mul_t(value))
    }

    #[inline(always)]
    pub fn mul_v(&self, vec: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self.row(0).dot(vec),
                  self.row(1).dot(vec))
    }

    #[inline(always)]
    pub fn add_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self.col(0).add_v(other.col(0)),
                        self.col(1).add_v(other.col(1)))
    }

    #[inline(always)]
    pub fn sub_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self.col(0).sub_v(other.col(0)),
                        self.col(1).sub_v(other.col(1)))
    }

    #[inline(always)]
    pub fn mul_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::new(self.row(0).dot(other.col(0)), self.row(1).dot(other.col(0)),
                  self.row(0).dot(other.col(1)), self.row(1).dot(other.col(1)))
    }

    #[inline(always)]
    pub fn mul_self_t(&mut self, value: T) {
        self.x.mul_self_t(value);
        self.y.mul_self_t(value);
    }

    #[inline(always)]
    pub fn add_self_m(&mut self, other: &Mat2<T>) {
        self.x.add_self_v(other.col(0));
        self.y.add_self_v(other.col(1));
    }

    #[inline(always)]
    pub fn sub_self_m(&mut self, other: &Mat2<T>) {
        self.x.sub_self_v(other.col(0));
        self.y.sub_self_v(other.col(1));
    }

    pub fn dot(&self, other: &Mat2<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    pub fn determinant(&self) -> T {
       *self.col(0).index(0) *
       *self.col(1).index(1) -
       *self.col(1).index(0) *
       *self.col(0).index(1)
    }

    pub fn trace(&self) -> T {
        *self.col(0).index(0) +
        *self.col(1).index(1)
    }

    #[inline(always)]
    pub fn to_identity(&mut self) {
        *self = Mat2::identity();
    }

    #[inline(always)]
    pub fn to_zero(&mut self) {
        *self = Mat2::zero();
    }

    /// Returns the the matrix with an extra row and column added
    /// ~~~
    ///       c0   c1                 c0   c1   c2
    ///     +----+----+             +----+----+----+
    ///  r0 |  a |  b |          r0 |  a |  b |  0 |
    ///     +----+----+             +----+----+----+
    ///  r1 |  c |  d |    =>    r1 |  c |  d |  0 |
    ///     +----+----+             +----+----+----+
    ///                          r2 |  0 |  0 |  1 |
    ///                             +----+----+----+
    /// ~~~
    #[inline(always)]
    pub fn to_mat3(&self) -> Mat3<T> {
        Mat3::new(*self.elem(0, 0), *self.elem(0, 1), Zero::zero(),
                  *self.elem(1, 0), *self.elem(1, 1), Zero::zero(),
                  Zero::zero(), Zero::zero(), One::one())
    }

    /// Returns the the matrix with an extra two rows and columns added
    /// ~~~
    ///       c0   c1                 c0   c1   c2   c3
    ///     +----+----+             +----+----+----+----+
    ///  r0 |  a |  b |          r0 |  a |  b |  0 |  0 |
    ///     +----+----+             +----+----+----+----+
    ///  r1 |  c |  d |    =>    r1 |  c |  d |  0 |  0 |
    ///     +----+----+             +----+----+----+----+
    ///                          r2 |  0 |  0 |  1 |  0 |
    ///                             +----+----+----+----+
    ///                          r3 |  0 |  0 |  0 |  1 |
    ///                             +----+----+----+----+
    /// ~~~
    #[inline(always)]
    pub fn to_mat4(&self) -> Mat4<T> {
        Mat4::new(*self.elem(0, 0), *self.elem(0, 1), Zero::zero(), Zero::zero(),
                  *self.elem(1, 0), *self.elem(1, 1), Zero::zero(), Zero::zero(),
                  Zero::zero(), Zero::zero(), One::one(), Zero::zero(),
                  Zero::zero(), Zero::zero(), Zero::zero(), One::one())
    }
}

impl<T:Copy + Num> Neg<Mat2<T>> for Mat2<T> {
    #[inline(always)]
    pub fn neg(&self) -> Mat2<T> {
        Mat2::from_cols(-self.col(0), -self.col(1))
    }
}

impl<T:Copy + Real> Mat2<T> {
    #[inline(always)]
    pub fn from_angle(radians: T) -> Mat2<T> {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat2::new(cos_theta, -sin_theta,
                  sin_theta,  cos_theta)
    }
}

impl<T:Copy + Real + ApproxEq<T>> Mat2<T> {
    #[inline(always)]
    pub fn inverse(&self) -> Option<Mat2<T>> {
        let d = self.determinant();
        if d.approx_eq(&Zero::zero()) {
            None
        } else {
            Some(Mat2::new(self.elem(1, 1) / d, -self.elem(0, 1) / d,
                           -self.elem(1, 0) / d, self.elem(0, 0) / d))
        }
    }

    #[inline(always)]
    pub fn invert_self(&mut self) {
        *self = self.inverse().expect("Couldn't invert the matrix!");
    }

    #[inline(always)]
    pub fn is_identity(&self) -> bool {
        self.approx_eq(&Mat2::identity())
    }

    #[inline(always)]
    pub fn is_diagonal(&self) -> bool {
        self.elem(0, 1).approx_eq(&Zero::zero()) &&
        self.elem(1, 0).approx_eq(&Zero::zero())
    }

    #[inline(always)]
    pub fn is_rotated(&self) -> bool {
        !self.approx_eq(&Mat2::identity())
    }

    #[inline(always)]
    pub fn is_symmetric(&self) -> bool {
        self.elem(0, 1).approx_eq(self.elem(1, 0)) &&
        self.elem(1, 0).approx_eq(self.elem(0, 1))
    }

    #[inline(always)]
    pub fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&Zero::zero())
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Mat2<T> {
    #[inline(always)]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    pub fn approx_eq(&self, other: &Mat2<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    pub fn approx_eq_eps(&self, other: &Mat2<T>, epsilon: &T) -> bool {
        self.col(0).approx_eq_eps(other.col(0), epsilon) &&
        self.col(1).approx_eq_eps(other.col(1), epsilon)
    }
}

// GLSL-style type aliases
pub type mat2  = Mat2<f32>;
pub type dmat2 = Mat2<f64>;

// Rust-style type aliases
pub type Mat2f   = Mat2<float>;
pub type Mat2f32 = Mat2<f32>;
pub type Mat2f64 = Mat2<f64>;

#[deriving(Eq)]
pub struct Mat3<T> { x: Vec3<T>, y: Vec3<T>, z: Vec3<T> }

impl<T> Mat3<T> {
    #[inline(always)]
    pub fn col<'a>(&'a self, i: uint) -> &'a Vec3<T> {
        &'a self.as_slice()[i]
    }

    #[inline(always)]
    pub fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut Vec3<T> {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline(always)]
    pub fn as_slice<'a>(&'a self) -> &'a [Vec3<T>,..3] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [Vec3<T>,..3] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn elem<'a>(&'a self, i: uint, j: uint) -> &'a T {
        self.col(i).index(j)
    }

    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn from_cols(c0: Vec3<T>,
                     c1: Vec3<T>,
                     c2: Vec3<T>) -> Mat3<T> {
        Mat3 { x: c0, y: c1, z: c2 }
    }

    #[inline(always)]
    pub fn row(&self, i: uint) -> Vec3<T> {
        Vec3::new(*self.elem(0, i),
                  *self.elem(1, i),
                  *self.elem(2, i))
    }

    #[inline(always)]
    pub fn swap_cols(&mut self, a: uint, b: uint) {
        let tmp = *self.col(a);
        *self.col_mut(a) = *self.col(b);
        *self.col_mut(b) = tmp;
    }

    #[inline(always)]
    pub fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
        self.z.swap(a, b);
    }

    #[inline(always)]
    pub fn transpose(&self) -> Mat3<T> {
        Mat3::new(*self.elem(0, 0), *self.elem(1, 0), *self.elem(2, 0),
                  *self.elem(0, 1), *self.elem(1, 1), *self.elem(2, 1),
                  *self.elem(0, 2), *self.elem(1, 2), *self.elem(2, 2))
    }

    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn zero() -> Mat3<T> {
        Mat3::new(Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn mul_t(&self, value: T) -> Mat3<T> {
        Mat3::from_cols(self.col(0).mul_t(value),
                        self.col(1).mul_t(value),
                        self.col(2).mul_t(value))
    }

    #[inline(always)]
    pub fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.row(0).dot(vec),
                  self.row(1).dot(vec),
                  self.row(2).dot(vec))
    }

    #[inline(always)]
    pub fn add_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self.col(0).add_v(other.col(0)),
                        self.col(1).add_v(other.col(1)),
                        self.col(2).add_v(other.col(2)))
    }

    #[inline(always)]
    pub fn sub_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self.col(0).sub_v(other.col(0)),
                        self.col(1).sub_v(other.col(1)),
                        self.col(2).sub_v(other.col(2)))
    }

    #[inline(always)]
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

    #[inline(always)]
    pub fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(value);
        self.col_mut(1).mul_self_t(value);
        self.col_mut(2).mul_self_t(value);
    }

    #[inline(always)]
    pub fn add_self_m(&mut self, other: &Mat3<T>) {
        self.col_mut(0).add_self_v(other.col(0));
        self.col_mut(1).add_self_v(other.col(1));
        self.col_mut(2).add_self_v(other.col(2));
    }

    #[inline(always)]
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

    #[inline(always)]
    pub fn to_identity(&mut self) {
        *self = Mat3::identity();
    }

    #[inline(always)]
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
    #[inline(always)]
    pub fn to_mat4(&self) -> Mat4<T> {
        Mat4::new(*self.elem(0, 0), *self.elem(0, 1), *self.elem(0, 2), Zero::zero(),
                  *self.elem(1, 0), *self.elem(1, 1), *self.elem(1, 2), Zero::zero(),
                  *self.elem(2, 0), *self.elem(2, 1), *self.elem(2, 2), Zero::zero(),
                  Zero::zero(), Zero::zero(), Zero::zero(), One::one())
    }
}

impl<T:Copy + Num> Neg<Mat3<T>> for Mat3<T> {
    #[inline(always)]
    pub fn neg(&self) -> Mat3<T> {
        Mat3::from_cols(-self.col(0), -self.col(1), -self.col(2))
    }
}

impl<T:Copy + Real> Mat3<T> {
    /// Construct a matrix from an angular rotation around the `x` axis
    #[inline(always)]
    pub fn from_angle_x(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat3::new(One::one(), Zero::zero(), Zero::zero(),
                  Zero::zero(), cos_theta, sin_theta,
                  Zero::zero(), -sin_theta, cos_theta)
    }

    /// Construct a matrix from an angular rotation around the `y` axis
    #[inline(always)]
    pub fn from_angle_y(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat3::new(cos_theta, Zero::zero(), -sin_theta,
                  Zero::zero(), One::one(), Zero::zero(),
                  sin_theta, Zero::zero(), cos_theta)
    }

    /// Construct a matrix from an angular rotation around the `z` axis
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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

    #[inline(always)]
    pub fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat3<T> {
        Mat3::from_cols(x, y, z)
    }

    #[inline(always)]
    pub fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> Mat3<T> {
        let dir_ = dir.normalize();
        let side = dir_.cross(&up.normalize());
        let up_  = side.cross(&dir_).normalize();

        Mat3::from_axes(up_, side, dir_)
    }

    /// Convert the matrix to a quaternion
    #[inline(always)]
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

    #[inline(always)]
    pub fn invert_self(&mut self) {
        *self = self.inverse().expect("Couldn't invert the matrix!");
    }

    #[inline(always)]
    pub fn is_identity(&self) -> bool {
        self.approx_eq(&Mat3::identity())
    }

    #[inline(always)]
    pub fn is_diagonal(&self) -> bool {
        self.elem(0, 1).approx_eq(&Zero::zero()) &&
        self.elem(0, 2).approx_eq(&Zero::zero()) &&

        self.elem(1, 0).approx_eq(&Zero::zero()) &&
        self.elem(1, 2).approx_eq(&Zero::zero()) &&

        self.elem(2, 0).approx_eq(&Zero::zero()) &&
        self.elem(2, 1).approx_eq(&Zero::zero())
    }

    #[inline(always)]
    pub fn is_rotated(&self) -> bool {
        !self.approx_eq(&Mat3::identity())
    }

    #[inline(always)]
    pub fn is_symmetric(&self) -> bool {
        self.elem(0, 1).approx_eq(self.elem(1, 0)) &&
        self.elem(0, 2).approx_eq(self.elem(2, 0)) &&

        self.elem(1, 0).approx_eq(self.elem(0, 1)) &&
        self.elem(1, 2).approx_eq(self.elem(2, 1)) &&

        self.elem(2, 0).approx_eq(self.elem(0, 2)) &&
        self.elem(2, 1).approx_eq(self.elem(1, 2))
    }

    #[inline(always)]
    pub fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&Zero::zero())
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Mat3<T> {
    #[inline(always)]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    pub fn approx_eq(&self, other: &Mat3<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    pub fn approx_eq_eps(&self, other: &Mat3<T>, epsilon: &T) -> bool {
        self.col(0).approx_eq_eps(other.col(0), epsilon) &&
        self.col(1).approx_eq_eps(other.col(1), epsilon) &&
        self.col(2).approx_eq_eps(other.col(2), epsilon)
    }
}

// GLSL-style type aliases
pub type mat3  = Mat3<f32>;
pub type dmat3 = Mat3<f64>;

// Rust-style type aliases
pub type Mat3f   = Mat3<float>;
pub type Mat3f32 = Mat3<f32>;
pub type Mat3f64 = Mat3<f64>;

///  A 4 x 4 column major matrix
///
/// # Type parameters
///
/// - `T` - The type of the elements of the matrix. Should be a floating point type.
///
/// # Fields
///
/// - `x`: the first column vector of the matrix
/// - `y`: the second column vector of the matrix
/// - `z`: the third column vector of the matrix
/// - `w`: the fourth column vector of the matrix
#[deriving(Eq)]
pub struct Mat4<T> { x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T> }

impl<T> Mat4<T> {
    #[inline(always)]
    pub fn col<'a>(&'a self, i: uint) -> &'a Vec4<T> {
        &'a self.as_slice()[i]
    }

    #[inline(always)]
    pub fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut Vec4<T> {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline(always)]
    pub fn as_slice<'a>(&'a self) -> &'a [Vec4<T>,..4] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [Vec4<T>,..4] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn elem<'a>(&'a self, i: uint, j: uint) -> &'a T {
        self.col(i).index(j)
    }

    #[inline(always)]
    pub fn elem_mut<'a>(&'a mut self, i: uint, j: uint) -> &'a mut T {
        self.col_mut(i).index_mut(j)
    }
}

impl<T:Copy> Mat4<T> {
    /// Construct a 4 x 4 matrix
    ///
    /// # Arguments
    ///
    /// - `c0r0`, `c0r1`, `c0r2`, `c0r3`: the first column of the matrix
    /// - `c1r0`, `c1r1`, `c1r2`, `c1r3`: the second column of the matrix
    /// - `c2r0`, `c2r1`, `c2r2`, `c2r3`: the third column of the matrix
    /// - `c3r0`, `c3r1`, `c3r2`, `c3r3`: the fourth column of the matrix
    ///
    /// ~~~
    ///        c0     c1     c2     c3
    ///     +------+------+------+------+
    ///  r0 | c0r0 | c1r0 | c2r0 | c3r0 |
    ///     +------+------+------+------+
    ///  r1 | c0r1 | c1r1 | c2r1 | c3r1 |
    ///     +------+------+------+------+
    ///  r2 | c0r2 | c1r2 | c2r2 | c3r2 |
    ///     +------+------+------+------+
    ///  r3 | c0r3 | c1r3 | c2r3 | c3r3 |
    ///     +------+------+------+------+
    /// ~~~
    #[inline(always)]
    pub fn new(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
               c1r0: T, c1r1: T, c1r2: T, c1r3: T,
               c2r0: T, c2r1: T, c2r2: T, c2r3: T,
               c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> Mat4<T>  {
        Mat4::from_cols(Vec4::new(c0r0, c0r1, c0r2, c0r3),
                        Vec4::new(c1r0, c1r1, c1r2, c1r3),
                        Vec4::new(c2r0, c2r1, c2r2, c2r3),
                        Vec4::new(c3r0, c3r1, c3r2, c3r3))
    }

    /// Construct a 4 x 4 matrix from column vectors
    ///
    /// # Arguments
    ///
    /// - `c0`: the first column vector of the matrix
    /// - `c1`: the second column vector of the matrix
    /// - `c2`: the third column vector of the matrix
    /// - `c3`: the fourth column vector of the matrix
    ///
    /// ~~~
    ///        c0     c1     c2     c3
    ///     +------+------+------+------+
    ///  r0 | c0.x | c1.x | c2.x | c3.x |
    ///     +------+------+------+------+
    ///  r1 | c0.y | c1.y | c2.y | c3.y |
    ///     +------+------+------+------+
    ///  r2 | c0.z | c1.z | c2.z | c3.z |
    ///     +------+------+------+------+
    ///  r3 | c0.w | c1.w | c2.w | c3.w |
    ///     +------+------+------+------+
    /// ~~~
    #[inline(always)]
    pub fn from_cols(c0: Vec4<T>,
                     c1: Vec4<T>,
                     c2: Vec4<T>,
                     c3: Vec4<T>) -> Mat4<T> {
        Mat4 { x: c0, y: c1, z: c2, w: c3 }
    }

    #[inline(always)]
    pub fn row(&self, i: uint) -> Vec4<T> {
        Vec4::new(*self.elem(0, i),
                  *self.elem(1, i),
                  *self.elem(2, i),
                  *self.elem(3, i))
    }

    #[inline(always)]
    pub fn swap_cols(&mut self, a: uint, b: uint) {
        let tmp = *self.col(a);
        *self.col_mut(a) = *self.col(b);
        *self.col_mut(b) = tmp;
    }

    #[inline(always)]
    pub fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
        self.z.swap(a, b);
        self.w.swap(a, b);
    }

    #[inline(always)]
    pub fn transpose(&self) -> Mat4<T> {
        Mat4::new(*self.elem(0, 0), *self.elem(1, 0), *self.elem(2, 0), *self.elem(3, 0),
                  *self.elem(0, 1), *self.elem(1, 1), *self.elem(2, 1), *self.elem(3, 1),
                  *self.elem(0, 2), *self.elem(1, 2), *self.elem(2, 2), *self.elem(3, 2),
                  *self.elem(0, 3), *self.elem(1, 3), *self.elem(2, 3), *self.elem(3, 3))
    }

    #[inline(always)]
    pub fn transpose_self(&mut self) {
        let tmp01 = *self.elem(0, 1);
        let tmp02 = *self.elem(0, 2);
        let tmp03 = *self.elem(0, 3);
        let tmp10 = *self.elem(1, 0);
        let tmp12 = *self.elem(1, 2);
        let tmp13 = *self.elem(1, 3);
        let tmp20 = *self.elem(2, 0);
        let tmp21 = *self.elem(2, 1);
        let tmp23 = *self.elem(2, 3);
        let tmp30 = *self.elem(3, 0);
        let tmp31 = *self.elem(3, 1);
        let tmp32 = *self.elem(3, 2);

        *self.elem_mut(0, 1) = *self.elem(1, 0);
        *self.elem_mut(0, 2) = *self.elem(2, 0);
        *self.elem_mut(0, 3) = *self.elem(3, 0);
        *self.elem_mut(1, 0) = *self.elem(0, 1);
        *self.elem_mut(1, 2) = *self.elem(2, 1);
        *self.elem_mut(1, 3) = *self.elem(3, 1);
        *self.elem_mut(2, 0) = *self.elem(0, 2);
        *self.elem_mut(2, 1) = *self.elem(1, 2);
        *self.elem_mut(2, 3) = *self.elem(3, 2);
        *self.elem_mut(3, 0) = *self.elem(0, 3);
        *self.elem_mut(3, 1) = *self.elem(1, 3);
        *self.elem_mut(3, 2) = *self.elem(2, 3);

        *self.elem_mut(1, 0) = tmp01;
        *self.elem_mut(2, 0) = tmp02;
        *self.elem_mut(3, 0) = tmp03;
        *self.elem_mut(0, 1) = tmp10;
        *self.elem_mut(2, 1) = tmp12;
        *self.elem_mut(3, 1) = tmp13;
        *self.elem_mut(0, 2) = tmp20;
        *self.elem_mut(1, 2) = tmp21;
        *self.elem_mut(3, 2) = tmp23;
        *self.elem_mut(0, 3) = tmp30;
        *self.elem_mut(1, 3) = tmp31;
        *self.elem_mut(2, 3) = tmp32;
    }
}

impl<T:Copy + Num> Mat4<T> {
    /// Construct a 4 x 4 diagonal matrix with the major diagonal set to `value`
    ///
    /// # Arguments
    ///
    /// - `value`: the value to set the major diagonal to
    ///
    /// ~~~
    ///        c0    c1    c2    c3
    ///     +-----+-----+-----+-----+
    ///  r0 | val |   0 |   0 |   0 |
    ///     +-----+-----+-----+-----+
    ///  r1 |   0 | val |   0 |   0 |
    ///     +-----+-----+-----+-----+
    ///  r2 |   0 |   0 | val |   0 |
    ///     +-----+-----+-----+-----+
    ///  r3 |   0 |   0 |   0 | val |
    ///     +-----+-----+-----+-----+
    /// ~~~
    #[inline(always)]
    pub fn from_value(value: T) -> Mat4<T> {
        Mat4::new(value, Zero::zero(), Zero::zero(), Zero::zero(),
                  Zero::zero(), value, Zero::zero(), Zero::zero(),
                  Zero::zero(), Zero::zero(), value, Zero::zero(),
                  Zero::zero(), Zero::zero(), Zero::zero(), value)
    }

    /// Returns the multiplicative identity matrix
    /// ~~~
    ///       c0   c1   c2   c3
    ///     +----+----+----+----+
    ///  r0 |  1 |  0 |  0 |  0 |
    ///     +----+----+----+----+
    ///  r1 |  0 |  1 |  0 |  0 |
    ///     +----+----+----+----+
    ///  r2 |  0 |  0 |  1 |  0 |
    ///     +----+----+----+----+
    ///  r3 |  0 |  0 |  0 |  1 |
    ///     +----+----+----+----+
    /// ~~~
    #[inline(always)]
    pub fn identity() -> Mat4<T> {
        Mat4::new(One::one::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), One::one::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), One::one::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), One::one::<T>())
    }

    /// Returns the additive identity matrix
    /// ~~~
    ///       c0   c1   c2   c3
    ///     +----+----+----+----+
    ///  r0 |  0 |  0 |  0 |  0 |
    ///     +----+----+----+----+
    ///  r1 |  0 |  0 |  0 |  0 |
    ///     +----+----+----+----+
    ///  r2 |  0 |  0 |  0 |  0 |
    ///     +----+----+----+----+
    ///  r3 |  0 |  0 |  0 |  0 |
    ///     +----+----+----+----+
    /// ~~~
    #[inline(always)]
    pub fn zero() -> Mat4<T> {
        Mat4::new(Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    pub fn mul_t(&self, value: T) -> Mat4<T> {
        Mat4::from_cols(self.col(0).mul_t(value),
                        self.col(1).mul_t(value),
                        self.col(2).mul_t(value),
                        self.col(3).mul_t(value))
    }

    #[inline(always)]
    pub fn mul_v(&self, vec: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self.row(0).dot(vec),
                  self.row(1).dot(vec),
                  self.row(2).dot(vec),
                  self.row(3).dot(vec))
    }

    #[inline(always)]
    pub fn add_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self.col(0).add_v(other.col(0)),
                        self.col(1).add_v(other.col(1)),
                        self.col(2).add_v(other.col(2)),
                        self.col(3).add_v(other.col(3)))
    }

    #[inline(always)]
    pub fn sub_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self.col(0).sub_v(other.col(0)),
                        self.col(1).sub_v(other.col(1)),
                        self.col(2).sub_v(other.col(2)),
                        self.col(3).sub_v(other.col(3)))
    }

    #[inline(always)]
    pub fn mul_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::new(self.row(0).dot(other.col(0)),
                  self.row(1).dot(other.col(0)),
                  self.row(2).dot(other.col(0)),
                  self.row(3).dot(other.col(0)),

                  self.row(0).dot(other.col(1)),
                  self.row(1).dot(other.col(1)),
                  self.row(2).dot(other.col(1)),
                  self.row(3).dot(other.col(1)),

                  self.row(0).dot(other.col(2)),
                  self.row(1).dot(other.col(2)),
                  self.row(2).dot(other.col(2)),
                  self.row(3).dot(other.col(2)),

                  self.row(0).dot(other.col(3)),
                  self.row(1).dot(other.col(3)),
                  self.row(2).dot(other.col(3)),
                  self.row(3).dot(other.col(3)))
    }

    #[inline(always)]
    pub fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(value);
        self.col_mut(1).mul_self_t(value);
        self.col_mut(2).mul_self_t(value);
        self.col_mut(3).mul_self_t(value);
    }

    #[inline(always)]
    pub fn add_self_m(&mut self, other: &Mat4<T>) {
        self.col_mut(0).add_self_v(other.col(0));
        self.col_mut(1).add_self_v(other.col(1));
        self.col_mut(2).add_self_v(other.col(2));
        self.col_mut(3).add_self_v(other.col(3));
    }

    #[inline(always)]
    pub fn sub_self_m(&mut self, other: &Mat4<T>) {
        self.col_mut(0).sub_self_v(other.col(0));
        self.col_mut(1).sub_self_v(other.col(1));
        self.col_mut(2).sub_self_v(other.col(2));
        self.col_mut(3).sub_self_v(other.col(3));
    }

    pub fn dot(&self, other: &Mat4<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    pub fn determinant(&self) -> T {
        let m0 = Mat3::new(*self.elem(1, 1), *self.elem(2, 1), *self.elem(3, 1),
                           *self.elem(1, 2), *self.elem(2, 2), *self.elem(3, 2),
                           *self.elem(1, 3), *self.elem(2, 3), *self.elem(3, 3));
        let m1 = Mat3::new(*self.elem(0, 1), *self.elem(2, 1), *self.elem(3, 1),
                           *self.elem(0, 2), *self.elem(2, 2), *self.elem(3, 2),
                           *self.elem(0, 3), *self.elem(2, 3), *self.elem(3, 3));
        let m2 = Mat3::new(*self.elem(0, 1), *self.elem(1, 1), *self.elem(3, 1),
                           *self.elem(0, 2), *self.elem(1, 2), *self.elem(3, 2),
                           *self.elem(0, 3), *self.elem(1, 3), *self.elem(3, 3));
        let m3 = Mat3::new(*self.elem(0, 1), *self.elem(1, 1), *self.elem(2, 1),
                           *self.elem(0, 2), *self.elem(1, 2), *self.elem(2, 2),
                           *self.elem(0, 3), *self.elem(1, 3), *self.elem(2, 3));

        self.elem(0, 0) * m0.determinant() -
        self.elem(1, 0) * m1.determinant() +
        self.elem(2, 0) * m2.determinant() -
        self.elem(3, 0) * m3.determinant()
    }

    pub fn trace(&self) -> T {
        *self.elem(0, 0) +
        *self.elem(1, 1) +
        *self.elem(2, 2) +
        *self.elem(3, 3)
    }

    #[inline(always)]
    pub fn to_identity(&mut self) {
        *self = Mat4::identity();
    }

    #[inline(always)]
    pub fn to_zero(&mut self) {
        *self = Mat4::zero();
    }
}

impl<T:Copy + Num> Neg<Mat4<T>> for Mat4<T> {
    #[inline(always)]
    pub fn neg(&self) -> Mat4<T> {
        Mat4::from_cols(-self.col(0), -self.col(1), -self.col(2), -self.col(3))
    }
}

impl<T:Copy + Real + ApproxEq<T>> Mat4<T> {
    pub fn inverse(&self) -> Option<Mat4<T>> {
        let d = self.determinant();
        if d.approx_eq(&Zero::zero()) {
            None
        } else {
            // Gauss Jordan Elimination with partial pivoting
            // So take this matrix, A, augmented with the identity
            // and essentially reduce [A|I]

            let mut A = *self;
            let mut I = Mat4::identity::<T>();

            for uint::range(0, 4) |j| {
                // Find largest element in col j
                let mut i1 = j;
                for uint::range(j + 1, 4) |i| {
                    if A.elem(j, i).abs() > A.elem(j, i1).abs() {
                        i1 = i;
                    }
                }

                // Swap columns i1 and j in A and I to
                // put pivot on diagonal
                A.swap_cols(i1, j);
                I.swap_cols(i1, j);

                // Scale col j to have a unit diagonal
                let ajj = *A.elem(j, j);
                I.col_mut(j).div_self_t(ajj);
                A.col_mut(j).div_self_t(ajj);

                // Eliminate off-diagonal elems in col j of A,
                // doing identical ops to I
                for uint::range(0, 4) |i| {
                    if i != j {
                        let ij_mul_aij = I.col(j).mul_t(*A.elem(i, j));
                        let aj_mul_aij = A.col(j).mul_t(*A.elem(i, j));
                        I.col_mut(i).sub_self_v(&ij_mul_aij);
                        A.col_mut(i).sub_self_v(&aj_mul_aij);
                    }
                }
            }
            Some(I)
        }
    }

    #[inline(always)]
    pub fn invert_self(&mut self) {
        *self = self.inverse().expect("Couldn't invert the matrix!");
    }

    #[inline(always)]
    pub fn is_identity(&self) -> bool {
        self.approx_eq(&Mat4::identity())
    }

    #[inline(always)]
    pub fn is_diagonal(&self) -> bool {
        self.elem(0, 1).approx_eq(&Zero::zero()) &&
        self.elem(0, 2).approx_eq(&Zero::zero()) &&
        self.elem(0, 3).approx_eq(&Zero::zero()) &&

        self.elem(1, 0).approx_eq(&Zero::zero()) &&
        self.elem(1, 2).approx_eq(&Zero::zero()) &&
        self.elem(1, 3).approx_eq(&Zero::zero()) &&

        self.elem(2, 0).approx_eq(&Zero::zero()) &&
        self.elem(2, 1).approx_eq(&Zero::zero()) &&
        self.elem(2, 3).approx_eq(&Zero::zero()) &&

        self.elem(3, 0).approx_eq(&Zero::zero()) &&
        self.elem(3, 1).approx_eq(&Zero::zero()) &&
        self.elem(3, 2).approx_eq(&Zero::zero())
    }

    #[inline(always)]
    pub fn is_rotated(&self) -> bool {
        !self.approx_eq(&Mat4::identity())
    }

    #[inline(always)]
    pub fn is_symmetric(&self) -> bool {
        self.elem(0, 1).approx_eq(self.elem(1, 0)) &&
        self.elem(0, 2).approx_eq(self.elem(2, 0)) &&
        self.elem(0, 3).approx_eq(self.elem(3, 0)) &&

        self.elem(1, 0).approx_eq(self.elem(0, 1)) &&
        self.elem(1, 2).approx_eq(self.elem(2, 1)) &&
        self.elem(1, 3).approx_eq(self.elem(3, 1)) &&

        self.elem(2, 0).approx_eq(self.elem(0, 2)) &&
        self.elem(2, 1).approx_eq(self.elem(1, 2)) &&
        self.elem(2, 3).approx_eq(self.elem(3, 2)) &&

        self.elem(3, 0).approx_eq(self.elem(0, 3)) &&
        self.elem(3, 1).approx_eq(self.elem(1, 3)) &&
        self.elem(3, 2).approx_eq(self.elem(2, 3))
    }

    #[inline(always)]
    pub fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&Zero::zero())
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Mat4<T> {
    #[inline(always)]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    pub fn approx_eq(&self, other: &Mat4<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    pub fn approx_eq_eps(&self, other: &Mat4<T>, epsilon: &T) -> bool {
        self.col(0).approx_eq_eps(other.col(0), epsilon) &&
        self.col(1).approx_eq_eps(other.col(1), epsilon) &&
        self.col(2).approx_eq_eps(other.col(2), epsilon) &&
        self.col(3).approx_eq_eps(other.col(3), epsilon)
    }
}

// GLSL-style type aliases, corresponding to Section 4.1.6 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

// a 4×4 single-precision floating-point matrix
pub type mat4  = Mat4<f32>;
// a 4×4 double-precision floating-point matrix
pub type dmat4 = Mat4<f64>;

// Rust-style type aliases
pub type Mat4f   = Mat4<float>;
pub type Mat4f32 = Mat4<f32>;
pub type Mat4f64 = Mat4<f64>;
