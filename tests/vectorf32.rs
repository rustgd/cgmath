// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0f32 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0f32
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate approx;
#[macro_use]
extern crate cgmath;

use cgmath::*;
use std::f32;

#[test]
fn test_constructor() {
    assert_eq!(vec2(1f32, 2f32), Vector2::new(1f32, 2f32));
    assert_eq!(vec3(1f32, 2f32, 3f32), Vector3::new(1f32, 2f32, 3f32));
    assert_eq!(vec4(1f32, 2f32, 3f32, 4f32), Vector4::new(1f32, 2f32, 3f32, 4f32));
}

#[test]
fn test_from_value() {
    assert_eq!(Vector2::from_value(102f32), Vector2::new(102f32, 102f32));
    assert_eq!(Vector3::from_value(22f32), Vector3::new(22f32, 22f32, 22f32));
    assert_eq!(Vector4::from_value(76.5f32), Vector4::new(76.5f32, 76.5f32, 76.5f32, 76.5f32));
}

macro_rules! impl_test_add {
    ($VectorN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // vector + vector ops
        assert_eq!($v + $v, $VectorN::new($($v.$field + $v.$field),+));
        assert_eq!(&$v + &$v, $v + $v);
        assert_eq!(&$v + $v, $v + $v);
        assert_eq!($v + &$v, $v + $v);
    )
}

macro_rules! impl_test_sub {
    ($VectorN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // vector - vector ops
        assert_eq!($v - $v, $VectorN::new($($v.$field - $v.$field),+));
        assert_eq!(&$v - &$v, $v - $v);
        assert_eq!(&$v - $v, $v - $v);
        assert_eq!($v - &$v, $v - $v);
    )
}

macro_rules! impl_test_mul {
    ($VectorN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // vector * scalar ops
        assert_eq!($v * $s, $VectorN::new($($v.$field * $s),+));
        assert_eq!($s * $v, $VectorN::new($($s * $v.$field),+));
        assert_eq!(&$v * $s, $v * $s);
        assert_eq!($s * &$v, $s * $v);
        // commutativity
        assert_eq!($v * $s, $s * $v);
    )
}

macro_rules! impl_test_div {
    ($VectorN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // vector / scalar ops
        assert_eq!($v / $s, $VectorN::new($($v.$field / $s),+));
        assert_eq!($s / $v, $VectorN::new($($s / $v.$field),+));
        assert_eq!(&$v / $s, $v / $s);
        assert_eq!($s / &$v, $s / $v);
    )
}

macro_rules! impl_test_rem {
    ($VectorN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // vector % scalar ops
        assert_eq!($v % $s, $VectorN::new($($v.$field % $s),+));
        assert_eq!($s % $v, $VectorN::new($($s % $v.$field),+));
        assert_eq!(&$v % $s, $v % $s);
        assert_eq!($s % &$v, $s % $v);
    )
}

#[test]
fn test_add() {
    impl_test_add!(Vector4 { x, y, z, w }, 2.0f32, vec4(2.0f32, 4.0f32, 6.0f32, 8.0f32));
}

#[test]
fn test_sub() {
    impl_test_sub!(Vector4 { x, y, z, w }, 2.0f32, vec4(2.0f32, 4.0f32, 6.0f32, 8.0f32));
    impl_test_sub!(Vector3 { x, y, z }, 2.0f32, vec3(2.0f32, 4.0f32, 6.0f32));
    impl_test_sub!(Vector2 { x, y }, 2.0f32, vec2(2.0f32, 4.0f32));
}

#[test]
fn test_mul() {
    impl_test_mul!(Vector4 { x, y, z, w }, 2.0f32, vec4(2.0f32, 4.0f32, 6.0f32, 8.0f32));
    impl_test_mul!(Vector3 { x, y, z }, 2.0f32, vec3(2.0f32, 4.0f32, 6.0f32));
    impl_test_mul!(Vector2 { x, y }, 2.0f32, vec2(2.0f32, 4.0f32));
}

#[test]
fn test_div() {
    impl_test_div!(Vector4 { x, y, z, w }, 2.0f32, vec4(2.0f32, 4.0f32, 6.0f32, 8.0f32));
    impl_test_div!(Vector3 { x, y, z }, 2.0f32, vec3(2.0f32, 4.0f32, 6.0f32));
    impl_test_div!(Vector2 { x, y }, 2.0f32, vec2(2.0f32, 4.0f32));
}

#[test]
fn test_rem() {
    impl_test_rem!(Vector4 { x, y, z, w }, 2.0f32, vec4(2.0f32, 4.0f32, 6.0f32, 8.0f32));
    impl_test_rem!(Vector3 { x, y, z }, 2.0f32, vec3(2.0f32, 4.0f32, 6.0f32));
    impl_test_rem!(Vector2 { x, y }, 2.0f32, vec2(2.0f32, 4.0f32));
}

