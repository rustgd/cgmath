use mat::*;
use quat::*;
use vec::*;

// TODO

#[test]
fn test_Quat() {
    let a = Quat { s: 1f, v: Vec3 { x: 2f, y: 3f, z: 4f } };
    
    assert a == Quat::from_sv(1f, Vec3::new(2f, 3f, 4f));
    assert a == Quat::new(1f, 2f, 3f, 4f);
    
    // assert Quat::zero()     == Quat::new(0f, 0f, 0f, 0f);
    // assert Quat::identity() == Quat::new(1f, 0f, 0f, 0f);
    
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