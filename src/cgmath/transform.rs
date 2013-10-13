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

pub trait Transform<S> {
    fn transform_vec(&self, point: Point3<S>) -> Point3<S>;
    fn transform_point(&self, point: Point3<S>) -> Point3<S>;
    fn transform_ray(&self, ray: Ray3<S>) -> Ray3<S>;
}

/// A homogeneous transformation matrix.
pub struct AffineMatrix3<S> {
    mat: Mat4<S>,
}

/// A transformation in three dimensions consisting of a rotation,
/// displacement vector and scale amount.
pub struct Transform3<S, R> {
    rot: R,
    disp: Vec3<S>,
    scale: S,
}

impl<S: Float, R: Rotation3<S>> Transform3<S, R> {
    #[inline]
    pub fn new(rot: R, disp: Vec3<S>, scale: S) -> Transform3<S, R> {
        Transform3 { rot: rot, disp: disp, scale: S }
    }
}