#[test]
fn test_dot() {
    assert_eq!(Vector2::new(1.0f32, 2.0f32).dot(Vector2::new(3.0f32, 4.0f32)), 11.0f32);
    assert_eq!(Vector3::new(1.0f32, 2.0f32, 3.0f32).dot(Vector3::new(4.0f32, 5.0f32, 6.0f32)), 32.0f32);
    assert_eq!(Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32).dot(Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32)), 70.0f32);
}

#[test]
fn test_sum() {
    assert_eq!(Vector2::new(1f32, 2f32).sum(), 3f32);
    assert_eq!(Vector3::new(1f32, 2f32, 3f32).sum(), 6f32);
    assert_eq!(Vector4::new(1f32, 2f32, 3f32, 4f32).sum(), 10f32);

    assert_eq!(Vector2::new(3.0f32, 4.0f32).sum(), 7.0f32);
    assert_eq!(Vector3::new(4.0f32, 5.0f32, 6.0f32).sum(), 15.0f32);
    assert_eq!(Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32).sum(), 26.0f32);
}

#[test]
fn test_product() {
    assert_eq!(Vector2::new(1f32, 2f32).product(), 2f32);
    assert_eq!(Vector3::new(1f32, 2f32, 3f32).product(), 6f32);
    assert_eq!(Vector4::new(1f32, 2f32, 3f32, 4f32).product(), 24f32);

    assert_eq!(Vector2::new(3.0f32, 4.0f32).product(), 12.0f32);
    assert_eq!(Vector3::new(4.0f32, 5.0f32, 6.0f32).product(), 120.0f32);
    assert_eq!(Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32).product(), 1680.0f32);
}

#[test]
fn test_min() {
    assert_eq!(Vector2::new(1f32, 2f32).min(), 1f32);
    assert_eq!(Vector3::new(1f32, 2f32, 3f32).min(), 1f32);
    assert_eq!(Vector4::new(1f32, 2f32, 3f32, 4f32).min(), 1f32);

    assert_eq!(Vector2::new(3.0f32, 4.0f32).min(), 3.0f32);
    assert_eq!(Vector3::new(4.0f32, 5.0f32, 6.0f32).min(), 4.0f32);
    assert_eq!(Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32).min(), 5.0f32);
}

#[test]
fn test_max() {
    assert_eq!(Vector2::new(1f32, 2f32).max(), 2f32);
    assert_eq!(Vector3::new(1f32, 2f32, 3f32).max(), 3f32);
    assert_eq!(Vector4::new(1f32, 2f32, 3f32, 4f32).max(), 4f32);

    assert_eq!(Vector2::new(3.0f32, 4.0f32).max(), 4.0f32);
    assert_eq!(Vector3::new(4.0f32, 5.0f32, 6.0f32).max(), 6.0f32);
    assert_eq!(Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32).max(), 8.0f32);
}

#[test]
fn test_cross() {
    let a = Vector3::new(1f32, 2f32, 3f32);
    let b = Vector3::new(4f32, 5f32, 6f32);
    let r = Vector3::new(-3f32, 6f32, -3f32);
    assert_eq!(a.cross(b), r);
}

#[test]
fn test_is_perpendicular() {
    assert!(Vector2::new(1.0f32, 0.0f32).is_perpendicular(Vector2::new(0.0f32, 1.0f32)));
    assert!(Vector3::new(0.0f32, 1.0f32, 0.0f32).is_perpendicular(Vector3::new(0.0f32, 0.0f32, 1.0f32)));
    assert!(Vector4::new(1.0f32, 0.0f32, 0.0f32, 0.0f32).is_perpendicular(Vector4::new(0.0f32, 0.0f32, 0.0f32, 1.0f32)));
}

#[cfg(test)]
mod test_magnitude {
    use cgmath::*;

    #[test]
    fn test_vector2(){
        let (a, a_res) = (Vector2::new(3.0f32, 4.0f32), 5.0f32); // (3, 4, 5) Pythagorean triple
        let (b, b_res) = (Vector2::new(5.0f32, 12.0f32), 13.0f32); // (5, 12, 13) Pythagorean triple

        assert_eq!(a.magnitude2(), a_res * a_res);
        assert_eq!(b.magnitude2(), b_res * b_res);

        assert_eq!(a.magnitude(), a_res);
        assert_eq!(b.magnitude(), b_res);
    }

    #[test]
    fn test_vector3(){
        let (a, a_res) = (Vector3::new(2.0f32, 3.0f32, 6.0f32), 7.0f32); // (2, 3, 6, 7) Pythagorean quadruple
        let (b, b_res) = (Vector3::new(1.0f32, 4.0f32, 8.0f32), 9.0f32); // (1, 4, 8, 9) Pythagorean quadruple

        assert_eq!(a.magnitude2(), a_res * a_res);
        assert_eq!(b.magnitude2(), b_res * b_res);

        assert_eq!(a.magnitude(), a_res);
        assert_eq!(b.magnitude(), b_res);
    }

