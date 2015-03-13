// Copyright 2013-2015 The CGMath Developers. For a full listing of the authors,
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

extern crate cgmath;

use cgmath::{Aabb3, Bound, Plane, Point, Point3, Relation, Sphere, Vector, vec3};

#[test]
fn point() {
    let point = Point3::new(1f32, 2.0, 3.0);
    let normal = vec3(0f32, -0.8, -0.36);
    let plane = Plane::from_point_normal(point, normal);

    assert_eq!(point.relate(&plane), Relation::Cross);
    assert_eq!(point.add_v(&normal).relate(&plane), Relation::In);
    assert_eq!(point.add_v(&normal.mul_s(-1.0)).relate(&plane), Relation::Out);
}

#[test]
fn sphere() {
    let point = Point3::new(1f32, 2.0, 3.0);
    let sphere = Sphere { center: point, radius: 1.0 };
    let normal = vec3(0f32, 0.0, 1.0);

    assert_eq!(sphere.relate(
        &Plane::from_point_normal(point, normal)
        ), Relation::Cross);
    assert_eq!(sphere.relate(
        &Plane::from_point_normal(point.add_v(&normal.mul_s(-3.0)), normal),
        ), Relation::In);
    assert_eq!(sphere.relate(
        &Plane::from_point_normal(point.add_v(&normal.mul_s(3.0)), normal),
        ), Relation::Out);
}

#[test]
fn aabb() {
    //TODO
}
