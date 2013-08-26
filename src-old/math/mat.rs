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

//! Matrix types

use math::{Dimensioned, SwapComponents};
use math::{Quat, ToQuat};
use math::{Vec2, Vec3, Vec4};

pub trait Mat<T,Vec,Slice>: Dimensioned<Vec,Slice>
                          + SwapComponents {
    pub fn c<'a>(&'a self, c: uint) -> &'a Vec;
    pub fn r(&self, r: uint) -> Vec;
    pub fn cr<'a>(&'a self, c: uint, r: uint) -> &'a T;
    pub fn mut_c<'a>(&'a mut self, c: uint) -> &'a mut Vec;
    pub fn mut_cr<'a>(&'a mut self, c: uint, r: uint) -> &'a mut T;
    pub fn swap_c(&mut self, a: uint, b: uint);
    pub fn swap_r(&mut self, a: uint, b: uint);
    pub fn swap_cr(&mut self, a: (uint, uint), b: (uint, uint));
    pub fn transpose(&self) -> Self;
    pub fn transpose_self(&mut self);
}

pub trait NumMat<T,Vec,Slice>: Mat<T,Vec,Slice> + Neg<Self> {
    pub fn mul_s(&self, value: T) -> Self;
    pub fn mul_v(&self, vec: &Vec) -> Vec;
    pub fn add_m(&self, other: &Self) -> Self;
    pub fn sub_m(&self, other: &Self) -> Self;
    pub fn mul_m(&self, other: &Self) -> Self;
    pub fn mul_self_s(&mut self, value: T);
    pub fn add_self_m(&mut self, other: &Self);
    pub fn sub_self_m(&mut self, other: &Self);
    pub fn dot(&self, other: &Self) -> T;
    pub fn determinant(&self) -> T;
    pub fn trace(&self) -> T;
    pub fn to_identity(&mut self);
    pub fn to_zero(&mut self);
}

pub trait FloatMat<T,Vec,Slice>: NumMat<T,Vec,Slice> {
    pub fn inverse(&self) -> Option<Self>;
    pub fn invert_self(&mut self);
    pub fn is_identity(&self) -> bool;
    pub fn is_diagonal(&self) -> bool;
    pub fn is_rotated(&self) -> bool;
    pub fn is_symmetric(&self) -> bool;
    pub fn is_invertible(&self) -> bool;
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

impl_dimensioned!(Mat2, Vec2<T>, 2)
impl_swap_components!(Mat2)
impl_approx!(Mat3 { x, y, z })

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

impl<T:Clone> Mat<T,Vec2<T>,[Vec2<T>,..2]> for Mat2<T> {
    #[inline]
    pub fn c<'a>(&'a self, c: uint) -> &'a Vec2<T> {
        self.i(c)
    }

    #[inline]
    pub fn r(&self, r: uint) -> Vec2<T> {
        Vec2::new(self.i(0).i(r).clone(),
                  self.i(1).i(r).clone())
    }

    #[inline]
    pub fn cr<'a>(&'a self, c: uint, r: uint) -> &'a T {
        self.i(c).i(r)
    }

    #[inline]
    pub fn mut_c<'a>(&'a mut self, c: uint) -> &'a mut Vec2<T> {
        self.mut_i(c)
    }

    #[inline]
    pub fn mut_cr<'a>(&'a mut self, c: uint, r: uint) -> &'a mut T {
        self.mut_i(c).mut_i(r)
    }

    #[inline]
    pub fn swap_c(&mut self, a: uint, b: uint) {
        let tmp = self.c(a).clone();
        *self.mut_c(a) = self.c(b).clone();
        *self.mut_c(b) = tmp;
    }

    #[inline]
    pub fn swap_r(&mut self, a: uint, b: uint) {
        self.mut_c(0).swap(a, b);
        self.mut_c(1).swap(a, b);
    }

    #[inline]
    pub fn swap_cr(&mut self, (col_a, row_a): (uint, uint),
                                (col_b, row_b): (uint, uint)) {
        let tmp = self.cr(col_a, row_a).clone();
        *self.mut_cr(col_a, row_a) = self.cr(col_b, row_b).clone();
        *self.mut_cr(col_b, row_b) = tmp;
    }

    #[inline]
    pub fn transpose(&self) -> Mat2<T> {
        Mat2::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(),
                  self.cr(0, 1).clone(), self.cr(1, 1).clone())
    }

    #[inline]
    pub fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
    }
}

