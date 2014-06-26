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
use cgmath::approx::ApproxEq;

#[test]
fn test_from_value() {
    assert_eq!(Vector2::from_value(102i), Vector2::new(102i, 102i));
    assert_eq!(Vector3::from_value(22i), Vector3::new(22i, 22i, 22i));
    assert_eq!(Vector4::from_value(76.5f64), Vector4::new(76.5f64, 76.5f64, 76.5f64, 76.5f64));
}

#[test]
fn test_dot() {
    assert_eq!(Vector2::new(1i, 2i).dot(&Vector2::new(3i, 4i)), 11i);
    assert_eq!(Vector3::new(1i, 2i, 3i).dot(&Vector3::new(4i, 5i, 6i)), 32i);
    assert_eq!(Vector4::new(1i, 2i, 3i, 4i).dot(&Vector4::new(5i, 6i, 7i, 8i)), 70i);
}

#[test]
fn test_comp_add() {
    assert_eq!(Vector2::new(1i, 2i).comp_add(), 3i);
    assert_eq!(Vector3::new(1i, 2i, 3i).comp_add(), 6i);
    assert_eq!(Vector4::new(1i, 2i, 3i, 4i).comp_add(), 10i);

    assert_eq!(Vector2::new(3.0f64, 4.0f64).comp_add(), 7.0f64);
    assert_eq!(Vector3::new(4.0f64, 5.0f64, 6.0f64).comp_add(), 15.0f64);
    assert_eq!(Vector4::new(5.0f64, 6.0f64, 7.0f64, 8.0f64).comp_add(), 26.0f64);
}

#[test]
fn test_comp_mul() {
    assert_eq!(Vector2::new(1i, 2i).comp_mul(), 2i);
    assert_eq!(Vector3::new(1i, 2i, 3i).comp_mul(), 6i);
    assert_eq!(Vector4::new(1i, 2i, 3i, 4i).comp_mul(), 24i);

    assert_eq!(Vector2::new(3.0f64, 4.0f64).comp_mul(), 12.0f64);
    assert_eq!(Vector3::new(4.0f64, 5.0f64, 6.0f64).comp_mul(), 120.0f64);
    assert_eq!(Vector4::new(5.0f64, 6.0f64, 7.0f64, 8.0f64).comp_mul(), 1680.0f64);
}

#[test]
fn test_comp_min() {
    assert_eq!(Vector2::new(1i, 2i).comp_min(), 1i);
    assert_eq!(Vector3::new(1i, 2i, 3i).comp_min(), 1i);
    assert_eq!(Vector4::new(1i, 2i, 3i, 4i).comp_min(), 1i);

    assert_eq!(Vector2::new(3.0f64, 4.0f64).comp_min(), 3.0f64);
    assert_eq!(Vector3::new(4.0f64, 5.0f64, 6.0f64).comp_min(), 4.0f64);
    assert_eq!(Vector4::new(5.0f64, 6.0f64, 7.0f64, 8.0f64).comp_min(), 5.0f64);
}

#[test]
fn test_comp_max() {
    assert_eq!(Vector2::new(1i, 2i).comp_max(), 2i);
    assert_eq!(Vector3::new(1i, 2i, 3i).comp_max(), 3i);
    assert_eq!(Vector4::new(1i, 2i, 3i, 4i).comp_max(), 4i);

    assert_eq!(Vector2::new(3.0f64, 4.0f64).comp_max(), 4.0f64);
    assert_eq!(Vector3::new(4.0f64, 5.0f64, 6.0f64).comp_max(), 6.0f64);
    assert_eq!(Vector4::new(5.0f64, 6.0f64, 7.0f64, 8.0f64).comp_max(), 8.0f64);
}

