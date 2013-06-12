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
use super::{Mat3, Mat4};

#[deriving(Eq)]
pub struct Mat2<T> { x: Vec2<T>, y: Vec2<T> }

impl<T> Mat2<T> {
    #[inline]
    pub fn col<'a>(&'a self, i: uint) -> &'a Vec2<T> {
        &'a self.as_slice()[i]
    }

    #[inline]
    pub fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut Vec2<T> {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [Vec2<T>,..2] {
        unsafe { transmute(self) }
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [Vec2<T>,..2] {
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
    #[inline]
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
    #[inline]
    pub fn from_cols(c0: Vec2<T>,
                     c1: Vec2<T>) -> Mat2<T> {
        Mat2 { x: c0, y: c1 }
    }

    #[inline]
    pub fn row(&self, i: uint) -> Vec2<T> {
        Vec2::new(*self.elem(0, i),
                  *self.elem(1, i))
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
    }

    #[inline]
    pub fn transpose(&self) -> Mat2<T> {
        Mat2::new(*self.elem(0, 0), *self.elem(1, 0),
                  *self.elem(0, 1), *self.elem(1, 1))
    }

    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn zero() -> Mat2<T> {
        Mat2::new(Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline]
    pub fn mul_t(&self, value: T) -> Mat2<T> {
        Mat2::from_cols(self.col(0).mul_t(value),
                        self.col(1).mul_t(value))
    }

    #[inline]
    pub fn mul_v(&self, vec: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self.row(0).dot(vec),
                  self.row(1).dot(vec))
    }

    #[inline]
    pub fn add_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self.col(0).add_v(other.col(0)),
                        self.col(1).add_v(other.col(1)))
    }

    #[inline]
    pub fn sub_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self.col(0).sub_v(other.col(0)),
                        self.col(1).sub_v(other.col(1)))
    }

    #[inline]
    pub fn mul_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::new(self.row(0).dot(other.col(0)), self.row(1).dot(other.col(0)),
                  self.row(0).dot(other.col(1)), self.row(1).dot(other.col(1)))
    }

    #[inline]
    pub fn mul_self_t(&mut self, value: T) {
        self.x.mul_self_t(value);
        self.y.mul_self_t(value);
    }

    #[inline]
    pub fn add_self_m(&mut self, other: &Mat2<T>) {
        self.x.add_self_v(other.col(0));
        self.y.add_self_v(other.col(1));
    }

    #[inline]
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

    #[inline]
    pub fn to_identity(&mut self) {
        *self = Mat2::identity();
    }

    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn to_mat4(&self) -> Mat4<T> {
        Mat4::new(*self.elem(0, 0), *self.elem(0, 1), Zero::zero(), Zero::zero(),
                  *self.elem(1, 0), *self.elem(1, 1), Zero::zero(), Zero::zero(),
                  Zero::zero(), Zero::zero(), One::one(), Zero::zero(),
                  Zero::zero(), Zero::zero(), Zero::zero(), One::one())
    }
}

impl<T:Copy + Num> Neg<Mat2<T>> for Mat2<T> {
    #[inline]
    pub fn neg(&self) -> Mat2<T> {
        Mat2::from_cols(-self.col(0), -self.col(1))
    }
}

impl<T:Copy + Real> Mat2<T> {
    #[inline]
    pub fn from_angle(radians: T) -> Mat2<T> {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat2::new(cos_theta, -sin_theta,
                  sin_theta,  cos_theta)
    }
}

impl<T:Copy + Real + ApproxEq<T>> Mat2<T> {
    #[inline]
    pub fn inverse(&self) -> Option<Mat2<T>> {
        let d = self.determinant();
        if d.approx_eq(&Zero::zero()) {
            None
        } else {
            Some(Mat2::new(self.elem(1, 1) / d, -self.elem(0, 1) / d,
                           -self.elem(1, 0) / d, self.elem(0, 0) / d))
        }
    }

    #[inline]
    pub fn invert_self(&mut self) {
        *self = self.inverse().expect("Couldn't invert the matrix!");
    }

    #[inline]
    pub fn is_identity(&self) -> bool {
        self.approx_eq(&Mat2::identity())
    }

    #[inline]
    pub fn is_diagonal(&self) -> bool {
        self.elem(0, 1).approx_eq(&Zero::zero()) &&
        self.elem(1, 0).approx_eq(&Zero::zero())
    }

    #[inline]
    pub fn is_rotated(&self) -> bool {
        !self.approx_eq(&Mat2::identity())
    }

    #[inline]
    pub fn is_symmetric(&self) -> bool {
        self.elem(0, 1).approx_eq(self.elem(1, 0)) &&
        self.elem(1, 0).approx_eq(self.elem(0, 1))
    }

    #[inline]
    pub fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&Zero::zero())
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Mat2<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &Mat2<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &Mat2<T>, epsilon: &T) -> bool {
        self.col(0).approx_eq_eps(other.col(0), epsilon) &&
        self.col(1).approx_eq_eps(other.col(1), epsilon)
    }
}
