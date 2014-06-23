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

//! Axis-aligned bounding boxes
//!
//! An AABB is a geometric object which encompasses a set of points and is not
//! rotated. It is either a rectangle or a rectangular prism (depending on the
//! dimension) where the slope of every line is either 0 or undefined. These
//! are useful for very cheap collision detection.

use point::{Point, Point2, Point3};
use vector::{Vector, Vector2, Vector3};
use num::BaseNum;
use std::fmt;
use std::num::{zero, one};

pub trait Aabb<S: BaseNum, V: Vector<S>, P: Point<S, V>> {
    /// Create a new AABB using two points as opposing corners.
    fn new(p1: P, p2: P) -> Self;

    /// Return a shared reference to the point nearest to (-inf, -inf).
    fn min<'a>(&'a self) -> &'a P;

    /// Return a shared reference to the point nearest to (inf, inf).
    fn max<'a>(&'a self) -> &'a P;

    /// Return the dimensions of this AABB.
    #[inline]
    fn dim(&self) -> V { self.max().sub_p(self.min()) }

    /// Return the volume this AABB encloses.
    #[inline]
    fn volume(&self) -> S { self.dim().comp_mul() }

    /// Return the center point of this AABB.
    #[inline]
    fn center(&self) -> P {
        let two = one::<S>() + one::<S>();
        self.min().add_v(&self.dim().div_s(two))
    }

    /// Tests whether a point is cointained in the box, inclusive for min corner
    /// and exclusive for the max corner.
    #[inline]
    fn contains(&self, p: &P) -> bool;

    /// Returns a new AABB that is grown to include the given point.
    fn grow(&self, p: &P) -> Self {
        let min = self.min().min(p);
        let max = self.max().max(p);
        Aabb::new(min, max)
    }

    /// Add a vector to every point in the AABB, returning a new AABB.
    fn add_v(&self, v: &V) -> Self {
        Aabb::new(self.min().add_v(v), self.max().add_v(v))
    }

    /// Multiply every point in the AABB by a scalar, returning a new AABB.
    fn mul_s(&self, s: S) -> Self {
        Aabb::new(self.min().mul_s(s.clone()), self.max().mul_s(s.clone()))
    }

    /// Multiply every point in the AABB by a vector, returning a new AABB.
    fn mul_v(&self, v: &V) -> Self {
        let min : P = Point::from_vec(&self.min().to_vec().mul_v(v));
        let max : P = Point::from_vec(&self.max().to_vec().mul_v(v));
        Aabb::new(min, max)
    }
}

/// A two-dimensional AABB, aka a rectangle.
#[deriving(Clone, PartialEq)]
pub struct Aabb2<S> {
    pub min: Point2<S>,
    pub max: Point2<S>,
}

impl<S: BaseNum> Aabb2<S> {
    /// Construct a new axis-aligned bounding box from two points.
    #[inline]
    pub fn new(p1: Point2<S>, p2: Point2<S>) -> Aabb2<S> {
        Aabb2 {
            min: Point2::new(p1.x.partial_min(p2.x),
                             p1.y.partial_min(p2.y)),
            max: Point2::new(p1.x.partial_max(p2.x),
                             p1.y.partial_max(p2.y)),
        }
    }
}

impl<S: BaseNum> Aabb<S, Vector2<S>, Point2<S>> for Aabb2<S> {
    #[inline]
    fn new(p1: Point2<S>, p2: Point2<S>) -> Aabb2<S> { Aabb2::new(p1, p2) }

    #[inline]
    fn min<'a>(&'a self) -> &'a Point2<S> { &self.min }

    #[inline]
    fn max<'a>(&'a self) -> &'a Point2<S> { &self.max }

    #[inline]
    fn contains(&self, p: &Point2<S>) -> bool {
        let v_min = p.sub_p(self.min());
        let v_max = self.max().sub_p(p);
        v_min.x >= zero() && v_min.y >= zero() &&
        v_max.x >  zero() && v_max.y >  zero()
    }
}

impl<S: BaseNum> fmt::Show for Aabb2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} - {}]", self.min, self.max)
    }
}

/// A three-dimensional AABB, aka a rectangular prism.
#[deriving(Clone, PartialEq)]
pub struct Aabb3<S> {
    pub min: Point3<S>,
    pub max: Point3<S>,
}

impl<S: BaseNum> Aabb3<S> {
    /// Construct a new axis-aligned bounding box from two points.
    #[inline]
    pub fn new(p1: Point3<S>, p2: Point3<S>) -> Aabb3<S> {
        Aabb3 {
            min: Point3::new(p1.x.partial_min(p2.x),
                             p1.y.partial_min(p2.y),
                             p1.z.partial_min(p2.z)),
            max: Point3::new(p1.x.partial_max(p2.x),
                             p1.y.partial_max(p2.y),
                             p1.z.partial_max(p2.z)),
        }
    }
}

impl<S: BaseNum> Aabb<S, Vector3<S>, Point3<S>> for Aabb3<S> {
    #[inline]
    fn new(p1: Point3<S>, p2: Point3<S>) -> Aabb3<S> { Aabb3::new(p1, p2) }

    #[inline]
    fn min<'a>(&'a self) -> &'a Point3<S> { &self.min }

    #[inline]
    fn max<'a>(&'a self) -> &'a Point3<S> { &self.max }

    #[inline]
    fn contains(&self, p: &Point3<S>) -> bool {
        let v_min = p.sub_p(self.min());
        let v_max = self.max().sub_p(p);
        v_min.x >= zero() && v_min.y >= zero() && v_min.z >= zero() &&
        v_max.x >  zero() && v_max.y >  zero() && v_max.z >  zero()
    }
}

impl<S: BaseNum> fmt::Show for Aabb3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} - {}]", self.min, self.max)
    }
}
