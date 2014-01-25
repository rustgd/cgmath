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

use cgmath::plane::*;
use cgmath::point::*;
use cgmath::vector::*;
use cgmath::ray::*;
use cgmath::intersect::Intersect;

#[test]
fn test_from_points() {
    assert_eq!(Plane::from_points(Point3::new(5.0, 0.0,  5.0),
                                  Point3::new(5.0, 5.0,  5.0),
                                  Point3::new(5.0, 0.0, -1.0)),
    	Some(Plane::from_abcd(-1.0, 0.0, 0.0, 5.0)));

    assert_eq!(Plane::from_points(Point3::new(0.0, 5.0, -5.0),
                                  Point3::new(0.0, 5.0,  0.0),
                                  Point3::new(0.0, 5.0,  5.0)),
        None);     // The points are parallel
}

#[test]
fn test_ray_intersection() {
    let p0 = Plane::from_abcd(1f64, 0f64, 0f64, -7f64);
    let r0: Ray3<f64> = Ray::new(Point3::new(2f64, 3f64, 4f64), Vec3::new(1f64, 1f64, 1f64).normalize());
    assert_eq!((p0, r0).intersection(), Some(Point3::new(7f64, 8f64, 9f64)));

    let p1 = Plane::from_points(Point3::new(5f64, 0f64,  5f64),
                                  Point3::new(5f64, 5f64,  5f64),
                                  Point3::new(5f64, 0f64, -1f64)).unwrap();
    let r1: Ray3<f64> = Ray::new(Point3::new(0f64, 0f64, 0f64), Vec3::new(-1f64, 0f64, 0f64).normalize());
    assert_eq!((p1, r1).intersection(), None); // r1 points away from p1
}
