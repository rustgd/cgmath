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

use extra;

type float = f32;

pub mod mat2 {
    use cgmath::matrix::*;
    use cgmath::vector::*;
    type float = f32;

    pub static A: Mat2<f32> = Mat2 { x: Vec2 { x: 1.0, y: 3.0 },
                                       y: Vec2 { x: 2.0, y: 4.0 } };
    pub static B: Mat2<f32> = Mat2 { x: Vec2 { x: 2.0, y: 4.0 },
                                       y: Vec2 { x: 3.0, y: 5.0 } };
    pub static C: Mat2<f32> = Mat2 { x: Vec2 { x: 2.0, y: 1.0 },
                                       y: Vec2 { x: 1.0, y: 2.0 } };

    pub static V: Vec2<f32> = Vec2 { x: 1.0, y: 2.0 };
    pub static F: f32 = 0.5;
}

pub mod mat3 {
    use cgmath::matrix::*;
    use cgmath::vector::*;
    type float = f32;

    pub static A: Mat3<f32> = Mat3 { x: Vec3 { x: 1.0, y: 4.0, z:  7.0 },
                                       y: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                                       z: Vec3 { x: 3.0, y: 6.0, z:  9.0 } };
    pub static B: Mat3<f32> = Mat3 { x: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                                       y: Vec3 { x: 3.0, y: 6.0, z:  9.0 },
                                       z: Vec3 { x: 4.0, y: 7.0, z: 10.0 } };
    pub static C: Mat3<f32> = Mat3 { x: Vec3 { x: 2.0, y: 4.0, z:  6.0 },
                                       y: Vec3 { x: 0.0, y: 2.0, z:  4.0 },
                                       z: Vec3 { x: 0.0, y: 0.0, z:  1.0 } };
    pub static D: Mat3<f32> = Mat3 { x: Vec3 { x: 3.0, y: 2.0, z:  1.0 },
                                       y: Vec3 { x: 2.0, y: 3.0, z:  2.0 },
                                       z: Vec3 { x: 1.0, y: 2.0, z:  3.0 } };

    pub static V: Vec3<f32> = Vec3 { x: 1.0, y: 2.0, z:  3.0 };
    pub static F: f32 = 0.5;
}

pub mod mat4 {
    use cgmath::matrix::*;
    use cgmath::vector::*;
    type float = f32;

    pub static A: Mat4<f32> = Mat4 { x: Vec4 { x: 1.0, y: 5.0, z:  9.0, w: 13.0 },
                                       y: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                                       z: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                                       w: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 } };
    pub static B: Mat4<f32> = Mat4 { x: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                                       y: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                                       z: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 },
                                       w: Vec4 { x: 5.0, y: 9.0, z: 13.0, w: 17.0 } };
    pub static C: Mat4<f32> = Mat4 { x: Vec4 { x: 3.0, y: 2.0, z:  1.0, w:  1.0 },
                                       y: Vec4 { x: 2.0, y: 3.0, z:  2.0, w:  2.0 },
                                       z: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  3.0 },
                                       w: Vec4 { x: 0.0, y: 1.0, z:  1.0, w:  0.0 } };
    pub static D: Mat4<f32> = Mat4 { x: Vec4 { x: 4.0, y: 3.0, z:  2.0, w:  1.0 },
                                       y: Vec4 { x: 3.0, y: 4.0, z:  3.0, w:  2.0 },
                                       z: Vec4 { x: 2.0, y: 3.0, z:  4.0, w:  3.0 },
                                       w: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  4.0 } };

    pub static V: Vec4<f32> = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    pub static F: f32 = 0.5;
}

#[bench]
fn bench_mat2_mul_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a = mat_a.mul_m(&mat2::B);
    }
}

#[bench]
fn bench_mat3_mul_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a = mat_a.mul_m(&mat3::B);
    }
}

#[bench]
fn bench_mat4_mul_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a = mat_a.mul_m(&mat4::B);
    }
}

#[bench]
fn bench_mat2_add_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a = mat_a.add_m(&mat2::B);
    }
}

