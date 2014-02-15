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

use point::{Point, Point2, Point3};
use vector::{Vector, Vec2, Vec3};
use array::build;
use std::fmt;
use std::num::{zero, one};
use std::iter::Iterator;
use std::cmp::{max, min};

pub trait Aabb
<
    S: Primitive,
    V: Vector<S, Slice>,
    P: Point<S, V, Slice>,
    Slice
> {
    fn new(p1: P, p2: P) -> Self;
    fn min<'a>(&'a self) -> &'a P;
    fn max<'a>(&'a self) -> &'a P;
    #[inline] fn dim(&self) -> V { self.max().sub_p(self.min()) }
    #[inline] fn volume(&self) -> S { self.dim().comp_mul() }
    #[inline] fn center(&self) -> P {
        let two = one::<S>() + one::<S>();
        self.min().add_v(&self.dim().div_s(two))
    }

    // Tests whether a point is cointained in the box, inclusive for min corner
    // and exclusive for the max corner.
    #[inline] fn contains(&self, p: &P) -> bool {
        p.sub_p(self.min()).iter().all(|x| *x >= zero::<S>()) &&
        self.max().sub_p(p).iter().all(|x| *x > zero::<S>())
    }

    // Returns a new AABB that is grown to include the given point.
    fn grow(&self, p: &P) -> Self {
        let min : P = build(|i| min(self.min().i(i).clone(), p.i(i).clone()));
        let max : P = build(|i| max(self.max().i(i).clone(), p.i(i).clone()));
        Aabb::new(min, max)
    }

    // Returns a new AABB that has its points translated by the given vector.
    fn add_v(&self, v: &V) -> Self {
        Aabb::new(self.min().add_v(v), self.max().add_v(v))
    }

    fn mul_s(&self, s: S) -> Self {
        Aabb::new(self.min().mul_s(s.clone()), self.max().mul_s(s.clone()))
    }

    fn mul_v(&self, v: &V) -> Self {
        let min : P = Point::from_vec(&self.min().to_vec().mul_v(v));
        let max : P = Point::from_vec(&self.max().to_vec().mul_v(v));
        Aabb::new(min, max)
    }
}

#[deriving(Clone, Eq)]
pub struct Aabb2<S> {
    min: Point2<S>,
    max: Point2<S>,
}

impl<S: Num + Ord + Clone> Aabb2<S> {
    /// Construct a new axis-aligned bounding box from two points.
    #[inline]
    pub fn new(p1: Point2<S>, p2: Point2<S>) -> Aabb2<S> {
        Aabb2 {
            min: Point2::new(min(p1.x.clone(), p2.x.clone()), min(p1.y.clone(), p2.y.clone())),
            max: Point2::new(max(p1.x.clone(), p2.x.clone()), max(p1.y.clone(), p2.y.clone())),
        }
    }
}

impl<S: Primitive> Aabb<S, Vec2<S>, Point2<S>, [S, ..2]> for Aabb2<S> {
    fn new(p1: Point2<S>, p2: Point2<S>) -> Aabb2<S> { Aabb2::new(p1, p2) }
    #[inline] fn min<'a>(&'a self) -> &'a Point2<S> { &self.min }
    #[inline] fn max<'a>(&'a self) -> &'a Point2<S> { &self.max }
}

impl<S: fmt::Show> ToStr for Aabb2<S> {
    fn to_str(&self) -> ~str {
        format!("[{} - {}]", self.min.to_str(), self.max.to_str())
    }
}

#[deriving(Clone, Eq)]
pub struct Aabb3<S> {
    min: Point3<S>,
    max: Point3<S>,
}

impl<S: Num + Ord + Clone> Aabb3<S> {
    /// Construct a new axis-aligned bounding box from two points.
    #[inline]
    pub fn new(p1: Point3<S>, p2: Point3<S>) -> Aabb3<S> {
        Aabb3 {
            min: Point3::new(min(p1.x.clone(), p2.x.clone()), min(p1.y.clone(), p2.y.clone()), min(p1.z.clone(), p2.z.clone())),
            max: Point3::new(max(p1.x.clone(), p2.x.clone()), max(p1.y.clone(), p2.y.clone()), max(p1.z.clone(), p2.z.clone())),
        }
    }
}

impl<S: Primitive> Aabb<S, Vec3<S>, Point3<S>, [S, ..3]> for Aabb3<S> {
    fn new(p1: Point3<S>, p2: Point3<S>) -> Aabb3<S> { Aabb3::new(p1, p2) }
    #[inline] fn min<'a>(&'a self) -> &'a Point3<S> { &self.min }
    #[inline] fn max<'a>(&'a self) -> &'a Point3<S> { &self.max }
}

impl<S: fmt::Show> ToStr for Aabb3<S> {
    fn to_str(&self) -> ~str {
        format!("[{} - {}]", self.min.to_str(), self.max.to_str())
    }
}
