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
type float = f64;

pub mod mat2 {
    use cgmath::matrix::*;
    use cgmath::vector::*;
    type float = f64;

    pub static A: Mat2<f64> = Mat2 { x: Vec2 { x: 1.0, y: 3.0 },
                                     y: Vec2 { x: 2.0, y: 4.0 } };
    pub static B: Mat2<f64> = Mat2 { x: Vec2 { x: 2.0, y: 4.0 },
                                     y: Vec2 { x: 3.0, y: 5.0 } };
    pub static C: Mat2<f64> = Mat2 { x: Vec2 { x: 2.0, y: 1.0 },
                                     y: Vec2 { x: 1.0, y: 2.0 } };

    pub static V: Vec2<f64> = Vec2 { x: 1.0, y: 2.0 };
    pub static F: f64 = 0.5;
}

pub mod mat3 {
    use cgmath::matrix::*;
    use cgmath::vector::*;
    type float = f64;

    pub static A: Mat3<f64> = Mat3 { x: Vec3 { x: 1.0, y: 4.0, z:  7.0 },
                                     y: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                                     z: Vec3 { x: 3.0, y: 6.0, z:  9.0 } };
    pub static B: Mat3<f64> = Mat3 { x: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                                     y: Vec3 { x: 3.0, y: 6.0, z:  9.0 },
                                     z: Vec3 { x: 4.0, y: 7.0, z: 10.0 } };
    pub static C: Mat3<f64> = Mat3 { x: Vec3 { x: 2.0, y: 4.0, z:  6.0 },
                                     y: Vec3 { x: 0.0, y: 2.0, z:  4.0 },
                                     z: Vec3 { x: 0.0, y: 0.0, z:  1.0 } };
    pub static D: Mat3<f64> = Mat3 { x: Vec3 { x: 3.0, y: 2.0, z:  1.0 },
                                     y: Vec3 { x: 2.0, y: 3.0, z:  2.0 },
                                     z: Vec3 { x: 1.0, y: 2.0, z:  3.0 } };

    pub static V: Vec3<f64> = Vec3 { x: 1.0, y: 2.0, z:  3.0 };
    pub static F: f64 = 0.5;
}

pub mod mat4 {
    use cgmath::matrix::*;
    use cgmath::vector::*;
    type float = f64;

    pub static A: Mat4<f64> = Mat4 { x: Vec4 { x: 1.0, y: 5.0, z:  9.0, w: 13.0 },
                                     y: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                                     z: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                                     w: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 } };
    pub static B: Mat4<f64> = Mat4 { x: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                                     y: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                                     z: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 },
                                     w: Vec4 { x: 5.0, y: 9.0, z: 13.0, w: 17.0 } };
    pub static C: Mat4<f64> = Mat4 { x: Vec4 { x: 3.0, y: 2.0, z:  1.0, w:  1.0 },
                                     y: Vec4 { x: 2.0, y: 3.0, z:  2.0, w:  2.0 },
                                     z: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  3.0 },
                                     w: Vec4 { x: 0.0, y: 1.0, z:  1.0, w:  0.0 } };
    pub static D: Mat4<f64> = Mat4 { x: Vec4 { x: 4.0, y: 3.0, z:  2.0, w:  1.0 },
                                     y: Vec4 { x: 3.0, y: 4.0, z:  3.0, w:  2.0 },
                                     z: Vec4 { x: 2.0, y: 3.0, z:  4.0, w:  3.0 },
                                     w: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  4.0 } };

    pub static V: Vec4<f64> = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    pub static F: f64 = 0.5;
}

#[test]
fn test_neg() {
    // Mat2
    assert_eq!(-mat2::A,
               Mat2::new(-1.0, -3.0,
                         -2.0, -4.0));
    // Mat3
    assert_eq!(-mat3::A,
               Mat3::new(-1.0, -4.0, -7.0,
                         -2.0, -5.0, -8.0,
                         -3.0, -6.0, -9.0));
    // Mat4
    assert_eq!(-mat4::A,
               Mat4::new(-1.0, -5.0,  -9.0, -13.0,
                         -2.0, -6.0, -10.0, -14.0,
                         -3.0, -7.0, -11.0, -15.0,
                         -4.0, -8.0, -12.0, -16.0));
}

#[test]
fn test_mul_s() {
    // Mat2
    assert_eq!(mat2::A.mul_s(mat2::F),
               Mat2::new(0.5, 1.5,
                         1.0, 2.0));
    let mut mut_a = mat2::A;
    mut_a.mul_self_s(mat2::F);
    assert_eq!(mut_a, mat2::A.mul_s(mat2::F));

    // Mat3
    assert_eq!(mat3::A.mul_s(mat3::F),
               Mat3::new(0.5, 2.0, 3.5,
                         1.0, 2.5, 4.0,
                         1.5, 3.0, 4.5));
    let mut mut_a = mat3::A;
    mut_a.mul_self_s(mat3::F);
    assert_eq!(mut_a, mat3::A.mul_s(mat3::F));

    // Mat4
    assert_eq!(mat4::A.mul_s(mat4::F),
               Mat4::new(0.5, 2.5, 4.5, 6.5,
                         1.0, 3.0, 5.0, 7.0,
                         1.5, 3.5, 5.5, 7.5,
                         2.0, 4.0, 6.0, 8.0));
    let mut mut_a = mat4::A;
    mut_a.mul_self_s(mat4::F);
    assert_eq!(mut_a, mat4::A.mul_s(mat4::F));
}