impl<T:Clone + Num> ToMat3<T> for Mat2<T> {
    #[inline]
    pub fn to_mat3(&self) -> Mat3<T> {
        Mat3::new(self.cr(0, 0).clone(), self.cr(0, 1).clone(), zero!(T),
                  self.cr(1, 0).clone(), self.cr(1, 1).clone(), zero!(T),
                  zero!(T), zero!(T), one!(T))
    }
}

impl<T:Clone + Num> ToMat4<T> for Mat2<T> {
    #[inline]
    pub fn to_mat4(&self) -> Mat4<T> {
        Mat4::new(self.cr(0, 0).clone(), self.cr(0, 1).clone(), zero!(T), zero!(T),
                  self.cr(1, 0).clone(), self.cr(1, 1).clone(), zero!(T), zero!(T),
                  zero!(T), zero!(T), one!(T), zero!(T),
                  zero!(T), zero!(T), zero!(T), one!(T))
    }
}

impl<T:Num> Mat2<T> {
    #[inline]
    pub fn identity() -> Mat2<T> {
        Mat2::from_cols(Vec2::unit_x(),
                        Vec2::unit_y())
    }

    #[inline]
    pub fn zero() -> Mat2<T> {
        Mat2::from_cols(Vec2::zero(),
                        Vec2::zero())
    }
}

impl<T:Clone + Num> Mat2<T> {
    #[inline]
    pub fn from_value(value: T) -> Mat2<T> {
        Mat2::new(value.clone(), zero!(T),
                  zero!(T), value.clone())
    }
}

impl<T:Clone + Num> NumMat<T,Vec2<T>,[Vec2<T>,..2]> for Mat2<T> {
    #[inline]
    pub fn mul_s(&self, value: T) -> Mat2<T> {
        Mat2::from_cols(self.c(0).mul_s(value.clone()),
                        self.c(1).mul_s(value.clone()))
    }

    #[inline]
    pub fn mul_v(&self, vec: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self.r(0).dot(vec),
                  self.r(1).dot(vec))
    }

    #[inline]
    pub fn add_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self.c(0).add_v(other.c(0)),
                        self.c(1).add_v(other.c(1)))
    }

    #[inline]
    pub fn sub_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self.c(0).sub_v(other.c(0)),
                        self.c(1).sub_v(other.c(1)))
    }

    #[inline]
    pub fn mul_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::new(self.r(0).dot(other.c(0)), self.r(1).dot(other.c(0)),
                  self.r(0).dot(other.c(1)), self.r(1).dot(other.c(1)))
    }

    #[inline]
    pub fn mul_self_s(&mut self, value: T) {
        self.mut_c(0).mul_self_s(value.clone());
        self.mut_c(1).mul_self_s(value.clone());
    }

    #[inline]
    pub fn add_self_m(&mut self, other: &Mat2<T>) {
        self.mut_c(0).add_self_v(other.c(0));
        self.mut_c(1).add_self_v(other.c(1));
    }

    #[inline]
    pub fn sub_self_m(&mut self, other: &Mat2<T>) {
        self.mut_c(0).sub_self_v(other.c(0));
        self.mut_c(1).sub_self_v(other.c(1));
    }

    pub fn dot(&self, other: &Mat2<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    pub fn determinant(&self) -> T {
       *self.cr(0, 0) * *self.cr(1, 1) - *self.cr(1, 0) * *self.cr(0, 1)
    }

    pub fn trace(&self) -> T {
        *self.cr(0, 0) + *self.cr(1, 1)
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
        Mat2::from_cols(-*self.c(0),
                        -*self.c(1))
    }
}

impl<T:Clone + Float> Mat2<T> {
    #[inline]
    pub fn from_angle(radians: T) -> Mat2<T> {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat2::new(cos_theta.clone(),  -sin_theta.clone(),
                  sin_theta.clone(),  cos_theta.clone())
    }
}