#[bench]
fn bench_mat3_add_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a = mat_a.add_m(&mat3::B);
    }
}

#[bench]
fn bench_mat4_add_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a = mat_a.add_m(&mat4::B);
    }
}

#[bench]
fn bench_mat2_sub_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a = mat_a.sub_m(&mat2::B);
    }
}

#[bench]
fn bench_mat3_sub_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a = mat_a.sub_m(&mat3::B);
    }
}

#[bench]
fn bench_mat4_sub_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a = mat_a.sub_m(&mat4::B);
    }
}

#[bench]
fn bench_mat2_mul_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a = mat_a.mul_s(2.0);
    }
}

#[bench]
fn bench_mat3_mul_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a = mat_a.mul_s(2.0);
    }
}

#[bench]
fn bench_mat4_mul_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a = mat_a.mul_s(2.0);
    }
}

#[bench]
fn bench_mat2_div_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a = mat_a.div_s(2.);
    }
}

#[bench]
fn bench_mat3_div_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a = mat_a.div_s(2.);
    }
}

#[bench]
fn bench_mat4_div_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a = mat_a.div_s(2.);
    }
}

#[bench]
fn bench_mat2_rem_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a = mat_a.rem_s(2.);
    }
}

#[bench]
fn bench_mat3_rem_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a = mat_a.rem_s(2.);
    }
}

#[bench]
fn bench_mat4_rem_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a = mat_a.rem_s(2.);
    }
}

#[bench]
fn bench_mat2_neg_self(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a.neg_self();
    }
}

#[bench]
fn bench_mat3_neg_self(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a.neg_self();
    }
}

#[bench]
fn bench_mat4_neg_self(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a.neg_self();
    }
}

#[bench]
fn bench_mat2_div_self_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a.div_self_s(2.);
    }
}

#[bench]
fn bench_mat3_div_self_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a.div_self_s(2.);
    }
}

#[bench]
fn bench_mat4_div_self_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a.div_self_s(2.);
    }
}

#[bench]
fn bench_mat2_rem_self_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a.rem_self_s(2.);
    }
}

#[bench]
fn bench_mat3_rem_self_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a.rem_self_s(2.);
    }
}

#[bench]
fn bench_mat4_rem_self_s(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a.rem_self_s(2.);
    }
}

#[bench]
fn bench_mat2_mul_self_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a.mul_self_m(&mat2::B);
    }
}

#[bench]
fn bench_mat3_mul_self_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a.mul_self_m(&mat3::B);
    }
}

#[bench]
fn bench_mat4_mul_self_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a.mul_self_m(&mat4::B);
    }
}

#[bench]
fn bench_mat2_add_self_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a.add_self_m(&mat2::B);
    }
}

#[bench]
fn bench_mat3_add_self_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a.add_self_m(&mat3::B);
    }
}

#[bench]
fn bench_mat4_add_self_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a.add_self_m(&mat4::B);
    }
}

#[bench]
fn bench_mat2_sub_self_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a.sub_self_m(&mat2::B);
    }
}

#[bench]
fn bench_mat3_sub_self_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a.sub_self_m(&mat3::B);
    }
}

#[bench]
fn bench_mat4_sub_self_m(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a.add_self_m(&mat4::B);
    }
}


#[bench]
fn bench_mat2_transpose(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a = mat_a.transpose();
    }
}

#[bench]
fn bench_mat3_transpose(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a = mat_a.transpose();
    }
}

#[bench]
fn bench_mat4_transpose(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a = mat_a.transpose();
    }
}

#[bench]
fn bench_mat2_transpose_self(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat2::A.clone();
    do b.iter {
        mat_a.transpose_self();
    }
}

#[bench]
fn bench_mat3_transpose_self(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat3::A.clone();
    do b.iter {
        mat_a.transpose_self();
    }
}

#[bench]
fn bench_mat4_transpose_self(b: &mut extra::test::BenchHarness) {
    let mut mat_a = mat4::A.clone();
    do b.iter {
        mat_a.transpose_self();
    }
}
