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

use vec::*;

// TODO

#[test]
fn test_vec2() {
    // assert_eq!(vec2::dim, 2);

    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 3.0, y: 4.0 };
    let f1 = 1.5;
    let f2 = 0.5;

    let mut mut_a = a;

    assert_eq!(vec2::new(1.0, 2.0), a);
    assert_eq!(vec2::from_value(1.0), vec2::new(1.0, 1.0));

    assert_eq!(vec2::zero(), vec2::new(0.0, 0.0));
    assert_eq!(vec2::unit_x(), vec2::new(1.0, 0.0));
    assert_eq!(vec2::unit_y(), vec2::new(0.0, 1.0));
    assert_eq!(vec2::identity(), vec2::new(1.0, 1.0));

    *mut_a.index_mut(0) = 42.0;
    *mut_a.index_mut(1) = 43.0;
    assert_eq!(mut_a, vec2::new(42.0, 43.0));
    mut_a = a;

    mut_a.swap(0, 1);
    assert_eq!(*mut_a.index(0), *a.index(1));
    assert_eq!(*mut_a.index(1), *a.index(0));
    mut_a = a;

    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(*a.index(0), 1.0);
    assert_eq!(*a.index(1), 2.0);

    assert_eq!(-a, vec2::new(-1.0, -2.0));
    assert_eq!(a.neg(), vec2::new(-1.0, -2.0));

    assert!(vec2::new(0.0, 0.0).is_zero());
    assert!(!vec2::new(1.0, 1.0).is_zero());

    assert_eq!(a.mul_t(f1), vec2::new( 1.5, 3.0));
    assert_eq!(a.div_t(f2), vec2::new( 2.0, 4.0));

    assert_eq!(a.add_v(&b), vec2::new(    4.0,     6.0));
    assert_eq!(a.sub_v(&b), vec2::new(   -2.0,    -2.0));
    assert_eq!(a.mul_v(&b), vec2::new(    3.0,     8.0));
    assert_eq!(a.div_v(&b), vec2::new(1.0/3.0, 2.0/4.0));

    mut_a.neg_self();
    assert_eq!(mut_a, -a);
    mut_a = a;

    mut_a.mul_self_t(f1);
    assert_eq!(mut_a, a.mul_t(f1));
    mut_a = a;

    mut_a.div_self_t(f2);
    assert_eq!(mut_a, a.div_t(f2));
    mut_a = a;

    mut_a.add_self_v(&b);
    assert_eq!(mut_a, a.add_v(&b));
    mut_a = a;

    mut_a.sub_self_v(&b);
    assert_eq!(mut_a, a.sub_v(&b));
    mut_a = a;

    mut_a.mul_self_v(&b);
    assert_eq!(mut_a, a.mul_v(&b));
    mut_a = a;

    mut_a.div_self_v(&b);
    assert_eq!(mut_a, a.div_v(&b));
    // mut_a = a;

    // assert_eq!(c.abs(), vec2::new( 2.0,  1.0));
    // assert_eq!(c.min(&d), vec2::new(-2.0, -1.0));
    // assert_eq!(c.max(&d), vec2::new( 1.0,  0.0));
}

#[test]
fn test_vec2_approx_eq() {
    assert!(!vec2::new(0.000001, 0.000001).approx_eq(&vec2::new(0.0, 0.0)));
    assert!(vec2::new(0.0000001, 0.0000001).approx_eq(&vec2::new(0.0, 0.0)));
}