impl<T:Clone + Float> FloatMat<T,Vec2<T>,[Vec2<T>,..2]> for Mat2<T> {
    #[inline]
    pub fn inverse(&self) -> Option<Mat2<T>> {
        let d = self.determinant();
        if d.approx_eq(&zero!(T)) {
            None
        } else {
            Some(Mat2::new(self.cr(1, 1) / d, -self.cr(0, 1) / d,
                           -self.cr(1, 0) / d, self.cr(0, 0) / d))
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
        self.cr(0, 1).approx_eq(&zero!(T)) &&
        self.cr(1, 0).approx_eq(&zero!(T))
    }

    #[inline]
    pub fn is_rotated(&self) -> bool {
        !self.approx_eq(&Mat2::identity())
    }

    #[inline]
    pub fn is_symmetric(&self) -> bool {
        self.cr(0, 1).approx_eq(self.cr(1, 0)) &&
        self.cr(1, 0).approx_eq(self.cr(0, 1))
    }

    #[inline]
    pub fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&zero!(T))
    }
}

#[cfg(test)]
mod mat2_tests{
    use math::mat::*;
    use math::vec::*;

    static A: Mat2<float> = Mat2 { x: Vec2 { x: 1.0, y: 3.0 },
                                   y: Vec2 { x: 2.0, y: 4.0 } };
    static B: Mat2<float> = Mat2 { x: Vec2 { x: 2.0, y: 4.0 },
                                   y: Vec2 { x: 3.0, y: 5.0 } };
    static C: Mat2<float> = Mat2 { x: Vec2 { x: 2.0, y: 1.0 },
                                   y: Vec2 { x: 1.0, y: 2.0 } };

    static V: Vec2<float> = Vec2 { x: 1.0, y: 2.0 };
    static F: float = 0.5;

    #[test]
    fn test_swap_c() {
        let mut mut_a = A;
        mut_a.swap_c(0, 1);
        assert_eq!(mut_a.c(0), A.c(1));
        assert_eq!(mut_a.c(1), A.c(0));
    }

    #[test]
    fn test_swap_r() {
        let mut mut_a = A;
        mut_a.swap_r(0, 1);
        assert_eq!(mut_a.r(0), A.r(1));
        assert_eq!(mut_a.r(1), A.r(0));
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
    fn test_mul_s() {
        assert_eq!(A.mul_s(F),
                   Mat2::new::<float>(0.5, 1.5,
                                      1.0, 2.0));
        let mut mut_a = A;
        mut_a.mul_self_s(F);
        assert_eq!(mut_a, A.mul_s(F));
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

impl_dimensioned!(Mat3, Vec3<T>, 3)
impl_swap_components!(Mat3)
impl_approx!(Mat2 { x, y })

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

    #[inline]
    pub fn from_axes(x: Vec3<T>,
                     y: Vec3<T>,
                     z: Vec3<T>) -> Mat3<T> {
        Mat3 { x: x, y: y, z: z }
    }
}

impl<T:Clone> Mat<T,Vec3<T>,[Vec3<T>,..3]> for Mat3<T> {
    #[inline]
    pub fn c<'a>(&'a self, c: uint) -> &'a Vec3<T> {
        self.i(c)
    }

    #[inline]
    pub fn r(&self, r: uint) -> Vec3<T> {
        Vec3::new(self.i(0).i(r).clone(),
                  self.i(1).i(r).clone(),
                  self.i(2).i(r).clone())
    }

