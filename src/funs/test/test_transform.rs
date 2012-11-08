use funs::transform::*;
use matrix::Mat4;
use vector::{Vec3, Vec4};

#[test]
fn test_mat4_from_rotation() {
    let a_theta = Degrees(180f).to_radians();
    let a_axis = Vec3::unit_z();
    let a_pos = Vec4::new(1f32, 0f32, 0f32, 1f32);
    
    let a_tform = mat4_from_rotation(a_theta, a_axis).mul_v(&a_pos);
    let a_expected = Vec4::new(-1f32, 0f32, 0f32, 1f32);
    
    assert a_tform == a_expected;
}