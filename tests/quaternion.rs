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


extern crate cgmath;

use cgmath::{ToMatrix4, ToMatrix3};
use cgmath::Quaternion;

use cgmath::{Rad, rad, ApproxEq};
use cgmath::Rotation3;

use std::f32;

#[test]
fn to_matrix4()
{
    let quaternion = Quaternion::new(2f32, 3f32, 4f32, 5f32);

    let matrix_short = quaternion.to_matrix4();
    let matrix_long = quaternion.to_matrix3().to_matrix4();

    assert!(matrix_short == matrix_long);
}

#[test]
fn to_and_from_quaternion()
{
    fn eq(a: (Rad<f32>, Rad<f32>, Rad<f32>), b: (Rad<f32>, Rad<f32>, Rad<f32>)) {
        let (ax, ay, az) = a;
        let (bx, by, bz) = b;
        if !(ax.approx_eq_eps(&bx, &0.001) &&
             ay.approx_eq_eps(&by, &0.001) &&
             az.approx_eq_eps(&bz, &0.001)) {
            panic!("{:?} != {:?}", a, b)
        }
    }

    let hpi = f32::consts::FRAC_PI_2;

    let zero: Quaternion<f32> = Rotation3::from_euler(rad(0f32), rad(0f32), rad(0f32));
    eq((rad(0f32), rad(0f32), rad(0f32)), zero.to_euler());

    let x_1: Quaternion<f32> = Rotation3::from_euler(rad(1f32), rad(0f32), rad(0f32));
    eq((rad(1f32), rad(0f32), rad(0f32)), x_1.to_euler());

    let y_1: Quaternion<f32> = Rotation3::from_euler(rad(0f32), rad(1f32), rad(0f32));
    eq((rad(0f32), rad(1f32), rad(0f32)), y_1.to_euler());

    let z_1: Quaternion<f32> = Rotation3::from_euler(rad(0f32), rad(0f32), rad(1f32));
    eq((rad(0f32), rad(0f32), rad(1f32)), z_1.to_euler());

    let x_n1: Quaternion<f32> = Rotation3::from_euler(rad(-1f32), rad(0f32), rad(0f32));
    eq((rad(-1f32), rad(0f32), rad(0f32)), x_n1.to_euler());

    let y_n1: Quaternion<f32> = Rotation3::from_euler(rad(0f32), rad(-1f32), rad(0f32));
    eq((rad(0f32), rad(-1f32), rad(0f32)), y_n1.to_euler());

    let z_n1: Quaternion<f32> = Rotation3::from_euler(rad(0f32), rad(0f32), rad(-1f32));
    eq((rad(0f32), rad(0f32), rad(-1f32)), z_n1.to_euler());

    let xzy_1: Quaternion<f32> = Rotation3::from_euler(rad(1f32), rad(1f32), rad(1f32));
    eq((rad(1f32), rad(1f32), rad(1f32)), xzy_1.to_euler());

    let xzy_n1: Quaternion<f32> = Rotation3::from_euler(rad(-1f32), rad(-1f32), rad(-1f32));
    eq((rad(-1f32), rad(-1f32), rad(-1f32)), xzy_n1.to_euler());

    let xzy_hp: Quaternion<f32> = Rotation3::from_euler(rad(0f32), rad(hpi), rad(1f32));
    eq((rad(0f32), rad(hpi), rad(1f32)), xzy_hp.to_euler());

    let xzy_nhp: Quaternion<f32> = Rotation3::from_euler(rad(0f32), rad(-hpi), rad(1f32));
    eq((rad(0f32), rad(-hpi), rad(1f32)), xzy_nhp.to_euler());

}
