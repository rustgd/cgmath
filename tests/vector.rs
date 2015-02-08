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

#![feature(core)]

#[macro_use]
extern crate cgmath;

use cgmath::*;
use std::f64;
use std::num::Float;

#[test]
fn test_constructor() {
    assert_eq!(vec2(1f32, 2f32), Vector2::new(1f32, 2f32));
    assert_eq!(vec3(1f64, 2f64, 3f64), Vector3::new(1f64, 2f64, 3f64));
    assert_eq!(vec4(1is, 2is, 3is, 4is), Vector4::new(1is, 2is, 3is, 4is));
}

#[test]
fn test_from_value() {
    assert_eq!(Vector2::from_value(102is), Vector2::new(102is, 102is));
    assert_eq!(Vector3::from_value(22is), Vector3::new(22is, 22is, 22is));
    assert_eq!(Vector4::from_value(76.5f64), Vector4::new(76.5f64, 76.5f64, 76.5f64, 76.5f64));
}

#[test]
fn test_dot() {
    assert_eq!(Vector2::new(1is, 2is).dot(&Vector2::new(3is, 4is)), 11is);
    assert_eq!(Vector3::new(1is, 2is, 3is).dot(&Vector3::new(4is, 5is, 6is)), 32is);
    assert_eq!(Vector4::new(1is, 2is, 3is, 4is).dot(&Vector4::new(5is, 6is, 7is, 8is)), 70is);
}

#[test]
fn test_comp_add() {
    assert_eq!(Vector2::new(1is, 2is).comp_add(), 3is);
    assert_eq!(Vector3::new(1is, 2is, 3is).comp_add(), 6is);
    assert_eq!(Vector4::new(1is, 2is, 3is, 4is).comp_add(), 10is);

    assert_eq!(Vector2::new(3.0f64, 4.0f64).comp_add(), 7.0f64);
    assert_eq!(Vector3::new(4.0f64, 5.0f64, 6.0f64).comp_add(), 15.0f64);
    assert_eq!(Vector4::new(5.0f64, 6.0f64, 7.0f64, 8.0f64).comp_add(), 26.0f64);
}

#[test]
fn test_comp_mul() {
    assert_eq!(Vector2::new(1is, 2is).comp_mul(), 2is);
    assert_eq!(Vector3::new(1is, 2is, 3is).comp_mul(), 6is);
    assert_eq!(Vector4::new(1is, 2is, 3is, 4is).comp_mul(), 24is);

    assert_eq!(Vector2::new(3.0f64, 4.0f64).comp_mul(), 12.0f64);
    assert_eq!(Vector3::new(4.0f64, 5.0f64, 6.0f64).comp_mul(), 120.0f64);
    assert_eq!(Vector4::new(5.0f64, 6.0f64, 7.0f64, 8.0f64).comp_mul(), 1680.0f64);
}

#[test]
fn test_comp_min() {
    assert_eq!(Vector2::new(1is, 2is).comp_min(), 1is);
    assert_eq!(Vector3::new(1is, 2is, 3is).comp_min(), 1is);
    assert_eq!(Vector4::new(1is, 2is, 3is, 4is).comp_min(), 1is);

    assert_eq!(Vector2::new(3.0f64, 4.0f64).comp_min(), 3.0f64);
    assert_eq!(Vector3::new(4.0f64, 5.0f64, 6.0f64).comp_min(), 4.0f64);
    assert_eq!(Vector4::new(5.0f64, 6.0f64, 7.0f64, 8.0f64).comp_min(), 5.0f64);
}

#[test]
fn test_comp_max() {
    assert_eq!(Vector2::new(1is, 2is).comp_max(), 2is);
    assert_eq!(Vector3::new(1is, 2is, 3is).comp_max(), 3is);
    assert_eq!(Vector4::new(1is, 2is, 3is, 4is).comp_max(), 4is);

    assert_eq!(Vector2::new(3.0f64, 4.0f64).comp_max(), 4.0f64);
    assert_eq!(Vector3::new(4.0f64, 5.0f64, 6.0f64).comp_max(), 6.0f64);
    assert_eq!(Vector4::new(5.0f64, 6.0f64, 7.0f64, 8.0f64).comp_max(), 8.0f64);
}

#[test]
fn test_cross() {
    let a = Vector3::new(1is, 2is, 3is);
    let b = Vector3::new(4is, 5is, 6is);
    let r = Vector3::new(-3is, 6is, -3is);
    assert_eq!(a.cross(&b), r);

    let mut a = a;
    a.cross_self(&b);
    assert_eq!(a, r);
}

#[test]
fn test_is_perpendicular() {
    assert!(Vector2::new(1.0f64, 0.0f64).is_perpendicular(&Vector2::new(0.0f64, 1.0f64)));
    assert!(Vector3::new(0.0f64, 1.0f64, 0.0f64).is_perpendicular(&Vector3::new(0.0f64, 0.0f64, 1.0f64)));
    assert!(Vector4::new(1.0f64, 0.0f64, 0.0f64, 0.0f64).is_perpendicular(&Vector4::new(0.0f64, 0.0f64, 0.0f64, 1.0f64)));
}

#[cfg(test)]
mod test_length {
    use cgmath::*;