#[test]
fn test_vec2_euclidean() {
    let a = vec2::new(5.0, 12.0); // (5, 12, 13) Pythagorean triple
    let b0 = vec2::new(3.0, 4.0); // (3, 4, 5) Pythagorean triple
    let b = a.add_v(&b0);

    assert_eq!(a.length(), 13.0);
    assert_eq!(a.length2(), 13.0 * 13.0);

    assert_eq!(b0.length(), 5.0);
    assert_eq!(b0.length2(), 5.0 * 5.0);

    assert_eq!(a.distance(&b), 5.0);
    assert_eq!(a.distance2(&b), 5.0 * 5.0);

    assert!(vec2::new(1.0, 0.0).angle(&vec2::new(0.0, 1.0)).approx_eq(&Real::frac_pi_2()));
    assert!(vec2::new(10.0, 0.0).angle(&vec2::new(0.0, 5.0)).approx_eq(&Real::frac_pi_2()));
    assert!(vec2::new(-1.0, 0.0).angle(&vec2::new(0.0, 1.0)).approx_eq(&-Real::frac_pi_2::<f32>()));

    assert!(vec2::new(3.0, 4.0).normalize().approx_eq(&vec2::new(3.0/5.0, 4.0/5.0)));
    // TODO: test normalize_to, normalize_self, and normalize_self_to

    let c = vec2::new(-2.0, -1.0);
    let d = vec2::new( 1.0,  0.0);

    assert_eq!(c.lerp(&d, 0.75), vec2::new(0.250, -0.250));

    let mut mut_c = c;
    mut_c.lerp_self(&d, 0.75);
    assert_eq!(mut_c, c.lerp(&d, 0.75));
}

#[test]
fn test_vec2_boolean() {
    let tf = bvec2::new(true, false);
    let ff = bvec2::new(false, false);
    let tt = bvec2::new(true, true);

    assert_eq!(tf.any(), true);
    assert_eq!(tf.all(), false);
    assert_eq!(tf.not(), bvec2::new(false, true));

    assert_eq!(ff.any(), false);
    assert_eq!(ff.all(), false);
    assert_eq!(ff.not(), bvec2::new(true, true));

    assert_eq!(tt.any(), true);
    assert_eq!(tt.all(), true);
    assert_eq!(tt.not(), bvec2::new(false, false));
}

#[test]
fn test_vec3() {
    // assert_eq!(Vec3::dim, 3);

    let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    let b = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    let f1 = 1.5;
    let f2 = 0.5;

    let mut mut_a = a;

    assert_eq!(vec3::new(1.0, 2.0, 3.0), a);
    assert_eq!(vec3::from_value(1.0), vec3::new(1.0, 1.0, 1.0));

    assert_eq!(vec3::zero(), vec3::new(0.0, 0.0, 0.0));
    assert_eq!(vec3::unit_x(), vec3::new(1.0, 0.0, 0.0));
    assert_eq!(vec3::unit_y(), vec3::new(0.0, 1.0, 0.0));
    assert_eq!(vec3::unit_z(), vec3::new(0.0, 0.0, 1.0));
    assert_eq!(vec3::identity(), vec3::new(1.0, 1.0, 1.0));

    *mut_a.index_mut(0) = 42.0;
    *mut_a.index_mut(1) = 43.0;
    *mut_a.index_mut(2) = 44.0;
    assert_eq!(mut_a, vec3::new(42.0, 43.0, 44.0));
    mut_a = a;

    mut_a.swap(0, 2);
    assert_eq!(*mut_a.index(0), *a.index(2));
    assert_eq!(*mut_a.index(2), *a.index(0));
    mut_a = a;

    mut_a.swap(1, 2);
    assert_eq!(*mut_a.index(1), *a.index(2));
    assert_eq!(*mut_a.index(2), *a.index(1));
    mut_a = a;

    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 3.0);
    assert_eq!(*a.index(0), 1.0);
    assert_eq!(*a.index(1), 2.0);
    assert_eq!(*a.index(2), 3.0);

    assert_eq!(a.cross(&b), vec3::new(-3.0, 6.0, -3.0));

    mut_a.cross_self(&b);
    assert_eq!(mut_a, a.cross(&b));
    mut_a = a;

    assert_eq!(-a, vec3::new(-1.0, -2.0, -3.0));
    assert_eq!(a.neg(), vec3::new(-1.0, -2.0, -3.0));

    assert!(vec3::new(0.0, 0.0, 0.0).is_zero());
    assert!(!vec3::new(1.0, 1.0, 1.0).is_zero());

    assert_eq!(a.mul_t(f1), vec3::new( 1.5, 3.0, 4.5));
    assert_eq!(a.div_t(f2), vec3::new( 2.0, 4.0, 6.0));

    assert_eq!(a.add_v(&b), vec3::new(    5.0,     7.0,     9.0));
    assert_eq!(a.sub_v(&b), vec3::new(   -3.0,    -3.0,    -3.0));
    assert_eq!(a.mul_v(&b), vec3::new(    4.0,    10.0,    18.0));
    assert_eq!(a.div_v(&b), vec3::new(1.0/4.0, 2.0/5.0, 3.0/6.0));

    mut_a.neg_self();
    assert_eq!(mut_a, -a);
    mut_a = a;

    mut_a.mul_self_t(f1);
    assert_eq!(mut_a, a.mul_t(f1));
    mut_a = a;

    mut_a.div_self_t(f2);
    assert_eq!(mut_a, a.div_t(f2));
    mut_a = a;

    mut_a.add_self_v(&b);
    assert_eq!(mut_a, a.add_v(&b));
    mut_a = a;

    mut_a.sub_self_v(&b);
    assert_eq!(mut_a, a.sub_v(&b));
    mut_a = a;

    mut_a.mul_self_v(&b);
    assert_eq!(mut_a, a.mul_v(&b));
    mut_a = a;

    mut_a.div_self_v(&b);
    assert_eq!(mut_a, a.div_v(&b));
    // mut_a = a;

    // exact_eq
    // approx_eq
    // eq

    // assert_eq!(c.abs(), vec3::new( 2.0,  1.0, 1.0));
    // assert_eq!(c.min(&d), vec3::new(-2.0, -1.0, 0.5));
    // assert_eq!(c.max(&d), vec3::new( 1.0,  0.0, 1.0));
}