#[test]
fn test_cross() {
    let a = Vector3::new(1i, 2i, 3i);
    let b = Vector3::new(4i, 5i, 6i);
    let r = Vector3::new(-3i, 6i, -3i);
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
    use cgmath::vector::*;

    #[test]
    fn test_vector2(){
        let (a, a_res) = (Vector2::new(3.0f64, 4.0f64), 5.0f64); // (3i, 4i, 5i) Pythagorean triple
        let (b, b_res) = (Vector2::new(5.0f64, 12.0f64), 13.0f64); // (5i, 12i, 13i) Pythagorean triple

        assert_eq!(a.length2(), a_res * a_res);
        assert_eq!(b.length2(), b_res * b_res);

        assert_eq!(a.length(), a_res);
        assert_eq!(b.length(), b_res);
    }

    #[test]
    fn test_vector3(){
        let (a, a_res) = (Vector3::new(2.0f64, 3.0f64, 6.0f64), 7.0f64); // (2i, 3i, 6i, 7i) Pythagorean quadruple
        let (b, b_res) = (Vector3::new(1.0f64, 4.0f64, 8.0f64), 9.0f64); // (1i, 4i, 8i, 9i) Pythagorean quadruple

        assert_eq!(a.length2(), a_res * a_res);
        assert_eq!(b.length2(), b_res * b_res);

        assert_eq!(a.length(), a_res);
        assert_eq!(b.length(), b_res);
    }

    #[test]
    fn test_vector4(){
        let (a, a_res) = (Vector4::new(1.0f64, 2.0f64, 4.0f64, 10.0f64), 11.0f64); // (1i, 2i, 4i, 10i, 11i) Pythagorean quintuple
        let (b, b_res) = (Vector4::new(1.0f64, 2.0f64, 8.0f64, 10.0f64), 13.0f64); // (1i, 2i, 8i, 10i, 13i) Pythagorean quintuple

        assert_eq!(a.length2(), a_res * a_res);
        assert_eq!(b.length2(), b_res * b_res);

        assert_eq!(a.length(), a_res);
        assert_eq!(b.length(), b_res);
    }
}

#[test]
fn test_angle() {
    assert!(Vector2::new(1.0f64, 0.0f64).angle(&Vector2::new(0.0f64, 1.0f64)).approx_eq( &rad(Float::frac_pi_2()) ));
    assert!(Vector2::new(10.0f64, 0.0f64).angle(&Vector2::new(0.0f64, 5.0f64)).approx_eq( &rad(Float::frac_pi_2()) ));
    assert!(Vector2::new(-1.0f64, 0.0f64).angle(&Vector2::new(0.0f64, 1.0f64)).approx_eq( &-rad(Float::frac_pi_2()) ));

    assert!(Vector3::new(1.0f64, 0.0f64, 1.0f64).angle(&Vector3::new(1.0f64, 1.0f64, 0.0f64)).approx_eq( &rad(Float::frac_pi_3()) ));
    assert!(Vector3::new(10.0f64, 0.0f64, 10.0f64).angle(&Vector3::new(5.0f64, 5.0f64, 0.0f64)).approx_eq( &rad(Float::frac_pi_3()) ));
    assert!(Vector3::new(-1.0f64, 0.0f64, -1.0f64).angle(&Vector3::new(1.0f64, -1.0f64, 0.0f64)).approx_eq( &rad(2.0f64 * Float::frac_pi_3()) ));

    assert!(Vector4::new(1.0f64, 0.0f64, 1.0f64, 0.0f64).angle(&Vector4::new(0.0f64, 1.0f64, 0.0f64, 1.0f64)).approx_eq( &rad(Float::frac_pi_2()) ));
    assert!(Vector4::new(10.0f64, 0.0f64, 10.0f64, 0.0f64).angle(&Vector4::new(0.0f64, 5.0f64, 0.0f64, 5.0f64)).approx_eq( &rad(Float::frac_pi_2()) ));
    assert!(Vector4::new(-1.0f64, 0.0f64, -1.0f64, 0.0f64).angle(&Vector4::new(0.0f64, 1.0f64, 0.0f64, 1.0f64)).approx_eq( &rad(Float::frac_pi_2()) ));
}

#[test]
fn test_normalize() {
    // TODO: test normalize_to, normalize_sel.0, and normalize_self_to
    assert!(Vector2::new(3.0f64, 4.0f64).normalize().approx_eq( &Vector2::new(3.0/5.0, 4.0/5.0) ));
    assert!(Vector3::new(2.0f64, 3.0f64, 6.0f64).normalize().approx_eq( &Vector3::new(2.0/7.0, 3.0/7.0, 6.0/7.0) ));
    assert!(Vector4::new(1.0f64, 2.0f64, 4.0f64, 10.0f64).normalize().approx_eq( &Vector4::new(1.0/11.0, 2.0/11.0, 4.0/11.0, 10.0/11.0) ));
}
