// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
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

#![feature(core)]


extern crate cgmath;

use cgmath::*;
use std::f64;

pub mod matrix2 {
    use cgmath::*;

    pub static A: Matrix2<f64> = Matrix2 { x: Vector2 { x: 1.0f64, y: 3.0f64 },
                                           y: Vector2 { x: 2.0f64, y: 4.0f64 } };
    pub static B: Matrix2<f64> = Matrix2 { x: Vector2 { x: 2.0f64, y: 4.0f64 },
                                           y: Vector2 { x: 3.0f64, y: 5.0f64 } };
    pub static C: Matrix2<f64> = Matrix2 { x: Vector2 { x: 2.0f64, y: 1.0f64 },
                                           y: Vector2 { x: 1.0f64, y: 2.0f64 } };

    pub static V: Vector2<f64> = Vector2 { x: 1.0f64, y: 2.0f64 };
    pub static F: f64 = 0.5;
}

pub mod matrix3 {
    use cgmath::*;

    pub static A: Matrix3<f64> = Matrix3 { x: Vector3 { x: 1.0f64, y: 4.0f64, z:  7.0f64 },
                                           y: Vector3 { x: 2.0f64, y: 5.0f64, z:  8.0f64 },
                                           z: Vector3 { x: 3.0f64, y: 6.0f64, z:  9.0f64 } };
    pub static B: Matrix3<f64> = Matrix3 { x: Vector3 { x: 2.0f64, y: 5.0f64, z:  8.0f64 },
                                           y: Vector3 { x: 3.0f64, y: 6.0f64, z:  9.0f64 },
                                           z: Vector3 { x: 4.0f64, y: 7.0f64, z: 10.0f64 } };
    pub static C: Matrix3<f64> = Matrix3 { x: Vector3 { x: 2.0f64, y: 4.0f64, z:  6.0f64 },
                                           y: Vector3 { x: 0.0f64, y: 2.0f64, z:  4.0f64 },
                                           z: Vector3 { x: 0.0f64, y: 0.0f64, z:  1.0f64 } };
    pub static D: Matrix3<f64> = Matrix3 { x: Vector3 { x: 3.0f64, y: 2.0f64, z:  1.0f64 },
                                           y: Vector3 { x: 2.0f64, y: 3.0f64, z:  2.0f64 },
                                           z: Vector3 { x: 1.0f64, y: 2.0f64, z:  3.0f64 } };

    pub static V: Vector3<f64> = Vector3 { x: 1.0f64, y: 2.0f64, z:  3.0f64 };
    pub static F: f64 = 0.5;
}

pub mod matrix4 {
    use cgmath::*;

    pub static A: Matrix4<f64> = Matrix4 { x: Vector4 { x: 1.0f64, y: 5.0f64, z:  9.0f64, w: 13.0f64 },
                                           y: Vector4 { x: 2.0f64, y: 6.0f64, z: 10.0f64, w: 14.0f64 },
                                           z: Vector4 { x: 3.0f64, y: 7.0f64, z: 11.0f64, w: 15.0f64 },
                                           w: Vector4 { x: 4.0f64, y: 8.0f64, z: 12.0f64, w: 16.0f64 } };
    pub static B: Matrix4<f64> = Matrix4 { x: Vector4 { x: 2.0f64, y: 6.0f64, z: 10.0f64, w: 14.0f64 },
                                           y: Vector4 { x: 3.0f64, y: 7.0f64, z: 11.0f64, w: 15.0f64 },
                                           z: Vector4 { x: 4.0f64, y: 8.0f64, z: 12.0f64, w: 16.0f64 },
                                           w: Vector4 { x: 5.0f64, y: 9.0f64, z: 13.0f64, w: 17.0f64 } };
    pub static C: Matrix4<f64> = Matrix4 { x: Vector4 { x: 3.0f64, y: 2.0f64, z:  1.0f64, w:  1.0f64 },
                                           y: Vector4 { x: 2.0f64, y: 3.0f64, z:  2.0f64, w:  2.0f64 },
                                           z: Vector4 { x: 1.0f64, y: 2.0f64, z:  3.0f64, w:  3.0f64 },
                                           w: Vector4 { x: 0.0f64, y: 1.0f64, z:  1.0f64, w:  0.0f64 } };
    pub static D: Matrix4<f64> = Matrix4 { x: Vector4 { x: 4.0f64, y: 3.0f64, z:  2.0f64, w:  1.0f64 },
                                           y: Vector4 { x: 3.0f64, y: 4.0f64, z:  3.0f64, w:  2.0f64 },
                                           z: Vector4 { x: 2.0f64, y: 3.0f64, z:  4.0f64, w:  3.0f64 },
                                           w: Vector4 { x: 1.0f64, y: 2.0f64, z:  3.0f64, w:  4.0f64 } };