#[test]
fn test_add_m() {
    // Mat2
    assert_eq!(mat2::A.add_m(&mat2::B),
               Mat2::new(3.0, 7.0,
                         5.0, 9.0));
    let mut mut_a = mat2::A;
    mut_a.add_self_m(&mat2::B);
    assert_eq!(mut_a, mat2::A.add_m(&mat2::B));

    // Mat3
    assert_eq!(mat3::A.add_m(&mat3::B),
               Mat3::new(3.0,  9.0, 15.0,
                         5.0, 11.0, 17.0,
                         7.0, 13.0, 19.0));
    let mut mut_a = mat3::A;
    mut_a.add_self_m(&mat3::B);
    assert_eq!(mut_a, mat3::A.add_m(&mat3::B));

    // Mat4
    assert_eq!(mat4::A.add_m(&mat4::B),
               Mat4::new(3.0, 11.0, 19.0, 27.0,
                         5.0, 13.0, 21.0, 29.0,
                         7.0, 15.0, 23.0, 31.0,
                         9.0, 17.0, 25.0, 33.0));
    let mut mut_a = mat4::A;
    mut_a.add_self_m(&mat4::B);
    assert_eq!(mut_a, mat4::A.add_m(&mat4::B));
}

#[test]
fn test_sub_m() {
    // Mat2
    assert_eq!(mat2::A.sub_m(&mat2::B),
               Mat2::new(-1.0, -1.0,
                         -1.0, -1.0));
    let mut mut_a = mat2::A;
    mut_a.sub_self_m(&mat2::B);
    assert_eq!(mut_a, mat2::A.sub_m(&mat2::B));

    // Mat3
    assert_eq!(mat3::A.sub_m(&mat3::B),
               Mat3::new(-1.0, -1.0, -1.0,
                         -1.0, -1.0, -1.0,
                         -1.0, -1.0, -1.0));
    let mut mut_a = mat3::A;
    mut_a.sub_self_m(&mat3::B);
    assert_eq!(mut_a, mat3::A.sub_m(&mat3::B));

    // Mat4
    assert_eq!(mat4::A.sub_m(&mat4::B),
               Mat4::new(-1.0, -1.0, -1.0, -1.0,
                         -1.0, -1.0, -1.0, -1.0,
                         -1.0, -1.0, -1.0, -1.0,
                         -1.0, -1.0, -1.0, -1.0));
    let mut mut_a = mat4::A;
    mut_a.sub_self_m(&mat4::B);
    assert_eq!(mut_a, mat4::A.sub_m(&mat4::B));
}

#[test]
fn test_mul_v() {
    assert_eq!(mat2::A.mul_v(&mat2::V), Vec2::new(5.0, 11.0));
    assert_eq!(mat3::A.mul_v(&mat3::V), Vec3::new(14.0, 32.0, 50.0));
    assert_eq!(mat4::A.mul_v(&mat4::V), Vec4::new(30.0, 70.0, 110.0, 150.0));
}

#[test]
fn test_mul_m() {
    assert_eq!(mat2::A.mul_m(&mat2::B),
               Mat2::new(10.0, 22.0,
                         13.0, 29.0));
    assert_eq!(mat3::A.mul_m(&mat3::B),
               Mat3::new(36.0,  81.0, 126.0,
                         42.0,  96.0, 150.0,
                         48.0, 111.0, 174.0));
    assert_eq!(mat4::A.mul_m(&mat4::B),
               Mat4::new(100.0, 228.0, 356.0, 484.0,
                         110.0, 254.0, 398.0, 542.0,
                         120.0, 280.0, 440.0, 600.0,
                         130.0, 306.0, 482.0, 658.0));
}

#[test]
fn test_determinant() {
    assert_eq!(mat2::A.determinant(), -2.0);
    assert_eq!(mat3::A.determinant(), 0.0);
    assert_eq!(mat4::A.determinant(), 0.0);
}

#[test]
fn test_trace() {
    assert_eq!(mat2::A.trace(), 5.0);
    assert_eq!(mat3::A.trace(), 15.0);
    assert_eq!(mat4::A.trace(), 34.0);
}

