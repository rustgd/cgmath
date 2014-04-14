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

use cgmath::matrix::*;
use cgmath::vector::*;
use cgmath::approx::ApproxEq;

pub mod matrix2 {
    use cgmath::matrix::*;
    use cgmath::vector::*;

    pub static A: Matrix2<f64> = Matrix2 { x: Vector2 { x: 1.0, y: 3.0 },
                                     y: Vector2 { x: 2.0, y: 4.0 } };
    pub static B: Matrix2<f64> = Matrix2 { x: Vector2 { x: 2.0, y: 4.0 },
                                     y: Vector2 { x: 3.0, y: 5.0 } };
    pub static C: Matrix2<f64> = Matrix2 { x: Vector2 { x: 2.0, y: 1.0 },
                                     y: Vector2 { x: 1.0, y: 2.0 } };

    pub static V: Vector2<f64> = Vector2 { x: 1.0, y: 2.0 };
    pub static F: f64 = 0.5;
}

pub mod matrix3 {
    use cgmath::matrix::*;
    use cgmath::vector::*;

    pub static A: Matrix3<f64> = Matrix3 { x: Vector3 { x: 1.0, y: 4.0, z:  7.0 },
                                     y: Vector3 { x: 2.0, y: 5.0, z:  8.0 },
                                     z: Vector3 { x: 3.0, y: 6.0, z:  9.0 } };
    pub static B: Matrix3<f64> = Matrix3 { x: Vector3 { x: 2.0, y: 5.0, z:  8.0 },
                                     y: Vector3 { x: 3.0, y: 6.0, z:  9.0 },
                                     z: Vector3 { x: 4.0, y: 7.0, z: 10.0 } };
    pub static C: Matrix3<f64> = Matrix3 { x: Vector3 { x: 2.0, y: 4.0, z:  6.0 },
                                     y: Vector3 { x: 0.0, y: 2.0, z:  4.0 },
                                     z: Vector3 { x: 0.0, y: 0.0, z:  1.0 } };
    pub static D: Matrix3<f64> = Matrix3 { x: Vector3 { x: 3.0, y: 2.0, z:  1.0 },
                                     y: Vector3 { x: 2.0, y: 3.0, z:  2.0 },
                                     z: Vector3 { x: 1.0, y: 2.0, z:  3.0 } };

    pub static V: Vector3<f64> = Vector3 { x: 1.0, y: 2.0, z:  3.0 };
    pub static F: f64 = 0.5;
}

pub mod matrix4 {
    use cgmath::matrix::*;
    use cgmath::vector::*;

    pub static A: Matrix4<f64> = Matrix4 { x: Vector4 { x: 1.0, y: 5.0, z:  9.0, w: 13.0 },
                                     y: Vector4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                                     z: Vector4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                                     w: Vector4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 } };
    pub static B: Matrix4<f64> = Matrix4 { x: Vector4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                                     y: Vector4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                                     z: Vector4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 },
                                     w: Vector4 { x: 5.0, y: 9.0, z: 13.0, w: 17.0 } };
    pub static C: Matrix4<f64> = Matrix4 { x: Vector4 { x: 3.0, y: 2.0, z:  1.0, w:  1.0 },
                                     y: Vector4 { x: 2.0, y: 3.0, z:  2.0, w:  2.0 },
                                     z: Vector4 { x: 1.0, y: 2.0, z:  3.0, w:  3.0 },
                                     w: Vector4 { x: 0.0, y: 1.0, z:  1.0, w:  0.0 } };
    pub static D: Matrix4<f64> = Matrix4 { x: Vector4 { x: 4.0, y: 3.0, z:  2.0, w:  1.0 },
                                     y: Vector4 { x: 3.0, y: 4.0, z:  3.0, w:  2.0 },
                                     z: Vector4 { x: 2.0, y: 3.0, z:  4.0, w:  3.0 },
                                     w: Vector4 { x: 1.0, y: 2.0, z:  3.0, w:  4.0 } };

    pub static V: Vector4<f64> = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    pub static F: f64 = 0.5;
}

