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
    
    assert Vec2::new(1f, 2f) == a;
    assert Vec2::from_value(1f32) == Vec2::new(1f32, 1f32);
    
    assert Vec2::zero()     == Vec2::new(0f, 0f);
    assert Vec2::unit_x()   == Vec2::new(1f, 0f);
    assert Vec2::unit_y()   == Vec2::new(0f, 1f);
    assert Vec2::identity() == Vec2::new(1f, 1f);
    
    assert a.x == 1f;
    assert a.y == 2f;
    assert a[0] == 1f;
    assert a[1] == 2f;
    
    assert -a      == Vec2::new(-1f, -2f);
    assert a.neg() == Vec2::new(-1f, -2f);
    
    assert a.add_t(f1) == Vec2::new( 2.5f, 3.5f);
    assert a.sub_t(f1) == Vec2::new(-0.5f, 0.5f);
    assert a.mul_t(f1) == Vec2::new( 1.5f, 3.0f);
    assert a.div_t(f2) == Vec2::new( 2.0f, 4.0f);
    
    assert a.add_v(&b) == Vec2::new( 4f,  6f);
    assert a.sub_v(&b) == Vec2::new(-2f, -2f);
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert a.magnitude2().fuzzy_eq(&5f);
    assert a.magnitude().fuzzy_eq(&2.236068f);
    
    let c = Vec2::new(-2.0f, -1.0f);
    let d = Vec2::new( 1.0f,  0.0f);
    let f3 = 0.75f;
    
    assert c.lerp(&d, f3) == Vec2::new(0.250f, -0.250f);
    assert c.abs()       == Vec2::new( 2.0f,  1.0f);
    assert c.min(&d)      == Vec2::new(-2.0f, -1.0f);
    assert c.max(&d)      == Vec2::new( 1.0f,  0.0f);
}

#[test]
fn test_Vec3() {
    // assert Vec3::dim == 3;
    
    let a = Vec3 { x: 1f, y: 2f, z: 3f };
    let b = Vec3 { x: 4f, y: 5f, z: 6f };
    let f1 = 1.5f;
    let f2 = 0.5f;
    
    assert Vec3::new(1f, 2f, 3f) == a;
    assert Vec3::from_value(1f32) == Vec3::new(1f32, 1f32, 1f32);
    
    assert Vec3::zero()     == Vec3::new(0f, 0f, 0f);
    assert Vec3::unit_x()   == Vec3::new(1f, 0f, 0f);
    assert Vec3::unit_y()   == Vec3::new(0f, 1f, 0f);
    assert Vec3::unit_z()   == Vec3::new(0f, 0f, 1f);
    assert Vec3::identity() == Vec3::new(1f, 1f, 1f);
    
    assert a.x == 1f;
    assert a.y == 2f;
    assert a.z == 3f;
    assert a[0] == 1f;
    assert a[1] == 2f;
    assert a[2] == 3f;
    
    assert a.cross(&b) == Vec3::new(-3f, 6f, -3f);
    
    assert -a      == Vec3::new(-1f, -2f, -3f);
    assert a.neg() == Vec3::new(-1f, -2f, -3f);
    
    assert a.add_t(f1) == Vec3::new( 2.5f, 3.5f, 4.5f);
    assert a.sub_t(f1) == Vec3::new(-0.5f, 0.5f, 1.5f);
    assert a.mul_t(f1) == Vec3::new( 1.5f, 3.0f, 4.5f);
    assert a.div_t(f2) == Vec3::new( 2.0f, 4.0f, 6.0f);
    
    assert a.add_v(&b) == Vec3::new( 5f,  7f,  9f);
    assert a.sub_v(&b) == Vec3::new(-3f, -3f, -3f);
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert a.magnitude2().fuzzy_eq(&14f);
    assert a.magnitude().fuzzy_eq(&3.74165738677f);
    
    let c = Vec3::new(-2.0f, -1.0f, 1.0f);
    let d = Vec3::new( 1.0f,  0.0f, 0.5f);
    let f3 = 0.75f;
    
    assert c.lerp(&d, f3) == Vec3::new(0.250f, -0.250f, 0.625f);
    assert c.abs()        == Vec3::new( 2.0f,  1.0f, 1.0f);
    assert c.min(&d)      == Vec3::new(-2.0f, -1.0f, 0.5f);
    assert c.max(&d)      == Vec3::new( 1.0f,  0.0f, 1.0f);
}

#[test]
fn test_Vec4() {
    // assert Vec4::dim == 4;
    
    let a = Vec4 { x: 1f, y: 2f, z: 3f, w: 4f };
    let b = Vec4 { x: 5f, y: 6f, z: 7f, w: 8f };
    let f1 = 1.5f;
    let f2 = 0.5f;
    
    assert Vec4::new(1f, 2f, 3f, 4f) == a;
    assert Vec4::from_value(1f32) == Vec4::new(1f32, 1f32, 1f32, 1f32);
    
    assert Vec4::zero()     == Vec4::new(0f, 0f, 0f, 0f);
    assert Vec4::unit_x()   == Vec4::new(1f, 0f, 0f, 0f);
    assert Vec4::unit_y()   == Vec4::new(0f, 1f, 0f, 0f);
    assert Vec4::unit_z()   == Vec4::new(0f, 0f, 1f, 0f);
    assert Vec4::unit_w()   == Vec4::new(0f, 0f, 0f, 1f);
    assert Vec4::identity() == Vec4::new(1f, 1f, 1f, 1f);
    
    assert a.x == 1f;
    assert a.y == 2f;
    assert a.z == 3f;
    assert a.w == 4f;
    assert a[0] == 1f;
    assert a[1] == 2f;
    assert a[2] == 3f;
    assert a[3] == 4f;
    
    assert -a      == Vec4::new(-1f, -2f, -3f, -4f);
    assert a.neg() == Vec4::new(-1f, -2f, -3f, -4f);
    
    assert a.add_t(f1) == Vec4::new( 2.5f, 3.5f, 4.5f, 5.5f);
    assert a.sub_t(f1) == Vec4::new(-0.5f, 0.5f, 1.5f, 2.5f);
    assert a.mul_t(f1) == Vec4::new( 1.5f, 3.0f, 4.5f, 6.0f);
    assert a.div_t(f2) == Vec4::new( 2.0f, 4.0f, 6.0f, 8.0f);
    
    assert a.add_v(&b) == Vec4::new( 6f,  8f, 10f, 12f);
    assert a.sub_v(&b) == Vec4::new(-4f, -4f, -4f, -4f);
    
    assert a.dot(&b) == 70f;
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert a.magnitude2().fuzzy_eq(&30f);
    assert a.magnitude().fuzzy_eq(&5.477226f);
    
    let c = Vec4::new(-2.0f, -1.0f, 1.0f, 2.0f);
    let d = Vec4::new( 1.0f,  0.0f, 0.5f, 1.0f);
    let f3 = 0.75f;
    
    assert c.lerp(&d, f3) == Vec4::new(0.250f, -0.250f, 0.625f, 1.250f);
    assert c.abs()        == Vec4::new( 2.0f,  1.0f, 1.0f, 2.0f);
    assert c.min(&d)      == Vec4::new(-2.0f, -1.0f, 0.5f, 1.0f);
    assert c.max(&d)      == Vec4::new( 1.0f,  0.0f, 1.0f, 2.0f);
}