    #[inline]
    pub fn cr<'a>(&'a self, c: uint, r: uint) -> &'a T {
        self.i(c).i(r)
    }

    #[inline]
    pub fn mut_c<'a>(&'a mut self, c: uint) -> &'a mut Vec3<T> {
        self.mut_i(c)
    }

    #[inline]
    pub fn mut_cr<'a>(&'a mut self, c: uint, r: uint) -> &'a mut T {
        self.mut_i(c).mut_i(r)
    }

    #[inline]
    pub fn swap_c(&mut self, a: uint, b: uint) {
        let tmp = self.c(a).clone();
        *self.mut_c(a) = self.c(b).clone();
        *self.mut_c(b) = tmp;
    }

    #[inline]
    pub fn swap_r(&mut self, a: uint, b: uint) {
        self.mut_c(0).swap(a, b);
        self.mut_c(1).swap(a, b);
        self.mut_c(2).swap(a, b);
    }

    #[inline]
    pub fn swap_cr(&mut self, (col_a, row_a): (uint, uint),
                                (col_b, row_b): (uint, uint)) {
        let tmp = self.cr(col_a, row_a).clone();
        *self.mut_cr(col_a, row_a) = self.cr(col_b, row_b).clone();
        *self.mut_cr(col_b, row_b) = tmp;
    }

    #[inline]
    pub fn transpose(&self) -> Mat3<T> {
        Mat3::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(), self.cr(2, 0).clone(),
                  self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(2, 1).clone(),
                  self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(2, 2).clone())
    }

    #[inline]
    pub fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
        self.swap_cr((0, 2), (2, 0));
        self.swap_cr((1, 2), (2, 1));
    }
}

impl<T:Clone + Num> ToMat4<T> for Mat3<T> {
    #[inline]
    pub fn to_mat4(&self) -> Mat4<T> {
        Mat4::new(self.cr(0, 0).clone(), self.cr(0, 1).clone(), self.cr(0, 2).clone(), zero!(T),
                  self.cr(1, 0).clone(), self.cr(1, 1).clone(), self.cr(1, 2).clone(), zero!(T),
                  self.cr(2, 0).clone(), self.cr(2, 1).clone(), self.cr(2, 2).clone(), zero!(T),
                  zero!(T), zero!(T), zero!(T), one!(T))
    }
}

impl<T:Num> Mat3<T> {
    #[inline]
    pub fn identity() -> Mat3<T> {
        Mat3::from_cols(Vec3::unit_x(),
                        Vec3::unit_y(),
                        Vec3::unit_z())
    }

    #[inline]
    pub fn zero() -> Mat3<T> {
        Mat3::from_cols(Vec3::zero(),
                        Vec3::zero(),
                        Vec3::zero())
    }
}

impl<T:Clone + Num> Mat3<T> {
    #[inline]
    pub fn from_value(value: T) -> Mat3<T> {
        Mat3::new(value.clone(), zero!(T), zero!(T),
                  zero!(T), value.clone(), zero!(T),
                  zero!(T), zero!(T), value.clone())
    }
}

impl<T:Clone + Num> NumMat<T,Vec3<T>,[Vec3<T>,..3]> for Mat3<T> {
    #[inline]
    pub fn mul_s(&self, value: T) -> Mat3<T> {
        Mat3::from_cols(self.c(0).mul_s(value.clone()),
                        self.c(1).mul_s(value.clone()),
                        self.c(2).mul_s(value.clone()))
    }

    #[inline]
    pub fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.r(0).dot(vec),
                  self.r(1).dot(vec),
                  self.r(2).dot(vec))
    }

    #[inline]
    pub fn add_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self.c(0).add_v(other.c(0)),
                        self.c(1).add_v(other.c(1)),
                        self.c(2).add_v(other.c(2)))
    }

    #[inline]
    pub fn sub_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self.c(0).sub_v(other.c(0)),
                        self.c(1).sub_v(other.c(1)),
                        self.c(2).sub_v(other.c(2)))
    }

    #[inline]
    pub fn mul_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::new(self.r(0).dot(other.c(0)),self.r(1).dot(other.c(0)),self.r(2).dot(other.c(0)),
                  self.r(0).dot(other.c(1)),self.r(1).dot(other.c(1)),self.r(2).dot(other.c(1)),
                  self.r(0).dot(other.c(2)),self.r(1).dot(other.c(2)),self.r(2).dot(other.c(2)))
    }

    #[inline]
    pub fn mul_self_s(&mut self, value: T) {
        self.mut_c(0).mul_self_s(value.clone());
        self.mut_c(1).mul_self_s(value.clone());
        self.mut_c(2).mul_self_s(value.clone());
    }

    #[inline]
    pub fn add_self_m(&mut self, other: &Mat3<T>) {
        self.mut_c(0).add_self_v(other.c(0));
        self.mut_c(1).add_self_v(other.c(1));
        self.mut_c(2).add_self_v(other.c(2));
    }

    #[inline]
    pub fn sub_self_m(&mut self, other: &Mat3<T>) {
        self.mut_c(0).sub_self_v(other.c(0));
        self.mut_c(1).sub_self_v(other.c(1));
        self.mut_c(2).sub_self_v(other.c(2));
    }

    pub fn dot(&self, other: &Mat3<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    pub fn determinant(&self) -> T {
        *self.cr(0, 0) * (*self.cr(1, 1) * *self.cr(2, 2) - *self.cr(2, 1) * *self.cr(1, 2))
      - *self.cr(1, 0) * (*self.cr(0, 1) * *self.cr(2, 2) - *self.cr(2, 1) * *self.cr(0, 2))
      + *self.cr(2, 0) * (*self.cr(0, 1) * *self.cr(1, 2) - *self.cr(1, 1) * *self.cr(0, 2))
    }

    pub fn trace(&self) -> T {
        (*self.cr(0, 0)) + (*self.cr(1, 1)) + (*self.cr(2, 2))
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
        Mat3::from_cols(-*self.c(0),
                        -*self.c(1),
                        -*self.c(2))
    }
}