#[test]
fn test_neg() {
    // Matrix2
    assert_eq!(-matrix2::A,
               Matrix2::new(-1.0, -3.0,
                         -2.0, -4.0));
    // Matrix3
    assert_eq!(-matrix3::A,
               Matrix3::new(-1.0, -4.0, -7.0,
                         -2.0, -5.0, -8.0,
                         -3.0, -6.0, -9.0));
    // Matrix4
    assert_eq!(-matrix4::A,
               Matrix4::new(-1.0, -5.0,  -9.0, -13.0,
                         -2.0, -6.0, -10.0, -14.0,
                         -3.0, -7.0, -11.0, -15.0,
                         -4.0, -8.0, -12.0, -16.0));
}

#[test]
fn test_mul_s() {
    // Matrix2
    assert_eq!(matrix2::A.mul_s(matrix2::F),
               Matrix2::new(0.5, 1.5,
                         1.0, 2.0));
    let mut mut_a = matrix2::A;
    mut_a.mul_self_s(matrix2::F);
    assert_eq!(mut_a, matrix2::A.mul_s(matrix2::F));

    // Matrix3
    assert_eq!(matrix3::A.mul_s(matrix3::F),
               Matrix3::new(0.5, 2.0, 3.5,
                         1.0, 2.5, 4.0,
                         1.5, 3.0, 4.5));
    let mut mut_a = matrix3::A;
    mut_a.mul_self_s(matrix3::F);
    assert_eq!(mut_a, matrix3::A.mul_s(matrix3::F));

    // Matrix4
    assert_eq!(matrix4::A.mul_s(matrix4::F),
               Matrix4::new(0.5, 2.5, 4.5, 6.5,
                         1.0, 3.0, 5.0, 7.0,
                         1.5, 3.5, 5.5, 7.5,
                         2.0, 4.0, 6.0, 8.0));
    let mut mut_a = matrix4::A;
    mut_a.mul_self_s(matrix4::F);
    assert_eq!(mut_a, matrix4::A.mul_s(matrix4::F));
}

#[test]
fn test_add_m() {
    // Matrix2
    assert_eq!(matrix2::A.add_m(&matrix2::B),
               Matrix2::new(3.0, 7.0,
                         5.0, 9.0));
    let mut mut_a = matrix2::A;
    mut_a.add_self_m(&matrix2::B);
    assert_eq!(mut_a, matrix2::A.add_m(&matrix2::B));

    // Matrix3
    assert_eq!(matrix3::A.add_m(&matrix3::B),
               Matrix3::new(3.0,  9.0, 15.0,
                         5.0, 11.0, 17.0,
                         7.0, 13.0, 19.0));
    let mut mut_a = matrix3::A;
    mut_a.add_self_m(&matrix3::B);
    assert_eq!(mut_a, matrix3::A.add_m(&matrix3::B));

    // Matrix4
    assert_eq!(matrix4::A.add_m(&matrix4::B),
               Matrix4::new(3.0, 11.0, 19.0, 27.0,
                         5.0, 13.0, 21.0, 29.0,
                         7.0, 15.0, 23.0, 31.0,
                         9.0, 17.0, 25.0, 33.0));
    let mut mut_a = matrix4::A;
    mut_a.add_self_m(&matrix4::B);
    assert_eq!(mut_a, matrix4::A.add_m(&matrix4::B));
}

