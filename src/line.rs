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

//! Line segments

use std::marker::PhantomData;

use rust_num::{Zero, One};

use num::{BaseNum, BaseFloat};
use point::{Point, Point2, Point3};
use vector::{Vector, Vector2, Vector3};
use ray::{Ray2};
use intersect::Intersect;

/// A generic directed line segment from `origin` to `dest`.
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Line<S, V, P> {
    pub origin: P,
    pub dest: P,
    phantom_s: PhantomData<S>,
    phantom_v: PhantomData<V>
}

impl<S: BaseNum, V: Vector<S>, P: Point<S, V>>  Line<S, V, P> {
    pub fn new(origin: P, dest: P) -> Line<S, V, P> {
        Line {
            origin: origin,
            dest: dest,
            phantom_v: PhantomData,
            phantom_s: PhantomData
        }
    }
}

pub type Line2<S> = Line<S, Vector2<S>, Point2<S>>;
pub type Line3<S> = Line<S, Vector3<S>, Point3<S>>;

/// Determines if an intersection between a ray and a line segment is found.
impl<S: BaseFloat> Intersect<Option<Point2<S>>> for (Ray2<S>, Line2<S>) {
    fn intersection(&self) -> Option<Point2<S>> {
        let (ref ray, ref line) = *self;

        let p = ray.origin;
        let q = line.origin;
        let r = ray.direction;
        let s = Vector2::new(line.dest.x - line.origin.x, line.dest.y - line.origin.y);

        let cross_1 = r.perp_dot(&s);
        let qmp = Vector2::new(q.x - p.x, q.y - p.y);
        let cross_2 = qmp.perp_dot(&r);

        if cross_1 == S::zero() {
            if cross_2 != S::zero() {
                // parallel
                return None;
            }

            // collinear
            let q2mp = Vector2::new(line.dest.x - p.x, line.dest.y - p.y);
            let dot_1 = qmp.dot(&r);
            let dot_2 = q2mp.dot(&r);
            if (dot_1 <= S::zero() && dot_2 >= S::zero()) || (dot_1 >= S::zero() && dot_2 <= S::zero()) {
                return Some(p);
            }
            else if dot_1 >= S::zero() && dot_2 >= S::zero() {
                if dot_1 <= dot_2 {
                    return Some(q);
                }
                else {
                    return Some(line.dest);
                }
            }

            // no overlap exists
            return None;
        }

        let t = qmp.perp_dot(&s) / cross_1;
        let u = cross_2 / cross_1;

        if S::zero() <= t && u >= S::zero() && u <= S::one() {
            return Some(Point2::new(p.x + t*r.x, p.y + t*r.y));
        }

        return None;
    }
}
