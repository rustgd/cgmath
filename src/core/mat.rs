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

use core::Dimensional;
use core::{Quat, ToQuat};
use core::{Vec2, Vec3, Vec4};

#[path = "../num_macros.rs"]
mod num_macros;
mod dim_macros;

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
            pub fn elem<'a>(&'a self, col: uint, row: uint) -> &'a T {
                self.index(col).index(row)
            }

            #[inline]
            pub fn elem_mut<'a>(&'a mut self, col: uint, row: uint) -> &'a mut T {
                self.index_mut(col).index_mut(row)
            }
        }
    )
)

macro_rules! impl_mat_swap(
    ($Mat:ident, $Vec:ident) => (
        impl<T:Clone> $Mat<T> {
            #[inline]
            pub fn swap_cols(&mut self, a: uint, b: uint) {
                let tmp = self.col(a).clone();
                *self.col_mut(a) = self.col(b).clone();
                *self.col_mut(b) = tmp;
            }

            #[inline]
            pub fn swap_elem(&mut self, (col_a, row_a): (uint, uint), (col_b, row_b): (uint, uint)) {
                let tmp = self.elem(col_a, row_a).clone();
                *self.elem_mut(col_a, row_a) = self.elem(col_b, row_b).clone();
                *self.elem_mut(col_b, row_b) = tmp;
            }
        }
    )
)

impl<T:Clone> Mat2<T> {
    #[inline] pub fn transpose(&self) -> Mat2<T> {
        Mat2::new(self.elem(0, 0).clone(), self.elem(1, 0).clone(),
                  self.elem(0, 1).clone(), self.elem(1, 1).clone())
    }

    #[inline] pub fn transpose_self(&mut self) {
        self.swap_elem((0, 1), (1, 0));
    }
}

impl<T:Clone> Mat3<T> {
    #[inline] pub fn transpose(&self) -> Mat3<T> {
        Mat3::new(self.elem(0, 0).clone(), self.elem(1, 0).clone(), self.elem(2, 0).clone(),
                  self.elem(0, 1).clone(), self.elem(1, 1).clone(), self.elem(2, 1).clone(),
                  self.elem(0, 2).clone(), self.elem(1, 2).clone(), self.elem(2, 2).clone())
    }

    #[inline] pub fn transpose_self(&mut self) {
        self.swap_elem((0, 1), (1, 0));
        self.swap_elem((0, 2), (2, 0));
        self.swap_elem((1, 2), (2, 1));
    }
}

impl<T:Clone> Mat4<T> {
    #[inline] pub fn transpose(&self) -> Mat4<T> {
        Mat4::new(self.elem(0, 0).clone(), self.elem(1, 0).clone(), self.elem(2, 0).clone(), self.elem(3, 0).clone(),
                  self.elem(0, 1).clone(), self.elem(1, 1).clone(), self.elem(2, 1).clone(), self.elem(3, 1).clone(),
                  self.elem(0, 2).clone(), self.elem(1, 2).clone(), self.elem(2, 2).clone(), self.elem(3, 2).clone(),
                  self.elem(0, 3).clone(), self.elem(1, 3).clone(), self.elem(2, 3).clone(), self.elem(3, 3).clone())
    }

    #[inline] pub fn transpose_self(&mut self) {
        self.swap_elem((0, 1), (1, 0));
        self.swap_elem((0, 2), (2, 0));
        self.swap_elem((0, 3), (3, 0));
        self.swap_elem((1, 2), (2, 1));
        self.swap_elem((1, 3), (3, 1));
        self.swap_elem((2, 3), (3, 2));
    }
}

#[deriving(Clone, Eq)]
pub struct Mat2<T> {
    x: Vec2<T>,
    y: Vec2<T>,
}

// GLSL-style type aliases
pub type mat2  = Mat2<f32>;
pub type dmat2 = Mat2<f64>;

// Rust-style type aliases
pub type Mat2f   = Mat2<float>;
pub type Mat2f32 = Mat2<f32>;
pub type Mat2f64 = Mat2<f64>;

impl_dimensional!(Mat2, Vec2<T>, 2)
impl_mat!(Mat2, Vec2)
impl_mat_swap!(Mat2, Vec2)

pub trait ToMat2<T> {
    pub fn to_mat2(&self) -> Mat2<T>;
}

impl<T> Mat2<T> {
    #[inline]
    pub fn new(c0r0: T, c0r1: T,
               c1r0: T, c1r1: T) -> Mat2<T> {
        Mat2::from_cols(Vec2::new(c0r0, c0r1),
                        Vec2::new(c1r0, c1r1))
    }

    #[inline]
    pub fn from_cols(c0: Vec2<T>,
                     c1: Vec2<T>) -> Mat2<T> {
        Mat2 { x: c0, y: c1 }
    }
}

impl<T:Clone> Mat2<T> {
    #[inline]
    pub fn row(&self, i: uint) -> Vec2<T> {
        Vec2::new(self.col(0).index(i).clone(),
                  self.col(1).index(i).clone())
    }

    #[inline]
    pub fn swap_rows(&mut self, a: uint, b: uint) {
        self.col_mut(0).swap(a, b);
        self.col_mut(1).swap(a, b);
    }
}

impl<T:Clone + Num> ToMat3<T> for Mat2<T> {
    #[inline]
    pub fn to_mat3(&self) -> Mat3<T> {
        Mat3::new(self.elem(0, 0).clone(), self.elem(0, 1).clone(), zero!(T),
                  self.elem(1, 0).clone(), self.elem(1, 1).clone(), zero!(T),
                  zero!(T), zero!(T), one!(T))
    }
}

