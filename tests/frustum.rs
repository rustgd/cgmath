// Copyright 2015 The CGMath Developers. For a full listing of the authors,
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

use cgmath::{PerspectiveFov, Point3, Projection, Relation, Sphere, rad};

#[test]
fn test_contains() {
    let frustum = PerspectiveFov {
        fovy: rad(1f32),
        aspect: 1f32,
        near: 1f32,
        far: 10f32,
    }.to_frustum();
    assert_eq!(frustum.contains(&Sphere {
            center: Point3::new(0f32, 0f32, -5f32),
            radius: 1f32,
        }), Relation::In);
    assert_eq!(frustum.contains(&Sphere {
            center: Point3::new(0f32, 3f32, -5f32),
            radius: 1f32,
        }), Relation::Cross);
    assert_eq!(frustum.contains(&Sphere {
            center: Point3::new(0f32, 0f32, 5f32),
            radius: 1f32,
        }), Relation::Out);
}
