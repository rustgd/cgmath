use std::cmp::FuzzyEq;
use vector::*;

// TODO

#[test]
fn test_Vec2() {
    // assert Vec2::dim == 2;
    
    let a = Vec2 { x: 1f, y: 2f };
    let b = Vec2 { x: 3f, y: 4f };
    let f1 = 1.5f;
    let f2 = 0.5f;
    
    assert a == Vec2(1f, 2f);
    
    assert vec2_zero     == Vec2(0f, 0f);
    assert vec2_unit_x   == Vec2(1f, 0f);
    assert vec2_unit_y   == Vec2(0f, 1f);
    assert vec2_identity == Vec2(1f, 1f);
    
    assert a.x == 1f;
    assert a.y == 2f;
    assert a[0] == 1f;
    assert a[1] == 2f;
    
    assert -a      == Vec2(-1f, -2f);
    assert a.neg() == Vec2(-1f, -2f);
    
    assert a.add_f(f1) == Vec2( 2.5f, 3.5f);
    assert a.sub_f(f1) == Vec2(-0.5f, 0.5f);
    assert a.mul_f(f1) == Vec2( 1.5f, 3.0f);
    assert a.div_f(f2) == Vec2( 2.0f, 4.0f);
    
    assert a.add_v(&b) == Vec2( 4f,  6f);
    assert a.sub_v(&b) == Vec2(-2f, -2f);
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert a.magnitude2().fuzzy_eq(&5f);
    assert a.magnitude().fuzzy_eq(&2.236068f);
    
    let c = Vec2(-2.0f, -1.0f);
    let d = Vec2( 1.0f,  0.0f);
    let f3 = 0.75f;
    
    assert c.lerp(&d, f3) == Vec2(0.250f, -0.250f);
    assert c.abs()       == Vec2( 2.0f,  1.0f);
    assert c.min(&d)      == Vec2(-2.0f, -1.0f);
    assert c.max(&d)      == Vec2( 1.0f,  0.0f);
}

#[test]
fn test_Vec3() {
    // assert Vec3::dim == 3;
    
    let a = Vec3 { x: 1f, y: 2f, z: 3f };
    let b = Vec3 { x: 4f, y: 5f, z: 6f };
    let f1 = 1.5f;
    let f2 = 0.5f;
    
    assert a == Vec3(1f, 2f, 3f);
    
    assert vec3_zero     == Vec3(0f, 0f, 0f);
    assert vec3_unit_x   == Vec3(1f, 0f, 0f);
    assert vec3_unit_y   == Vec3(0f, 1f, 0f);
    assert vec3_unit_z   == Vec3(0f, 0f, 1f);
    assert vec3_identity == Vec3(1f, 1f, 1f);
    
    assert a.x == 1f;
    assert a.y == 2f;
    assert a.z == 3f;
    assert a[0] == 1f;
    assert a[1] == 2f;
    assert a[2] == 3f;
    
    assert a.cross(&b) == Vec3(-3f, 6f, -3f);
    
    assert -a      == Vec3(-1f, -2f, -3f);
    assert a.neg() == Vec3(-1f, -2f, -3f);
    
    assert a.add_f(f1) == Vec3( 2.5f, 3.5f, 4.5f);
    assert a.sub_f(f1) == Vec3(-0.5f, 0.5f, 1.5f);
    assert a.mul_f(f1) == Vec3( 1.5f, 3.0f, 4.5f);
    assert a.div_f(f2) == Vec3( 2.0f, 4.0f, 6.0f);
    
    assert a.add_v(&b) == Vec3( 5f,  7f,  9f);
    assert a.sub_v(&b) == Vec3(-3f, -3f, -3f);
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert a.magnitude2().fuzzy_eq(&14f);
    assert a.magnitude().fuzzy_eq(&3.74165738677f);
    
    let c = Vec3(-2.0f, -1.0f, 1.0f);
    let d = Vec3( 1.0f,  0.0f, 0.5f);
    let f3 = 0.75f;
    
    assert c.lerp(&d, f3) == Vec3(0.250f, -0.250f, 0.625f);
    assert c.abs()        == Vec3( 2.0f,  1.0f, 1.0f);
    assert c.min(&d)      == Vec3(-2.0f, -1.0f, 0.5f);
    assert c.max(&d)      == Vec3( 1.0f,  0.0f, 1.0f);
}

#[test]
fn test_Vec4() {
    // assert Vec4::dim == 4;
    
    let a = Vec4 { x: 1f, y: 2f, z: 3f, w: 4f };
    let b = Vec4 { x: 5f, y: 6f, z: 7f, w: 8f };
    let f1 = 1.5f;
    let f2 = 0.5f;
    
    assert a == Vec4(1f, 2f, 3f, 4f);
    
    assert vec4_zero     == Vec4(0f, 0f, 0f, 0f);
    assert vec4_unit_x   == Vec4(1f, 0f, 0f, 0f);
    assert vec4_unit_y   == Vec4(0f, 1f, 0f, 0f);
    assert vec4_unit_z   == Vec4(0f, 0f, 1f, 0f);
    assert vec4_unit_w   == Vec4(0f, 0f, 0f, 1f);
    assert vec4_identity == Vec4(1f, 1f, 1f, 1f);
    
    assert a.x == 1f;
    assert a.y == 2f;
    assert a.z == 3f;
    assert a.w == 4f;
    assert a[0] == 1f;
    assert a[1] == 2f;
    assert a[2] == 3f;
    assert a[3] == 4f;
    
    assert -a      == Vec4(-1f, -2f, -3f, -4f);
    assert a.neg() == Vec4(-1f, -2f, -3f, -4f);
    
    assert a.add_f(f1) == Vec4( 2.5f, 3.5f, 4.5f, 5.5f);
    assert a.sub_f(f1) == Vec4(-0.5f, 0.5f, 1.5f, 2.5f);
    assert a.mul_f(f1) == Vec4( 1.5f, 3.0f, 4.5f, 6.0f);
    assert a.div_f(f2) == Vec4( 2.0f, 4.0f, 6.0f, 8.0f);
    
    assert a.add_v(&b) == Vec4( 6f,  8f, 10f, 12f);
    assert a.sub_v(&b) == Vec4(-4f, -4f, -4f, -4f);
    
    assert a.dot(&b) == 70f;
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert a.magnitude2().fuzzy_eq(&30f);
    assert a.magnitude().fuzzy_eq(&5.477226f);
    
    let c = Vec4(-2.0f, -1.0f, 1.0f, 2.0f);
    let d = Vec4( 1.0f,  0.0f, 0.5f, 1.0f);
    let f3 = 0.75f;
    
    assert c.lerp(&d, f3) == Vec4(0.250f, -0.250f, 0.625f, 1.250f);
    assert c.abs()        == Vec4( 2.0f,  1.0f, 1.0f, 2.0f);
    assert c.min(&d)      == Vec4(-2.0f, -1.0f, 0.5f, 1.0f);
    assert c.max(&d)      == Vec4( 1.0f,  0.0f, 1.0f, 2.0f);
}