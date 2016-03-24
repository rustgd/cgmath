// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
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

//! Computer graphics-centric math.

#![cfg_attr(feature = "unstable", feature(augmented_assignments, op_assign_traits))]

extern crate num as rust_num;
extern crate rustc_serialize;
extern crate rand;

// Re-exports

pub use array::*;
pub use matrix::*;
pub use quaternion::*;
pub use vector::*;

pub use angle::*;
pub use point::*;

pub use projection::*;

pub use approx::ApproxEq;
pub use num::*;

pub use rust_num::{One, Zero, one, zero};

// Modules

mod macros;

mod array;

mod matrix;
mod quaternion;
mod vector;

mod angle;
mod point;

mod projection;

mod approx;
mod num;