#[test]
fn test_vec3_approx_eq() {
    assert!(!vec3::new(0.000001, 0.000001, 0.000001).approx_eq(&vec3::new(0.0, 0.0, 0.0)));
    assert!(vec3::new(0.0000001, 0.0000001, 0.0000001).approx_eq(&vec3::new(0.0, 0.0, 0.0)));
}

#[test]
fn test_vec3_euclidean() {
    let a = vec3::new(2.0, 3.0, 6.0); // (2, 3, 6, 7) Pythagorean quadruple
    let b0 = vec3::new(1.0, 4.0, 8.0); // (1, 4, 8, 9) Pythagorean quadruple
    let b = a.add_v(&b0);

    assert_eq!(a.length(), 7.0);
    assert_eq!(a.length2(), 7.0 * 7.0);

    assert_eq!(b0.length(), 9.0);
    assert_eq!(b0.length2(), 9.0 * 9.0);

    assert_eq!(a.distance(&b), 9.0);
    assert_eq!(a.distance2(&b), 9.0 * 9.0);

    assert!(vec3::new(1.0, 0.0, 1.0).angle(&vec3::new(1.0, 1.0, 0.0)).approx_eq(&Real::frac_pi_3()));
    assert!(vec3::new(10.0, 0.0, 10.0).angle(&vec3::new(5.0, 5.0, 0.0)).approx_eq(&Real::frac_pi_3()));
    assert!(vec3::new(-1.0, 0.0, -1.0).angle(&vec3::new(1.0, -1.0, 0.0)).approx_eq(&(2.0 * Real::frac_pi_3())));

    assert!(vec3::new(2.0, 3.0, 6.0).normalize().approx_eq(&vec3::new(2.0/7.0, 3.0/7.0, 6.0/7.0)));
    // TODO: test normalize_to, normalize_self, and normalize_self_to

    let c = vec3::new(-2.0, -1.0, 1.0);
    let d = vec3::new( 1.0,  0.0, 0.5);

    assert_eq!(c.lerp(&d, 0.75), vec3::new(0.250, -0.250, 0.625));

    let mut mut_c = c;
    mut_c.lerp_self(&d, 0.75);
    assert_eq!(mut_c, c.lerp(&d, 0.75));
}