impl<T:Clone + Num> ToMat4<T> for Mat2<T> {
    #[inline]
    pub fn to_mat4(&self) -> Mat4<T> {
        Mat4::new(self.elem(0, 0).clone(), self.elem(0, 1).clone(), zero!(T), zero!(T),
                  self.elem(1, 0).clone(), self.elem(1, 1).clone(), zero!(T), zero!(T),
                  zero!(T), zero!(T), one!(T), zero!(T),
                  zero!(T), zero!(T), zero!(T), one!(T))
    }
}

impl<T:Clone + Num> Mat2<T> {
    #[inline]
    pub fn from_value(value: T) -> Mat2<T> {
        Mat2::new(value.clone(), zero!(T),
                  zero!(T), value.clone())
    }

    #[inline]
    pub fn identity() -> Mat2<T> { Mat2::from_value(one!(T)) }

    #[inline]
    pub fn zero() -> Mat2<T> { Mat2::from_value(zero!(T)) }

    #[inline]
    pub fn mul_t(&self, value: T) -> Mat2<T> {
        Mat2::from_cols(self.col(0).mul_t(value.clone()),
                        self.col(1).mul_t(value.clone()))
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
        Mat2::new(self.row(0).dot(other.col(0)),
                  self.row(1).dot(other.col(0)),

                  self.row(0).dot(other.col(1)),
                  self.row(1).dot(other.col(1)))
    }

    #[inline]
    pub fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(value.clone());
        self.col_mut(1).mul_self_t(value.clone());
    }

    #[inline]
    pub fn add_self_m(&mut self, other: &Mat2<T>) {
        self.col_mut(0).add_self_v(other.col(0));
        self.col_mut(1).add_self_v(other.col(1));
    }

    #[inline]
    pub fn sub_self_m(&mut self, other: &Mat2<T>) {
        self.col_mut(0).sub_self_v(other.col(0));
        self.col_mut(1).sub_self_v(other.col(1));
    }

    pub fn dot(&self, other: &Mat2<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    pub fn determinant(&self) -> T {
       *self.elem(0, 0) * *self.elem(1, 1) - *self.elem(1, 0) * *self.elem(0, 1)
    }

    pub fn trace(&self) -> T {
        *self.elem(0, 0) + *self.elem(1, 1)
    }

    #[inline]
    pub fn to_identity(&mut self) {
        *self = Mat2::identity();
    }

    #[inline]
    pub fn to_zero(&mut self) {
        *self = Mat2::zero();
    }
}

impl<T:Clone + Num> Neg<Mat2<T>> for Mat2<T> {
    #[inline]
    pub fn neg(&self) -> Mat2<T> {
        Mat2::from_cols(-*self.col(0),
                        -*self.col(1))
    }
}

impl<T:Clone + Real> Mat2<T> {
    #[inline]
    pub fn from_angle(radians: T) -> Mat2<T> {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat2::new(cos_theta.clone(),  -sin_theta.clone(),
                  sin_theta.clone(),  cos_theta.clone())
    }
}

