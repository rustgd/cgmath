// Copyright 2013 The CGMath Developers. For a full listing of the authors,
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

use std::num::{One, one, zero};

use traits::alg::*;
use types::vector::*;

#[deriving(Clone, Eq, Zero)] pub struct Mat2<S> { x: Vec2<S>, y: Vec2<S> }
#[deriving(Clone, Eq, Zero)] pub struct Mat3<S> { x: Vec3<S>, y: Vec3<S>, z: Vec3<S> }
#[deriving(Clone, Eq, Zero)] pub struct Mat4<S> { x: Vec4<S>, y: Vec4<S>, z: Vec4<S>, w: Vec4<S> }

// Constructors

impl<S: Clone + Field> Mat2<S> {
    #[inline]
    pub fn new(c0r0: S, c0r1: S,
               c1r0: S, c1r1: S) -> Mat2<S> {
        Mat2::from_cols(Vec2::new(c0r0, c0r1),
                        Vec2::new(c1r0, c1r1))
    }

    #[inline]
    pub fn from_cols(c0: Vec2<S>, c1: Vec2<S>) -> Mat2<S> {
        Mat2 { x: c0, y: c1 }
    }

    #[inline]
    pub fn from_value(value: S) -> Mat2<S> {
        Mat2::new(value.clone(), zero(),
                  zero(), value.clone())
    }

    #[inline]
    pub fn zero() -> Mat2<S> {
        Mat2::from_value(zero())
    }

    #[inline]
    pub fn ident() -> Mat2<S> {
        Mat2::from_value(one())
    }
}

impl<S: Clone + Field> Mat3<S> {
    #[inline]
    pub fn new(c0r0:S, c0r1:S, c0r2:S,
               c1r0:S, c1r1:S, c1r2:S,
               c2r0:S, c2r1:S, c2r2:S) -> Mat3<S> {
        Mat3::from_cols(Vec3::new(c0r0, c0r1, c0r2),
                        Vec3::new(c1r0, c1r1, c1r2),
                        Vec3::new(c2r0, c2r1, c2r2))
    }

    #[inline]
    pub fn from_cols(c0: Vec3<S>, c1: Vec3<S>, c2: Vec3<S>) -> Mat3<S> {
        Mat3 { x: c0, y: c1, z: c2 }
    }

    #[inline]
    pub fn from_value(value: S) -> Mat3<S> {
        Mat3::new(value.clone(), zero(), zero(),
                  zero(), value.clone(), zero(),
                  zero(), zero(), value.clone())
    }

    #[inline]
    pub fn zero() -> Mat3<S> {
        Mat3::from_value(zero())
    }

    #[inline]
    pub fn ident() -> Mat3<S> {
        Mat3::from_value(one())
    }
}

impl<S: Clone + Field> Mat4<S> {
    #[inline]
    pub fn new(c0r0: S, c0r1: S, c0r2: S, c0r3: S,
               c1r0: S, c1r1: S, c1r2: S, c1r3: S,
               c2r0: S, c2r1: S, c2r2: S, c2r3: S,
               c3r0: S, c3r1: S, c3r2: S, c3r3: S) -> Mat4<S>  {
        Mat4::from_cols(Vec4::new(c0r0, c0r1, c0r2, c0r3),
                        Vec4::new(c1r0, c1r1, c1r2, c1r3),
                        Vec4::new(c2r0, c2r1, c2r2, c2r3),
                        Vec4::new(c3r0, c3r1, c3r2, c3r3))
    }

    #[inline]
    pub fn from_cols(c0: Vec4<S>, c1: Vec4<S>, c2: Vec4<S>, c3: Vec4<S>) -> Mat4<S> {
        Mat4 { x: c0, y: c1, z: c2, w: c3 }
    }