#[test]
fn test_sub_m() {
    // Matrix2
    assert_eq!(matrix2::A.sub_m(&matrix2::B),
               Matrix2::new(-1.0, -1.0,
                         -1.0, -1.0));
    let mut mut_a = matrix2::A;
    mut_a.sub_self_m(&matrix2::B);
    assert_eq!(mut_a, matrix2::A.sub_m(&matrix2::B));

    // Matrix3
    assert_eq!(matrix3::A.sub_m(&matrix3::B),
               Matrix3::new(-1.0, -1.0, -1.0,
                         -1.0, -1.0, -1.0,
                         -1.0, -1.0, -1.0));
    let mut mut_a = matrix3::A;
    mut_a.sub_self_m(&matrix3::B);
    assert_eq!(mut_a, matrix3::A.sub_m(&matrix3::B));

    // Matrix4
    assert_eq!(matrix4::A.sub_m(&matrix4::B),
               Matrix4::new(-1.0, -1.0, -1.0, -1.0,
                         -1.0, -1.0, -1.0, -1.0,
                         -1.0, -1.0, -1.0, -1.0,
                         -1.0, -1.0, -1.0, -1.0));
    let mut mut_a = matrix4::A;
    mut_a.sub_self_m(&matrix4::B);
    assert_eq!(mut_a, matrix4::A.sub_m(&matrix4::B));
}

#[test]
fn test_mul_v() {
    assert_eq!(matrix2::A.mul_v(&matrix2::V), Vector2::new(5.0, 11.0));
    assert_eq!(matrix3::A.mul_v(&matrix3::V), Vector3::new(14.0, 32.0, 50.0));
    assert_eq!(matrix4::A.mul_v(&matrix4::V), Vector4::new(30.0, 70.0, 110.0, 150.0));
}

#[test]
fn test_mul_m() {
    assert_eq!(matrix2::A.mul_m(&matrix2::B),
               Matrix2::new(10.0, 22.0,
                         13.0, 29.0));
    assert_eq!(matrix3::A.mul_m(&matrix3::B),
               Matrix3::new(36.0,  81.0, 126.0,
                         42.0,  96.0, 150.0,
                         48.0, 111.0, 174.0));
    assert_eq!(matrix4::A.mul_m(&matrix4::B),
               Matrix4::new(100.0, 228.0, 356.0, 484.0,
                         110.0, 254.0, 398.0, 542.0,
                         120.0, 280.0, 440.0, 600.0,
                         130.0, 306.0, 482.0, 658.0));
}

#[test]
fn test_determinant() {
    assert_eq!(matrix2::A.determinant(), -2.0);
    assert_eq!(matrix3::A.determinant(), 0.0);
    assert_eq!(matrix4::A.determinant(), 0.0);
}

#[test]
fn test_trace() {
    assert_eq!(matrix2::A.trace(), 5.0);
    assert_eq!(matrix3::A.trace(), 15.0);
    assert_eq!(matrix4::A.trace(), 34.0);
}

#[test]
fn test_transpose() {
    // Matrix2
    assert_eq!(matrix2::A.transpose(),
               Matrix2::<f64>::new(1.0, 2.0,
                                  3.0, 4.0));
    let mut mut_a = matrix2::A;
    mut_a.transpose_self();
    assert_eq!(mut_a, matrix2::A.transpose());

    // Matrix3
    assert_eq!(matrix3::A.transpose(),
               Matrix3::<f64>::new(1.0, 2.0, 3.0,
                                  4.0, 5.0, 6.0,
                                  7.0, 8.0, 9.0));
    let mut mut_a = matrix3::A;
    mut_a.transpose_self();
    assert_eq!(mut_a, matrix3::A.transpose());

    // Matrix4
    assert_eq!(matrix4::A.transpose(),
               Matrix4::<f64>::new( 1.0,  2.0,  3.0,  4.0,
                                   5.0,  6.0,  7.0,  8.0,
                                   9.0, 10.0, 11.0, 12.0,
                                  13.0, 14.0, 15.0, 16.0));
    let mut mut_a = matrix4::A;
    mut_a.transpose_self();
    assert_eq!(mut_a, matrix4::A.transpose());
}

