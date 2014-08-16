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

use cgmath::*;

use test::Bencher;

pub mod matrix2 {
    use cgmath::*;

    pub static A: Matrix2<f32> = Matrix2 { x: Vector2 { x: 1.0, y: 3.0 },
                                           y: Vector2 { x: 2.0, y: 4.0 } };
    pub static B: Matrix2<f32> = Matrix2 { x: Vector2 { x: 2.0, y: 4.0 },
                                           y: Vector2 { x: 3.0, y: 5.0 } };
}

pub mod matrix3 {
    use cgmath::*;

    pub static A: Matrix3<f32> = Matrix3 { x: Vector3 { x: 1.0, y: 4.0, z:  7.0 },
                                           y: Vector3 { x: 2.0, y: 5.0, z:  8.0 },
                                           z: Vector3 { x: 3.0, y: 6.0, z:  9.0 } };
    pub static B: Matrix3<f32> = Matrix3 { x: Vector3 { x: 2.0, y: 5.0, z:  8.0 },
                                           y: Vector3 { x: 3.0, y: 6.0, z:  9.0 },
                                           z: Vector3 { x: 4.0, y: 7.0, z: 10.0 } };
}

pub mod matrix4 {
    use cgmath::*;

    pub static A: Matrix4<f32> = Matrix4 { x: Vector4 { x: 1.0, y: 5.0, z:  9.0, w: 13.0 },
                                           y: Vector4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                                           z: Vector4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                                           w: Vector4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 } };
    pub static B: Matrix4<f32> = Matrix4 { x: Vector4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                                           y: Vector4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                                           z: Vector4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 },
                                           w: Vector4 { x: 5.0, y: 9.0, z: 13.0, w: 17.0 } };
}

#[bench]
fn bench_matrix2_mul_m(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a = matrix_a.mul_m(&matrix2::B); })
}

#[bench]
fn bench_matrix3_mul_m(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a = matrix_a.mul_m(&matrix3::B); })
}

#[bench]
fn bench_matrix4_mul_m(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a = matrix_a.mul_m(&matrix4::B); })
}

#[bench]
fn bench_matrix2_add_m(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a = matrix_a.add_m(&matrix2::B); })
}

#[bench]
fn bench_matrix3_add_m(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a = matrix_a.add_m(&matrix3::B); })
}

#[bench]
fn bench_matrix4_add_m(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a = matrix_a.add_m(&matrix4::B); })
}

#[bench]
fn bench_matrix2_sub_m(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a = matrix_a.sub_m(&matrix2::B); })
}

#[bench]
fn bench_matrix3_sub_m(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a = matrix_a.sub_m(&matrix3::B); })
}

#[bench]
fn bench_matrix4_sub_m(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a = matrix_a.sub_m(&matrix4::B); })
}

#[bench]
fn bench_matrix2_mul_s(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a = matrix_a.mul_s(2.0); })
}

#[bench]
fn bench_matrix3_mul_s(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a = matrix_a.mul_s(2.0); })
}

#[bench]
fn bench_matrix4_mul_s(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a = matrix_a.mul_s(2.0); })
}

#[bench]
fn bench_matrix2_div_s(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a = matrix_a.div_s(2.); })
}

#[bench]
fn bench_matrix3_div_s(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a = matrix_a.div_s(2.); })
}

#[bench]
fn bench_matrix4_div_s(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a = matrix_a.div_s(2.); })
}

#[bench]
fn bench_matrix2_rem_s(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a = matrix_a.rem_s(2.); })
}

#[bench]
fn bench_matrix3_rem_s(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a = matrix_a.rem_s(2.); })
}

#[bench]
fn bench_matrix4_rem_s(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a = matrix_a.rem_s(2.); })
}

#[bench]
fn bench_matrix2_neg_self(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a.neg_self(); })
}

#[bench]
fn bench_matrix3_neg_self(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a.neg_self(); })
}

#[bench]
fn bench_matrix4_neg_self(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a.neg_self(); })
}

#[bench]
fn bench_matrix2_div_self_s(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a.div_self_s(2.); })
}

#[bench]
fn bench_matrix3_div_self_s(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a.div_self_s(2.); })
}

#[bench]
fn bench_matrix4_div_self_s(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a.div_self_s(2.); })
}

#[bench]
fn bench_matrix2_rem_self_s(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a.rem_self_s(2.); })
}

#[bench]
fn bench_matrix3_rem_self_s(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a.rem_self_s(2.); })
}

#[bench]
fn bench_matrix4_rem_self_s(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a.rem_self_s(2.); })
}

#[bench]
fn bench_matrix2_mul_self_m(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a.mul_self_m(&matrix2::B); })
}

#[bench]
fn bench_matrix3_mul_self_m(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a.mul_self_m(&matrix3::B); })
}

#[bench]
fn bench_matrix4_mul_self_m(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a.mul_self_m(&matrix4::B); })
}

#[bench]
fn bench_matrix2_add_self_m(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a.add_self_m(&matrix2::B); })
}

#[bench]
fn bench_matrix3_add_self_m(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a.add_self_m(&matrix3::B); })
}

#[bench]
fn bench_matrix4_add_self_m(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a.add_self_m(&matrix4::B); })
}

#[bench]
fn bench_matrix2_sub_self_m(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a.sub_self_m(&matrix2::B); })
}

#[bench]
fn bench_matrix3_sub_self_m(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a.sub_self_m(&matrix3::B); })
}

#[bench]
fn bench_matrix4_sub_self_m(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a.add_self_m(&matrix4::B); })
}


#[bench]
fn bench_matrix2_transpose(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a = matrix_a.transpose(); })
}

#[bench]
fn bench_matrix3_transpose(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a = matrix_a.transpose(); })
}

#[bench]
fn bench_matrix4_transpose(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a = matrix_a.transpose(); })
}

#[bench]
fn bench_matrix2_transpose_self(b: &mut Bencher) {
    let mut matrix_a = matrix2::A.clone();
    b.iter(|| { matrix_a.transpose_self(); })
}

#[bench]
fn bench_matrix3_transpose_self(b: &mut Bencher) {
    let mut matrix_a = matrix3::A.clone();
    b.iter(|| { matrix_a.transpose_self(); })
}

#[bench]
fn bench_matrix4_transpose_self(b: &mut Bencher) {
    let mut matrix_a = matrix4::A.clone();
    b.iter(|| { matrix_a.transpose_self(); })
}
