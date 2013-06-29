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

#[macro_escape];

macro_rules! impl_mat(
    ($Mat:ident, $Vec:ident) => (
        impl<T> $Mat<T> {
            #[inline]
            pub fn col<'a>(&'a self, i: uint) -> &'a $Vec<T> {
                self.index(i)
            }

            #[inline]
            pub fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut $Vec<T> {
                self.index_mut(i)
            }

            #[inline]
            pub fn elem<'a>(&'a self, i: uint, j: uint) -> &'a T {
                self.index(i).index(j)
            }

            #[inline]
            pub fn elem_mut<'a>(&'a mut self, i: uint, j: uint) -> &'a mut T {
                self.index_mut(i).index_mut(j)
            }
        }
    )
)

macro_rules! impl_mat_copyable(
    ($Mat:ident, $Vec:ident) => (
        impl<T:Copy> $Mat<T> {
            #[inline]
            pub fn row(&self, i: uint) -> $Vec<T> {
                $Vec::from_slice(self.map(|c| copy *c.index(i)))
            }

            #[inline]
            pub fn swap_cols(&mut self, a: uint, b: uint) {
                let tmp = copy *self.col(a);
                *self.col_mut(a) = copy *self.col(b);
                *self.col_mut(b) = tmp;
            }

            #[inline]
            pub fn swap_rows(&mut self, a: uint, b: uint) {
                self.map_mut(|x| x.swap(a, b))
            }

            #[inline]
            pub fn swap_elem(&mut self, (ai, aj): (uint, uint), (bi, bj): (uint, uint)) {
                let tmp = copy *self.elem(ai, aj);
                *self.elem_mut(ai, aj) = copy *self.elem(bi, bj);
                *self.elem_mut(bi, bj) = tmp;
            }

            #[inline] pub fn transpose(&self) -> $Mat<T> { mat_transpose!($Mat) }

            #[inline] pub fn transpose_self(&mut self) { mat_transpose_self!($Mat) }
        }
    )
)

macro_rules! mat_transpose(
    (Mat2) => (
        Mat2::new(copy *self.elem(0, 0), copy *self.elem(1, 0),
                  copy *self.elem(0, 1), copy *self.elem(1, 1))
    );
    (Mat3) => (
        Mat3::new(copy *self.elem(0, 0), copy *self.elem(1, 0), copy *self.elem(2, 0),
                  copy *self.elem(0, 1), copy *self.elem(1, 1), copy *self.elem(2, 1),
                  copy *self.elem(0, 2), copy *self.elem(1, 2), copy *self.elem(2, 2))
    );
    (Mat4) => (
        Mat4::new(copy *self.elem(0, 0), copy *self.elem(1, 0), copy *self.elem(2, 0), copy *self.elem(3, 0),
                  copy *self.elem(0, 1), copy *self.elem(1, 1), copy *self.elem(2, 1), copy *self.elem(3, 1),
                  copy *self.elem(0, 2), copy *self.elem(1, 2), copy *self.elem(2, 2), copy *self.elem(3, 2),
                  copy *self.elem(0, 3), copy *self.elem(1, 3), copy *self.elem(2, 3), copy *self.elem(3, 3))
    );
)

macro_rules! mat_transpose_self(
    (Mat2) => (
        self.swap_elem((0, 1), (1, 0));
        self.swap_elem((1, 0), (0, 1));
    );
    (Mat3) => (
        self.swap_elem((0, 1), (1, 0));
        self.swap_elem((0, 2), (2, 0));

        self.swap_elem((1, 0), (0, 1));
        self.swap_elem((1, 2), (2, 1));

        self.swap_elem((2, 0), (0, 2));
        self.swap_elem((2, 1), (1, 2));
    );
    (Mat4) => (
        self.swap_elem((0, 1), (1, 0));
        self.swap_elem((0, 2), (2, 0));
        self.swap_elem((0, 3), (3, 0));

        self.swap_elem((1, 0), (0, 1));
        self.swap_elem((1, 2), (2, 1));
        self.swap_elem((1, 3), (3, 1));

        self.swap_elem((2, 0), (0, 2));
        self.swap_elem((2, 1), (1, 2));
        self.swap_elem((2, 3), (3, 2));

        self.swap_elem((3, 0), (0, 3));
        self.swap_elem((3, 1), (1, 3));
        self.swap_elem((3, 2), (2, 3));
    );
)

