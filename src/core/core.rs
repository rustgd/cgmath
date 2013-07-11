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

//! Core datatypes and conversion traits for 3D mathematics

pub use self::dim::Dimensional;
pub use self::swap::Swap;

pub use self::mat::{Mat2, ToMat2, Mat3, ToMat3, Mat4, ToMat4};
pub use self::quat::{Quat, ToQuat};
pub use self::vec::{Vec2, Vec3, Vec4};

pub mod dim;
pub mod swap;

pub mod mat;
pub mod quat;
pub mod vec;
