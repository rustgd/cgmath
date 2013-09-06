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

use cgmath::vector::*;

#[test]
fn test_from_value() {
    assert_eq!(Vec2::from_value(1), Vec2::new(1, 1));
    assert_eq!(Vec3::from_value(1), Vec3::new(1, 1, 1));
    assert_eq!(Vec4::from_value(1), Vec4::new(1, 1, 1, 1));
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

    assert_eq!(Vec2::new(3.0, 4.0).comp_add(), 7.0);
    assert_eq!(Vec3::new(4.0, 5.0, 6.0).comp_add(), 15.0);
    assert_eq!(Vec4::new(5.0, 6.0, 7.0, 8.0).comp_add(), 26.0);
}

#[test]
fn test_comp_mul() {
    assert_eq!(Vec2::new(1, 2).comp_mul(), 2);
    assert_eq!(Vec3::new(1, 2, 3).comp_mul(), 6);
    assert_eq!(Vec4::new(1, 2, 3, 4).comp_mul(), 24);

    assert_eq!(Vec2::new(3.0, 4.0).comp_mul(), 12.0);
    assert_eq!(Vec3::new(4.0, 5.0, 6.0).comp_mul(), 120.0);
    assert_eq!(Vec4::new(5.0, 6.0, 7.0, 8.0).comp_mul(), 1680.0);
}

#[test]
fn test_comp_min() {
    assert_eq!(Vec2::new(1, 2).comp_min(), 1);
    assert_eq!(Vec3::new(1, 2, 3).comp_min(), 1);
    assert_eq!(Vec4::new(1, 2, 3, 4).comp_min(), 1);

    assert_eq!(Vec2::new(3.0, 4.0).comp_min(), 3.0);
    assert_eq!(Vec3::new(4.0, 5.0, 6.0).comp_min(), 4.0);
    assert_eq!(Vec4::new(5.0, 6.0, 7.0, 8.0).comp_min(), 5.0);
}

#[test]
fn test_comp_max() {
    assert_eq!(Vec2::new(1, 2).comp_max(), 2);
    assert_eq!(Vec3::new(1, 2, 3).comp_max(), 3);
    assert_eq!(Vec4::new(1, 2, 3, 4).comp_max(), 4);

    assert_eq!(Vec2::new(3.0, 4.0).comp_max(), 4.0);
    assert_eq!(Vec3::new(4.0, 5.0, 6.0).comp_max(), 6.0);
    assert_eq!(Vec4::new(5.0, 6.0, 7.0, 8.0).comp_max(), 8.0);
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
