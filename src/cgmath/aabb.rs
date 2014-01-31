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

//! Axis-aligned bounding boxes

use point::{Point2, Point3};

#[deriving(Clone, Eq)]
pub struct Aabb2<S> {
    min: Point2<S>,
    max: Point2<S>,
}

impl<S: Num + Orderable> Aabb2<S> {
    /// Construct a new axis-aligned bounding box from two points.
    #[inline]
    pub fn new(p1: &Point2<S>, p2: &Point2<S>) -> Aabb2<S> {
        Aabb2 {
            min: Point2::new(p1.x.min(&p2.x), p1.y.min(&p2.y)),
            max: Point2::new(p1.x.max(&p2.x), p1.y.max(&p2.y)),
        }
    }
}

#[deriving(Clone, Eq)]
pub struct Aabb3<S> {
    min: Point3<S>,
    max: Point3<S>,
}

impl<S: Num + Orderable> Aabb3<S> {
    /// Construct a new axis-aligned bounding box from two points.
    #[inline]
    pub fn new(p1: &Point3<S>, p2: &Point3<S>) -> Aabb3<S> {
        Aabb3 {
            min: Point3::new(p1.x.min(&p2.x), p1.y.min(&p2.y), p1.z.min(&p2.z)),
            max: Point3::new(p1.x.max(&p2.x), p1.y.max(&p2.y), p1.z.max(&p2.z)),
        }
    }
}
