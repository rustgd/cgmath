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

bench_binop!(_bench_vector2_add_v, Vector2<f32>, Vector2<f32>, add_v);
bench_binop!(_bench_vector3_add_v, Vector3<f32>, Vector3<f32>, add_v);
bench_binop!(_bench_vector4_add_v, Vector4<f32>, Vector4<f32>, add_v);

bench_binop!(_bench_vector2_sub_v, Vector2<f32>, Vector2<f32>, sub_v);
bench_binop!(_bench_vector3_sub_v, Vector3<f32>, Vector3<f32>, sub_v);
bench_binop!(_bench_vector4_sub_v, Vector4<f32>, Vector4<f32>, sub_v);

bench_binop!(_bench_vector2_mul_v, Vector2<f32>, Vector2<f32>, mul_v);
bench_binop!(_bench_vector3_mul_v, Vector3<f32>, Vector3<f32>, mul_v);
bench_binop!(_bench_vector4_mul_v, Vector4<f32>, Vector4<f32>, mul_v);

bench_binop!(_bench_vector2_div_v, Vector2<f32>, Vector2<f32>, div_v);
bench_binop!(_bench_vector3_div_v, Vector3<f32>, Vector3<f32>, div_v);
bench_binop!(_bench_vector4_div_v, Vector4<f32>, Vector4<f32>, div_v);

bench_binop_deref!(_bench_vector2_add_s, Vector2<f32>, f32, add_s);
bench_binop_deref!(_bench_vector3_add_s, Vector3<f32>, f32, add_s);
bench_binop_deref!(_bench_vector4_add_s, Vector4<f32>, f32, add_s);

bench_binop_deref!(_bench_vector2_sub_s, Vector2<f32>, f32, sub_s);
bench_binop_deref!(_bench_vector3_sub_s, Vector3<f32>, f32, sub_s);
bench_binop_deref!(_bench_vector4_sub_s, Vector4<f32>, f32, sub_s);

bench_binop_deref!(_bench_vector2_mul_s, Vector2<f32>, f32, mul_s);
bench_binop_deref!(_bench_vector3_mul_s, Vector3<f32>, f32, mul_s);
bench_binop_deref!(_bench_vector4_mul_s, Vector4<f32>, f32, mul_s);

bench_binop_deref!(_bench_vector2_div_s, Vector2<f32>, f32, div_s);
bench_binop_deref!(_bench_vector3_div_s, Vector3<f32>, f32, div_s);
bench_binop_deref!(_bench_vector4_div_s, Vector4<f32>, f32, div_s);


bench_binop!(_bench_vector2_dot, Vector2<f32>, Vector2<f32>, dot);
bench_binop!(_bench_vector3_dot, Vector3<f32>, Vector3<f32>, dot);
bench_binop!(_bench_vector4_dot, Vector4<f32>, Vector4<f32>, dot);

bench_binop!(_bench_vector3_cross, Vector3<f32>, Vector3<f32>, cross);

bench_unop!(_bench_vector2_norm, Vector2<f32>, length);
bench_unop!(_bench_vector3_norm, Vector3<f32>, length);
bench_unop!(_bench_vector4_norm, Vector4<f32>, length);

bench_unop!(_bench_vector2_normalize, Vector2<f32>, normalize);
bench_unop!(_bench_vector3_normalize, Vector3<f32>, normalize);
bench_unop!(_bench_vector4_normalize, Vector4<f32>, normalize);