    pub static V: Vector4<f64> = Vector4 { x: 1.0f64, y: 2.0f64, z: 3.0f64, w: 4.0f64 };
    pub static F: f64 = 0.5;
}

#[test]
fn test_neg() {
    // Matrix2
    assert_eq!(-matrix2::A,
               Matrix2::new(-1.0f64, -3.0f64,
                            -2.0f64, -4.0f64));
    // Matrix3
    assert_eq!(-matrix3::A,
               Matrix3::new(-1.0f64, -4.0f64, -7.0f64,
                            -2.0f64, -5.0f64, -8.0f64,
                            -3.0f64, -6.0f64, -9.0f64));
    // Matrix4
    assert_eq!(-matrix4::A,
               Matrix4::new(-1.0f64, -5.0f64,  -9.0f64, -13.0f64,
                            -2.0f64, -6.0f64, -10.0f64, -14.0f64,
                            -3.0f64, -7.0f64, -11.0f64, -15.0f64,
                            -4.0f64, -8.0f64, -12.0f64, -16.0f64));
}

#[test]
fn test_mul_s() {
    // Matrix2
    assert_eq!(matrix2::A.mul_s(matrix2::F),
               Matrix2::new(0.5f64, 1.5f64,
                            1.0f64, 2.0f64));
    let mut mut_a = matrix2::A;
    mut_a.mul_self_s(matrix2::F);
    assert_eq!(mut_a, matrix2::A.mul_s(matrix2::F));

    // Matrix3
    assert_eq!(matrix3::A.mul_s(matrix3::F),
               Matrix3::new(0.5f64, 2.0f64, 3.5f64,
                            1.0f64, 2.5f64, 4.0f64,
                            1.5f64, 3.0f64, 4.5f64));
    let mut mut_a = matrix3::A;
    mut_a.mul_self_s(matrix3::F);
    assert_eq!(mut_a, matrix3::A.mul_s(matrix3::F));

    // Matrix4
    assert_eq!(matrix4::A.mul_s(matrix4::F),
               Matrix4::new(0.5f64, 2.5f64, 4.5f64, 6.5f64,
                            1.0f64, 3.0f64, 5.0f64, 7.0f64,
                            1.5f64, 3.5f64, 5.5f64, 7.5f64,
                            2.0f64, 4.0f64, 6.0f64, 8.0f64));
    let mut mut_a = matrix4::A;
    mut_a.mul_self_s(matrix4::F);
    assert_eq!(mut_a, matrix4::A.mul_s(matrix4::F));
}

#[test]
fn test_add_m() {
    // Matrix2
    assert_eq!(matrix2::A.add_m(&matrix2::B),
               Matrix2::new(3.0f64, 7.0f64,
                            5.0f64, 9.0f64));
    let mut mut_a = matrix2::A;
    mut_a.add_self_m(&matrix2::B);
    assert_eq!(mut_a, matrix2::A.add_m(&matrix2::B));
    assert_eq!(mut_a, matrix2::A + matrix2::B);

    // Matrix3
    assert_eq!(matrix3::A.add_m(&matrix3::B),
               Matrix3::new(3.0f64,  9.0f64, 15.0f64,
                            5.0f64, 11.0f64, 17.0f64,
                            7.0f64, 13.0f64, 19.0f64));
    let mut mut_a = matrix3::A;
    mut_a.add_self_m(&matrix3::B);
    assert_eq!(mut_a, matrix3::A.add_m(&matrix3::B));
    assert_eq!(mut_a, matrix3::A + matrix3::B);

    // Matrix4
    assert_eq!(matrix4::A.add_m(&matrix4::B),
               Matrix4::new(3.0f64, 11.0f64, 19.0f64, 27.0f64,
                            5.0f64, 13.0f64, 21.0f64, 29.0f64,
                            7.0f64, 15.0f64, 23.0f64, 31.0f64,
                            9.0f64, 17.0f64, 25.0f64, 33.0f64));
    let mut mut_a = matrix4::A;
    mut_a.add_self_m(&matrix4::B);
    assert_eq!(mut_a, matrix4::A.add_m(&matrix4::B));
    assert_eq!(mut_a, matrix4::A + matrix4::B);
}

