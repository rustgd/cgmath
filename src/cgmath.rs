// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
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

#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(old_impl_check, plugin, core, hash, std_misc)]
#![plugin(rand_macros)]

//! Computer graphics-centric math.
//!
//! This crate provides useful mathematical primitives and operations on them.
//! It is organized into one module per primitive. The core structures are
//! vectors and matrices. A strongly-typed interface is provided, to prevent
//! mixing units or violating mathematical invariants.
//!
//! Transformations are not usually done directly on matrices, but go through
//! transformation objects that can be converted to matrices. Rotations go
//! through the `Basis` types, which are guaranteed to be orthogonal matrices.
//! Despite this, one can directly create a limited rotation matrix using the
//! `look_at`, `from_angle`, `from_euler`, and `from_axis_angle` methods.
//! These are provided for convenience.

extern crate "rustc-serialize" as rustc_serialize;
extern crate rand;

// Re-exports

pub use array::{Array1, Array2, FixedArray};

pub use matrix::Matrix;
pub use matrix::{Matrix2, Matrix3, Matrix4};
pub use matrix::{ToMatrix2, ToMatrix3, ToMatrix4};
pub use quaternion::{Quaternion, ToQuaternion};
pub use vector::{Vector, EuclideanVector};
pub use vector::{Vector2, Vector3, Vector4};
pub use vector::{dot, vec2, vec3, vec4};

pub use angle::{rad, deg};
pub use angle::{Angle, Rad, Deg};
pub use angle::{ToRad, ToDeg};
pub use angle::bisect;
pub use angle::{sin, cos, tan, sin_cos};
pub use angle::{cot, sec, csc};
pub use angle::{acos, asin, atan, atan2};
pub use plane::Plane;
pub use point::{Point, Point2, Point3};
pub use line::{Line, Line2, Line3};
pub use ray::{Ray, Ray2, Ray3};
pub use rotation::{Rotation, Rotation2, Rotation3};
pub use rotation::{Basis3, Basis2};
pub use rotation::{ToBasis2, ToBasis3};
pub use transform::{Transform, Transform3};
pub use transform::{Decomposed, AffineMatrix3};

pub use projection::{perspective, frustum, ortho};
pub use projection::{Projection, PerspectiveFov, Perspective, Ortho};

pub use aabb::{Aabb, Aabb2, Aabb3};
pub use cylinder::Cylinder;
pub use frustum::{Frustum, FrustumPoints};
pub use intersect::Intersect;
pub use obb::{Obb2, Obb3};
pub use sphere::Sphere;

pub use approx::ApproxEq;
pub use num::{PartialOrd, BaseNum, BaseInt, BaseFloat, One, one, Zero, zero};

// Modules

mod array;

mod matrix;
mod quaternion;
mod vector;

mod angle;
mod plane;
mod point;
mod line;
mod ray;
mod rotation;
mod transform;

mod projection;

mod aabb;
mod cylinder;
mod frustum;
mod intersect;
mod obb;
mod sphere;

mod approx;
mod num;
