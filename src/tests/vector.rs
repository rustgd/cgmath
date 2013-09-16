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

use cgmath::angle::*;
use cgmath::vector::*;

#[test]
fn test_from_value() {
    assert_eq!(Vec2::from_value(102), Vec2::new(102, 102));
    assert_eq!(Vec3::from_value(22), Vec3::new(22, 22, 22));
    assert_eq!(Vec4::from_value(76.5), Vec4::new(76.5, 76.5, 76.5, 76.5));
}

#[test]
fn test_dot() {
    assert_eq!(Vec2::new(1, 2).dot(&Vec2::new(3, 4)), 11);
    assert_eq!(Vec3::new(1, 2, 3).dot(&Vec3::new(4, 5, 6)), 32);
    assert_eq!(Vec4::new(1, 2, 3, 4).dot(&Vec4::new(5, 6, 7, 8)), 70);
}

#[test]
fn test_comp_add() {
    assert_eq!(Vec2::new(1, 2).comp_add(), 3);
    assert_eq!(Vec3::new(1, 2, 3).comp_add(), 6);
    assert_eq!(Vec4::new(1, 2, 3, 4).comp_add(), 10);

    assert_eq!(Vec2::new(3f, 4f).comp_add(), 7f);
    assert_eq!(Vec3::new(4f, 5f, 6f).comp_add(), 15f);
    assert_eq!(Vec4::new(5f, 6f, 7f, 8f).comp_add(), 26f);
}

#[test]
fn test_comp_mul() {
    assert_eq!(Vec2::new(1, 2).comp_mul(), 2);
    assert_eq!(Vec3::new(1, 2, 3).comp_mul(), 6);
    assert_eq!(Vec4::new(1, 2, 3, 4).comp_mul(), 24);

    assert_eq!(Vec2::new(3f, 4f).comp_mul(), 12f);
    assert_eq!(Vec3::new(4f, 5f, 6f).comp_mul(), 120f);
    assert_eq!(Vec4::new(5f, 6f, 7f, 8f).comp_mul(), 1680f);
}

#[test]
fn test_comp_min() {
    assert_eq!(Vec2::new(1, 2).comp_min(), 1);
    assert_eq!(Vec3::new(1, 2, 3).comp_min(), 1);
    assert_eq!(Vec4::new(1, 2, 3, 4).comp_min(), 1);

    assert_eq!(Vec2::new(3f, 4f).comp_min(), 3f);
    assert_eq!(Vec3::new(4f, 5f, 6f).comp_min(), 4f);
    assert_eq!(Vec4::new(5f, 6f, 7f, 8f).comp_min(), 5f);
}

#[test]
fn test_comp_max() {
    assert_eq!(Vec2::new(1, 2).comp_max(), 2);
    assert_eq!(Vec3::new(1, 2, 3).comp_max(), 3);
    assert_eq!(Vec4::new(1, 2, 3, 4).comp_max(), 4);

    assert_eq!(Vec2::new(3f, 4f).comp_max(), 4f);
    assert_eq!(Vec3::new(4f, 5f, 6f).comp_max(), 6f);
    assert_eq!(Vec4::new(5f, 6f, 7f, 8f).comp_max(), 8f);
}

#[test]
fn test_cross() {
    let a = Vec3::new(1, 2, 3);
    let b = Vec3::new(4, 5, 6);
    let r = Vec3::new(-3, 6, -3);
    assert_eq!(a.cross(&b), r);

    let mut a = a;
    a.cross_self(&b);
    assert_eq!(a, r);
}

#[test]
fn test_is_perpendicular() {
    assert!(Vec2::new(1f, 0f).is_perpendicular(&Vec2::new(0f, 1f)));
    assert!(Vec3::new(0f, 1f, 0f).is_perpendicular(&Vec3::new(0f, 0f, 1f)));
    assert!(Vec4::new(1f, 0f, 0f, 0f).is_perpendicular(&Vec4::new(0f, 0f, 0f, 1f)));
}