#[test]
fn test_sub_m() {
    // Matrix2
    assert_eq!(matrix2::A.sub_m(&matrix2::B),
               Matrix2::new(-1.0f64, -1.0f64,
                            -1.0f64, -1.0f64));
    let mut mut_a = matrix2::A;
    mut_a.sub_self_m(&matrix2::B);
    assert_eq!(mut_a, matrix2::A.sub_m(&matrix2::B));
    assert_eq!(matrix2::A.sub_m(&matrix2::B), matrix2::A - matrix2::B);

    // Matrix3
    assert_eq!(matrix3::A.sub_m(&matrix3::B),
               Matrix3::new(-1.0f64, -1.0f64, -1.0f64,
                            -1.0f64, -1.0f64, -1.0f64,
                            -1.0f64, -1.0f64, -1.0f64));
    let mut mut_a = matrix3::A;
    mut_a.sub_self_m(&matrix3::B);
    assert_eq!(mut_a, matrix3::A.sub_m(&matrix3::B));
    assert_eq!(matrix3::A.sub_m(&matrix3::B), matrix3::A - matrix3::B);

    // Matrix4
    assert_eq!(matrix4::A.sub_m(&matrix4::B),
               Matrix4::new(-1.0f64, -1.0f64, -1.0f64, -1.0f64,
                            -1.0f64, -1.0f64, -1.0f64, -1.0f64,
                            -1.0f64, -1.0f64, -1.0f64, -1.0f64,
                            -1.0f64, -1.0f64, -1.0f64, -1.0f64));
    let mut mut_a = matrix4::A;
    mut_a.sub_self_m(&matrix4::B);
    assert_eq!(mut_a, matrix4::A.sub_m(&matrix4::B));
    assert_eq!(matrix4::A.sub_m(&matrix4::B), matrix4::A - matrix4::B);
}

#[test]
fn test_mul_v() {
    assert_eq!(matrix2::A.mul_v(&matrix2::V), Vector2::new(5.0f64, 11.0f64));
    assert_eq!(matrix3::A.mul_v(&matrix3::V), Vector3::new(14.0f64, 32.0f64, 50.0f64));
    assert_eq!(matrix4::A.mul_v(&matrix4::V), Vector4::new(30.0f64, 70.0f64, 110.0f64, 150.0f64));
}

#[test]
fn test_mul_m() {
    assert_eq!(matrix2::A.mul_m(&matrix2::B),
               Matrix2::new(10.0f64, 22.0f64,
                            13.0f64, 29.0f64));
    assert_eq!(matrix3::A.mul_m(&matrix3::B),
               Matrix3::new(36.0f64,  81.0f64, 126.0f64,
                            42.0f64,  96.0f64, 150.0f64,
                            48.0f64, 111.0f64, 174.0f64));
    assert_eq!(matrix4::A.mul_m(&matrix4::B),
               Matrix4::new(100.0f64, 228.0f64, 356.0f64, 484.0f64,
                            110.0f64, 254.0f64, 398.0f64, 542.0f64,
                            120.0f64, 280.0f64, 440.0f64, 600.0f64,
                            130.0f64, 306.0f64, 482.0f64, 658.0f64));

    assert_eq!(matrix2::A.mul_m(&matrix2::B), matrix2::A * matrix2::B);
    assert_eq!(matrix3::A.mul_m(&matrix3::B), matrix3::A * matrix3::B);
    assert_eq!(matrix4::A.mul_m(&matrix4::B), matrix4::A * matrix4::B);
}

#[test]
fn test_determinant() {
    assert_eq!(matrix2::A.determinant(), -2.0f64);
    assert_eq!(matrix3::A.determinant(), 0.0f64);
    assert_eq!(matrix4::A.determinant(), 0.0f64);
}

#[test]
fn test_trace() {
    assert_eq!(matrix2::A.trace(), 5.0f64);
    assert_eq!(matrix3::A.trace(), 15.0f64);
    assert_eq!(matrix4::A.trace(), 34.0f64);
}

