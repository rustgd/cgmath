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

pub use super::Dimensional;

use mat::{Mat3, ToMat3};
use mat::{Mat4, ToMat4};
use vec::Vec2;

mod macros;
mod dim_macros;
mod mat_macros;

#[deriving(Eq)]
pub struct Mat2<T> {
    x: Vec2<T>,
    y: Vec2<T>,
}

impl_dimensional!(Mat2, Vec2<T>, 2)
impl_dimensional_fns!(Mat2, Vec2<T>, 2)
impl_approx!(Mat2)

impl_mat!(Mat2, Vec2)
impl_mat_copyable!(Mat2, Vec2)
impl_mat_numeric!(Mat2, Vec2)
impl_mat_approx_numeric!(Mat2)
impl_mat_neg!(Mat2)

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

impl<T:Copy + Num> ToMat3<T> for Mat2<T> {
    #[inline]
    pub fn to_mat3(&self) -> Mat3<T> {
        Mat3::new(*self.elem(0, 0), *self.elem(0, 1), zero!(T),
                  *self.elem(1, 0), *self.elem(1, 1), zero!(T),
                  zero!(T), zero!(T), one!(T))
    }
}

impl<T:Copy + Num> ToMat4<T> for Mat2<T> {
    #[inline]
    pub fn to_mat4(&self) -> Mat4<T> {
        Mat4::new(*self.elem(0, 0), *self.elem(0, 1), zero!(T), zero!(T),
                  *self.elem(1, 0), *self.elem(1, 1), zero!(T), zero!(T),
                  zero!(T), zero!(T), one!(T), zero!(T),
                  zero!(T), zero!(T), zero!(T), one!(T))
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

#[cfg(test)]
mod tests{
    use mat::*;
    use vec::*;

    #[test]
    fn test_mat2() {
        let a = Mat2 { x: Vec2 { x: 1.0, y: 3.0 },
                       y: Vec2 { x: 2.0, y: 4.0 } };
        let b = Mat2 { x: Vec2 { x: 2.0, y: 4.0 },
                       y: Vec2 { x: 3.0, y: 5.0 } };

        let v1 = Vec2::new::<float>(1.0, 2.0);
        let f1 = 0.5;

        assert_eq!(a, Mat2::new::<float>(1.0, 3.0,
                                         2.0, 4.0));

        assert_eq!(a, Mat2::from_cols::<float>(Vec2::new::<float>(1.0, 3.0),
                                               Vec2::new::<float>(2.0, 4.0)));

        assert_eq!(Mat2::from_value::<float>(4.0),
                   Mat2::new::<float>(4.0, 0.0,
                                      0.0, 4.0));

        assert_eq!(*a.col(0), Vec2::new::<float>(1.0, 3.0));
        assert_eq!(*a.col(1), Vec2::new::<float>(2.0, 4.0));

        assert_eq!(a.row(0), Vec2::new::<float>(1.0, 2.0));
        assert_eq!(a.row(1), Vec2::new::<float>(3.0, 4.0));

        assert_eq!(*a.col(0), Vec2::new::<float>(1.0, 3.0));
        assert_eq!(*a.col(1), Vec2::new::<float>(2.0, 4.0));

        assert_eq!(Mat2::identity::<float>(),
                   Mat2::new::<float>(1.0, 0.0,
                                      0.0, 1.0));

        assert_eq!(Mat2::zero::<float>(),
                   Mat2::new::<float>(0.0, 0.0,
                                      0.0, 0.0));

        assert_eq!(a.determinant(), -2.0);
        assert_eq!(a.trace(), 5.0);

        assert_eq!(a.neg(),
                   Mat2::new::<float>(-1.0, -3.0,
                                      -2.0, -4.0));
        assert_eq!(-a, a.neg());
        assert_eq!(a.mul_t(f1),
                   Mat2::new::<float>(0.5, 1.5,
                                      1.0, 2.0));
        assert_eq!(a.mul_v(&v1), Vec2::new::<float>(5.0, 11.0));
        assert_eq!(a.add_m(&b),
                   Mat2::new::<float>(3.0, 7.0,
                                      5.0, 9.0));
        assert_eq!(a.sub_m(&b),
                   Mat2::new::<float>(-1.0, -1.0,
                                      -1.0, -1.0));
        assert_eq!(a.mul_m(&b),
                   Mat2::new::<float>(10.0, 22.0,
                                      13.0, 29.0));
        assert_eq!(a.dot(&b), 40.0);

        assert_eq!(a.transpose(),
                   Mat2::new::<float>(1.0, 2.0,
                                      3.0, 4.0));

        assert_eq!(a.inverse().unwrap(),
                   Mat2::new::<float>(-2.0,  1.5,
                                      1.0, -0.5));

        assert!(Mat2::new::<float>(0.0, 2.0,
                                   0.0, 5.0).inverse().is_none());

        let ident = Mat2::identity::<float>();

        assert!(ident.is_identity());
        assert!(ident.is_symmetric());
        assert!(ident.is_diagonal());
        assert!(!ident.is_rotated());
        assert!(ident.is_invertible());

        assert!(!a.is_identity());
        assert!(!a.is_symmetric());
        assert!(!a.is_diagonal());
        assert!(a.is_rotated());
        assert!(a.is_invertible());

        let c = Mat2::new::<float>(2.0, 1.0,
                                   1.0, 2.0);
        assert!(!c.is_identity());
        assert!(c.is_symmetric());
        assert!(!c.is_diagonal());
        assert!(c.is_rotated());
        assert!(c.is_invertible());

        assert!(Mat2::from_value::<float>(6.0).is_diagonal());

        assert_eq!(a.to_mat3(),
                   Mat3::new::<float>(1.0, 3.0, 0.0,
                                      2.0, 4.0, 0.0,
                                      0.0, 0.0, 1.0));

        assert_eq!(a.to_mat4(),
                   Mat4::new::<float>(1.0, 3.0, 0.0, 0.0,
                                      2.0, 4.0, 0.0, 0.0,
                                      0.0, 0.0, 1.0, 0.0,
                                      0.0, 0.0, 0.0, 1.0));
    }

    fn test_mat2_mut() {
        let a = Mat2 { x: Vec2 { x: 1.0, y: 3.0 },
                       y: Vec2 { x: 2.0, y: 4.0 } };
        let b = Mat2 { x: Vec2 { x: 2.0, y: 4.0 },
                       y: Vec2 { x: 3.0, y: 5.0 } };

        let f1 = 0.5;

        let mut mut_a = a;

        mut_a.swap_cols(0, 1);
        assert_eq!(mut_a.col(0), a.col(1));
        assert_eq!(mut_a.col(1), a.col(0));
        mut_a = a;

        mut_a.swap_rows(0, 1);
        assert_eq!(mut_a.row(0), a.row(1));
        assert_eq!(mut_a.row(1), a.row(0));
        mut_a = a;

        mut_a.to_identity();
        assert!(mut_a.is_identity());
        mut_a = a;

        mut_a.to_zero();
        assert_eq!(mut_a, Mat2::zero::<float>());
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

        mut_a.invert_self();
        assert_eq!(mut_a, a.inverse().unwrap());
        mut_a = a;

        mut_a.transpose_self();
        assert_eq!(mut_a, a.transpose());
        // mut_a = a;
    }

    #[test]
    fn test_mat2_approx_eq() {
        assert!(!Mat2::new::<float>(0.000001, 0.000001,
                                    0.000001, 0.000001).approx_eq(&Mat2::zero::<float>()));
        assert!(Mat2::new::<float>(0.0000001, 0.0000001,
                                   0.0000001, 0.0000001).approx_eq(&Mat2::zero::<float>()));
    }
}
