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

//! View frustum for visibility determination

use matrix::{Matrix, Mat4};
use plane::Plane;
use point::Point3;
use vector::{Vector, EuclideanVector};
use partial_ord::PartOrdFloat;

#[deriving(Clone, Eq)]
pub struct Frustum<S> {
    left:   Plane<S>,
    right:  Plane<S>,
    bottom: Plane<S>,
    top:    Plane<S>,
    near:   Plane<S>,
    far:    Plane<S>,
}

impl<S: PartOrdFloat<S>>
Frustum<S> {
    /// Constructs a frustum
    pub fn new(left:   Plane<S>, right:  Plane<S>,
               bottom: Plane<S>, top:    Plane<S>,
               near:   Plane<S>, far:    Plane<S>) -> Frustum<S> {
        Frustum {
            left:   left,
            right:  right,
            bottom: bottom,
            top:    top,
            near:   near,
            far:    far,
        }
    }

    /// Extracts frustum planes from a projection matrix
    pub fn from_mat4(mat: Mat4<S>) -> Frustum<S> {
        Frustum::new(Plane::from_vec4(mat.r(3).add_v(&mat.r(0)).normalize()),
                     Plane::from_vec4(mat.r(3).sub_v(&mat.r(0)).normalize()),
                     Plane::from_vec4(mat.r(3).add_v(&mat.r(1)).normalize()),
                     Plane::from_vec4(mat.r(3).sub_v(&mat.r(1)).normalize()),
                     Plane::from_vec4(mat.r(3).add_v(&mat.r(2)).normalize()),
                     Plane::from_vec4(mat.r(3).sub_v(&mat.r(2)).normalize()))
    }
}

#[deriving(Clone, Eq)]
pub struct FrustumPoints<S> {
    near_top_left:     Point3<S>,
    near_top_right:    Point3<S>,
    near_bottom_left:  Point3<S>,
    near_bottom_right: Point3<S>,
    far_top_left:      Point3<S>,
    far_top_right:     Point3<S>,
    far_bottom_left:   Point3<S>,
    far_bottom_right:  Point3<S>,
}