#[test]
fn test_transpose() {
    // Matrix2
    assert_eq!(matrix2::A.transpose(),
               Matrix2::<f64>::new(1.0f64, 2.0f64,
                                   3.0f64, 4.0f64));
    let mut mut_a = matrix2::A;
    mut_a.transpose_self();
    assert_eq!(mut_a, matrix2::A.transpose());

    // Matrix3
    assert_eq!(matrix3::A.transpose(),
               Matrix3::<f64>::new(1.0f64, 2.0f64, 3.0f64,
                                   4.0f64, 5.0f64, 6.0f64,
                                   7.0f64, 8.0f64, 9.0f64));
    let mut mut_a = matrix3::A;
    mut_a.transpose_self();
    assert_eq!(mut_a, matrix3::A.transpose());

    // Matrix4
    assert_eq!(matrix4::A.transpose(),
               Matrix4::<f64>::new( 1.0f64,  2.0f64,  3.0f64,  4.0f64,
                                    5.0f64,  6.0f64,  7.0f64,  8.0f64,
                                    9.0f64, 10.0f64, 11.0f64, 12.0f64,
                                   13.0f64, 14.0f64, 15.0f64, 16.0f64));
    let mut mut_a = matrix4::A;
    mut_a.transpose_self();
    assert_eq!(mut_a, matrix4::A.transpose());
}

#[test]
fn test_invert() {
    // Matrix2
    assert!(Matrix2::<f64>::identity().invert().unwrap().is_identity());

    assert_eq!(matrix2::A.invert().unwrap(),
               Matrix2::new(-2.0f64,  1.5f64,
                             1.0f64, -0.5f64));
    assert!(Matrix2::new(0.0f64, 2.0f64,
                         0.0f64, 5.0f64).invert().is_none());
    let mut mut_a = matrix2::A;
    mut_a.invert_self();
    assert_eq!(mut_a, matrix2::A.invert().unwrap());

    // Matrix3
    assert!(Matrix3::<f64>::identity().invert().unwrap().is_identity());

    assert_eq!(matrix3::A.invert(), None);

    assert_eq!(matrix3::C.invert().unwrap(),
               Matrix3::new(0.5f64, -1.0f64,  1.0f64,
                            0.0f64,  0.5f64, -2.0f64,
                            0.0f64,  0.0f64,  1.0f64));
    let mut mut_c = matrix3::C;
    mut_c.invert_self();
    assert_eq!(mut_c, matrix3::C.invert().unwrap());

    // Matrix4
    assert!(Matrix4::<f64>::identity().invert().unwrap().is_identity());

    assert!(matrix4::C.invert().unwrap().approx_eq(&
            Matrix4::new( 5.0f64, -4.0f64,  1.0f64,  0.0f64,
                         -4.0f64,  8.0f64, -4.0f64,  0.0f64,
                          4.0f64, -8.0f64,  4.0f64,  8.0f64,
                         -3.0f64,  4.0f64,  1.0f64, -8.0f64).mul_s(0.125f64)));
    let mut mut_c = matrix4::C;
    mut_c.invert_self();
    assert_eq!(mut_c, matrix4::C.invert().unwrap());

    let mat_c = Matrix4::new(-0.131917f64, -0.76871f64,   0.625846f64, 0.0f64,
                             -0.,        0.631364f64,  0.775487f64, 0.0f64,
                             -0.991261f64,  0.1023f64,   -0.083287f64, 0.0f64,
                              0.,       -1.262728f64, -1.550973f64, 1.0f64);
    assert!(mat_c.invert().unwrap().mul_m(&mat_c).is_identity());

    let mat_d = Matrix4::new( 0.065455f64, -0.720002f64,  0.690879f64, 0.0f64,
                             -0.,        0.692364f64,  0.721549f64, 0.0f64,
                             -0.997856f64, -0.047229f64,  0.045318f64, 0.0f64,
                              0.,       -1.384727f64, -1.443098f64, 1.0f64);
    assert!(mat_d.invert().unwrap().mul_m(&mat_d).is_identity());

    let mat_e = Matrix4::new( 0.409936f64,  0.683812f64, -0.603617f64, 0.0f64,
                              0.,        0.661778f64,  0.7497f64,   0.0f64,
                              0.912114f64, -0.307329f64,  0.271286f64, 0.0f64,
                             -0.,       -1.323555f64, -1.499401f64, 1.0f64);
    assert!(mat_e.invert().unwrap().mul_m(&mat_e).is_identity());

    let mat_f = Matrix4::new(-0.160691f64, -0.772608f64,  0.614211f64, 0.0f64,
                             -0.,        0.622298f64,  0.78278f64,  0.0f64,
                             -0.987005f64,  0.125786f64, -0.099998f64, 0.0f64,
                              0.,       -1.244597f64, -1.565561f64, 1.0f64);
    assert!(mat_f.invert().unwrap().mul_m(&mat_f).is_identity());
}