impl<T: Clone + Float> Mat3<T> {
    pub fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> Mat3<T> {
        let dir_ = dir.normalize();
        let side = dir_.cross(&up.normalize());
        let up_  = side.cross(&dir_).normalize();

        Mat3::from_axes(up_, side, dir_)
    }
}

impl<T:Clone + Float> ToQuat<T> for Mat3<T> {
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
                x = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                y = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                z = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
            }
            ((*self.cr(0, 0) > *self.cr(1, 1))
            && (*self.cr(0, 0) > *self.cr(2, 2))) {
                s = (half + (*self.cr(0, 0) - *self.cr(1, 1) - *self.cr(2, 2))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                y = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                z = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
            }
            (*self.cr(1, 1) > *self.cr(2, 2)) {
                s = (half + (*self.cr(1, 1) - *self.cr(0, 0) - *self.cr(2, 2))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                y = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                z = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
            }
            _ {
                s = (half + (*self.cr(2, 2) - *self.cr(0, 0) - *self.cr(1, 1))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                y = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                z = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
            }
        )
        Quat::new(w, x, y, z)
    }
}

impl<T:Clone + Float> FloatMat<T,Vec3<T>,[Vec3<T>,..3]> for Mat3<T> {
    #[inline]
    pub fn inverse(&self) -> Option<Mat3<T>> {
        let d = self.determinant();
        if d.approx_eq(&zero!(T)) {
            None
        } else {
            Some(Mat3::from_cols(self.c(1).cross(self.c(2)).div_s(d.clone()),
                                 self.c(2).cross(self.c(0)).div_s(d.clone()),
                                 self.c(0).cross(self.c(1)).div_s(d.clone())).transpose())
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
        self.cr(0, 1).approx_eq(&zero!(T)) &&
        self.cr(0, 2).approx_eq(&zero!(T)) &&

        self.cr(1, 0).approx_eq(&zero!(T)) &&
        self.cr(1, 2).approx_eq(&zero!(T)) &&

        self.cr(2, 0).approx_eq(&zero!(T)) &&
        self.cr(2, 1).approx_eq(&zero!(T))
    }

    #[inline]
    pub fn is_rotated(&self) -> bool {
        !self.approx_eq(&Mat3::identity())
    }

    #[inline]
    pub fn is_symmetric(&self) -> bool {
        self.cr(0, 1).approx_eq(self.cr(1, 0)) &&
        self.cr(0, 2).approx_eq(self.cr(2, 0)) &&

        self.cr(1, 0).approx_eq(self.cr(0, 1)) &&
        self.cr(1, 2).approx_eq(self.cr(2, 1)) &&

        self.cr(2, 0).approx_eq(self.cr(0, 2)) &&
        self.cr(2, 1).approx_eq(self.cr(1, 2))
    }

    #[inline]
    pub fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&zero!(T))
    }
}

#[cfg(test)]
mod mat3_tests{
    use math::mat::*;
    use math::vec::*;

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
    fn test_swap_c() {
        let mut mut_a0 = A;
        mut_a0.swap_c(0, 2);
        assert_eq!(mut_a0.c(0), A.c(2));
        assert_eq!(mut_a0.c(2), A.c(0));

        let mut mut_a1 = A;
        mut_a1.swap_c(1, 2);
        assert_eq!(mut_a1.c(1), A.c(2));
        assert_eq!(mut_a1.c(2), A.c(1));
    }

