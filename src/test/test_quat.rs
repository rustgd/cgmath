use mat::*;
use quat::*;
use vec::*;

// TODO

#[test]
fn test_Quat() {
    let a = Quat { w: 1f, x: 2f, y: 3f, z: 4f };
    // let b = Quat { data: [ 5f, 6f, 7f, 8f ] };
    // let f1 = 1.5f;
    // let f2 = 0.5f;
    
    assert a == Quat(1f, 2f, 3f, 4f);
    
    assert quat_zero     == Quat(0f, 0f, 0f, 0f);
    assert quat_identity == Quat(1f, 0f, 0f, 0f);
    
    assert a.w == 1f;
    assert a.x == 2f;
    assert a.y == 3f;
    assert a.z == 4f;
    assert a[0] == 1f;
    assert a[1] == 2f;
    assert a[2] == 3f;
    assert a[3] == 4f;
    
    // TODO
}