#[test]
fn test_transpose() {
    // Mat2
    assert_eq!(mat2::A.transpose(),
               Mat2::<f64>::new(1.0, 2.0,
                                  3.0, 4.0));
    let mut mut_a = mat2::A;
    mut_a.transpose_self();
    assert_eq!(mut_a, mat2::A.transpose());

    // Mat3
    assert_eq!(mat3::A.transpose(),
               Mat3::<f64>::new(1.0, 2.0, 3.0,
                                  4.0, 5.0, 6.0,
                                  7.0, 8.0, 9.0));
    let mut mut_a = mat3::A;
    mut_a.transpose_self();
    assert_eq!(mut_a, mat3::A.transpose());

    // Mat4
    assert_eq!(mat4::A.transpose(),
               Mat4::<f64>::new( 1.0,  2.0,  3.0,  4.0,
                                   5.0,  6.0,  7.0,  8.0,
                                   9.0, 10.0, 11.0, 12.0,
                                  13.0, 14.0, 15.0, 16.0));
    let mut mut_a = mat4::A;
    mut_a.transpose_self();
    assert_eq!(mut_a, mat4::A.transpose());
}

#[test]
fn test_invert() {
    // Mat2
    assert!(Mat2::<f64>::identity().invert().unwrap().is_identity());

    assert_eq!(mat2::A.invert().unwrap(),
               Mat2::new(-2.0,  1.5,
                          1.0, -0.5));
    assert!(Mat2::new(0.0, 2.0,
                      0.0, 5.0).invert().is_none());
    let mut mut_a = mat2::A;
    mut_a.invert_self();
    assert_eq!(mut_a, mat2::A.invert().unwrap());

    // Mat3
    assert!(Mat3::<f64>::identity().invert().unwrap().is_identity());

    assert_eq!(mat3::A.invert(), None);

    assert_eq!(mat3::C.invert().unwrap(),
               Mat3::new(0.5, -1.0,  1.0,
                         0.0,  0.5, -2.0,
                         0.0,  0.0,  1.0));
    let mut mut_c = mat3::C;
    mut_c.invert_self();
    assert_eq!(mut_c, mat3::C.invert().unwrap());

    // Mat4
    assert!(Mat4::<f64>::identity().invert().unwrap().is_identity());

    assert!(mat4::C.invert().unwrap().approx_eq(&
            Mat4::new( 5.0, -4.0,  1.0,  0.0,
                      -4.0,  8.0, -4.0,  0.0,
                       4.0, -8.0,  4.0,  8.0,
                      -3.0,  4.0,  1.0, -8.0).mul_s(0.125)));
    let mut mut_c = mat4::C;
    mut_c.invert_self();
    assert_eq!(mut_c, mat4::C.invert().unwrap());
}

#[test]
fn test_predicates() {
    // Mat2

    assert!(Mat2::<f64>::identity().is_identity());
    assert!(Mat2::<f64>::identity().is_symmetric());
    assert!(Mat2::<f64>::identity().is_diagonal());
    assert!(!Mat2::<f64>::identity().is_rotated());
    assert!(Mat2::<f64>::identity().is_invertible());

    assert!(!mat2::A.is_identity());
    assert!(!mat2::A.is_symmetric());
    assert!(!mat2::A.is_diagonal());
    assert!(mat2::A.is_rotated());
    assert!(mat2::A.is_invertible());

    assert!(!mat2::C.is_identity());
    assert!(mat2::C.is_symmetric());
    assert!(!mat2::C.is_diagonal());
    assert!(mat2::C.is_rotated());
    assert!(mat2::C.is_invertible());

    assert!(Mat2::from_value(6.0).is_diagonal());

    // Mat3

    assert!(Mat3::<f64>::identity().is_identity());
    assert!(Mat3::<f64>::identity().is_symmetric());
    assert!(Mat3::<f64>::identity().is_diagonal());
    assert!(!Mat3::<f64>::identity().is_rotated());
    assert!(Mat3::<f64>::identity().is_invertible());

    assert!(!mat3::A.is_identity());
    assert!(!mat3::A.is_symmetric());
    assert!(!mat3::A.is_diagonal());
    assert!(mat3::A.is_rotated());
    assert!(!mat3::A.is_invertible());

    assert!(!mat3::D.is_identity());
    assert!(mat3::D.is_symmetric());
    assert!(!mat3::D.is_diagonal());
    assert!(mat3::D.is_rotated());
    assert!(mat3::D.is_invertible());

    assert!(Mat3::from_value(6.0).is_diagonal());

    // Mat4

    assert!(Mat4::<f64>::identity().is_identity());
    assert!(Mat4::<f64>::identity().is_symmetric());
    assert!(Mat4::<f64>::identity().is_diagonal());
    assert!(!Mat4::<f64>::identity().is_rotated());
    assert!(Mat4::<f64>::identity().is_invertible());

    assert!(!mat4::A.is_identity());
    assert!(!mat4::A.is_symmetric());
    assert!(!mat4::A.is_diagonal());
    assert!(mat4::A.is_rotated());
    assert!(!mat4::A.is_invertible());

    assert!(!mat4::D.is_identity());
    assert!(mat4::D.is_symmetric());
    assert!(!mat4::D.is_diagonal());
    assert!(mat4::D.is_rotated());
    assert!(mat4::D.is_invertible());

    assert!(Mat4::from_value(6.0).is_diagonal());
}