    #[test]
    fn test_swap_r() {
        let mut mut_a0 = A;
        mut_a0.swap_r(0, 2);
        assert_eq!(mut_a0.r(0), A.r(2));
        assert_eq!(mut_a0.r(2), A.r(0));

        let mut mut_a1 = A;
        mut_a1.swap_r(1, 2);
        assert_eq!(mut_a1.r(1), A.r(2));
        assert_eq!(mut_a1.r(2), A.r(1));
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
    fn test_mul_s() {
        assert_eq!(A.mul_s(F),
                   Mat3::new::<float>(0.5, 2.0, 3.5,
                                      1.0, 2.5, 4.0,
                                      1.5, 3.0, 4.5));
        let mut mut_a = A;
        mut_a.mul_self_s(F);
        assert_eq!(mut_a, A.mul_s(F));
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

impl_dimensioned!(Mat4, Vec4<T>, 4)
impl_swap_components!(Mat4)
impl_approx!(Mat4 { x, y, z, w })

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

impl<T:Clone> Mat<T,Vec4<T>,[Vec4<T>,..4]> for Mat4<T> {
    #[inline]
    pub fn c<'a>(&'a self, c: uint) -> &'a Vec4<T> {
        self.i(c)
    }

    #[inline]
    pub fn r(&self, r: uint) -> Vec4<T> {
        Vec4::new(self.i(0).i(r).clone(),
                  self.i(1).i(r).clone(),
                  self.i(2).i(r).clone(),
                  self.i(3).i(r).clone())
    }

    #[inline]
    pub fn cr<'a>(&'a self, c: uint, r: uint) -> &'a T {
        self.i(c).i(r)
    }

    #[inline]
    pub fn mut_c<'a>(&'a mut self, c: uint) -> &'a mut Vec4<T> {
        self.mut_i(c)
    }

    #[inline]
    pub fn mut_cr<'a>(&'a mut self, c: uint, r: uint) -> &'a mut T {
        self.mut_i(c).mut_i(r)
    }

    #[inline]
    pub fn swap_c(&mut self, a: uint, b: uint) {
        let tmp = self.c(a).clone();
        *self.mut_c(a) = self.c(b).clone();
        *self.mut_c(b) = tmp;
    }

    #[inline]
    pub fn swap_r(&mut self, a: uint, b: uint) {
        self.mut_c(0).swap(a, b);
        self.mut_c(1).swap(a, b);
        self.mut_c(2).swap(a, b);
        self.mut_c(3).swap(a, b);
    }

    #[inline]
    pub fn swap_cr(&mut self, (col_a, row_a): (uint, uint),
                                (col_b, row_b): (uint, uint)) {
        let tmp = self.cr(col_a, row_a).clone();
        *self.mut_cr(col_a, row_a) = self.cr(col_b, row_b).clone();
        *self.mut_cr(col_b, row_b) = tmp;
    }

    #[inline]
    pub fn transpose(&self) -> Mat4<T> {
        Mat4::new(self.cr(0, 0).clone(),self.cr(1, 0).clone(),self.cr(2, 0).clone(),self.cr(3, 0).clone(),
                  self.cr(0, 1).clone(),self.cr(1, 1).clone(),self.cr(2, 1).clone(),self.cr(3, 1).clone(),
                  self.cr(0, 2).clone(),self.cr(1, 2).clone(),self.cr(2, 2).clone(),self.cr(3, 2).clone(),
                  self.cr(0, 3).clone(),self.cr(1, 3).clone(),self.cr(2, 3).clone(),self.cr(3, 3).clone())
    }

    #[inline]
    pub fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
        self.swap_cr((0, 2), (2, 0));
        self.swap_cr((0, 3), (3, 0));
        self.swap_cr((1, 2), (2, 1));
        self.swap_cr((1, 3), (3, 1));
        self.swap_cr((2, 3), (3, 2));
    }
}