macro_rules! impl_mat_numeric(
    ($Mat:ident, $Vec:ident) => (
        impl<T:Copy + Num> $Mat<T> {
            #[inline]
            pub fn from_value(value: T) -> $Mat<T> { mat_from_value!($Mat) }

            #[inline]
            pub fn identity() -> $Mat<T> { $Mat::from_value(one!(T)) }

            #[inline]
            pub fn zero() -> $Mat<T> { $Mat::from_value(zero!(T)) }

            #[inline]
            pub fn mul_t(&self, value: T) -> $Mat<T> {
                $Mat::from_slice(self.map(|&c| c.mul_t(copy value)))
            }

            #[inline]
            pub fn mul_v(&self, vec: &$Vec<T>) -> $Vec<T> {
                mat_mul_v!($Mat)
            }

            #[inline]
            pub fn mul_m(&self, other: &$Mat<T>) -> $Mat<T> {
                mat_mul_m!($Mat)
            }

            #[inline]
            pub fn add_m(&self, other: &$Mat<T>) -> $Mat<T> {
                $Mat::from_slice(self.zip(other, |a, b| a.add_v(b)))
            }

            #[inline]
            pub fn sub_m(&self, other: &$Mat<T>) -> $Mat<T> {
                $Mat::from_slice(self.zip(other, |a, b| a.sub_v(b)))
            }

            #[inline]
            pub fn mul_self_t(&mut self, value: T) {
                self.map_mut(|x| x.mul_self_t(copy value))
            }

            #[inline]
            pub fn add_self_m(&mut self, other: &$Mat<T>) {
                self.zip_mut(other, |a, b| a.add_self_v(b))
            }

            #[inline]
            pub fn sub_self_m(&mut self, other: &$Mat<T>) {
                self.zip_mut(other, |a, b| a.sub_self_v(b))
            }

            pub fn dot(&self, other: &$Mat<T>) -> T {
                other.transpose().mul_m(self).trace()
            }

            pub fn determinant(&self) -> T {
                mat_determinant!($Mat)
            }

            pub fn trace(&self) -> T {
                mat_trace!($Mat)
            }

            #[inline]
            pub fn to_identity(&mut self) {
                *self = $Mat::identity();
            }

            #[inline]
            pub fn to_zero(&mut self) {
                *self = $Mat::zero();
            }
        }
    )
)

macro_rules! mat_from_value(
    (Mat2) => (
        Mat2::new(copy value, zero!(T),
                  zero!(T), copy value)
    );
    (Mat3) => (
        Mat3::new(copy value, zero!(T), zero!(T),
                  zero!(T), copy value, zero!(T),
                  zero!(T), zero!(T), copy value)
    );
    (Mat4) => (
        Mat4::new(copy value, zero!(T), zero!(T), zero!(T),
                  zero!(T), copy value, zero!(T), zero!(T),
                  zero!(T), zero!(T), copy value, zero!(T),
                  zero!(T), zero!(T), zero!(T), copy value)
    );
)

macro_rules! mat_mul_v(
    (Mat2) => (
        Vec2::new(self.row(0).dot(vec),
                  self.row(1).dot(vec))
    );
    (Mat3) => (
        Vec3::new(self.row(0).dot(vec),
                  self.row(1).dot(vec),
                  self.row(2).dot(vec))
    );
    (Mat4) => (
        Vec4::new(self.row(0).dot(vec),
                  self.row(1).dot(vec),
                  self.row(2).dot(vec),
                  self.row(3).dot(vec))
    );
)