#[test]
fn test_invert() {
    // Matrix2
    assert!(Matrix2::<f64>::identity().invert().unwrap().is_identity());

    assert_eq!(matrix2::A.invert().unwrap(),
               Matrix2::new(-2.0,  1.5,
                          1.0, -0.5));
    assert!(Matrix2::new(0.0, 2.0,
                      0.0, 5.0).invert().is_none());
    let mut mut_a = matrix2::A;
    mut_a.invert_self();
    assert_eq!(mut_a, matrix2::A.invert().unwrap());

    // Matrix3
    assert!(Matrix3::<f64>::identity().invert().unwrap().is_identity());

    assert_eq!(matrix3::A.invert(), None);

    assert_eq!(matrix3::C.invert().unwrap(),
               Matrix3::new(0.5, -1.0,  1.0,
                         0.0,  0.5, -2.0,
                         0.0,  0.0,  1.0));
    let mut mut_c = matrix3::C;
    mut_c.invert_self();
    assert_eq!(mut_c, matrix3::C.invert().unwrap());

    // Matrix4
    assert!(Matrix4::<f64>::identity().invert().unwrap().is_identity());

    assert!(matrix4::C.invert().unwrap().approx_eq(&
            Matrix4::new( 5.0, -4.0,  1.0,  0.0,
                      -4.0,  8.0, -4.0,  0.0,
                       4.0, -8.0,  4.0,  8.0,
                      -3.0,  4.0,  1.0, -8.0).mul_s(0.125)));
    let mut mut_c = matrix4::C;
    mut_c.invert_self();
    assert_eq!(mut_c, matrix4::C.invert().unwrap());
}

#[test]
fn test_predicates() {
    // Matrix2

    assert!(Matrix2::<f64>::identity().is_identity());
    assert!(Matrix2::<f64>::identity().is_symmetric());
    assert!(Matrix2::<f64>::identity().is_diagonal());
    assert!(!Matrix2::<f64>::identity().is_rotated());
    assert!(Matrix2::<f64>::identity().is_invertible());

    assert!(!matrix2::A.is_identity());
    assert!(!matrix2::A.is_symmetric());
    assert!(!matrix2::A.is_diagonal());
    assert!(matrix2::A.is_rotated());
    assert!(matrix2::A.is_invertible());

    assert!(!matrix2::C.is_identity());
    assert!(matrix2::C.is_symmetric());
    assert!(!matrix2::C.is_diagonal());
    assert!(matrix2::C.is_rotated());
    assert!(matrix2::C.is_invertible());

    assert!(Matrix2::from_value(6.0).is_diagonal());

    // Matrix3

    assert!(Matrix3::<f64>::identity().is_identity());
    assert!(Matrix3::<f64>::identity().is_symmetric());
    assert!(Matrix3::<f64>::identity().is_diagonal());
    assert!(!Matrix3::<f64>::identity().is_rotated());
    assert!(Matrix3::<f64>::identity().is_invertible());

    assert!(!matrix3::A.is_identity());
    assert!(!matrix3::A.is_symmetric());
    assert!(!matrix3::A.is_diagonal());
    assert!(matrix3::A.is_rotated());
    assert!(!matrix3::A.is_invertible());

    assert!(!matrix3::D.is_identity());
    assert!(matrix3::D.is_symmetric());
    assert!(!matrix3::D.is_diagonal());
    assert!(matrix3::D.is_rotated());
    assert!(matrix3::D.is_invertible());

    assert!(Matrix3::from_value(6.0).is_diagonal());

    // Matrix4

    assert!(Matrix4::<f64>::identity().is_identity());
    assert!(Matrix4::<f64>::identity().is_symmetric());
    assert!(Matrix4::<f64>::identity().is_diagonal());
    assert!(!Matrix4::<f64>::identity().is_rotated());
    assert!(Matrix4::<f64>::identity().is_invertible());

    assert!(!matrix4::A.is_identity());
    assert!(!matrix4::A.is_symmetric());
    assert!(!matrix4::A.is_diagonal());
    assert!(matrix4::A.is_rotated());
    assert!(!matrix4::A.is_invertible());

    assert!(!matrix4::D.is_identity());
    assert!(matrix4::D.is_symmetric());
    assert!(!matrix4::D.is_diagonal());
    assert!(matrix4::D.is_rotated());
    assert!(matrix4::D.is_invertible());

    assert!(Matrix4::from_value(6.0).is_diagonal());
}