    #[inline]
    pub fn from_value(value: S) -> Mat4<S> {
        Mat4::new(value.clone(), zero(), zero(), zero(),
                  zero(), value.clone(), zero(), zero(),
                  zero(), zero(), value.clone(), zero(),
                  zero(), zero(), zero(), value.clone())
    }

    #[inline]
    pub fn zero() -> Mat4<S> {
        Mat4::from_value(zero())
    }

    #[inline]
    pub fn ident() -> Mat4<S> {
        Mat4::from_value(one())
    }
}

// Trait impls

array!(impl<S> Mat2<S> -> [Vec2<S>, ..2])
array!(impl<S> Mat3<S> -> [Vec3<S>, ..3])
array!(impl<S> Mat4<S> -> [Vec4<S>, ..4])

impl<S: Clone + Field> ClonableArray<Vec2<S>, [Vec2<S>, ..2]> for Mat2<S>;
impl<S: Clone + Field> ClonableArray<Vec3<S>, [Vec3<S>, ..3]> for Mat3<S>;
impl<S: Clone + Field> ClonableArray<Vec4<S>, [Vec4<S>, ..4]> for Mat4<S>;

scalar_op!(impl Mat2<S> * S -> Mat2<S>)
scalar_op!(impl Mat3<S> * S -> Mat3<S>)
scalar_op!(impl Mat4<S> * S -> Mat4<S>)
scalar_op!(impl Mat2<S> / S -> Mat2<S>)
scalar_op!(impl Mat3<S> / S -> Mat3<S>)
scalar_op!(impl Mat4<S> / S -> Mat4<S>)
scalar_op!(impl Mat2<S> % S -> Mat2<S>)
scalar_op!(impl Mat3<S> % S -> Mat3<S>)
scalar_op!(impl Mat4<S> % S -> Mat4<S>)

impl<S: Field> ScalarMul<S> for Mat2<S>;
impl<S: Field> ScalarMul<S> for Mat3<S>;
impl<S: Field> ScalarMul<S> for Mat4<S>;

array_op!(impl<S> Mat2<S> + Mat2<S> -> Mat2<S>)
array_op!(impl<S> Mat3<S> + Mat3<S> -> Mat3<S>)
array_op!(impl<S> Mat4<S> + Mat4<S> -> Mat4<S>)
array_op!(impl<S> Mat2<S> - Mat2<S> -> Mat2<S>)
array_op!(impl<S> Mat3<S> - Mat3<S> -> Mat3<S>)
array_op!(impl<S> Mat4<S> - Mat4<S> -> Mat4<S>)
array_op!(impl<S> -Mat2<S> -> Mat2<S>)
array_op!(impl<S> -Mat3<S> -> Mat3<S>)
array_op!(impl<S> -Mat4<S> -> Mat4<S>)

impl<S: Field> Module<S> for Mat2<S>;
impl<S: Field> Module<S> for Mat3<S>;
impl<S: Field> Module<S> for Mat4<S>;

impl<S: Clone + Field> One for Mat2<S> {
    #[inline] fn one() -> Mat2<S> { Mat2::ident() }
}

impl<S: Clone + Field> One for Mat3<S> {
    #[inline] fn one() -> Mat3<S> { Mat3::ident() }
}

impl<S: Clone + Field> One for Mat4<S> {
    #[inline] fn one() -> Mat4<S> { Mat4::ident() }
}

impl<S: Clone + Field> Ring<S> for Mat2<S>;
impl<S: Clone + Field> Ring<S> for Mat3<S>;
impl<S: Clone + Field> Ring<S> for Mat4<S>;

impl<S: Clone + Field + ApproxEq<S>>
Matrix
<
    S,
    Vec2<S>, [S, ..2], [Vec2<S>, ..2],
    Vec2<S>, [S, ..2], [Vec2<S>, ..2],
    Mat2<S>
