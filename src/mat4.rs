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
use super::Mat3;

#[deriving(Eq)]
pub struct Mat4<T> {
    x: Vec4<T>,
    y: Vec4<T>,
    z: Vec4<T>,
    w: Vec4<T>,
}

impl<T> Mat4<T> {
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
    #[inline]
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
    #[inline]
    pub fn from_cols(c0: Vec4<T>,
                     c1: Vec4<T>,
                     c2: Vec4<T>,
                     c3: Vec4<T>) -> Mat4<T> {
        Mat4 { x: c0, y: c1, z: c2, w: c3 }
    }

    #[inline]
    pub fn col<'a>(&'a self, i: uint) -> &'a Vec4<T> {
        &'a self.as_slice()[i]
    }

    #[inline]
    pub fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut Vec4<T> {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [Vec4<T>,..4] {
        unsafe { transmute(self) }
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [Vec4<T>,..4] {
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

    #[inline(always)]
    pub fn map(&self, f: &fn(&Vec4<T>) -> Vec4<T>) -> Mat4<T> {
        Mat4::from_cols(f(self.col(0)),
                        f(self.col(1)),
                        f(self.col(2)),
                        f(self.col(3)))
    }
}

impl<T:Copy> Mat4<T> {
    #[inline]
    pub fn row(&self, i: uint) -> Vec4<T> {
        Vec4::new(*self.elem(0, i),
                  *self.elem(1, i),
                  *self.elem(2, i),
                  *self.elem(3, i))
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
        self.w.swap(a, b);
    }

    #[inline]
    pub fn transpose(&self) -> Mat4<T> {
        Mat4::new(*self.elem(0, 0), *self.elem(1, 0), *self.elem(2, 0), *self.elem(3, 0),
                  *self.elem(0, 1), *self.elem(1, 1), *self.elem(2, 1), *self.elem(3, 1),
                  *self.elem(0, 2), *self.elem(1, 2), *self.elem(2, 2), *self.elem(3, 2),
                  *self.elem(0, 3), *self.elem(1, 3), *self.elem(2, 3), *self.elem(3, 3))
    }

    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn zero() -> Mat4<T> {
        Mat4::new(Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                  Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline]
    pub fn mul_t(&self, value: T) -> Mat4<T> {
        Mat4::from_cols(self.col(0).mul_t(value),
                        self.col(1).mul_t(value),
                        self.col(2).mul_t(value),
                        self.col(3).mul_t(value))
    }

    #[inline]
    pub fn mul_v(&self, vec: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self.row(0).dot(vec),
                  self.row(1).dot(vec),
                  self.row(2).dot(vec),
                  self.row(3).dot(vec))
    }

    #[inline]
    pub fn add_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self.col(0).add_v(other.col(0)),
                        self.col(1).add_v(other.col(1)),
                        self.col(2).add_v(other.col(2)),
                        self.col(3).add_v(other.col(3)))
    }

    #[inline]
    pub fn sub_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self.col(0).sub_v(other.col(0)),
                        self.col(1).sub_v(other.col(1)),
                        self.col(2).sub_v(other.col(2)),
                        self.col(3).sub_v(other.col(3)))
    }

    #[inline]
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

    #[inline]
    pub fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(value);
        self.col_mut(1).mul_self_t(value);
        self.col_mut(2).mul_self_t(value);
        self.col_mut(3).mul_self_t(value);
    }

    #[inline]
    pub fn add_self_m(&mut self, other: &Mat4<T>) {
        self.col_mut(0).add_self_v(other.col(0));
        self.col_mut(1).add_self_v(other.col(1));
        self.col_mut(2).add_self_v(other.col(2));
        self.col_mut(3).add_self_v(other.col(3));
    }

    #[inline]
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

    #[inline]
    pub fn to_identity(&mut self) {
        *self = Mat4::identity();
    }

    #[inline]
    pub fn to_zero(&mut self) {
        *self = Mat4::zero();
    }
}

impl<T:Copy + Num> Neg<Mat4<T>> for Mat4<T> {
    #[inline]
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

    #[inline]
    pub fn invert_self(&mut self) {
        *self = self.inverse().expect("Couldn't invert the matrix!");
    }

    #[inline]
    pub fn is_identity(&self) -> bool {
        self.approx_eq(&Mat4::identity())
    }

    #[inline]
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

    #[inline]
    pub fn is_rotated(&self) -> bool {
        !self.approx_eq(&Mat4::identity())
    }

    #[inline]
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

    #[inline]
    pub fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&Zero::zero())
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Mat4<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &Mat4<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &Mat4<T>, epsilon: &T) -> bool {
        self.col(0).approx_eq_eps(other.col(0), epsilon) &&
        self.col(1).approx_eq_eps(other.col(1), epsilon) &&
        self.col(2).approx_eq_eps(other.col(2), epsilon) &&
        self.col(3).approx_eq_eps(other.col(3), epsilon)
    }
}

#[cfg(test)]
mod tests{
    use mat::*;
    use vec::*;

