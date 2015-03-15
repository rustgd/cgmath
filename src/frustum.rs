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

//! View frustum for visibility determination

use array::Array2;
use bound::*;
use matrix::Matrix4;
use num::BaseFloat;
use plane::Plane;
use point::Point3;
use vector::{Vector, EuclideanVector};

#[derive(Copy, Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Frustum<S: BaseFloat> {
    pub left:   Plane<S>,
    pub right:  Plane<S>,
    pub bottom: Plane<S>,
    pub top:    Plane<S>,
    pub near:   Plane<S>,
    pub far:    Plane<S>,
}

impl<S: BaseFloat + 'static>
Frustum<S> {
    /// Construct a frustum.
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

    /// Extract frustum planes from a projection matrix.
    pub fn from_matrix4(mat: Matrix4<S>) -> Option<Frustum<S>> {
        Some(Frustum::new(
            match Plane::from_vector4_alt(mat.row(3).add_v(&mat.row(0))).normalize()
                { Some(p) => p, None => return None },
            match Plane::from_vector4_alt(mat.row(3).sub_v(&mat.row(0))).normalize()
                { Some(p) => p, None => return None },
            match Plane::from_vector4_alt(mat.row(3).add_v(&mat.row(1))).normalize()
                { Some(p) => p, None => return None },
            match Plane::from_vector4_alt(mat.row(3).sub_v(&mat.row(1))).normalize()
                { Some(p) => p, None => return None },
            match Plane::from_vector4_alt(mat.row(3).add_v(&mat.row(2))).normalize()
                { Some(p) => p, None => return None },
            match Plane::from_vector4_alt(mat.row(3).sub_v(&mat.row(2))).normalize()
                { Some(p) => p, None => return None }
        ))
    }

    /// Find the spatial relation of a bound inside this frustum.
    pub fn contains<B: Bound<S>>(&self, bound: &B) -> Relation {
        [&self.left, &self.right, &self.top, &self.bottom, &self.near, &self.far]
            .iter().fold(Relation::In, |cur, p| {
            use std::cmp::max;
            let r = bound.relate_plane(p);
            // If any of the planes are `Out`, the bound is outside.
            // Otherwise, if any are `Cross`, the bound is crossing.
            // Otherwise, the bound is fully inside.
            max(cur, r)
        })
    }
}

#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct FrustumPoints<S> {
    pub near_top_left:     Point3<S>,
    pub near_top_right:    Point3<S>,
    pub near_bottom_left:  Point3<S>,
    pub near_bottom_right: Point3<S>,
    pub far_top_left:      Point3<S>,
    pub far_top_right:     Point3<S>,
    pub far_bottom_left:   Point3<S>,
    pub far_bottom_right:  Point3<S>,
}
