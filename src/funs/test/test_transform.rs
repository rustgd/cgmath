use funs::transform::*;
use mat::Mat4;
use vec::{Vec3, Vec4};

#[test]
fn test_mat4_from_rotation() {
    {
        let pos = Vec4::new(1f32, 0f32, 0f32, 1f32);
        let tform = mat4_from_rotation(180f32, Vec3::unit_z());
        let newpos = tform.mul_v(&pos);
        
        let expected = Vec4::new(-1f32, 0f32, 0f32, 1f32);
        
        assert newpos == expected;
    }
    {
        let pos = Vec4::new(4f32, 0f32, 0f32, 1f32);
        
        let tform_a = mat4_from_rotation(90f32,  Vec3::unit_y());
        let tform_b = mat4_from_rotation(90f32, -Vec3::unit_y());
        let newpos_a = tform_a.mul_v(&pos);
        let newpos_b = tform_b.mul_v(&pos);
        
        let expected_a = Vec4::new(0f32, 0f32, -4f32, 1f32);
        let expected_b = Vec4::new(0f32, 0f32,  4f32, 1f32);
        
        assert newpos_a == expected_a;
        assert newpos_b == expected_b;
    }
}