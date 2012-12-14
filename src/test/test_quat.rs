use mat::*;
use quat::*;
use vec::*;
use funs::exponential::sqrt;

// TODO

#[test]
fn test_Quat() {
    let a = Quat { s: 1f, v: Vec3 { x: 2f, y: 3f, z: 4f } };
    
    assert a == Quat::from_sv(1f, Vec3::new(2f, 3f, 4f));
    assert a == Quat::new(1f, 2f, 3f, 4f);
    
    assert Quat::zero()     == Quat::new(0f, 0f, 0f, 0f);
    assert Quat::identity() == Quat::new(1f, 0f, 0f, 0f);
    
    assert a.s == 1f;
    assert a.v.x == 2f;
    assert a.v.y == 3f;
    assert a.v.z == 4f;
    assert a[0] == 1f;
    assert a[1] == 2f;
    assert a[2] == 3f;
    assert a[3] == 4f;
    // TODO
}

#[test]
fn test_quat_2() {
    let v = Vec3::new(1.0, 0.0, 0.0);
    
    // let q: Quat<float> = Quaternion::from_axis_angle(&Vec3::new(0.0, 0.0, -1.0), Degrees(-90.0));
    let q = Quat::from_axis_angle(&Vec3::new(0.0, 0.0, -1.0), Degrees(-45.0));
    
    // http://www.wolframalpha.com/input/?i={1,0}+rotate+-45+degrees
    assert q.mul_v(&v).fuzzy_eq(&Vec3::new(1.0/sqrt(&2.0), 1.0/sqrt(&2.0), 0.0));
    assert q.mul_v(&v).length() == v.length();
    assert q.to_mat3().fuzzy_eq(&Mat3::new( 1.0/sqrt(&2.0), 1.0/sqrt(&2.0), 0.0,
                                           -1.0/sqrt(&2.0), 1.0/sqrt(&2.0), 0.0,
                                                       0.0,            0.0, 1.0));
}