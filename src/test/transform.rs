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

use cgmath::quaternion::*;
use cgmath::transform::*;
use cgmath::point::*;
use cgmath::vector::*;
use cgmath::approx::ApproxEq;

#[test]
fn test_invert() {
	let v = Vec3::new(1.0, 2.0, 3.0);
	let t = Transform3D::new(1.5, Quat::new(0.5,0.5,0.5,0.5), Vec3::new(6.0,-7.0,8.0));
	let ti = t.get().invert().expect("Expected successful inversion");
	let vt = t.get().transform_vec( &v );
    assert!(v.approx_eq( &ti.transform_vec( &vt ) ));
}

#[test]
fn test_look_at() {
	let eye = Point3::new(0.0, 0.0, -5.0);
	let center = Point3::new(0.0, 0.0, 0.0);
	let up = Vec3::new(1.0, 0.0, 0.0);
	let t: Decomposed<f64,Vec3<f64>,Quat<f64>> = Transform::look_at(&eye, &center, &up);
	let point = Point3::new(1.0, 0.0, 0.0);
	let view_point = Point3::new(0.0, 1.0, 5.0);
	assert!( t.transform_point(&point).approx_eq(&view_point) );
}