#[test]
fn test_vec3_boolean() {
    let tft = bvec3::new(true, false, true);
    let fff = bvec3::new(false, false, false);
    let ttt = bvec3::new(true, true, true);

    assert_eq!(tft.any(), true);
    assert_eq!(tft.all(), false);
    assert_eq!(tft.not(), bvec3::new(false, true, false));

    assert_eq!(fff.any(), false);
    assert_eq!(fff.all(), false);
    assert_eq!(fff.not(), bvec3::new(true, true, true));

    assert_eq!(ttt.any(), true);
    assert_eq!(ttt.all(), true);
    assert_eq!(ttt.not(), bvec3::new(false, false, false));
}

#[test]
fn test_vec4() {
    // assert_eq!(Vec4::dim, 4);

    let a = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    let b = Vec4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let f1 = 1.5;
    let f2 = 0.5;

    let mut mut_a = a;

    assert_eq!(vec4::new(1.0, 2.0, 3.0, 4.0), a);
    assert_eq!(vec4::from_value(1.0), vec4::new(1.0, 1.0, 1.0, 1.0));

    *mut_a.index_mut(0) = 42.0;
    *mut_a.index_mut(1) = 43.0;
    *mut_a.index_mut(2) = 44.0;
    *mut_a.index_mut(3) = 45.0;
    assert_eq!(mut_a, vec4::new(42.0, 43.0, 44.0, 45.0));
    mut_a = a;

    mut_a.swap(0, 3);
    assert_eq!(*mut_a.index(0), *a.index(3));
    assert_eq!(*mut_a.index(3), *a.index(0));
    mut_a = a;

    mut_a.swap(1, 2);
    assert_eq!(*mut_a.index(1), *a.index(2));
    assert_eq!(*mut_a.index(2), *a.index(1));
    mut_a = a;

    assert_eq!(vec4::zero(), vec4::new(0.0, 0.0, 0.0, 0.0));
    assert_eq!(vec4::unit_x(), vec4::new(1.0, 0.0, 0.0, 0.0));
    assert_eq!(vec4::unit_y(), vec4::new(0.0, 1.0, 0.0, 0.0));
    assert_eq!(vec4::unit_z(), vec4::new(0.0, 0.0, 1.0, 0.0));
    assert_eq!(vec4::unit_w(), vec4::new(0.0, 0.0, 0.0, 1.0));
    assert_eq!(vec4::identity(), vec4::new(1.0, 1.0, 1.0, 1.0));

    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 3.0);
    assert_eq!(a.w, 4.0);
    assert_eq!(*a.index(0), 1.0);
    assert_eq!(*a.index(1), 2.0);
    assert_eq!(*a.index(2), 3.0);
    assert_eq!(*a.index(3), 4.0);

    assert_eq!(-a, vec4::new(-1.0, -2.0, -3.0, -4.0));
    assert_eq!(a.neg(), vec4::new(-1.0, -2.0, -3.0, -4.0));

    assert!(vec4::new(0.0, 0.0, 0.0, 0.0).is_zero());
    assert!(!vec4::new(1.0, 1.0, 1.0, 1.0).is_zero());

    assert_eq!(a.mul_t(f1), vec4::new( 1.5, 3.0, 4.5, 6.0));
    assert_eq!(a.div_t(f2), vec4::new( 2.0, 4.0, 6.0, 8.0));

    assert_eq!(a.add_v(&b), vec4::new(    6.0,     8.0,    10.0,    12.0));
    assert_eq!(a.sub_v(&b), vec4::new(   -4.0,    -4.0,    -4.0,    -4.0));
    assert_eq!(a.mul_v(&b), vec4::new(    5.0,    12.0,    21.0,    32.0));
    assert_eq!(a.div_v(&b), vec4::new(1.0/5.0, 2.0/6.0, 3.0/7.0, 4.0/8.0));

    assert_eq!(a.dot(&b), 70.0);

    mut_a.neg_self();
    assert_eq!(mut_a, -a);
    mut_a = a;

    mut_a.mul_self_t(f1);
    assert_eq!(mut_a, a.mul_t(f1));
    mut_a = a;

    mut_a.div_self_t(f2);
    assert_eq!(mut_a, a.div_t(f2));
    mut_a = a;

    mut_a.add_self_v(&b);
    assert_eq!(mut_a, a.add_v(&b));
    mut_a = a;

    mut_a.sub_self_v(&b);
    assert_eq!(mut_a, a.sub_v(&b));
    mut_a = a;

    mut_a.mul_self_v(&b);
    assert_eq!(mut_a, a.mul_v(&b));
    mut_a = a;

    mut_a.div_self_v(&b);
    assert_eq!(mut_a, a.div_v(&b));
    // mut_a = a;

    // assert_eq!(c.abs(), vec4::new( 2.0,  1.0, 1.0, 2.0));
    // assert_eq!(c.min(&d), vec4::new(-2.0, -1.0, 0.5, 1.0));
    // assert_eq!(c.max(&d), vec4::new( 1.0,  0.0, 1.0, 2.0));
}