    #[test]
    fn test_vector2(){
        let (a, a_res) = (Vector2::new(3.0f64, 4.0f64), 5.0f64); // (3is, 4is, 5is) Pythagorean triple
        let (b, b_res) = (Vector2::new(5.0f64, 12.0f64), 13.0f64); // (5is, 12is, 13is) Pythagorean triple

        assert_eq!(a.length2(), a_res * a_res);
        assert_eq!(b.length2(), b_res * b_res);

        assert_eq!(a.length(), a_res);
        assert_eq!(b.length(), b_res);
    }

    #[test]
    fn test_vector3(){
        let (a, a_res) = (Vector3::new(2.0f64, 3.0f64, 6.0f64), 7.0f64); // (2is, 3is, 6is, 7is) Pythagorean quadruple
        let (b, b_res) = (Vector3::new(1.0f64, 4.0f64, 8.0f64), 9.0f64); // (1is, 4is, 8is, 9is) Pythagorean quadruple

        assert_eq!(a.length2(), a_res * a_res);
        assert_eq!(b.length2(), b_res * b_res);

        assert_eq!(a.length(), a_res);
        assert_eq!(b.length(), b_res);
    }

    #[test]
    fn test_vector4(){
        let (a, a_res) = (Vector4::new(1.0f64, 2.0f64, 4.0f64, 10.0f64), 11.0f64); // (1is, 2is, 4is, 10is, 11is) Pythagorean quintuple
        let (b, b_res) = (Vector4::new(1.0f64, 2.0f64, 8.0f64, 10.0f64), 13.0f64); // (1is, 2is, 8is, 10is, 13is) Pythagorean quintuple

        assert_eq!(a.length2(), a_res * a_res);
        assert_eq!(b.length2(), b_res * b_res);

        assert_eq!(a.length(), a_res);
        assert_eq!(b.length(), b_res);
    }
}

#[test]
fn test_angle() {
    assert!(Vector2::new(1.0f64, 0.0f64).angle(&Vector2::new(0.0f64, 1.0f64)).approx_eq( &rad(f64::consts::FRAC_PI_2) ));
    assert!(Vector2::new(10.0f64, 0.0f64).angle(&Vector2::new(0.0f64, 5.0f64)).approx_eq( &rad(f64::consts::FRAC_PI_2) ));
    assert!(Vector2::new(-1.0f64, 0.0f64).angle(&Vector2::new(0.0f64, 1.0f64)).approx_eq( &-rad(f64::consts::FRAC_PI_2) ));

    assert!(Vector3::new(1.0f64, 0.0f64, 1.0f64).angle(&Vector3::new(1.0f64, 1.0f64, 0.0f64)).approx_eq( &rad(f64::consts::FRAC_PI_3) ));
    assert!(Vector3::new(10.0f64, 0.0f64, 10.0f64).angle(&Vector3::new(5.0f64, 5.0f64, 0.0f64)).approx_eq( &rad(f64::consts::FRAC_PI_3) ));
    assert!(Vector3::new(-1.0f64, 0.0f64, -1.0f64).angle(&Vector3::new(1.0f64, -1.0f64, 0.0f64)).approx_eq( &rad(2.0f64 * f64::consts::FRAC_PI_3) ));

    assert!(Vector4::new(1.0f64, 0.0f64, 1.0f64, 0.0f64).angle(&Vector4::new(0.0f64, 1.0f64, 0.0f64, 1.0f64)).approx_eq( &rad(f64::consts::FRAC_PI_2) ));
    assert!(Vector4::new(10.0f64, 0.0f64, 10.0f64, 0.0f64).angle(&Vector4::new(0.0f64, 5.0f64, 0.0f64, 5.0f64)).approx_eq( &rad(f64::consts::FRAC_PI_2) ));
    assert!(Vector4::new(-1.0f64, 0.0f64, -1.0f64, 0.0f64).angle(&Vector4::new(0.0f64, 1.0f64, 0.0f64, 1.0f64)).approx_eq( &rad(f64::consts::FRAC_PI_2) ));
}

#[test]
fn test_normalize() {
    // TODO: test normalize_to, normalize_sel.0, and normalize_self_to
    assert!(Vector2::new(3.0f64, 4.0f64).normalize().approx_eq( &Vector2::new(3.0/5.0, 4.0/5.0) ));
    assert!(Vector3::new(2.0f64, 3.0f64, 6.0f64).normalize().approx_eq( &Vector3::new(2.0/7.0, 3.0/7.0, 6.0/7.0) ));
    assert!(Vector4::new(1.0f64, 2.0f64, 4.0f64, 10.0f64).normalize().approx_eq( &Vector4::new(1.0/11.0, 2.0/11.0, 4.0/11.0, 10.0/11.0) ));
}

#[test]
fn test_map() {
    assert_eq!(Vector3::new(7.12f64, 3.8f64, -6.98f64).map(|x| x.floor()), Vector3::new(7.0f64, 3.0f64, -7.0f64));
    assert_eq!(Vector3::new(7.12f64, 3.8f64, -6.98f64).map(|x| x.max(0.0f64)), Vector3::new(7.12f64, 3.8f64, 0.0f64));
}

#[test]
fn test_cast() {
    assert_approx_eq!(Vector2::new(0.9f64, 1.5).cast(), Vector2::new(0.9f32, 1.5));
    assert_approx_eq!(Vector3::new(1.0f64, 2.4, -3.13).cast(), Vector3::new(1.0f32, 2.4, -3.13));
    assert_approx_eq!(Vector4::new(13.5f64, -4.6, -8.3, 2.41).cast(), Vector4::new(13.5f32, -4.6, -8.3, 2.41));
}