>
for Mat2<S>
{
    #[inline]
    fn r(&self, r: uint) -> Vec2<S> {
        Vec2::new(self.i(0).i(r).clone(),
                  self.i(1).i(r).clone())
    }

    fn transpose(&self) -> Mat2<S> {
        Mat2::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(),
                  self.cr(0, 1).clone(), self.cr(1, 1).clone())
    }
}

impl<S: Clone + Field + ApproxEq<S>>
Matrix
<
    S,
    Vec3<S>, [S, ..3], [Vec3<S>, ..3],
    Vec3<S>, [S, ..3], [Vec3<S>, ..3],
    Mat3<S>
>
for Mat3<S>
{
    #[inline]
    fn r(&self, r: uint) -> Vec3<S> {
        Vec3::new(self.i(0).i(r).clone(),
                  self.i(1).i(r).clone(),
                  self.i(2).i(r).clone())
    }

    fn transpose(&self) -> Mat3<S> {
        Mat3::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(), self.cr(2, 0).clone(),
                  self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(2, 1).clone(),
                  self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(2, 2).clone())
    }
}

impl<S: Clone + Field + ApproxEq<S>>
Matrix
<
    S,
    Vec4<S>, [S, ..4], [Vec4<S>, ..4],
    Vec4<S>, [S, ..4], [Vec4<S>, ..4],
    Mat4<S>
>
for Mat4<S>
{
    #[inline]
    fn r(&self, r: uint) -> Vec4<S> {
        Vec4::new(self.i(0).i(r).clone(),
                  self.i(1).i(r).clone(),
                  self.i(2).i(r).clone(),
                  self.i(2).i(r).clone())
    }

    fn transpose(&self) -> Mat4<S> {
        Mat4::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(), self.cr(2, 0).clone(), self.cr(3, 0).clone(),
                  self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(2, 1).clone(), self.cr(3, 1).clone(),
                  self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(2, 2).clone(), self.cr(3, 2).clone(),
                  self.cr(0, 3).clone(), self.cr(1, 3).clone(), self.cr(2, 3).clone(), self.cr(3, 3).clone())
    }
}

impl<S: Clone + Field + ApproxEq<S>>
SquareMatrix<S, Vec2<S>, [S, ..2], [Vec2<S>, ..2]>
for Mat2<S>
{
    #[inline]
    fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
    }

    #[inline]
    fn trace(&self) -> S {
        *self.cr(0, 0) + *self.cr(1, 1)
    }

    #[inline]
    fn determinant(&self) -> S {
        *self.cr(0, 0) * *self.cr(1, 1) - *self.cr(1, 0) * *self.cr(0, 1)
    }

    fn invert(&self) -> Option<Mat2<S>> {
        let det = self.determinant();
        if det.approx_eq(&zero()) {
            None
        } else {
            Some(Mat2::new( self.cr(1, 1) / det, -self.cr(0, 1) / det,
                           -self.cr(1, 0) / det,  self.cr(0, 0) / det))
        }
    }
}

impl<S: Clone + Field + ApproxEq<S>>
SquareMatrix<S, Vec3<S>, [S, ..3], [Vec3<S>, ..3]>
for Mat3<S>
{
    #[inline]
    fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
        self.swap_cr((0, 2), (2, 0));
        self.swap_cr((1, 2), (2, 1));
    }

    #[inline]
    fn trace(&self) -> S {
        *self.cr(0, 0) + *self.cr(1, 1) + *self.cr(2, 2)
    }

    fn determinant(&self) -> S {
        *self.cr(0, 0) * (*self.cr(1, 1) * *self.cr(2, 2) - *self.cr(2, 1) * *self.cr(1, 2)) -
        *self.cr(1, 0) * (*self.cr(0, 1) * *self.cr(2, 2) - *self.cr(2, 1) * *self.cr(0, 2)) +
        *self.cr(2, 0) * (*self.cr(0, 1) * *self.cr(1, 2) - *self.cr(1, 1) * *self.cr(0, 2))
    }

    fn invert(&self) -> Option<Mat3<S>> {
        let det = self.determinant();
        if det.approx_eq(&zero()) {
            None
        } else {
            Some(Mat3::from_cols(self.c(1).cross(self.c(2)) / det,
                                 self.c(2).cross(self.c(0)) / det,
                                 self.c(0).cross(self.c(1)) / det).transpose())
        }
    }
}