#[test]
fn test_from_translation() {
    let mat = Matrix4::from_translation(&Vector3::new(1.0f64, 2.0f64, 3.0f64));
    let vertex = Vector4::new(0.0f64, 0.0f64, 0.0f64, 1.0f64);
    let res = mat.mul_v(&vertex);
    assert_eq!(res, Vector4::new(1., 2., 3., 1.));
}

#[test]
fn test_predicates() {
    // Matrix2

    assert!(Matrix2::<f64>::identity().is_identity());
    assert!(Matrix2::<f64>::identity().is_symmetric());
    assert!(Matrix2::<f64>::identity().is_diagonal());
    assert!(Matrix2::<f64>::identity().is_invertible());

    assert!(!matrix2::A.is_identity());
    assert!(!matrix2::A.is_symmetric());
    assert!(!matrix2::A.is_diagonal());
    assert!(matrix2::A.is_invertible());

    assert!(!matrix2::C.is_identity());
    assert!(matrix2::C.is_symmetric());
    assert!(!matrix2::C.is_diagonal());
    assert!(matrix2::C.is_invertible());

    assert!(Matrix2::from_value(6.0f64).is_diagonal());

    // Matrix3

    assert!(Matrix3::<f64>::identity().is_identity());
    assert!(Matrix3::<f64>::identity().is_symmetric());
    assert!(Matrix3::<f64>::identity().is_diagonal());
    assert!(Matrix3::<f64>::identity().is_invertible());

    assert!(!matrix3::A.is_identity());
    assert!(!matrix3::A.is_symmetric());
    assert!(!matrix3::A.is_diagonal());
    assert!(!matrix3::A.is_invertible());

    assert!(!matrix3::D.is_identity());
    assert!(matrix3::D.is_symmetric());
    assert!(!matrix3::D.is_diagonal());
    assert!(matrix3::D.is_invertible());

    assert!(Matrix3::from_value(6.0f64).is_diagonal());

    // Matrix4

    assert!(Matrix4::<f64>::identity().is_identity());
    assert!(Matrix4::<f64>::identity().is_symmetric());
    assert!(Matrix4::<f64>::identity().is_diagonal());
    assert!(Matrix4::<f64>::identity().is_invertible());

    assert!(!matrix4::A.is_identity());
    assert!(!matrix4::A.is_symmetric());
    assert!(!matrix4::A.is_diagonal());
    assert!(!matrix4::A.is_invertible());

    assert!(!matrix4::D.is_identity());
    assert!(matrix4::D.is_symmetric());
    assert!(!matrix4::D.is_diagonal());
    assert!(matrix4::D.is_invertible());

    assert!(Matrix4::from_value(6.0f64).is_diagonal());
}

#[test]
fn test_from_angle() {
    // Rotate the vector (1, 0) by π/2 radians to the vector (0, 1)
    let rot1 = Matrix2::from_angle(rad(0.5f64 * f64::consts::PI));
    assert!(rot1.mul_v(&Vector2::unit_x()).approx_eq(&Vector2::unit_y()));

    // Rotate the vector (-1, 0) by -π/2 radians to the vector (0, 1)
    let rot2 = -rot1;
    assert!(rot2.mul_v(&-Vector2::unit_x()).approx_eq(&Vector2::unit_y()));

    // Rotate the vector (1, 1) by π radians to the vector (-1, -1)
    let rot3: Matrix2<f64> = Matrix2::from_angle(rad(f64::consts::PI));
    assert!(rot3.mul_v(&Vector2::new(1.0, 1.0)).approx_eq(&Vector2::new(-1.0, -1.0)));
}
