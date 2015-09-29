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

//! Bounding sphere

use rust_num::Zero;

use bound::*;
use intersect::Intersect;
use num::BaseFloat;
use point::{Point, Point3};
use plane::Plane;
use ray::Ray3;
use vector::Vector;

#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Sphere<S> {
    pub center: Point3<S>,
    pub radius: S,
}

impl<S: BaseFloat> Intersect<Option<Point3<S>>> for (Sphere<S>, Ray3<S>) {
    fn intersection(&self) -> Option<Point3<S>> {
        let (ref s, ref r) = *self;

        let l = s.center.sub_p(&r.origin);
        let tca = l.dot(&r.direction);
        if tca < S::zero() { return None; }
        let d2 = l.dot(&l) - tca*tca;
        if d2 > s.radius*s.radius { return None; }
        let thc = (s.radius*s.radius - d2).sqrt();
        Some(r.origin.add_v(&r.direction.mul_s(tca - thc)))
    }
}

impl<S: BaseFloat + 'static> Bound<S> for Sphere<S> {
    fn relate_plane(&self, plane: &Plane<S>) -> Relation {
        let dist = self.center.dot(&plane.n) - plane.d;
        if dist > self.radius {
            Relation::In
        }else if dist < - self.radius {
            Relation::Out
        }else {
            Relation::Cross
        }
    }
}
