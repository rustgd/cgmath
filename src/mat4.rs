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

use mat::Mat3;
use vec::Vec4;

mod macros;
mod dim_macros;
mod mat_macros;

#[deriving(Eq)]
pub struct Mat4<T> {
    x: Vec4<T>,
    y: Vec4<T>,
    z: Vec4<T>,
    w: Vec4<T>,
}

impl_dimensional!(Mat4, Vec4<T>, 4)
impl_dimensional_fns!(Mat4, Vec4<T>, 4)
impl_approx!(Mat4)

impl_mat!(Mat4, Vec4)
impl_mat_copyable!(Mat4, Vec4)
impl_mat_numeric!(Mat4, Vec4)
impl_mat_approx_numeric!(Mat4)
impl_mat_neg!(Mat4)

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
