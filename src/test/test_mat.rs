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

#[test]
fn test_mat3() {
    let a = Mat3 { x: Vec3 { x: 1.0, y: 4.0, z:  7.0 },
                   y: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                   z: Vec3 { x: 3.0, y: 6.0, z:  9.0 } };
    let b = Mat3 { x: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                   y: Vec3 { x: 3.0, y: 6.0, z:  9.0 },
                   z: Vec3 { x: 4.0, y: 7.0, z: 10.0 } };

    let v1 = Vec3::new::<float>(1.0, 2.0, 3.0);
    let f1 = 0.5;

    assert_eq!(a, Mat3::new::<float>(1.0, 4.0, 7.0,
                                     2.0, 5.0, 8.0,
                                     3.0, 6.0, 9.0));

    assert_eq!(a, Mat3::from_cols::<float>(Vec3::new::<float>(1.0, 4.0, 7.0),
                                           Vec3::new::<float>(2.0, 5.0, 8.0),
                                           Vec3::new::<float>(3.0, 6.0, 9.0)));

    assert_eq!(*a.col(0), Vec3::new::<float>(1.0, 4.0, 7.0));
    assert_eq!(*a.col(1), Vec3::new::<float>(2.0, 5.0, 8.0));
    assert_eq!(*a.col(2), Vec3::new::<float>(3.0, 6.0, 9.0));

    assert_eq!(a.row(0), Vec3::new::<float>(1.0, 2.0, 3.0));
    assert_eq!(a.row(1), Vec3::new::<float>(4.0, 5.0, 6.0));
    assert_eq!(a.row(2), Vec3::new::<float>(7.0, 8.0, 9.0));

    assert_eq!(*a.col(0), Vec3::new::<float>(1.0, 4.0, 7.0));
    assert_eq!(*a.col(1), Vec3::new::<float>(2.0, 5.0, 8.0));
    assert_eq!(*a.col(2), Vec3::new::<float>(3.0, 6.0, 9.0));

    assert_eq!(Mat3::identity::<float>(),
               Mat3::new::<float>(1.0, 0.0, 0.0,
                                  0.0, 1.0, 0.0,
                                  0.0, 0.0, 1.0));

    assert_eq!(Mat3::zero::<float>(),
               Mat3::new::<float>(0.0, 0.0, 0.0,
                                  0.0, 0.0, 0.0,
                                  0.0, 0.0, 0.0));

    assert_eq!(a.determinant(), 0.0);
    assert_eq!(a.trace(), 15.0);

    assert_eq!(a.neg(),
               Mat3::new::<float>(-1.0, -4.0, -7.0,
                                  -2.0, -5.0, -8.0,
                                  -3.0, -6.0, -9.0));
    assert_eq!(-a, a.neg());

    assert_eq!(a.mul_t(f1),
               Mat3::new::<float>(0.5, 2.0, 3.5,
                                  1.0, 2.5, 4.0,
                                  1.5, 3.0, 4.5));
    assert_eq!(a.mul_v(&v1), Vec3::new::<float>(14.0, 32.0, 50.0));

    assert_eq!(a.add_m(&b),
               Mat3::new::<float>(3.0,  9.0, 15.0,
                                  5.0, 11.0, 17.0,
                                  7.0, 13.0, 19.0));
    assert_eq!(a.sub_m(&b),
               Mat3::new::<float>(-1.0, -1.0, -1.0,
                                  -1.0, -1.0, -1.0,
                                  -1.0, -1.0, -1.0));
    assert_eq!(a.mul_m(&b),
               Mat3::new::<float>(36.0,  81.0, 126.0,
                                  42.0,  96.0, 150.0,
                                  48.0, 111.0, 174.0));
    assert_eq!(a.dot(&b), 330.0);

    assert_eq!(a.transpose(),
               Mat3::new::<float>(1.0, 2.0, 3.0,
                                  4.0, 5.0, 6.0,
                                  7.0, 8.0, 9.0));

    assert!(a.inverse().is_none());

    assert_eq!(Mat3::new::<float>(2.0, 4.0, 6.0,
                                  0.0, 2.0, 4.0,
                                  0.0, 0.0, 1.0).inverse().unwrap(),
               Mat3::new::<float>(0.5, -1.0,  1.0,
                                  0.0,  0.5, -2.0,
                                  0.0,  0.0,  1.0));

    let ident = Mat3::identity::<float>();

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

    let c = Mat3::new::<float>(3.0, 2.0, 1.0,
                               2.0, 3.0, 2.0,
                               1.0, 2.0, 3.0);
    assert!(!c.is_identity());
    assert!(c.is_symmetric());
    assert!(!c.is_diagonal());
    assert!(c.is_rotated());
    assert!(c.is_invertible());

    assert!(Mat3::from_value::<float>(6.0).is_diagonal());

    assert_eq!(a.to_mat4(),
               Mat4::new::<float>(1.0, 4.0, 7.0, 0.0,
                                  2.0, 5.0, 8.0, 0.0,
                                  3.0, 6.0, 9.0, 0.0,
                                  0.0, 0.0, 0.0, 1.0));

    // to_Quaternion
}

fn test_mat3_mut() {
    let a = Mat3 { x: Vec3 { x: 1.0, y: 4.0, z:  7.0 },
                   y: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                   z: Vec3 { x: 3.0, y: 6.0, z:  9.0 } };
    let b = Mat3 { x: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                   y: Vec3 { x: 3.0, y: 6.0, z:  9.0 },
                   z: Vec3 { x: 4.0, y: 7.0, z: 10.0 } };
    let c = Mat3 { x: Vec3 { x: 2.0, y: 4.0, z:  6.0 },
                   y: Vec3 { x: 0.0, y: 2.0, z:  4.0 },
                   z: Vec3 { x: 0.0, y: 0.0, z:  1.0 } };

    let f1 = 0.5;

    let mut mut_a = a;
    let mut mut_c = c;

    mut_a.swap_cols(0, 2);
    assert_eq!(mut_a.col(0), a.col(2));
    assert_eq!(mut_a.col(2), a.col(0));
    mut_a = a;

    mut_a.swap_cols(1, 2);
    assert_eq!(mut_a.col(1), a.col(2));
    assert_eq!(mut_a.col(2), a.col(1));
    mut_a = a;

    mut_a.swap_rows(0, 2);
    assert_eq!(mut_a.row(0), a.row(2));
    assert_eq!(mut_a.row(2), a.row(0));
    mut_a = a;

    mut_a.swap_rows(1, 2);
    assert_eq!(mut_a.row(1), a.row(2));
    assert_eq!(mut_a.row(2), a.row(1));
    mut_a = a;

    mut_a.to_identity();
    assert!(mut_a.is_identity());
    mut_a = a;

    mut_a.to_zero();
    assert_eq!(mut_a, Mat3::zero::<float>());
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
fn test_mat3_approx_eq() {
    assert!(!Mat3::new::<float>(0.000001, 0.000001, 0.000001,
                                0.000001, 0.000001, 0.000001,
                                0.000001, 0.000001, 0.000001)
            .approx_eq(&Mat3::zero::<float>()));
    assert!(Mat3::new::<float>(0.0000001, 0.0000001, 0.0000001,
                                0.0000001, 0.0000001, 0.0000001,
                                0.0000001, 0.0000001, 0.0000001)
            .approx_eq(&Mat3::zero::<float>()));
}

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