    #[test]
    fn test_vector4(){
        let (a, a_res) = (Vector4::new(1.0f32, 2.0f32, 4.0f32, 10.0f32), 11.0f32); // (1, 2, 4, 10, 11) Pythagorean quintuple
        let (b, b_res) = (Vector4::new(1.0f32, 2.0f32, 8.0f32, 10.0f32), 13.0f32); // (1, 2, 8, 10, 13) Pythagorean quintuple

        assert_eq!(a.magnitude2(), a_res * a_res);
        assert_eq!(b.magnitude2(), b_res * b_res);

        assert_eq!(a.magnitude(), a_res);
        assert_eq!(b.magnitude(), b_res);

        #[cfg(feature = "use_simd")]
        {
            let a = Vector4::new(1f32, 4f32, 9f32, 16f32);
            assert_ulps_eq!(a.sqrt_element_wide(), Vector4::new(1f32, 2f32, 3f32, 4f32));
            assert_relative_eq!(a.sqrt_element_wide().recip_element_wide(), Vector4::new(1f32, 1f32/2f32, 1f32/3f32, 1f32/4f32), max_relative = 0.005f32);
            assert_relative_eq!(a.rsqrt_element_wide(), Vector4::new(1f32, 1f32/2f32, 1f32/3f32, 1f32/4f32), max_relative= 0.005f32);
        }
        
    }
}

#[test]
fn test_angle() {
    assert_ulps_eq!(Vector2::new(1.0f32, 0.0f32).angle(Vector2::new(0.0f32, 1.0f32)), &Rad(f32::consts::FRAC_PI_2));
    assert_ulps_eq!(Vector2::new(10.0f32, 0.0f32).angle(Vector2::new(0.0f32, 5.0f32)), &Rad(f32::consts::FRAC_PI_2));
    assert_ulps_eq!(Vector2::new(-1.0f32, 0.0f32).angle(Vector2::new(0.0f32, 1.0f32)), &-Rad(f32::consts::FRAC_PI_2));

    assert_ulps_eq!(Vector3::new(1.0f32, 0.0f32, 1.0f32).angle(Vector3::new(1.0f32, 1.0f32, 0.0f32)), &Rad(f32::consts::FRAC_PI_3));
    assert_ulps_eq!(Vector3::new(10.0f32, 0.0f32, 10.0f32).angle(Vector3::new(5.0f32, 5.0f32, 0.0f32)), &Rad(f32::consts::FRAC_PI_3));
    assert_ulps_eq!(Vector3::new(-1.0f32, 0.0f32, -1.0f32).angle(Vector3::new(1.0f32, -1.0f32, 0.0f32)), &Rad(2.0f32 * f32::consts::FRAC_PI_3));

    assert_ulps_eq!(Vector4::new(1.0f32, 0.0f32, 1.0f32, 0.0f32).angle(Vector4::new(0.0f32, 1.0f32, 0.0f32, 1.0f32)), &Rad(f32::consts::FRAC_PI_2));
    assert_ulps_eq!(Vector4::new(10.0f32, 0.0f32, 10.0f32, 0.0f32).angle(Vector4::new(0.0f32, 5.0f32, 0.0f32, 5.0f32)), &Rad(f32::consts::FRAC_PI_2));
    assert_ulps_eq!(Vector4::new(-1.0f32, 0.0f32, -1.0f32, 0.0f32).angle(Vector4::new(0.0f32, 1.0f32, 0.0f32, 1.0f32)), &Rad(f32::consts::FRAC_PI_2));
}

#[test]
fn test_normalize() {
    // TODO: test normalize_to, normalize_sel.0f32, and normalize_self_to
    assert_ulps_eq!(Vector2::new(3.0f32, 4.0f32).normalize(), &Vector2::new(3.0f32/5.0f32, 4.0f32/5.0f32));
    assert_ulps_eq!(Vector3::new(2.0f32, 3.0f32, 6.0f32).normalize(), &Vector3::new(2.0f32/7.0f32, 3.0f32/7.0f32, 6.0f32/7.0f32));
    assert_ulps_eq!(Vector4::new(1.0f32, 2.0f32, 4.0f32, 10.0f32).normalize(), &Vector4::new(1.0f32/11.0f32, 2.0f32/11.0f32, 4.0f32/11.0f32, 10.0f32/11.0f32));
}

#[test]
fn test_cast() {
    assert_ulps_eq!(Vector2::new(0.9f32, 1.5).cast(), Vector2::new(0.9f32, 1.5));
    assert_ulps_eq!(Vector3::new(1.0f32, 2.4, -3.13).cast(), Vector3::new(1.0f32, 2.4, -3.13));
    assert_ulps_eq!(Vector4::new(13.5f32, -4.6, -8.3, 2.41).cast(), Vector4::new(13.5f32, -4.6, -8.3, 2.41));
}