#[cfg(test)]
mod test_length {
    use cgmath::vector::*;

    #[test]
    fn test_vec2(){
        let (a, a_res) = (Vec2::new(3f, 4f), 5f); // (3, 4, 5) Pythagorean triple
        let (b, b_res) = (Vec2::new(5f, 12f), 13f); // (5, 12, 13) Pythagorean triple

        assert_eq!(a.length2(), a_res * a_res);
        assert_eq!(b.length2(), b_res * b_res);

        assert_eq!(a.length(), a_res);
        assert_eq!(b.length(), b_res);
    }

    #[test]
    fn test_vec3(){
        let (a, a_res) = (Vec3::new(2f, 3f, 6f), 7f); // (2, 3, 6, 7) Pythagorean quadruple
        let (b, b_res) = (Vec3::new(1f, 4f, 8f), 9f); // (1, 4, 8, 9) Pythagorean quadruple

        assert_eq!(a.length2(), a_res * a_res);
        assert_eq!(b.length2(), b_res * b_res);

        assert_eq!(a.length(), a_res);
        assert_eq!(b.length(), b_res);
    }

    #[test]
    fn test_vec4(){
        let (a, a_res) = (Vec4::new(1f, 2f, 4f, 10f), 11f); // (1, 2, 4, 10, 11) Pythagorean quintuple
        let (b, b_res) = (Vec4::new(1f, 2f, 8f, 10f), 13f); // (1, 2, 8, 10, 13) Pythagorean quintuple

        assert_eq!(a.length2(), a_res * a_res);
        assert_eq!(b.length2(), b_res * b_res);

        assert_eq!(a.length(), a_res);
        assert_eq!(b.length(), b_res);
    }
}

#[test]
fn test_angle() {
    assert_approx_eq!(Vec2::new(1f, 0f).angle(&Vec2::new(0f, 1f)), rad(Real::frac_pi_2()));
    assert_approx_eq!(Vec2::new(10f, 0f).angle(&Vec2::new(0f, 5f)), rad(Real::frac_pi_2()));
    assert_approx_eq!(Vec2::new(-1f, 0f).angle(&Vec2::new(0f, 1f)), -rad(Real::frac_pi_2()));

    assert_approx_eq!(Vec3::new(1f, 0f, 1f).angle(&Vec3::new(1f, 1f, 0f)), rad(Real::frac_pi_3()));
    assert_approx_eq!(Vec3::new(10f, 0f, 10f).angle(&Vec3::new(5f, 5f, 0f)), rad(Real::frac_pi_3()));
    assert_approx_eq!(Vec3::new(-1f, 0f, -1f).angle(&Vec3::new(1f, -1f, 0f)), rad(2f * Real::frac_pi_3()));

    assert_approx_eq!(Vec4::new(1f, 0f, 1f, 0f).angle(&Vec4::new(0f, 1f, 0f, 1f)), rad(Real::frac_pi_2()));
    assert_approx_eq!(Vec4::new(10f, 0f, 10f, 0f).angle(&Vec4::new(0f, 5f, 0f, 5f)), rad(Real::frac_pi_2()));
    assert_approx_eq!(Vec4::new(-1f, 0f, -1f, 0f).angle(&Vec4::new(0f, 1f, 0f, 1f)), rad(Real::frac_pi_2()));
}

#[test]
fn test_normalize() {
    // TODO: test normalize_to, normalize_self, and normalize_self_to
    assert_approx_eq!(Vec2::new(3f, 4f).normalize(), Vec2::new(3f/5f, 4f/5f));
    assert_approx_eq!(Vec3::new(2f, 3f, 6f).normalize(), Vec3::new(2f/7f, 3f/7f, 6f/7f));
    assert_approx_eq!(Vec4::new(1f, 2f, 4f, 10f).normalize(), Vec4::new(1f/11f, 2f/11f, 4f/11f, 10f/11f));
}