#[test]
fn test_vec4_approx_eq() {
    assert!(!vec4::new(0.000001, 0.000001, 0.000001, 0.000001).approx_eq(&vec4::new(0.0, 0.0, 0.0, 0.0)));
    assert!(vec4::new(0.0000001, 0.0000001, 0.0000001, 0.0000001).approx_eq(&vec4::new(0.0, 0.0, 0.0, 0.0)));
}

#[test]
fn test_vec4_euclidean() {
    let a = vec4::new(1.0, 2.0, 4.0, 10.0); // (1, 2, 4, 10, 11) Pythagorean quintuple
    let b0 = vec4::new(1.0, 2.0, 8.0, 10.0); // (1, 2, 8, 10, 13) Pythagorean quintuple
    let b = a.add_v(&b0);

    assert_eq!(a.length(), 11.0);
    assert_eq!(a.length2(), 11.0 * 11.0);

    assert_eq!(b0.length(), 13.0);
    assert_eq!(b0.length2(), 13.0 * 13.0);

    assert_eq!(a.distance(&b), 13.0);
    assert_eq!(a.distance2(&b), 13.0 * 13.0);

    assert!(vec4::new(1.0, 0.0, 1.0, 0.0).angle(&vec4::new(0.0, 1.0, 0.0, 1.0)).approx_eq(&Real::frac_pi_2()));
    assert!(vec4::new(10.0, 0.0, 10.0, 0.0).angle(&vec4::new(0.0, 5.0, 0.0, 5.0)).approx_eq(&Real::frac_pi_2()));
    assert!(vec4::new(-1.0, 0.0, -1.0, 0.0).angle(&vec4::new(0.0, 1.0, 0.0, 1.0)).approx_eq(&Real::frac_pi_2()));

    assert!(vec4::new(1.0, 2.0, 4.0, 10.0).normalize().approx_eq(&vec4::new(1.0/11.0, 2.0/11.0, 4.0/11.0, 10.0/11.0)));
    // TODO: test normalize_to, normalize_self, and normalize_self_to

    let c = vec4::new(-2.0, -1.0, 1.0, 2.0);
    let d = vec4::new( 1.0,  0.0, 0.5, 1.0);

    assert_eq!(c.lerp(&d, 0.75), vec4::new(0.250, -0.250, 0.625, 1.250));

    let mut mut_c = c;
    mut_c.lerp_self(&d, 0.75);
    assert_eq!(mut_c, c.lerp(&d, 0.75));
}

#[test]
fn test_vec4_boolean() {
    let tftf = bvec4::new(true, false, true, false);
    let ffff = bvec4::new(false, false, false, false);
    let tttt = bvec4::new(true, true, true, true);

    assert_eq!(tftf.any(), true);
    assert_eq!(tftf.all(), false);
    assert_eq!(tftf.not(), bvec4::new(false, true, false, true));

    assert_eq!(ffff.any(), false);
    assert_eq!(ffff.all(), false);
    assert_eq!(ffff.not(), bvec4::new(true, true, true, true));

    assert_eq!(tttt.any(), true);
    assert_eq!(tttt.all(), true);
    assert_eq!(tttt.not(), bvec4::new(false, false, false, false));
}