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

//! Generic spatial bounds.

use aabb::{Aabb, Aabb3};
use num::BaseNum;
use plane::Plane;
use point::{Point, Point3};
use sphere::Sphere;

/// Spatial relation between two objects.
pub enum Relation {
	/// Completely inside
	In,
	/// Crosses the boundary
	Cross,
	/// Completely outside
	Out,
}

/// Generic bound.
pub trait Bound<S> {
	/// Classify the spatial relation with a plane
	fn relate(&self, &Plane<S>) -> Relation;
}

impl<S: BaseNum> Bound<S> for Point3<S> {
	fn relate(&self, plane: &Plane<S>) -> Relation {
		let dist = self.dot(&plane.n);
		if dist > plane.d {
			Relation::In
		}else if dist < plane.d {
			Relation::Out
		}else {
			Relation::Cross
		}
	}
}

impl<S: BaseNum> Bound<S> for Aabb3<S> {
	fn relate(&self, plane: &Plane<S>) -> Relation {
		match (self.min.relate(plane), self.max.relate(plane)) {
			(Relation::In, Relation::In) => Relation::In,
			(Relation::Out, Relation::Out) => Relation::Out,
			(_, _) => Relation::Cross,
		}
	}
}

impl<S: BaseNum> Bound<S> for Sphere<S> {
	fn relate(&self, plane: &Plane<S>) -> Relation {
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