impl<T:Num> Mat4<T> {
    #[inline]
    pub fn identity() -> Mat4<T> {
        Mat4::from_cols(Vec4::unit_x(),
                        Vec4::unit_y(),
                        Vec4::unit_z(),
                        Vec4::unit_w())
    }

    #[inline]
    pub fn zero() -> Mat4<T> {
        Mat4::from_cols(Vec4::zero(),
                        Vec4::zero(),
                        Vec4::zero(),
                        Vec4::zero())
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
}

impl<T:Clone + Num> NumMat<T,Vec4<T>,[Vec4<T>,..4]> for Mat4<T> {
    #[inline]
    pub fn mul_s(&self, value: T) -> Mat4<T> {
        Mat4::from_cols(self.c(0).mul_s(value.clone()),
                        self.c(1).mul_s(value.clone()),
                        self.c(2).mul_s(value.clone()),
                        self.c(3).mul_s(value.clone()))
    }

    #[inline]
    pub fn mul_v(&self, vec: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self.r(0).dot(vec),
                  self.r(1).dot(vec),
                  self.r(2).dot(vec),
                  self.r(3).dot(vec))
    }

    #[inline]
    pub fn add_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self.c(0).add_v(other.c(0)),
                        self.c(1).add_v(other.c(1)),
                        self.c(2).add_v(other.c(2)),
                        self.c(3).add_v(other.c(3)))
    }

    #[inline]
    pub fn sub_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self.c(0).sub_v(other.c(0)),
                        self.c(1).sub_v(other.c(1)),
                        self.c(2).sub_v(other.c(2)),
                        self.c(3).sub_v(other.c(3)))
    }

    #[inline]
    pub fn mul_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::new(self.r(0).dot(other.c(0)), self.r(1).dot(other.c(0)), self.r(2).dot(other.c(0)), self.r(3).dot(other.c(0)),
                  self.r(0).dot(other.c(1)), self.r(1).dot(other.c(1)), self.r(2).dot(other.c(1)), self.r(3).dot(other.c(1)),
                  self.r(0).dot(other.c(2)), self.r(1).dot(other.c(2)), self.r(2).dot(other.c(2)), self.r(3).dot(other.c(2)),
                  self.r(0).dot(other.c(3)), self.r(1).dot(other.c(3)), self.r(2).dot(other.c(3)), self.r(3).dot(other.c(3)))
    }

    #[inline]
    pub fn mul_self_s(&mut self, value: T) {
        self.mut_c(0).mul_self_s(value.clone());
        self.mut_c(1).mul_self_s(value.clone());
        self.mut_c(2).mul_self_s(value.clone());
        self.mut_c(3).mul_self_s(value.clone());
    }

    #[inline]
    pub fn add_self_m(&mut self, other: &Mat4<T>) {
        self.mut_c(0).add_self_v(other.c(0));
        self.mut_c(1).add_self_v(other.c(1));
        self.mut_c(2).add_self_v(other.c(2));
        self.mut_c(3).add_self_v(other.c(3));
    }

    #[inline]
    pub fn sub_self_m(&mut self, other: &Mat4<T>) {
        self.mut_c(0).sub_self_v(other.c(0));
        self.mut_c(1).sub_self_v(other.c(1));
        self.mut_c(2).sub_self_v(other.c(2));
        self.mut_c(3).sub_self_v(other.c(3));
    }

    pub fn dot(&self, other: &Mat4<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    pub fn determinant(&self) -> T {
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

    pub fn trace(&self) -> T {
        *self.cr(0, 0) + *self.cr(1, 1) + *self.cr(2, 2) + *self.cr(3, 3)
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
        Mat4::from_cols(-*self.c(0),
                        -*self.c(1),
                        -*self.c(2),
                        -*self.c(3))
    }
}

impl<T:Clone + Float> FloatMat<T,Vec4<T>,[Vec4<T>,..4]> for Mat4<T> {
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
                    if A.cr(j, i).abs() > A.cr(j, i1).abs() {
                        i1 = i;
                    }
                }

                // SwapComponents columns i1 and j in A and I to
                // put pivot on diagonal
                A.swap_c(i1, j);
                I.swap_c(i1, j);

                // Scale col j to have a unit diagonal
                let ajj = A.cr(j, j).clone();
                I.mut_c(j).div_self_s(ajj.clone());
                A.mut_c(j).div_self_s(ajj.clone());

                // Eliminate off-diagonal elems in col j of A,
                // doing identical ops to I
                for uint::range(0, 4) |i| {
                    if i != j {
                        let ij_mul_aij = I.c(j).mul_s(A.cr(i, j).clone());
                        let aj_mul_aij = A.c(j).mul_s(A.cr(i, j).clone());
                        I.mut_c(i).sub_self_v(&ij_mul_aij);
                        A.mut_c(i).sub_self_v(&aj_mul_aij);
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
        self.cr(0, 1).approx_eq(&zero!(T)) &&
        self.cr(0, 2).approx_eq(&zero!(T)) &&
        self.cr(0, 3).approx_eq(&zero!(T)) &&

        self.cr(1, 0).approx_eq(&zero!(T)) &&
        self.cr(1, 2).approx_eq(&zero!(T)) &&
        self.cr(1, 3).approx_eq(&zero!(T)) &&

        self.cr(2, 0).approx_eq(&zero!(T)) &&
        self.cr(2, 1).approx_eq(&zero!(T)) &&
        self.cr(2, 3).approx_eq(&zero!(T)) &&

        self.cr(3, 0).approx_eq(&zero!(T)) &&
        self.cr(3, 1).approx_eq(&zero!(T)) &&
        self.cr(3, 2).approx_eq(&zero!(T))
    }

    #[inline]
    pub fn is_rotated(&self) -> bool {
        !self.approx_eq(&Mat4::identity())
    }

    #[inline]
    pub fn is_symmetric(&self) -> bool {
        self.cr(0, 1).approx_eq(self.cr(1, 0)) &&
        self.cr(0, 2).approx_eq(self.cr(2, 0)) &&
        self.cr(0, 3).approx_eq(self.cr(3, 0)) &&

        self.cr(1, 0).approx_eq(self.cr(0, 1)) &&
        self.cr(1, 2).approx_eq(self.cr(2, 1)) &&
        self.cr(1, 3).approx_eq(self.cr(3, 1)) &&

        self.cr(2, 0).approx_eq(self.cr(0, 2)) &&
        self.cr(2, 1).approx_eq(self.cr(1, 2)) &&
        self.cr(2, 3).approx_eq(self.cr(3, 2)) &&

        self.cr(3, 0).approx_eq(self.cr(0, 3)) &&
        self.cr(3, 1).approx_eq(self.cr(1, 3)) &&
        self.cr(3, 2).approx_eq(self.cr(2, 3))
    }

    #[inline]
    pub fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&zero!(T))
    }
}

