// Copyright 2014 The CGMath Developers. For a full listing of the authors,
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

#![feature(globs)]

extern crate cgmath;

use cgmath::{Aabb, Aabb2, Aabb3};
use cgmath::{Point2, Point3};
use cgmath::{Vector2, Vector3};
use cgmath::{Ray, Intersect};

#[test]
fn test_aabb() {
    let aabb = Aabb2::new(Point2::new(-20i, 30i), Point2::new(10i, -10i));
    assert_eq!(aabb.min(), &Point2::new(-20i, -10i));
    assert_eq!(aabb.max(), &Point2::new(10i, 30i));
    assert_eq!(aabb.dim(), Vector2::new(30i, 40i));
    assert_eq!(aabb.volume(), 30i * 40i);
    assert_eq!(aabb.center(), Point2::new(-5i, 10i));

    assert!(aabb.contains(&Point2::new(0i, 0i)));
    assert!(!aabb.contains(&Point2::new(-50i, -50i)));
    assert!(!aabb.contains(&Point2::new(50i, 50i)));

    assert_eq!(aabb.grow(&Point2::new(0i, 0i)), aabb);
    assert_eq!(aabb.grow(&Point2::new(100i, 100i)),
        Aabb2::new(Point2::new(-20i, -10i), Point2::new(100i, 100i)));
    assert_eq!(aabb.grow(&Point2::new(-100i, -100i)),
        Aabb2::new(Point2::new(-100i, -100i), Point2::new(10i, 30i)));

    let aabb = Aabb3::new(Point3::new(-20i, 30i, 5i), Point3::new(10i, -10i, -5i));
    assert_eq!(aabb.min(), &Point3::new(-20i, -10i, -5i));
    assert_eq!(aabb.max(), &Point3::new(10i, 30i, 5i));
    assert_eq!(aabb.dim(), Vector3::new(30i, 40i, 10i));
    assert_eq!(aabb.volume(), 30i * 40i * 10i);
    assert_eq!(aabb.center(), Point3::new(-5i, 10i, 0i));

    assert!(aabb.contains(&Point3::new(0i, 0i, 0i)));
    assert!(!aabb.contains(&Point3::new(-100i, 0i, 0i)));
    assert!(!aabb.contains(&Point3::new(100i, 0i, 0i)));
    assert!(aabb.contains(&Point3::new(9i, 29i, -1i)));
    assert!(!aabb.contains(&Point3::new(10i, 30i, 5i)));
    assert!(aabb.contains(&Point3::new(-20i, -10i, -5i)));
    assert!(!aabb.contains(&Point3::new(-21i, -11i, -6i)));

    assert_eq!(aabb.add_v(&Vector3::new(1i, 2i, 3i)),
        Aabb3::new(Point3::new(-19i, 32i, 8i), Point3::new(11i, -8i, -2i)));

    assert_eq!(aabb.mul_s(2i),
        Aabb3::new(Point3::new(-40i, -20i, -10i), Point3::new(20i, 60i, 10i)));

    assert_eq!(aabb.mul_v(&Vector3::new(1i, 2i, 3i)),
        Aabb3::new(Point3::new(-20i, -20i, -15i), Point3::new(10i, 60i, 15i)));
}

#[test]
fn test_aabb_ray_intersect() {
    let aabb = Aabb2::new(Point2::new(-5.0f32, 5.0), Point2::new(5.0, 10.0));
    let ray1 = Ray::new(Point2::new(0.0f32, 0.0), Vector2::new(0.0, 1.0));
    let ray2 = Ray::new(Point2::new(-10.0f32, 0.0), Vector2::new(2.5, 1.0));
    let ray3 = Ray::new(Point2::new(0.0f32, 0.0), Vector2::new(-1.0, -1.0));
    let ray4 = Ray::new(Point2::new(3.0f32, 7.0), Vector2::new(1.0, 1.0));

    assert_eq!((ray1, aabb).intersection(), Some(Point2::new(0.0, 5.0)));
    assert_eq!((ray2, aabb).intersection(), Some(Point2::new(2.5, 5.0)));
    assert_eq!((ray3, aabb).intersection(), None);
    assert_eq!((ray4, aabb).intersection(), Some(Point2::new(5.0, 9.0)));
}
