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

use mat::Mat4;
use plane::Plane;
use point::Point3;

mod num_macros;

#[deriving(Clone, Eq)]
pub struct Frustum<T> {
    left:   Plane<T>,
    right:  Plane<T>,
    bottom: Plane<T>,
    top:    Plane<T>,
    near:   Plane<T>,
    far:    Plane<T>,
}

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

impl<T:Clone + Real> Frustum<T> {
    /// Constructs a frustum
    pub fn from_planes(left:   Plane<T>, right:  Plane<T>,
                       bottom: Plane<T>, top:    Plane<T>,
                       near:   Plane<T>, far:    Plane<T>) -> Frustum<T> {
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
            left:   Plane::from_vec4(mat.row(3).add_v(&mat.row(0)).normalize()),
            right:  Plane::from_vec4(mat.row(3).sub_v(&mat.row(0)).normalize()),
            bottom: Plane::from_vec4(mat.row(3).add_v(&mat.row(1)).normalize()),
            top:    Plane::from_vec4(mat.row(3).sub_v(&mat.row(1)).normalize()),
            near:   Plane::from_vec4(mat.row(3).add_v(&mat.row(2)).normalize()),
            far:    Plane::from_vec4(mat.row(3).sub_v(&mat.row(2)).normalize()),
        }
    }

    pub fn base() -> Frustum<T> {
        Frustum {
            left:   Plane::from_abcd( one!(T),  zero!(T),  zero!(T), one!(T)),
            right:  Plane::from_abcd(-one!(T),  zero!(T),  zero!(T), one!(T)),
            bottom: Plane::from_abcd( zero!(T),  one!(T),  zero!(T), one!(T)),
            top:    Plane::from_abcd( zero!(T), -one!(T),  zero!(T), one!(T)),
            near:   Plane::from_abcd( zero!(T),  zero!(T), -one!(T), one!(T)),
            far:    Plane::from_abcd( zero!(T),  zero!(T),  one!(T), one!(T)),
        }
    }
}

impl<T:Clone + Real + ApproxEq<T>> Frustum<T> {
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

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for Frustum<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &Frustum<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &Frustum<T>, epsilon: &T) -> bool {
        self.left.approx_eq_eps(&other.left, epsilon) &&
        self.right.approx_eq_eps(&other.right, epsilon) &&
        self.bottom.approx_eq_eps(&other.bottom, epsilon) &&
        self.top.approx_eq_eps(&other.top, epsilon) &&
        self.near.approx_eq_eps(&other.near, epsilon) &&
        self.far.approx_eq_eps(&other.far, epsilon)
    }
}