// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directionectory of this distribution.
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

use std::num::{Zero, zero, One, one};

use point::{Point, Point2, Point3};
use vector::{Vector, Vector2};
use partial_ord::PartOrdFloat;
use intersect::Intersect;

/// A generic directed line segment from `origin` to `dest`.
#[deriving(Clone, Eq)]
pub struct Line<P>
{
    pub origin: P,
    pub dest: P,
}

impl
<
    S: Primitive,
    Slice,
    V: Vector<S,Slice>,
    P: Point<S,V,Slice>
>  Line<P>
{
    pub fn new(origin: P, dest: P) -> Line<P> {
        Line { origin:origin, dest:dest }
    }
}

pub type Line2<S> = Line<Point2<S>>;
pub type Line3<S> = Line<Point3<S>>;

/// Determines if an intersection between two line segments is found. If the segments are
/// collinear and overlapping, the intersection point that will be returned will be the first
/// intersection point found by traversing the first line segment, starting at its origin.
impl<S: PartOrdFloat<S>> Intersect<Option<Point2<S>>> for (Line2<S>, Line2<S>) {
    fn intersection(&self) -> Option<Point2<S>> {
        match *self {
            (ref l1, ref l2) => {
                let p = l1.origin;
                let mut q = l2.origin;
                let r = Vector2::new(l1.dest.x - l1.origin.x, l1.dest.y - l1.origin.y);
                let mut s = Vector2::new(l2.dest.x - l2.origin.x, l2.dest.y - l2.origin.y);
                let zero: S = Zero::zero();

                let cross = r.perp_dot(&s);
                let mut q_minus_p = Vector2::new(q.x - p.x, q.y - p.y);
                let mut p_minus_q = Vector2::new(p.x - q.x, p.y - q.y);
                let cross_r = q_minus_p.perp_dot(&r);
                let cross_s = q_minus_p.perp_dot(&s);

                if cross.is_zero() {
                    if cross_r.is_zero() {
                        // line segments are collinear

                        // special case of both lines being the same single point
                        if r.x == zero && r.y == zero && s.x == zero && s.y == zero && p == q {
                            return Some(p);
                        }

                        // ensure l2 (q,q+s) is pointing the same direction as l1 (p,p+r)
                        // if it is not, then swap the two endpoints of the second segment.
                        // If this is not done, the algorithm below will not find an
                        // intersection if the two directed line segments  point towards the
                        // opposite segment's origin.
                        if (r.x != zero && s.x != zero && r.x.signum() != s.x.signum()) ||
                            (r.y != zero && s.y != zero && r.y.signum() != s.y.signum()) {
                            q = Point2::new(q.x + s.x, q.y + s.y);
                            s = Vector2::new(-s.x, -s.y);
                            q_minus_p = Vector2::new(q.x - p.x, q.y - p.y);
                            p_minus_q = Vector2::new(p.x - q.x, p.y - q.y);
                        }
                        let d1 = q_minus_p.dot(&r);
                        let d2 = p_minus_q.dot(&s);
                        let rdotr = r.dot(&r);
                        let sdots = s.dot(&s);

                        // make sure to take into account that one or both of the segments could
                        // be a single point (r.r or s.s = 0) and ignore that case
                        if (rdotr > zero && zero <= d1 && d1 <= rdotr) ||
                            (sdots > zero && zero <= d2 && d2 <= sdots) {
                            // overlapping
                            if (q_minus_p.x != zero && q_minus_p.x.signum() == r.x.signum()) ||
                               (q_minus_p.y != zero && q_minus_p.y.signum() == r.y.signum()) {
                                return Some(q);
                            }
                            return Some(p);
                        }
                    }
                    return None;
                }

                let t = cross_s / cross;
                let u = cross_r / cross;

                if zero <= t && t <= One::one() &&
                    zero <= One::one() && u <= One::one() {
                    return Some(Point2::new(p.x + t*r.x, p.y + t*r.y));
                }

                return None;
            }
        }
    }
}