#[cfg(test)]
mod mat4_tests {
    use math::mat::*;
    use math::vec::*;

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
    fn test_swap_c() {
        let mut mut_a0 = A;
        mut_a0.swap_c(0, 2);
        assert_eq!(mut_a0.c(0), A.c(2));
        assert_eq!(mut_a0.c(2), A.c(0));

        let mut mut_a1 = A;
        mut_a1.swap_c(1, 2);
        assert_eq!(mut_a1.c(1), A.c(2));
        assert_eq!(mut_a1.c(2), A.c(1));
    }

    #[test]
    fn test_swap_r() {
        let mut mut_a0 = A;
        mut_a0.swap_r(0, 2);
        assert_eq!(mut_a0.r(0), A.r(2));
        assert_eq!(mut_a0.r(2), A.r(0));

        let mut mut_a1 = A;
        mut_a1.swap_r(1, 2);
        assert_eq!(mut_a1.r(1), A.r(2));
        assert_eq!(mut_a1.r(2), A.r(1));
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
    fn test_mul_s() {
        assert_eq!(A.mul_s(F),
                   Mat4::new::<float>(0.5, 2.5, 4.5, 6.5,
                                      1.0, 3.0, 5.0, 7.0,
                                      1.5, 3.5, 5.5, 7.5,
                                      2.0, 4.0, 6.0, 8.0));
        let mut mut_a = A;
        mut_a.mul_self_s(F);
        assert_eq!(mut_a, A.mul_s(F));
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
                                             -3.0,  4.0,  1.0, -8.0).mul_s(0.125));
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
