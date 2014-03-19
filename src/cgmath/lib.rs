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

#[crate_id="cgmath#0.1"];
#[crate_type = "rlib"];
#[crate_type = "dylib"];
#[comment = "A mathematics library for computer graphics."];
#[license = "ASL2"];

#[feature(globs)];
#[feature(macro_rules)];

pub mod array;
pub mod matrix;
pub mod quaternion;
pub mod vector;

pub mod angle;
pub mod plane;
pub mod point;
pub mod ray;
pub mod rotation;
pub mod transform;

pub mod projection;

pub mod aabb;
pub mod cylinder;
pub mod frustum;
pub mod intersect;
pub mod obb;
pub mod sphere;

pub mod approx;
pub mod ptr;

pub mod partial_ord;
