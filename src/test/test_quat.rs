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

/*

Here is what I use to test my quaternions:

* First, rotate an arbitrary vector around an arbitrary axis using an arbitrary angle (of course with a quaternion).

* Then compare the original to the result vector, considering length and angle. If the length is constant and the angle is as desired, everything is fine and you are "green".

* Now generate a series of rotation quaternions and concatenate them. Transform an arbitrary vector with it.

* Now use each individual transformation of that series to transform the vector again, but this time checking angle and length like in the single transformation case. Only proceed while you stay "green".

* Finally, compare both results. If they match, you are "green".

*/