    #[test]
    fn test_mat4() {
        let a = Mat4 { x: Vec4 { x: 1.0, y: 5.0, z:  9.0, w: 13.0 },
                       y: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                       z: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                       w: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 } };
        let b = Mat4 { x: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                       y: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                       z: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 },
                       w: Vec4 { x: 5.0, y: 9.0, z: 13.0, w: 17.0 } };
        let c = Mat4 { x: Vec4 { x: 3.0, y: 2.0, z:  1.0, w:  1.0 },
                       y: Vec4 { x: 2.0, y: 3.0, z:  2.0, w:  2.0 },
                       z: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  3.0 },
                       w: Vec4 { x: 0.0, y: 1.0, z:  1.0, w:  0.0 } };

        let v1 = Vec4::new::<float>(1.0, 2.0, 3.0, 4.0);
        let f1 = 0.5;

        assert_eq!(a, Mat4::new::<float>(1.0, 5.0,  9.0, 13.0,
                                         2.0, 6.0, 10.0, 14.0,
                                         3.0, 7.0, 11.0, 15.0,
                                         4.0, 8.0, 12.0, 16.0));

        assert_eq!(a, Mat4::from_cols::<float>(Vec4::new::<float>(1.0, 5.0,  9.0, 13.0),
                                               Vec4::new::<float>(2.0, 6.0, 10.0, 14.0),
                                               Vec4::new::<float>(3.0, 7.0, 11.0, 15.0),
                                               Vec4::new::<float>(4.0, 8.0, 12.0, 16.0)));

        assert_eq!(Mat4::from_value::<float>(4.0),
                   Mat4::new::<float>(4.0, 0.0, 0.0, 0.0,
                                      0.0, 4.0, 0.0, 0.0,
                                      0.0, 0.0, 4.0, 0.0,
                                      0.0, 0.0, 0.0, 4.0));

        assert_eq!(*a.col(0), Vec4::new::<float>(1.0, 5.0,  9.0, 13.0));
        assert_eq!(*a.col(1), Vec4::new::<float>(2.0, 6.0, 10.0, 14.0));
        assert_eq!(*a.col(2), Vec4::new::<float>(3.0, 7.0, 11.0, 15.0));
        assert_eq!(*a.col(3), Vec4::new::<float>(4.0, 8.0, 12.0, 16.0));

        assert_eq!(a.row(0), Vec4::new::<float>( 1.0,  2.0,  3.0,  4.0));
        assert_eq!(a.row(1), Vec4::new::<float>( 5.0,  6.0,  7.0,  8.0));
        assert_eq!(a.row(2), Vec4::new::<float>( 9.0, 10.0, 11.0, 12.0));
        assert_eq!(a.row(3), Vec4::new::<float>(13.0, 14.0, 15.0, 16.0));

        assert_eq!(*a.col(0), Vec4::new::<float>(1.0, 5.0,  9.0, 13.0));
        assert_eq!(*a.col(1), Vec4::new::<float>(2.0, 6.0, 10.0, 14.0));
        assert_eq!(*a.col(2), Vec4::new::<float>(3.0, 7.0, 11.0, 15.0));
        assert_eq!(*a.col(3), Vec4::new::<float>(4.0, 8.0, 12.0, 16.0));

        assert_eq!(Mat4::identity::<float>(),
                   Mat4::new::<float>(1.0, 0.0, 0.0, 0.0,
                                      0.0, 1.0, 0.0, 0.0,
                                      0.0, 0.0, 1.0, 0.0,
                                      0.0, 0.0, 0.0, 1.0));

        assert_eq!(Mat4::zero::<float>(),
                   Mat4::new::<float>(0.0, 0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0, 0.0));

        assert_eq!(a.determinant(), 0.0);
        assert_eq!(a.trace(), 34.0);

        assert_eq!(a.neg(),
                   Mat4::new::<float>(-1.0, -5.0,  -9.0, -13.0,
                                      -2.0, -6.0, -10.0, -14.0,
                                      -3.0, -7.0, -11.0, -15.0,
                                      -4.0, -8.0, -12.0, -16.0));
        assert_eq!(-a, a.neg());
        assert_eq!(a.mul_t(f1),
                   Mat4::new::<float>(0.5, 2.5, 4.5, 6.5,
                                      1.0, 3.0, 5.0, 7.0,
                                      1.5, 3.5, 5.5, 7.5,
                                      2.0, 4.0, 6.0, 8.0));
        assert_eq!(a.mul_v(&v1),
                   Vec4::new::<float>(30.0, 70.0, 110.0, 150.0));
        assert_eq!(a.add_m(&b),
                   Mat4::new::<float>(3.0, 11.0, 19.0, 27.0,
                                      5.0, 13.0, 21.0, 29.0,
                                      7.0, 15.0, 23.0, 31.0,
                                      9.0, 17.0, 25.0, 33.0));
        assert_eq!(a.sub_m(&b),
                   Mat4::new::<float>(-1.0, -1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0, -1.0));
        assert_eq!(a.mul_m(&b),
                   Mat4::new::<float>(100.0, 228.0, 356.0, 484.0,
                                      110.0, 254.0, 398.0, 542.0,
                                      120.0, 280.0, 440.0, 600.0,
                                      130.0, 306.0, 482.0, 658.0));
        assert_eq!(a.dot(&b), 1632.0);
        assert_eq!(a.transpose(),
                   Mat4::new::<float>( 1.0,  2.0,  3.0,  4.0,
                                       5.0,  6.0,  7.0,  8.0,
                                       9.0, 10.0, 11.0, 12.0,
                                      13.0, 14.0, 15.0, 16.0));

        assert_approx_eq!(c.inverse().unwrap(),
                          Mat4::new::<float>( 5.0, -4.0,  1.0,  0.0,
                                             -4.0,  8.0, -4.0,  0.0,
                                              4.0, -8.0,  4.0,  8.0,
                                             -3.0,  4.0,  1.0, -8.0).mul_t(0.125));

        let ident = Mat4::identity::<float>();

        assert_eq!(ident.inverse().unwrap(), ident);

        assert!(ident.is_identity());
        assert!(ident.is_symmetric());
        assert!(ident.is_diagonal());
        assert!(!ident.is_rotated());
        assert!(ident.is_invertible());

        assert!(!a.is_identity());
        assert!(!a.is_symmetric());
        assert!(!a.is_diagonal());
        assert!(a.is_rotated());
        assert!(!a.is_invertible());

        let c = Mat4::new::<float>(4.0, 3.0, 2.0, 1.0,
                                   3.0, 4.0, 3.0, 2.0,
                                   2.0, 3.0, 4.0, 3.0,
                                   1.0, 2.0, 3.0, 4.0);
        assert!(!c.is_identity());
        assert!(c.is_symmetric());
        assert!(!c.is_diagonal());
        assert!(c.is_rotated());
        assert!(c.is_invertible());

        assert!(Mat4::from_value::<float>(6.0).is_diagonal());
    }

    fn test_mat4_mut() {
        let a = Mat4 { x: Vec4 { x: 1.0, y: 5.0, z:  9.0, w: 13.0 },
                       y: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                       z: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                       w: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 } };
        let b = Mat4 { x: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                       y: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                       z: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 },
                       w: Vec4 { x: 5.0, y: 9.0, z: 13.0, w: 17.0 } };
        let c = Mat4 { x: Vec4 { x: 3.0, y: 2.0, z:  1.0, w:  1.0 },
                       y: Vec4 { x: 2.0, y: 3.0, z:  2.0, w:  2.0 },
                       z: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  3.0 },
                       w: Vec4 { x: 0.0, y: 1.0, z:  1.0, w:  0.0 } };

        let f1 = 0.5;

        let mut mut_a = a;
        let mut mut_c = c;

        mut_a.swap_cols(0, 3);
        assert_eq!(mut_a.col(0), a.col(3));
        assert_eq!(mut_a.col(3), a.col(0));
        mut_a = a;

        mut_a.swap_cols(1, 2);
        assert_eq!(mut_a.col(1), a.col(2));
        assert_eq!(mut_a.col(2), a.col(1));
        mut_a = a;

        mut_a.swap_rows(0, 3);
        assert_eq!(mut_a.row(0), a.row(3));
        assert_eq!(mut_a.row(3), a.row(0));
        mut_a = a;

        mut_a.swap_rows(1, 2);
        assert_eq!(mut_a.row(1), a.row(2));
        assert_eq!(mut_a.row(2), a.row(1));
        mut_a = a;

        mut_a.to_identity();
        assert!(mut_a.is_identity());
        mut_a = a;

        mut_a.to_zero();
        assert_eq!(mut_a, Mat4::zero::<float>());
        mut_a = a;

        mut_a.mul_self_t(f1);
        assert_eq!(mut_a, a.mul_t(f1));
        mut_a = a;

        mut_a.add_self_m(&b);
        assert_eq!(mut_a, a.add_m(&b));
        mut_a = a;

        mut_a.sub_self_m(&b);
        assert_eq!(mut_a, a.sub_m(&b));
        mut_a = a;

        mut_c.invert_self();
        assert_eq!(mut_c, c.inverse().unwrap());
        // mut_c = c;

        mut_a.transpose_self();
        assert_eq!(mut_a, a.transpose());
        // mut_a = a;
    }

    #[test]
    fn test_mat4_approx_eq() {
        assert!(!Mat4::new::<float>(0.000001, 0.000001, 0.000001, 0.000001,
                                    0.000001, 0.000001, 0.000001, 0.000001,
                                    0.000001, 0.000001, 0.000001, 0.000001,
                                    0.000001, 0.000001, 0.000001, 0.000001)
                .approx_eq(&Mat4::zero::<float>()));
        assert!(Mat4::new::<float>(0.0000001, 0.0000001, 0.0000001, 0.0000001,
                                   0.0000001, 0.0000001, 0.0000001, 0.0000001,
                                   0.0000001, 0.0000001, 0.0000001, 0.0000001,
                                   0.0000001, 0.0000001, 0.0000001, 0.0000001)
                .approx_eq(&Mat4::zero::<float>()));
    }
}
