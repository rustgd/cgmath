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

//! View frustum for visibility determination

use math::*;

#[deriving(Clone, Eq)]
pub struct Frustum<T> {
    left:   Plane3<T>,
    right:  Plane3<T>,
    bottom: Plane3<T>,
    top:    Plane3<T>,
    near:   Plane3<T>,
    far:    Plane3<T>,
}

impl_dimensioned!(Frustum, Plane3<T>, 6)
impl_approx!(Frustum {
    left, right,
    top, bottom,
    near, far
})

#[deriving(Clone, Eq)]
pub struct FrustumPoints<T> {
    near_top_left:     Point3<T>,
    near_top_right:    Point3<T>,
    near_bottom_left:  Point3<T>,
    near_bottom_right: Point3<T>,
    far_top_left:      Point3<T>,
    far_top_right:     Point3<T>,
    far_bottom_left:   Point3<T>,
    far_bottom_right:  Point3<T>,
}

impl_dimensioned!(FrustumPoints, Point3<T>, 8)
impl_approx!(FrustumPoints {
    near_top_left,
    near_top_right,
    near_bottom_left,
    near_bottom_right,
    far_top_left,
    far_top_right,
    far_bottom_left,
    far_bottom_right
})

impl<T:Clone + Float> Frustum<T> {
    /// Constructs a frustum
    pub fn from_planes(left:   Plane3<T>, right:  Plane3<T>,
                       bottom: Plane3<T>, top:    Plane3<T>,
                       near:   Plane3<T>, far:    Plane3<T>) -> Frustum<T> {
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
    pub fn from_matrix(mat: Mat4<T>) -> Frustum<T> {
        Frustum {
            left:   Plane3::from_vec4(mat.row(3).add_v(&mat.row(0)).normalize()),
            right:  Plane3::from_vec4(mat.row(3).sub_v(&mat.row(0)).normalize()),
            bottom: Plane3::from_vec4(mat.row(3).add_v(&mat.row(1)).normalize()),
            top:    Plane3::from_vec4(mat.row(3).sub_v(&mat.row(1)).normalize()),
            near:   Plane3::from_vec4(mat.row(3).add_v(&mat.row(2)).normalize()),
            far:    Plane3::from_vec4(mat.row(3).sub_v(&mat.row(2)).normalize()),
        }
    }
}

impl<T:Clone + Float> Frustum<T> {
    /// Computes where the frustum planes intersect to form corners and returns
    /// a struct containing the eight resulting position vectors.
    pub fn to_points(&self) -> FrustumPoints<T> {
        FrustumPoints {
            near_top_left:     self.near.intersection_3pl(&self.top, &self.left).unwrap(),
            near_top_right:    self.near.intersection_3pl(&self.top, &self.right).unwrap(),
            near_bottom_left:  self.near.intersection_3pl(&self.bottom, &self.left).unwrap(),
            near_bottom_right: self.near.intersection_3pl(&self.bottom, &self.right).unwrap(),
            far_top_left:      self.far.intersection_3pl(&self.top, &self.left).unwrap(),
            far_top_right:     self.far.intersection_3pl(&self.top, &self.right).unwrap(),
            far_bottom_left:   self.far.intersection_3pl(&self.bottom, &self.left).unwrap(),
            far_bottom_right:  self.far.intersection_3pl(&self.bottom, &self.right).unwrap(),
        }
    }
}
