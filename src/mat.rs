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

pub use self::mat2::Mat2;
pub use self::mat3::Mat3;
pub use self::mat4::Mat4;

pub mod mat2;
pub mod mat3;
pub mod mat4;

// GLSL-style type aliases

pub type mat2  = Mat2<f32>;
pub type dmat2 = Mat2<f64>;

pub type mat3  = Mat3<f32>;
pub type dmat3 = Mat3<f64>;

pub type mat4  = Mat4<f32>;
pub type dmat4 = Mat4<f64>;

// Rust-style type aliases

pub type Mat2f   = Mat2<float>;
pub type Mat2f32 = Mat2<f32>;
pub type Mat2f64 = Mat2<f64>;

pub type Mat3f   = Mat3<float>;
pub type Mat3f32 = Mat3<f32>;
pub type Mat3f64 = Mat3<f64>;

pub type Mat4f   = Mat4<float>;
pub type Mat4f32 = Mat4<f32>;
pub type Mat4f64 = Mat4<f64>;
