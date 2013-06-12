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
pub use self::vec2::Vec2;
pub use self::vec3::Vec3;
pub use self::vec4::Vec4;

pub mod vec2;
pub mod vec3;
pub mod vec4;

// GLSL-style type aliases

pub type vec2  = Vec2<f32>;
pub type dvec2 = Vec2<f64>;
pub type bvec2 = Vec2<bool>;
pub type ivec2 = Vec2<i32>;
pub type uvec2 = Vec2<u32>;

pub type vec3  = Vec3<f32>;
pub type dvec3 = Vec3<f64>;
pub type bvec3 = Vec3<bool>;
pub type ivec3 = Vec3<i32>;
pub type uvec3 = Vec3<u32>;

pub type vec4  = Vec4<f32>;
pub type dvec4 = Vec4<f64>;
pub type bvec4 = Vec4<bool>;
pub type ivec4 = Vec4<i32>;
pub type uvec4 = Vec4<u32>;

// Rust-style type aliases

pub type Vec2f   = Vec2<float>;
pub type Vec2f32 = Vec2<f32>;
pub type Vec2f64 = Vec2<f64>;
pub type Vec2i   = Vec2<int>;
pub type Vec2i8  = Vec2<i8>;
pub type Vec2i16 = Vec2<i16>;
pub type Vec2i32 = Vec2<i32>;
pub type Vec2i64 = Vec2<i64>;
pub type Vec2u   = Vec2<uint>;
pub type Vec2u8  = Vec2<u8>;
pub type Vec2u16 = Vec2<u16>;
pub type Vec2u32 = Vec2<u32>;
pub type Vec2u64 = Vec2<u64>;
pub type Vec2b   = Vec2<bool>;

pub type Vec3f   = Vec3<float>;
pub type Vec3f32 = Vec3<f32>;
pub type Vec3f64 = Vec3<f64>;
pub type Vec3i   = Vec3<int>;
pub type Vec3i8  = Vec3<i8>;
pub type Vec3i16 = Vec3<i16>;
pub type Vec3i32 = Vec3<i32>;
pub type Vec3i64 = Vec3<i64>;
pub type Vec3u   = Vec3<uint>;
pub type Vec3u8  = Vec3<u8>;
pub type Vec3u16 = Vec3<u16>;
pub type Vec3u32 = Vec3<u32>;
pub type Vec3u64 = Vec3<u64>;
pub type Vec3b   = Vec3<bool>;

pub type Vec4f   = Vec4<float>;
pub type Vec4f32 = Vec4<f32>;
pub type Vec4f64 = Vec4<f64>;
pub type Vec4i   = Vec4<int>;
pub type Vec4i8  = Vec4<i8>;
pub type Vec4i16 = Vec4<i16>;
pub type Vec4i32 = Vec4<i32>;
pub type Vec4i64 = Vec4<i64>;
pub type Vec4u   = Vec4<uint>;
pub type Vec4u8  = Vec4<u8>;
pub type Vec4u16 = Vec4<u16>;
pub type Vec4u32 = Vec4<u32>;
pub type Vec4u64 = Vec4<u64>;
pub type Vec4b   = Vec4<bool>;