macro_rules! mat_mul_m(
    (Mat2) => (
        Mat2::new(self.row(0).dot(other.col(0)), self.row(1).dot(other.col(0)),
                  self.row(0).dot(other.col(1)), self.row(1).dot(other.col(1)))
    );
    (Mat3) => (
        Mat3::new(self.row(0).dot(other.col(0)),
                  self.row(1).dot(other.col(0)),
                  self.row(2).dot(other.col(0)),

                  self.row(0).dot(other.col(1)),
                  self.row(1).dot(other.col(1)),
                  self.row(2).dot(other.col(1)),

                  self.row(0).dot(other.col(2)),
                  self.row(1).dot(other.col(2)),
                  self.row(2).dot(other.col(2)))
    );
    (Mat4) => (
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
    );
)

macro_rules! mat_determinant(
    (Mat2) => (
       *self.elem(0, 0) *
       *self.elem(1, 1) -
       *self.elem(1, 0) *
       *self.elem(0, 1)
    );
    (Mat3) => (
        self.col(0).dot(&self.col(1).cross(self.col(2)))
    );
    (Mat4) => ({
        let m0 = Mat3::new(copy *self.elem(1, 1), copy *self.elem(2, 1), copy *self.elem(3, 1),
                           copy *self.elem(1, 2), copy *self.elem(2, 2), copy *self.elem(3, 2),
                           copy *self.elem(1, 3), copy *self.elem(2, 3), copy *self.elem(3, 3));
        let m1 = Mat3::new(copy *self.elem(0, 1), copy *self.elem(2, 1), copy *self.elem(3, 1),
                           copy *self.elem(0, 2), copy *self.elem(2, 2), copy *self.elem(3, 2),
                           copy *self.elem(0, 3), copy *self.elem(2, 3), copy *self.elem(3, 3));
        let m2 = Mat3::new(copy *self.elem(0, 1), copy *self.elem(1, 1), copy *self.elem(3, 1),
                           copy *self.elem(0, 2), copy *self.elem(1, 2), copy *self.elem(3, 2),
                           copy *self.elem(0, 3), copy *self.elem(1, 3), copy *self.elem(3, 3));
        let m3 = Mat3::new(copy *self.elem(0, 1), copy *self.elem(1, 1), copy *self.elem(2, 1),
                           copy *self.elem(0, 2), copy *self.elem(1, 2), copy *self.elem(2, 2),
                           copy *self.elem(0, 3), copy *self.elem(1, 3), copy *self.elem(2, 3));

        self.elem(0, 0) * m0.determinant() -
        self.elem(1, 0) * m1.determinant() +
        self.elem(2, 0) * m2.determinant() -
        self.elem(3, 0) * m3.determinant()
    });
)

macro_rules! mat_trace(
    (Mat2) => (*self.elem(0, 0) + *self.elem(1, 1));
    (Mat3) => (*self.elem(0, 0) + *self.elem(1, 1) + *self.elem(2, 2));
    (Mat4) => (*self.elem(0, 0) + *self.elem(1, 1) + *self.elem(2, 2) + *self.elem(3, 3));
)

macro_rules! impl_mat_approx_numeric(
    ($Mat:ident) => (
        impl<T:Copy + Real + ApproxEq<T>> $Mat<T> {
            #[inline]
            pub fn inverse(&self) -> Option<$Mat<T>> {
                mat_inverse!($Mat)
            }

            #[inline]
            pub fn invert_self(&mut self) {
                *self = self.inverse().expect("Couldn't invert the matrix!");
            }

            #[inline]
            pub fn is_identity(&self) -> bool {
                self.approx_eq(&$Mat::identity())
            }

            #[inline]
            pub fn is_diagonal(&self) -> bool {
                mat_is_diagonal!($Mat)
            }

            #[inline]
            pub fn is_rotated(&self) -> bool {
                !self.approx_eq(&$Mat::identity())
            }

            #[inline]
            pub fn is_symmetric(&self) -> bool {
                mat_is_symmetric!($Mat)
            }

            #[inline]
            pub fn is_invertible(&self) -> bool {
                !self.determinant().approx_eq(&zero!(T))
            }
        }
    )
)