impl<S: Clone + Real + Field + ApproxEq<S>>
SquareMatrix<S, Vec4<S>, [S, ..4], [Vec4<S>, ..4]>
for Mat4<S>
{
    #[inline]
    fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
        self.swap_cr((0, 2), (2, 0));
        self.swap_cr((0, 3), (3, 0));
        self.swap_cr((1, 2), (2, 1));
        self.swap_cr((1, 3), (3, 1));
        self.swap_cr((2, 3), (3, 2));
    }

    #[inline]
    fn trace(&self) -> S {
        *self.cr(0, 0) + *self.cr(1, 1) + *self.cr(2, 2) + *self.cr(3, 3)
    }

    fn determinant(&self) -> S {
        let m0 = Mat3::new(self.cr(1, 1).clone(), self.cr(2, 1).clone(), self.cr(3, 1).clone(),
                           self.cr(1, 2).clone(), self.cr(2, 2).clone(), self.cr(3, 2).clone(),
                           self.cr(1, 3).clone(), self.cr(2, 3).clone(), self.cr(3, 3).clone());
        let m1 = Mat3::new(self.cr(0, 1).clone(), self.cr(2, 1).clone(), self.cr(3, 1).clone(),
                           self.cr(0, 2).clone(), self.cr(2, 2).clone(), self.cr(3, 2).clone(),
                           self.cr(0, 3).clone(), self.cr(2, 3).clone(), self.cr(3, 3).clone());
        let m2 = Mat3::new(self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(3, 1).clone(),
                           self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(3, 2).clone(),
                           self.cr(0, 3).clone(), self.cr(1, 3).clone(), self.cr(3, 3).clone());
        let m3 = Mat3::new(self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(2, 1).clone(),
                           self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(2, 2).clone(),
                           self.cr(0, 3).clone(), self.cr(1, 3).clone(), self.cr(2, 3).clone());

        self.cr(0, 0) * m0.determinant() -
        self.cr(1, 0) * m1.determinant() +
        self.cr(2, 0) * m2.determinant() -
        self.cr(3, 0) * m3.determinant()
    }

    fn invert(&self) -> Option<Mat4<S>> {
        if self.is_invertible() {
            // Gauss Jordan Elimination with partial pivoting
            // So take this matrix, A, augmented with the identity
            // and essentially reduce [A|I]

            let mut A = self.clone();
            let mut I = one::<Mat4<S>>();

            for j in range(0u, 4u) {
                // Find largest element in col j
                let mut i1 = j;
                for i in range(j + 1, 4) {
                    if A.cr(j, i).abs() > A.cr(j, i1).abs() {
                        i1 = i;
                    }
                }

                // Swap columns i1 and j in A and I to
                // put pivot on diagonal
                A.swap_c(i1, j);
                I.swap_c(i1, j);

                // Scale col j to have a unit diagonal
                *I.mut_c(j) = I.c(j) / *A.cr(j, j);
                *A.mut_c(j) = A.c(j) / *A.cr(j, j);

                // Eliminate off-diagonal elems in col j of A,
                // doing identical ops to I
                for i in range(0u, 4u) {
                    if i != j {
                        let ij_mul_aij = I.c(j) * *A.cr(i, j);
                        let aj_mul_aij = A.c(j) * *A.cr(i, j);
                        *I.mut_c(i) = I.c(i) - ij_mul_aij;
                        *A.mut_c(i) = A.c(i) - aj_mul_aij;
                    }
                }
            }
            Some(I)
        } else {
            None
        }
    }
}