impl<T:Clone + Real + ApproxEq<T>> Mat2<T> {
    #[inline]
    pub fn inverse(&self) -> Option<Mat2<T>> {
        let d = self.determinant();
        if d.approx_eq(&zero!(T)) {
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
        self.elem(0, 1).approx_eq(&zero!(T)) &&
        self.elem(1, 0).approx_eq(&zero!(T))
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
        !self.determinant().approx_eq(&zero!(T))
    }
}

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for Mat2<T> {
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

#[cfg(test)]
mod mat2_tests{
    use core::mat::*;
    use core::vec::*;

    static A: Mat2<float> = Mat2 { x: Vec2 { x: 1.0, y: 3.0 },
                                   y: Vec2 { x: 2.0, y: 4.0 } };
    static B: Mat2<float> = Mat2 { x: Vec2 { x: 2.0, y: 4.0 },
                                   y: Vec2 { x: 3.0, y: 5.0 } };
    static C: Mat2<float> = Mat2 { x: Vec2 { x: 2.0, y: 1.0 },
                                   y: Vec2 { x: 1.0, y: 2.0 } };

    static V: Vec2<float> = Vec2 { x: 1.0, y: 2.0 };
    static F: float = 0.5;

    #[test]
    fn test_swap_cols() {
        let mut mut_a = A;
        mut_a.swap_cols(0, 1);
        assert_eq!(mut_a.col(0), A.col(1));
        assert_eq!(mut_a.col(1), A.col(0));
    }

    #[test]
    fn test_swap_rows() {
        let mut mut_a = A;
        mut_a.swap_rows(0, 1);
        assert_eq!(mut_a.row(0), A.row(1));
        assert_eq!(mut_a.row(1), A.row(0));
    }

    #[test]
    fn test_identity() {
        assert_eq!(Mat2::identity::<float>(),
                   Mat2::new::<float>(1.0, 0.0,
                                      0.0, 1.0));
        let mut mut_a = A;
        mut_a.to_identity();
        assert!(mut_a.is_identity());
    }

    #[test]
    fn test_zero() {
        assert_eq!(Mat2::zero::<float>(),
                   Mat2::new::<float>(0.0, 0.0,
                                      0.0, 0.0));
        let mut mut_a = A;
        mut_a.to_zero();
        assert_eq!(mut_a, Mat2::zero::<float>());
    }
    #[test]
    fn test_determinant() {
        assert_eq!(A.determinant(), -2.0);
    }

    #[test]
    fn test_trace() {
        assert_eq!(A.trace(), 5.0);
    }

    #[test]
    fn test_neg() {
        assert_eq!(A.neg(),
                   Mat2::new::<float>(-1.0, -3.0,
                                      -2.0, -4.0));
        assert_eq!(-A, A.neg());
    }

    #[test]
    fn test_mul_t() {
        assert_eq!(A.mul_t(F),
                   Mat2::new::<float>(0.5, 1.5,
                                      1.0, 2.0));
        let mut mut_a = A;
        mut_a.mul_self_t(F);
        assert_eq!(mut_a, A.mul_t(F));
    }

    #[test]
    fn test_mul_v() {
        assert_eq!(A.mul_v(&V), Vec2::new::<float>(5.0, 11.0));
    }

    #[test]
    fn test_add_m() {
        assert_eq!(A.add_m(&B),
                   Mat2::new::<float>(3.0, 7.0,
                                      5.0, 9.0));
        let mut mut_a = A;
        mut_a.add_self_m(&B);
        assert_eq!(mut_a, A.add_m(&B));
    }

    #[test]
    fn test_sub_m() {
        assert_eq!(A.sub_m(&B),
                   Mat2::new::<float>(-1.0, -1.0,
                                      -1.0, -1.0));
        let mut mut_a = A;
        mut_a.sub_self_m(&B);
        assert_eq!(mut_a, A.sub_m(&B));
    }

    #[test]
    fn test_mul_m() {
        assert_eq!(A.mul_m(&B),
                   Mat2::new::<float>(10.0, 22.0,
                                      13.0, 29.0));
    }

    #[test]
    fn test_dot() {
        assert_eq!(A.dot(&B), 40.0);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(A.transpose(),
                   Mat2::new::<float>(1.0, 2.0,
                                      3.0, 4.0));
        let mut mut_a = A;
        mut_a.transpose_self();
        assert_eq!(mut_a, A.transpose());
    }

    #[test]
    fn test_inverse() {
        assert!(Mat2::identity::<float>().inverse().unwrap().is_identity());

        assert_eq!(A.inverse().unwrap(),
                   Mat2::new::<float>(-2.0,  1.5,
                                      1.0, -0.5));
        assert!(Mat2::new::<float>(0.0, 2.0,
                                   0.0, 5.0).inverse().is_none());
        let mut mut_a = A;
        mut_a.invert_self();
        assert_eq!(mut_a, A.inverse().unwrap());
    }

    #[test]
    fn test_predicates() {
        assert!(Mat2::identity::<float>().is_identity());
        assert!(Mat2::identity::<float>().is_symmetric());
        assert!(Mat2::identity::<float>().is_diagonal());
        assert!(!Mat2::identity::<float>().is_rotated());
        assert!(Mat2::identity::<float>().is_invertible());

        assert!(!A.is_identity());
        assert!(!A.is_symmetric());
        assert!(!A.is_diagonal());
        assert!(A.is_rotated());
        assert!(A.is_invertible());

        assert!(!C.is_identity());
        assert!(C.is_symmetric());
        assert!(!C.is_diagonal());
        assert!(C.is_rotated());
        assert!(C.is_invertible());

        assert!(Mat2::from_value::<float>(6.0).is_diagonal());
    }

    #[test]
    fn test_to_mat3() {
        assert_eq!(A.to_mat3(),
                   Mat3::new::<float>(1.0, 3.0, 0.0,
                                      2.0, 4.0, 0.0,
                                      0.0, 0.0, 1.0));
    }

    #[test]
    fn test_to_mat4() {
        assert_eq!(A.to_mat4(),
                   Mat4::new::<float>(1.0, 3.0, 0.0, 0.0,
                                      2.0, 4.0, 0.0, 0.0,
                                      0.0, 0.0, 1.0, 0.0,
                                      0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn test_approx() {
        assert!(!Mat2::new::<float>(0.000001, 0.000001,
                                    0.000001, 0.000001).approx_eq(&Mat2::zero::<float>()));
        assert!(Mat2::new::<float>(0.0000001, 0.0000001,
                                   0.0000001, 0.0000001).approx_eq(&Mat2::zero::<float>()));
    }
}

#[deriving(Clone, Eq)]
pub struct Mat3<T> {
    x: Vec3<T>,
    y: Vec3<T>,
    z: Vec3<T>,
}

// GLSL-style type aliases
pub type mat3  = Mat3<f32>;
pub type dmat3 = Mat3<f64>;

// Rust-style type aliases
pub type Mat3f   = Mat3<float>;
pub type Mat3f32 = Mat3<f32>;
pub type Mat3f64 = Mat3<f64>;

impl_dimensional!(Mat3, Vec3<T>, 3)
impl_mat!(Mat3, Vec3)
impl_mat_swap!(Mat3, Vec3)

pub trait ToMat3<T> {
    pub fn to_mat3(&self) -> Mat3<T>;
}

impl<T> Mat3<T> {
    #[inline]
    pub fn new(c0r0:T, c0r1:T, c0r2:T,
               c1r0:T, c1r1:T, c1r2:T,
               c2r0:T, c2r1:T, c2r2:T) -> Mat3<T> {
        Mat3::from_cols(Vec3::new(c0r0, c0r1, c0r2),
                        Vec3::new(c1r0, c1r1, c1r2),
                        Vec3::new(c2r0, c2r1, c2r2))
    }

    #[inline]
    pub fn from_cols(c0: Vec3<T>,
                     c1: Vec3<T>,
                     c2: Vec3<T>) -> Mat3<T> {
        Mat3 { x: c0, y: c1, z: c2 }
    }
}

impl<T:Clone> Mat3<T> {
    #[inline]
    pub fn row(&self, i: uint) -> Vec3<T> {
        Vec3::new(self.col(0).index(i).clone(),
                  self.col(1).index(i).clone(),
                  self.col(2).index(i).clone())
    }

    #[inline]
    pub fn swap_rows(&mut self, a: uint, b: uint) {
        self.col_mut(0).swap(a, b);
        self.col_mut(1).swap(a, b);
        self.col_mut(2).swap(a, b);
    }
}

impl<T:Clone + Num> ToMat4<T> for Mat3<T> {
    #[inline]
    pub fn to_mat4(&self) -> Mat4<T> {
        Mat4::new(self.elem(0, 0).clone(), self.elem(0, 1).clone(), self.elem(0, 2).clone(), zero!(T),
                  self.elem(1, 0).clone(), self.elem(1, 1).clone(), self.elem(1, 2).clone(), zero!(T),
                  self.elem(2, 0).clone(), self.elem(2, 1).clone(), self.elem(2, 2).clone(), zero!(T),
                  zero!(T), zero!(T), zero!(T), one!(T))
    }
}

impl<T:Clone + Num> Mat3<T> {
    #[inline]
    pub fn from_value(value: T) -> Mat3<T> {
        Mat3::new(value.clone(), zero!(T), zero!(T),
                  zero!(T), value.clone(), zero!(T),
                  zero!(T), zero!(T), value.clone())
    }

    #[inline]
    pub fn identity() -> Mat3<T> { Mat3::from_value(one!(T)) }

    #[inline]
    pub fn zero() -> Mat3<T> { Mat3::from_value(zero!(T)) }

    #[inline]
    pub fn mul_t(&self, value: T) -> Mat3<T> {
        Mat3::from_cols(self.col(0).mul_t(value.clone()),
                        self.col(1).mul_t(value.clone()),
                        self.col(2).mul_t(value.clone()))
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
        self.col_mut(0).mul_self_t(value.clone());
        self.col_mut(1).mul_self_t(value.clone());
        self.col_mut(2).mul_self_t(value.clone());
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
        *self.elem(0, 0) * (*self.elem(1, 1) * *self.elem(2, 2) - *self.elem(2, 1) * *self.elem(1, 2))
      - *self.elem(1, 0) * (*self.elem(0, 1) * *self.elem(2, 2) - *self.elem(2, 1) * *self.elem(0, 2))
      + *self.elem(2, 0) * (*self.elem(0, 1) * *self.elem(1, 2) - *self.elem(1, 1) * *self.elem(0, 2))
    }

    pub fn trace(&self) -> T {
        (*self.elem(0, 0)) + (*self.elem(1, 1)) + (*self.elem(2, 2))
    }

    #[inline]
    pub fn to_identity(&mut self) {
        *self = Mat3::identity();
    }

    #[inline]
    pub fn to_zero(&mut self) {
        *self = Mat3::zero();
    }
}

impl<T:Clone + Num> Neg<Mat3<T>> for Mat3<T> {
    #[inline]
    pub fn neg(&self) -> Mat3<T> {
        Mat3::from_cols(-*self.col(0),
                        -*self.col(1),
                        -*self.col(2))
    }
}

impl<T:Clone + Real> Mat3<T> {
    /// Construct a matrix from an angular rotation around the `x` axis
    pub fn from_angle_x(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat3::new(one!(T), zero!(T), zero!(T),
                  zero!(T), cos_theta.clone(), sin_theta.clone(),
                  zero!(T), -sin_theta.clone(), cos_theta.clone())
    }

    /// Construct a matrix from an angular rotation around the `y` axis
    pub fn from_angle_y(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat3::new(cos_theta.clone(), zero!(T), -sin_theta.clone(),
                  zero!(T), one!(T), zero!(T),
                  sin_theta.clone(), zero!(T), cos_theta.clone())
    }

    /// Construct a matrix from an angular rotation around the `z` axis
    pub fn from_angle_z(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat3::new(cos_theta.clone(), sin_theta.clone(), zero!(T),
                  -sin_theta.clone(), cos_theta.clone(), zero!(T),
                  zero!(T), zero!(T), one!(T))
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
        let _1_c = one!(T) - c;

        let x = axis.x.clone();
        let y = axis.y.clone();
        let z = axis.z.clone();

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
}

impl<T:Clone + Real> ToQuat<T> for Mat3<T> {
    /// Convert the matrix to a quaternion
    pub fn to_quat(&self) -> Quat<T> {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf

        let mut s;
        let w; let x; let y; let z;
        let trace = self.trace();

        // FIXME: We don't have any numeric conversions in std yet :P
        let half = one!(T) / two!(T);

        cond! (
            (trace >= zero!(T)) {
                s = (one!(T) + trace).sqrt();
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

impl<T:Clone + Real + ApproxEq<T>> Mat3<T> {
    #[inline]
    pub fn inverse(&self) -> Option<Mat3<T>> {
        let d = self.determinant();
        if d.approx_eq(&zero!(T)) {
            None
        } else {
            Some(Mat3::from_cols(self.col(1).cross(self.col(2)).div_t(d.clone()),
                                 self.col(2).cross(self.col(0)).div_t(d.clone()),
                                 self.col(0).cross(self.col(1)).div_t(d.clone())).transpose())
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
        self.elem(0, 1).approx_eq(&zero!(T)) &&
        self.elem(0, 2).approx_eq(&zero!(T)) &&

        self.elem(1, 0).approx_eq(&zero!(T)) &&
        self.elem(1, 2).approx_eq(&zero!(T)) &&

        self.elem(2, 0).approx_eq(&zero!(T)) &&
        self.elem(2, 1).approx_eq(&zero!(T))
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
        !self.determinant().approx_eq(&zero!(T))
    }
}

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for Mat3<T> {
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

#[cfg(test)]
mod mat3_tests{
    use core::mat::*;
    use core::vec::*;

    static A: Mat3<float> = Mat3 { x: Vec3 { x: 1.0, y: 4.0, z:  7.0 },
                                   y: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                                   z: Vec3 { x: 3.0, y: 6.0, z:  9.0 } };
    static B: Mat3<float> = Mat3 { x: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                                   y: Vec3 { x: 3.0, y: 6.0, z:  9.0 },
                                   z: Vec3 { x: 4.0, y: 7.0, z: 10.0 } };
    static C: Mat3<float> = Mat3 { x: Vec3 { x: 2.0, y: 4.0, z:  6.0 },
                                   y: Vec3 { x: 0.0, y: 2.0, z:  4.0 },
                                   z: Vec3 { x: 0.0, y: 0.0, z:  1.0 } };
    static D: Mat3<float> = Mat3 { x: Vec3 { x: 3.0, y: 2.0, z:  1.0 },
                                   y: Vec3 { x: 2.0, y: 3.0, z:  2.0 },
                                   z: Vec3 { x: 1.0, y: 2.0, z:  3.0 } };

    static V: Vec3<float> = Vec3 { x: 1.0, y: 2.0, z:  3.0 };
    static F: float = 0.5;

    #[test]
    fn test_swap_cols() {
        let mut mut_a0 = A;
        mut_a0.swap_cols(0, 2);
        assert_eq!(mut_a0.col(0), A.col(2));
        assert_eq!(mut_a0.col(2), A.col(0));

        let mut mut_a1 = A;
        mut_a1.swap_cols(1, 2);
        assert_eq!(mut_a1.col(1), A.col(2));
        assert_eq!(mut_a1.col(2), A.col(1));
    }

    #[test]
    fn test_swap_rows() {
        let mut mut_a0 = A;
        mut_a0.swap_rows(0, 2);
        assert_eq!(mut_a0.row(0), A.row(2));
        assert_eq!(mut_a0.row(2), A.row(0));

        let mut mut_a1 = A;
        mut_a1.swap_rows(1, 2);
        assert_eq!(mut_a1.row(1), A.row(2));
        assert_eq!(mut_a1.row(2), A.row(1));
    }

    #[test]
    fn test_identity() {
        assert_eq!(Mat3::identity::<float>(),
                   Mat3::new::<float>(1.0, 0.0, 0.0,
                                      0.0, 1.0, 0.0,
                                      0.0, 0.0, 1.0));
        let mut mut_a = A;
        mut_a.to_identity();
        assert!(mut_a.is_identity());
    }

    #[test]
    fn test_zero() {
        assert_eq!(Mat3::zero::<float>(),
                   Mat3::new::<float>(0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0));
        let mut mut_a = A;
        mut_a.to_zero();
        assert_eq!(mut_a, Mat3::zero::<float>());
    }

    #[test]
    fn test_determinant() {
        // assert_eq!(A.determinant(), 0.0);
        // TODO
    }

    #[test]
    fn test_trace() {
        assert_eq!(A.trace(), 15.0);
    }

    #[test]
    fn test_neg() {
        assert_eq!(A.neg(),
                   Mat3::new::<float>(-1.0, -4.0, -7.0,
                                      -2.0, -5.0, -8.0,
                                      -3.0, -6.0, -9.0));
        assert_eq!(-A, A.neg());
    }

    #[test]
    fn test_mul_t() {
        assert_eq!(A.mul_t(F),
                   Mat3::new::<float>(0.5, 2.0, 3.5,
                                      1.0, 2.5, 4.0,
                                      1.5, 3.0, 4.5));
        let mut mut_a = A;
        mut_a.mul_self_t(F);
        assert_eq!(mut_a, A.mul_t(F));
    }

    #[test]
    fn test_mul_v() {
        assert_eq!(A.mul_v(&V),
                   Vec3::new::<float>(14.0, 32.0, 50.0));
    }

    #[test]
    fn test_add_m() {
        assert_eq!(A.add_m(&B),
                   Mat3::new::<float>(3.0,  9.0, 15.0,
                                      5.0, 11.0, 17.0,
                                      7.0, 13.0, 19.0));
        let mut mut_a = A;
        mut_a.add_self_m(&B);
        assert_eq!(mut_a, A.add_m(&B));
    }

    #[test]
    fn test_sub_m() {
        assert_eq!(A.sub_m(&B),
                   Mat3::new::<float>(-1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0));
        let mut mut_a = A;
        mut_a.sub_self_m(&B);
        assert_eq!(mut_a, A.sub_m(&B));
    }

    #[test]
    fn test_mul_m() {
        assert_eq!(A.mul_m(&B),
                   Mat3::new::<float>(36.0,  81.0, 126.0,
                                      42.0,  96.0, 150.0,
                                      48.0, 111.0, 174.0));
    }

    #[test]
    fn test_dot() {
        assert_eq!(A.dot(&B), 330.0);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(A.transpose(),
                   Mat3::new::<float>(1.0, 2.0, 3.0,
                                      4.0, 5.0, 6.0,
                                      7.0, 8.0, 9.0));
        let mut mut_a = A;
        mut_a.transpose_self();
        assert_eq!(mut_a, A.transpose());
    }

    #[test]
    fn test_inverse() {
        assert!(Mat3::identity::<float>().inverse().unwrap().is_identity());

        assert_eq!(A.inverse(), None);

        assert_eq!(C.inverse().unwrap(),
                   Mat3::new::<float>(0.5, -1.0,  1.0,
                                      0.0,  0.5, -2.0,
                                      0.0,  0.0,  1.0));
        let mut mut_c = C;
        mut_c.invert_self();
        assert_eq!(mut_c, C.inverse().unwrap());
    }

    #[test]
    fn test_predicates() {
        assert!(Mat3::identity::<float>().is_identity());
        assert!(Mat3::identity::<float>().is_symmetric());
        assert!(Mat3::identity::<float>().is_diagonal());
        assert!(!Mat3::identity::<float>().is_rotated());
        assert!(Mat3::identity::<float>().is_invertible());

        assert!(!A.is_identity());
        assert!(!A.is_symmetric());
        assert!(!A.is_diagonal());
        assert!(A.is_rotated());
        assert!(!A.is_invertible());

        assert!(!D.is_identity());
        assert!(D.is_symmetric());
        assert!(!D.is_diagonal());
        assert!(D.is_rotated());
        assert!(D.is_invertible());

        assert!(Mat3::from_value::<float>(6.0).is_diagonal());
    }

    #[test]
    fn test_to_mat4() {
        assert_eq!(A.to_mat4(),
                   Mat4::new::<float>(1.0, 4.0, 7.0, 0.0,
                                      2.0, 5.0, 8.0, 0.0,
                                      3.0, 6.0, 9.0, 0.0,
                                      0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn test_approx() {
        assert!(!Mat3::new::<float>(0.000001, 0.000001, 0.000001,
                                    0.000001, 0.000001, 0.000001,
                                    0.000001, 0.000001, 0.000001)
                .approx_eq(&Mat3::zero::<float>()));
        assert!(Mat3::new::<float>(0.0000001, 0.0000001, 0.0000001,
                                   0.0000001, 0.0000001, 0.0000001,
                                   0.0000001, 0.0000001, 0.0000001)
                .approx_eq(&Mat3::zero::<float>()));
    }
}

#[deriving(Clone, Eq)]
pub struct Mat4<T> {
    x: Vec4<T>,
    y: Vec4<T>,
    z: Vec4<T>,
    w: Vec4<T>,
}

// GLSL-style type aliases
pub type mat4  = Mat4<f32>;
pub type dmat4 = Mat4<f64>;

// Rust-style type aliases
pub type Mat4f   = Mat4<float>;
pub type Mat4f32 = Mat4<f32>;
pub type Mat4f64 = Mat4<f64>;

impl_dimensional!(Mat4, Vec4<T>, 4)
impl_mat!(Mat4, Vec4)
impl_mat_swap!(Mat4, Vec4)

pub trait ToMat4<T> {
    pub fn to_mat4(&self) -> Mat4<T>;
}

impl<T> Mat4<T> {
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

    #[inline]
    pub fn from_cols(c0: Vec4<T>,
                     c1: Vec4<T>,
                     c2: Vec4<T>,
                     c3: Vec4<T>) -> Mat4<T> {
        Mat4 { x: c0, y: c1, z: c2, w: c3 }
    }
}

impl<T:Clone> Mat4<T> {
    #[inline]
    pub fn row(&self, i: uint) -> Vec4<T> {
        Vec4::new(self.col(0).index(i).clone(),
                  self.col(1).index(i).clone(),
                  self.col(2).index(i).clone(),
                  self.col(3).index(i).clone())
    }

    #[inline]
    pub fn swap_rows(&mut self, a: uint, b: uint) {
        self.col_mut(0).swap(a, b);
        self.col_mut(1).swap(a, b);
        self.col_mut(2).swap(a, b);
        self.col_mut(3).swap(a, b);
    }
}

impl<T:Clone + Num> Mat4<T> {
    #[inline]
    pub fn from_value(value: T) -> Mat4<T> {
        Mat4::new(value.clone(), zero!(T), zero!(T), zero!(T),
                  zero!(T), value.clone(), zero!(T), zero!(T),
                  zero!(T), zero!(T), value.clone(), zero!(T),
                  zero!(T), zero!(T), zero!(T), value.clone())
    }

    #[inline]
    pub fn identity() -> Mat4<T> { Mat4::from_value(one!(T)) }

    #[inline]
    pub fn zero() -> Mat4<T> { Mat4::from_value(zero!(T)) }

    #[inline]
    pub fn mul_t(&self, value: T) -> Mat4<T> {
        Mat4::from_cols(self.col(0).mul_t(value.clone()),
                        self.col(1).mul_t(value.clone()),
                        self.col(2).mul_t(value.clone()),
                        self.col(3).mul_t(value.clone()))
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
        self.col_mut(0).mul_self_t(value.clone());
        self.col_mut(1).mul_self_t(value.clone());
        self.col_mut(2).mul_self_t(value.clone());
        self.col_mut(3).mul_self_t(value.clone());
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
        let m0 = Mat3::new(self.elem(1, 1).clone(), self.elem(2, 1).clone(), self.elem(3, 1).clone(),
                           self.elem(1, 2).clone(), self.elem(2, 2).clone(), self.elem(3, 2).clone(),
                           self.elem(1, 3).clone(), self.elem(2, 3).clone(), self.elem(3, 3).clone());
        let m1 = Mat3::new(self.elem(0, 1).clone(), self.elem(2, 1).clone(), self.elem(3, 1).clone(),
                           self.elem(0, 2).clone(), self.elem(2, 2).clone(), self.elem(3, 2).clone(),
                           self.elem(0, 3).clone(), self.elem(2, 3).clone(), self.elem(3, 3).clone());
        let m2 = Mat3::new(self.elem(0, 1).clone(), self.elem(1, 1).clone(), self.elem(3, 1).clone(),
                           self.elem(0, 2).clone(), self.elem(1, 2).clone(), self.elem(3, 2).clone(),
                           self.elem(0, 3).clone(), self.elem(1, 3).clone(), self.elem(3, 3).clone());
        let m3 = Mat3::new(self.elem(0, 1).clone(), self.elem(1, 1).clone(), self.elem(2, 1).clone(),
                           self.elem(0, 2).clone(), self.elem(1, 2).clone(), self.elem(2, 2).clone(),
                           self.elem(0, 3).clone(), self.elem(1, 3).clone(), self.elem(2, 3).clone());

        self.elem(0, 0) * m0.determinant() -
        self.elem(1, 0) * m1.determinant() +
        self.elem(2, 0) * m2.determinant() -
        self.elem(3, 0) * m3.determinant()
    }

    pub fn trace(&self) -> T {
        *self.elem(0, 0) + *self.elem(1, 1) + *self.elem(2, 2) + *self.elem(3, 3)
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

impl<T:Clone + Num> Neg<Mat4<T>> for Mat4<T> {
    #[inline]
    pub fn neg(&self) -> Mat4<T> {
        Mat4::from_cols(-*self.col(0),
                        -*self.col(1),
                        -*self.col(2),
                        -*self.col(3))
    }
}

impl<T:Clone + Real + ApproxEq<T>> Mat4<T> {
    #[inline]
    pub fn inverse(&self) -> Option<Mat4<T>> {
        use std::uint;

        if self.is_invertible() {
            // Gauss Jordan Elimination with partial pivoting
            // So take this matrix, A, augmented with the identity
            // and essentially reduce [A|I]

            let mut A = self.clone();
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
                let ajj = A.elem(j, j).clone();
                I.col_mut(j).div_self_t(ajj.clone());
                A.col_mut(j).div_self_t(ajj.clone());

                // Eliminate off-diagonal elems in col j of A,
                // doing identical ops to I
                for uint::range(0, 4) |i| {
                    if i != j {
                        let ij_mul_aij = I.col(j).mul_t(A.elem(i, j).clone());
                        let aj_mul_aij = A.col(j).mul_t(A.elem(i, j).clone());
                        I.col_mut(i).sub_self_v(&ij_mul_aij);
                        A.col_mut(i).sub_self_v(&aj_mul_aij);
                    }
                }
            }
            Some(I)
        } else {
            None
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
        !self.determinant().approx_eq(&zero!(T))
    }
}

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for Mat4<T> {
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
mod mat4_tests {
    use core::mat::*;
    use core::vec::*;

    static A: Mat4<float> = Mat4 { x: Vec4 { x: 1.0, y: 5.0, z:  9.0, w: 13.0 },
                                   y: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                                   z: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                                   w: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 } };
    static B: Mat4<float> = Mat4 { x: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                                   y: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                                   z: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 },
                                   w: Vec4 { x: 5.0, y: 9.0, z: 13.0, w: 17.0 } };
    static C: Mat4<float> = Mat4 { x: Vec4 { x: 3.0, y: 2.0, z:  1.0, w:  1.0 },
                                   y: Vec4 { x: 2.0, y: 3.0, z:  2.0, w:  2.0 },
                                   z: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  3.0 },
                                   w: Vec4 { x: 0.0, y: 1.0, z:  1.0, w:  0.0 } };
    static D: Mat4<float> = Mat4 { x: Vec4 { x: 4.0, y: 3.0, z:  2.0, w:  1.0 },
                                   y: Vec4 { x: 3.0, y: 4.0, z:  3.0, w:  2.0 },
                                   z: Vec4 { x: 2.0, y: 3.0, z:  4.0, w:  3.0 },
                                   w: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  4.0 } };

    static V: Vec4<float> = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    static F: float = 0.5;

    #[test]
    fn test_swap_cols() {
        let mut mut_a0 = A;
        mut_a0.swap_cols(0, 2);
        assert_eq!(mut_a0.col(0), A.col(2));
        assert_eq!(mut_a0.col(2), A.col(0));

        let mut mut_a1 = A;
        mut_a1.swap_cols(1, 2);
        assert_eq!(mut_a1.col(1), A.col(2));
        assert_eq!(mut_a1.col(2), A.col(1));
    }

    #[test]
    fn test_swap_rows() {
        let mut mut_a0 = A;
        mut_a0.swap_rows(0, 2);
        assert_eq!(mut_a0.row(0), A.row(2));
        assert_eq!(mut_a0.row(2), A.row(0));

        let mut mut_a1 = A;
        mut_a1.swap_rows(1, 2);
        assert_eq!(mut_a1.row(1), A.row(2));
        assert_eq!(mut_a1.row(2), A.row(1));
    }

    #[test]
    fn test_identity() {
        assert_eq!(Mat4::identity::<float>(),
                   Mat4::new::<float>(1.0, 0.0, 0.0, 0.0,
                                      0.0, 1.0, 0.0, 0.0,
                                      0.0, 0.0, 1.0, 0.0,
                                      0.0, 0.0, 0.0, 1.0));
        let mut mut_a = A;
        mut_a.to_identity();
        assert!(mut_a.is_identity());
    }

    #[test]
    fn test_zero() {
        assert_eq!(Mat4::zero::<float>(),
                   Mat4::new::<float>(0.0, 0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0, 0.0));
        let mut mut_a = A;
        mut_a.to_zero();
        assert_eq!(mut_a, Mat4::zero::<float>());
    }
    #[test]
    fn test_determinant() {
        assert_eq!(A.determinant(), 0.0);
    }

    #[test]
    fn test_trace() {
        assert_eq!(A.trace(), 34.0);
    }

    #[test]
    fn test_neg() {
        assert_eq!(A.neg(),
                   Mat4::new::<float>(-1.0, -5.0,  -9.0, -13.0,
                                      -2.0, -6.0, -10.0, -14.0,
                                      -3.0, -7.0, -11.0, -15.0,
                                      -4.0, -8.0, -12.0, -16.0));
        assert_eq!(-A, A.neg());
    }

    #[test]
    fn test_mul_t() {
        assert_eq!(A.mul_t(F),
                   Mat4::new::<float>(0.5, 2.5, 4.5, 6.5,
                                      1.0, 3.0, 5.0, 7.0,
                                      1.5, 3.5, 5.5, 7.5,
                                      2.0, 4.0, 6.0, 8.0));
        let mut mut_a = A;
        mut_a.mul_self_t(F);
        assert_eq!(mut_a, A.mul_t(F));
    }

    #[test]
    fn test_mul_v() {
        assert_eq!(A.mul_v(&V),
                   Vec4::new::<float>(30.0, 70.0, 110.0, 150.0));
    }

    #[test]
    fn test_add_m() {
        assert_eq!(A.add_m(&B),
                   Mat4::new::<float>(3.0, 11.0, 19.0, 27.0,
                                      5.0, 13.0, 21.0, 29.0,
                                      7.0, 15.0, 23.0, 31.0,
                                      9.0, 17.0, 25.0, 33.0));
        let mut mut_a = A;
        mut_a.add_self_m(&B);
        assert_eq!(mut_a, A.add_m(&B));
    }

    #[test]
    fn test_sub_m() {
        assert_eq!(A.sub_m(&B),
                   Mat4::new::<float>(-1.0, -1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0, -1.0));
        let mut mut_a = A;
        mut_a.sub_self_m(&B);
        assert_eq!(mut_a, A.sub_m(&B));
    }

    #[test]
    fn test_mul_m() {
        assert_eq!(A.mul_m(&B),
                   Mat4::new::<float>(100.0, 228.0, 356.0, 484.0,
                                      110.0, 254.0, 398.0, 542.0,
                                      120.0, 280.0, 440.0, 600.0,
                                      130.0, 306.0, 482.0, 658.0));
    }

    #[test]
    fn test_dot() {
        assert_eq!(A.dot(&B), 1632.0);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(A.transpose(),
                   Mat4::new::<float>( 1.0,  2.0,  3.0,  4.0,
                                       5.0,  6.0,  7.0,  8.0,
                                       9.0, 10.0, 11.0, 12.0,
                                      13.0, 14.0, 15.0, 16.0));
        let mut mut_a = A;
        mut_a.transpose_self();
        assert_eq!(mut_a, A.transpose());
    }

    #[test]
    fn test_inverse() {
        assert!(Mat4::identity::<float>().inverse().unwrap().is_identity());

        assert_approx_eq!(C.inverse().unwrap(),
                          Mat4::new::<float>( 5.0, -4.0,  1.0,  0.0,
                                             -4.0,  8.0, -4.0,  0.0,
                                              4.0, -8.0,  4.0,  8.0,
                                             -3.0,  4.0,  1.0, -8.0).mul_t(0.125));
        let mut mut_c = C;
        mut_c.invert_self();
        assert_eq!(mut_c, C.inverse().unwrap());
    }

    #[test]
    fn test_predicates() {
        assert!(Mat3::identity::<float>().is_identity());
        assert!(Mat3::identity::<float>().is_symmetric());
        assert!(Mat3::identity::<float>().is_diagonal());
        assert!(!Mat3::identity::<float>().is_rotated());
        assert!(Mat3::identity::<float>().is_invertible());

        assert!(!A.is_identity());
        assert!(!A.is_symmetric());
        assert!(!A.is_diagonal());
        assert!(A.is_rotated());
        assert!(!A.is_invertible());

        assert!(!D.is_identity());
        assert!(D.is_symmetric());
        assert!(!D.is_diagonal());
        assert!(D.is_rotated());
        assert!(D.is_invertible());

        assert!(Mat3::from_value::<float>(6.0).is_diagonal());
    }

    #[test]
    fn test_approx() {
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
