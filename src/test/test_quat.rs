use std::cmp::FuzzyEq;
use numeric::*;

use mat::*;
use quat::*;
use vec::*;

// TODO

#[test]
fn test_quat() {
    let a = Quat { s: 1.0, v: Vec3 { x: 2.0, y: 3.0, z: 4.0 } };
    
    assert!(a == quat::from_sv(1.0, vec3::new(2.0, 3.0, 4.0)));
    assert!(a == quat::new(1.0, 2.0, 3.0, 4.0));
    
    assert!(quat::zero()     == quat::new(0.0, 0.0, 0.0, 0.0));
    assert!(quat::identity() == quat::new(1.0, 0.0, 0.0, 0.0));
    
    assert!(a.s == 1.0);
    assert!(a.v.x == 2.0);
    assert!(a.v.y == 3.0);
    assert!(a.v.z == 4.0);
    assert!(a[0] == 1.0);
    assert!(a[1] == 2.0);
    assert!(a[2] == 3.0);
    assert!(a[3] == 4.0);
    // TODO
}

#[test]
fn test_quat_2() {
    let v = vec3::new(1f32, 0f32, 0f32);
    
    let q = quat::from_angle_axis(radians(-45f32), &vec3::new(0f32, 0f32, -1f32));
    
    // http://www.wolframalpha.com/input/?i={1,0}+rotate+-45+degrees
    assert!(q.mul_v(&v).fuzzy_eq(&vec3::new(1f32/sqrt(2f32), 1f32/sqrt(2f32), 0f32)));
    assert!(q.mul_v(&v).length() == v.length());
    assert!(q.to_mat3().fuzzy_eq(&mat3::new( 1f32/sqrt(2f32), 1f32/sqrt(2f32), 0f32,
                                            -1f32/sqrt(2f32), 1f32/sqrt(2f32), 0f32,
                                                       0f32,           0f32, 1f32)));
}

#[test]
fn test_quat_fuzzy_eq() {
    assert!(!quat::new(0.000001, 0.000001, 0.000001, 0.000001).fuzzy_eq(&quat::new(0.0, 0.0, 0.0, 0.0)));
    assert!(quat::new(0.0000001, 0.0000001, 0.0000001, 0.0000001).fuzzy_eq(&quat::new(0.0, 0.0, 0.0, 0.0)));
}