macro_rules! mat_inverse(
    (Mat2) => ({
        let d = self.determinant();
        if d.approx_eq(&zero!(T)) {
            None
        } else {
            Some(Mat2::new(self.elem(1, 1) / d, -self.elem(0, 1) / d,
                           -self.elem(1, 0) / d, self.elem(0, 0) / d))
        }
    });
    (Mat3) => ({
        let d = self.determinant();
        if d.approx_eq(&zero!(T)) {
            None
        } else {
            Some(Mat3::from_cols(self.col(1).cross(self.col(2)).div_t(copy d),
                                 self.col(2).cross(self.col(0)).div_t(copy d),
                                 self.col(0).cross(self.col(1)).div_t(copy d)).transpose())
        }
    });
    (Mat4) => ({
        use std::uint;

        let d = self.determinant();
        if d.approx_eq(&zero!(T)) {
            None
        } else {
            // Gauss Jordan Elimination with partial pivoting
            // So take this matrix, A, augmented with the identity
            // and essentially reduce [A|I]

            let mut A = copy *self;
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
                let ajj = copy *A.elem(j, j);
                I.col_mut(j).div_self_t(copy ajj);
                A.col_mut(j).div_self_t(copy ajj);

                // Eliminate off-diagonal elems in col j of A,
                // doing identical ops to I
                for uint::range(0, 4) |i| {
                    if i != j {
                        let ij_mul_aij = I.col(j).mul_t(copy *A.elem(i, j));
                        let aj_mul_aij = A.col(j).mul_t(copy *A.elem(i, j));
                        I.col_mut(i).sub_self_v(&ij_mul_aij);
                        A.col_mut(i).sub_self_v(&aj_mul_aij);
                    }
                }
            }
            Some(I)
        }
    });
)

macro_rules! mat_is_diagonal(
    (Mat2) => (
        self.elem(0, 1).approx_eq(&zero!(T)) &&
        self.elem(1, 0).approx_eq(&zero!(T))
    );
    (Mat3) => (
        self.elem(0, 1).approx_eq(&zero!(T)) &&
        self.elem(0, 2).approx_eq(&zero!(T)) &&

        self.elem(1, 0).approx_eq(&zero!(T)) &&
        self.elem(1, 2).approx_eq(&zero!(T)) &&

        self.elem(2, 0).approx_eq(&zero!(T)) &&
        self.elem(2, 1).approx_eq(&zero!(T))
    );
    (Mat4) => (
        self.elem(0, 1).approx_eq(&zero!(T)) &&
        self.elem(0, 2).approx_eq(&zero!(T)) &&
        self.elem(0, 3).approx_eq(&zero!(T)) &&

        self.elem(1, 0).approx_eq(&zero!(T)) &&
        self.elem(1, 2).approx_eq(&zero!(T)) &&
        self.elem(1, 3).approx_eq(&zero!(T)) &&

        self.elem(2, 0).approx_eq(&zero!(T)) &&
        self.elem(2, 1).approx_eq(&zero!(T)) &&
        self.elem(2, 3).approx_eq(&zero!(T)) &&

        self.elem(3, 0).approx_eq(&zero!(T)) &&
        self.elem(3, 1).approx_eq(&zero!(T)) &&
        self.elem(3, 2).approx_eq(&zero!(T))
    );
)

macro_rules! mat_is_symmetric(
    (Mat2) => (
        self.elem(0, 1).approx_eq(self.elem(1, 0)) &&
        self.elem(1, 0).approx_eq(self.elem(0, 1))
    );
    (Mat3) => (
        self.elem(0, 1).approx_eq(self.elem(1, 0)) &&
        self.elem(0, 2).approx_eq(self.elem(2, 0)) &&

        self.elem(1, 0).approx_eq(self.elem(0, 1)) &&
        self.elem(1, 2).approx_eq(self.elem(2, 1)) &&

        self.elem(2, 0).approx_eq(self.elem(0, 2)) &&
        self.elem(2, 1).approx_eq(self.elem(1, 2))
    );
    (Mat4) => (
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
    );
)

macro_rules! impl_mat_neg(
    ($Mat:ident) => (
        impl<T:Copy + Num> Neg<$Mat<T>> for $Mat<T> {
            #[inline]
            pub fn neg(&self) -> $Mat<T> {
                $Mat::from_slice(self.map(|&x| -x))
            }
        }
    )
)
