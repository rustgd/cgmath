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

use cgmath::quaternion::*;
use cgmath::transform::*;
use cgmath::vector::*;
use cgmath::approx::ApproxEq;
use cgmath::matrix::*;
use cgmath::angle::*;

#[test]
fn test_invert() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let t = Transform3D::new(1.5, Quat::new(0.5,0.5,0.5,0.5), Vec3::new(6.0,-7.0,8.0));
    let ti = t.get().invert().expect("Expected successful inversion");
    let vt = t.get().transform_vec( &v );
    assert!(v.approx_eq( &ti.transform_vec( &vt ) ));
}

#[test]
fn test_translate_only() {
    let v = Vec4::new(10., 0., 0., 1.);
    let t = Transform3D::new(100.0, Quat::from_angle_z(Rad::turn_div_4()), Vec3::new(6.0,-7.0,8.0));
    let t_mat = t.translate().to_mat4();

    let v_out = t_mat.mul_v(&v);
    let v_out = v_out.mul_s(1./v_out.w);
    let v_exp = Vec4::new(16., -7., 8., 1.);

    assert!(v_exp.approx_eq(&v_out));
}

#[test]
fn test_scale_only() {
    let v = Vec4::new(1., -2., 3., 1.);
    let t = Transform3D::new(10.0, Quat::from_angle_z(Rad::turn_div_4()), Vec3::new(6.0,-7.0,8.0));
    let t_mat = t.scale().to_mat4();

    let v_out = t_mat.mul_v(&v);
    let v_out = v_out.mul_s(1./v_out.w);
    let v_exp = Vec4::new(10., -20., 30., 1.);

    assert!(v_exp.approx_eq(&v_out));
}


#[test]
fn test_rotate_only() {
    let v = Vec4::new(1., -2., 3., 1.);
    let t = Transform3D::new(10.0,
                             Quat::from_euler(-Rad::turn_div_4(), Rad::zero(), Rad::zero()),
                             Vec3::new(6.0,-7.0,8.0));
    let t_mat = t.rotate().to_mat4();

    let v_out = t_mat.mul_v(&v);
    let v_out = v_out.mul_s(1./v_out.w);
    let v_exp = Vec4::new(-3., -2., 1., 1.);

    assert!(v_exp.approx_eq(&v_out));
}

#[test]
fn test_rotate_scale_translate_only() {
    let v = Vec4::new(1., -2., 3., 1.);
    let t = Transform3D::new(2.0,
                             Quat::from_euler(-Rad::turn_div_4(), Rad::zero(), Rad::zero()),
                             Vec3::new(6.0,-7.0,8.0));
    let t_mat = t.translate().scale().rotate().to_mat4();
    let t_mat2 = t.get().to_mat4();

    let v_out = t_mat.mul_v(&v);
    let v_out = v_out.mul_s(1./v_out.w);

    let v_out2 = t_mat2.mul_v(&v);
    let v_out2 = v_out2.mul_s(1./v_out2.w);

    assert!(v_out2.approx_eq(&v_out));
}