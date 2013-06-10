// Copyright 2013 The Lmath Developers. For a full listing of the authors,
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

use mat::*;
use quat::*;
use vec::*;

// TODO

#[test]
fn test_quat() {
    let a = Quat { s: 1.0, v: Vec3 { x: 2.0, y: 3.0, z: 4.0 } };

    assert_eq!(a, Quat::from_sv::<float>(1.0, Vec3::new::<float>(2.0, 3.0, 4.0)));
    assert_eq!(a, Quat::new::<float>(1.0, 2.0, 3.0, 4.0));

    assert_eq!(Quat::zero::<float>(), Quat::new::<float>(0.0, 0.0, 0.0, 0.0));
    assert_eq!(Quat::identity::<float>(), Quat::new::<float>(1.0, 0.0, 0.0, 0.0));

    assert_eq!(a.s, 1.0);
    assert_eq!(a.v.x, 2.0);
    assert_eq!(a.v.y, 3.0);
    assert_eq!(a.v.z, 4.0);
    assert_eq!(*a.index(0), 1.0);
    assert_eq!(*a.index(1), 2.0);
    assert_eq!(*a.index(2), 3.0);
    assert_eq!(*a.index(3), 4.0);
    // TODO
}

#[test]
fn test_quat_2() {
    let v = Vec3::new(1f32, 0f32, 0f32);

    let q = Quat::from_angle_axis((-45f32).to_radians(), &Vec3::new(0f32, 0f32, -1f32));

    // http://www.wolframalpha.com/input/?i={1,0}+rotate+-45+degrees
    assert_approx_eq!(q.mul_v(&v), Vec3::new(1f32/2f32.sqrt(), 1f32/2f32.sqrt(), 0f32));
    assert_eq!(q.mul_v(&v).length(), v.length());
    assert_approx_eq!(q.to_mat3(), Mat3::new( 1f32/2f32.sqrt(), 1f32/2f32.sqrt(), 0f32,
                                             -1f32/2f32.sqrt(), 1f32/2f32.sqrt(), 0f32,
                                                          0f32,             0f32, 1f32));
}

#[test]
fn test_quat_approx_eq() {
    assert!(!Quat::new::<float>(0.000001, 0.000001, 0.000001, 0.000001)
            .approx_eq(&Quat::new::<float>(0.0, 0.0, 0.0, 0.0)));
    assert!(Quat::new::<float>(0.0000001, 0.0000001, 0.0000001, 0.0000001)
            .approx_eq(&Quat::new::<float>(0.0, 0.0, 0.0, 0.0)));
}