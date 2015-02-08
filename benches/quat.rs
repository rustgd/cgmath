// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directionectory of this distribution.
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

#![feature(test)]

extern crate rand;
extern crate test;
extern crate cgmath;

use rand::{IsaacRng, Rng};
use test::Bencher;
use cgmath::*;

#[path="common/macros.rs"]
#[macro_use] mod macros;

bench_binop!(_bench_quat_add_q, Quaternion<f32>, Quaternion<f32>, add_q);
bench_binop!(_bench_quat_sub_q, Quaternion<f32>, Quaternion<f32>, sub_q);
bench_binop!(_bench_quat_mul_q, Quaternion<f32>, Quaternion<f32>, mul_q);
bench_binop!(_bench_quat_mul_v, Quaternion<f32>, Vector3<f32>, mul_v);
bench_binop_deref!(_bench_quat_mul_s, Quaternion<f32>, f32, mul_s);
bench_binop_deref!(_bench_quat_div_s, Quaternion<f32>, f32, div_s);
bench_unop!(_bench_quat_invert, Quaternion<f32>, invert);
bench_unop!(_bench_quat_conjugate, Quaternion<f32>, conjugate);
bench_unop!(_bench_quat_normalize, Quaternion<f32>, normalize);
