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


extern crate cgmath;

use cgmath::*;
use std::num::Float;

#[test]
fn test_intersection() {
    let sphere = Sphere {center: Point3::new(0f64,0f64,0f64), radius: 1f64};
    let r0 = Ray::new(Point3::new(0f64, 0f64, 5f64), Vector3::new(0f64, 0f64, -5f64).normalize());
    let r1 = Ray::new(Point3::new(1f64.cos(), 0f64, 5f64), Vector3::new(0f64, 0f64, -5f64).normalize());
    let r2 = Ray::new(Point3::new(1f64, 0f64, 5f64), Vector3::new(0f64, 0f64, -5f64).normalize());
    let r3 = Ray::new(Point3::new(2f64, 0f64, 5f64), Vector3::new(0f64, 0f64, -5f64).normalize());
    assert_eq!((sphere,r0).intersection(), Some(Point3::new(0f64, 0f64, 1f64)));
    assert!((sphere,r1).intersection().unwrap().approx_eq( &Point3::new(1f64.cos(), 0f64, 1f64.sin()) ));
    assert_eq!((sphere,r2).intersection(), Some(Point3::new(1f64, 0f64, 0f64)));
    assert_eq!((sphere,r3).intersection(), None);
}
