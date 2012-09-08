import omath::mat::*;
import omath::quat::*;
import omath::vec::*;

// TODO

#[test]
fn test_quat() {
    let a = quat { data: [ 1f, 2f, 3f, 4f ] };
    // let b = quat { data: [ 5f, 6f, 7f, 8f ] };
    // let f1 = 1.5f;
    // let f2 = 0.5f;
    
    assert a == quat(1f, 2f, 3f, 4f);
    
    assert quat_zero     == quat(0f, 0f, 0f, 0f);
    assert quat_identity == quat(1f, 0f, 0f, 0f);
    
    assert a[0] == 1f;
    assert a[1] == 2f;
    assert a[2] == 3f;
    assert a[3] == 4f;
    assert a.w() == 1f;
    assert a.x() == 2f;
    assert a.y() == 3f;
    assert a.z() == 4f;
    
    // TODO
}