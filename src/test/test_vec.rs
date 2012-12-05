use std::cmp::FuzzyEq;
use vec::*;

// TODO

#[test]
fn test_Vec2() {
    // assert Vec2::dim == 2;
    
    let a = Vec2 { x: 1f, y: 2f };
    let b = Vec2 { x: 3f, y: 4f };
    let f1 = 1.5f;
    let f2 = 0.5f;
    
    let mut mut_a = a;
    
    assert Vec2::new(1f, 2f) == a;
    // assert Vec2::from_value(1f32) == Vec2::new(1f32, 1f32);
    
    // assert Vec2::zero()     == Vec2::new(0f, 0f);
    // assert Vec2::unit_x()   == Vec2::new(1f, 0f);
    // assert Vec2::unit_y()   == Vec2::new(0f, 1f);
    // assert Vec2::identity() == Vec2::new(1f, 1f);
    
    *mut_a.index_mut(0) = 42f;
    *mut_a.index_mut(1) = 43f;
    assert mut_a == Vec2::new(42f, 43f);
    mut_a = a;
    
    mut_a.swap(0, 1);
    assert mut_a[0] == a[1];
    assert mut_a[1] == a[0];
    mut_a = a;
    
    assert a.x == 1f;
    assert a.y == 2f;
    assert a[0] == 1f;
    assert a[1] == 2f;
    
    assert -a      == Vec2::new(-1f, -2f);
    assert a.neg() == Vec2::new(-1f, -2f);
    
    assert a.mul_t(f1) == Vec2::new( 1.5f, 3.0f);
    assert a.div_t(f2) == Vec2::new( 2.0f, 4.0f);
    
    assert a.add_v(&b) == Vec2::new( 4f,  6f);
    assert a.sub_v(&b) == Vec2::new(-2f, -2f);
    
    mut_a.neg_self();
    assert mut_a == -a;
    mut_a = a;
    
    mut_a.mul_self_t(&f1);
    assert mut_a == a.mul_t(f1);
    mut_a = a;
    
    mut_a.div_self_t(&f2);
    assert mut_a == a.div_t(f2);
    mut_a = a;
    
    mut_a.add_self_v(&b);
    assert mut_a == a.add_v(&b);
    mut_a = a;
    
    mut_a.sub_self_v(&b);
    assert mut_a == a.sub_v(&b);
    // mut_a = a;
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    // assert c.abs()       == Vec2::new( 2.0f,  1.0f);
    // assert c.min(&d)      == Vec2::new(-2.0f, -1.0f);
    // assert c.max(&d)      == Vec2::new( 1.0f,  0.0f);
}

#[test]
fn test_Vec2_euclidean() {
    let a = Vec2::new(5f, 12f); // (5, 12, 13) Pythagorean triple
    let b0 = Vec2::new(3f, 4f); // (3, 4, 5) Pythagorean triple
    let b = a.add_v(&b0);
    
    // TODO: test normalize and normalize_self
    
    assert a.length() == 13f;
    assert a.length2() == 13f * 13f;
    
    assert b0.length() == 5f;
    assert b0.length2() == 5f * 5f;
    
    assert a.distance(&b) == 5f;
    assert a.distance2(&b) == 5f * 5f;
    
    let c = Vec2::new(-2.0f, -1.0f);
    let d = Vec2::new( 1.0f,  0.0f);
    
    assert c.lerp(&d, 0.75f) == Vec2::new(0.250f, -0.250f);
    
    let mut mut_c = c;
    mut_c.lerp_self(&d, &0.75f);
    assert mut_c == c.lerp(&d, 0.75f);
}

#[test]
fn test_Vec3() {
    // assert Vec3::dim == 3;
    
    let a = Vec3 { x: 1f, y: 2f, z: 3f };
    let b = Vec3 { x: 4f, y: 5f, z: 6f };
    let f1 = 1.5f;
    let f2 = 0.5f;
    
    let mut mut_a = a;
    
    assert Vec3::new(1f, 2f, 3f) == a;
    // assert Vec3::from_value(1f32) == Vec3::new(1f32, 1f32, 1f32);
    
    // assert Vec3::zero()     == Vec3::new(0f, 0f, 0f);
    // assert Vec3::unit_x()   == Vec3::new(1f, 0f, 0f);
    // assert Vec3::unit_y()   == Vec3::new(0f, 1f, 0f);
    // assert Vec3::unit_z()   == Vec3::new(0f, 0f, 1f);
    // assert Vec3::identity() == Vec3::new(1f, 1f, 1f);
    
    *mut_a.index_mut(0) = 42f;
    *mut_a.index_mut(1) = 43f;
    *mut_a.index_mut(2) = 44f;
    assert mut_a == Vec3::new(42f, 43f, 44f);
    mut_a = a;
    
    mut_a.swap(0, 2);
    assert mut_a[0] == a[2];
    assert mut_a[2] == a[0];
    mut_a = a;
    
    mut_a.swap(1, 2);
    assert mut_a[1] == a[2];
    assert mut_a[2] == a[1];
    mut_a = a;
    
    assert a.x == 1f;
    assert a.y == 2f;
    assert a.z == 3f;
    assert a[0] == 1f;
    assert a[1] == 2f;
    assert a[2] == 3f;
    
    assert a.cross(&b) == Vec3::new(-3f, 6f, -3f);
    
    mut_a.cross_self(&b);
    assert mut_a == a.cross(&b);
    mut_a = a;
    
    assert -a      == Vec3::new(-1f, -2f, -3f);
    assert a.neg() == Vec3::new(-1f, -2f, -3f);
    
    assert a.mul_t(f1) == Vec3::new( 1.5f, 3.0f, 4.5f);
    assert a.div_t(f2) == Vec3::new( 2.0f, 4.0f, 6.0f);
    
    assert a.add_v(&b) == Vec3::new( 5f,  7f,  9f);
    assert a.sub_v(&b) == Vec3::new(-3f, -3f, -3f);
    
    mut_a.neg_self();
    assert mut_a == -a;
    mut_a = a;
    
    mut_a.mul_self_t(&f1);
    assert mut_a == a.mul_t(f1);
    mut_a = a;
    
    mut_a.div_self_t(&f2);
    assert mut_a == a.div_t(f2);
    mut_a = a;
    
    mut_a.add_self_v(&b);
    assert mut_a == a.add_v(&b);
    mut_a = a;
    
    mut_a.sub_self_v(&b);
    assert mut_a == a.sub_v(&b);
    // mut_a = a;
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    // assert c.abs()        == Vec3::new( 2.0f,  1.0f, 1.0f);
    // assert c.min(&d)      == Vec3::new(-2.0f, -1.0f, 0.5f);
    // assert c.max(&d)      == Vec3::new( 1.0f,  0.0f, 1.0f);
}

#[test]
fn test_Vec3_euclidean() {
    let a = Vec3::new(2f, 3f, 6f); // (2, 3, 6, 7) Pythagorean quadruple
    let b0 = Vec3::new(1f, 4f, 8f); // (1, 4, 8, 9) Pythagorean quadruple
    let b = a.add_v(&b0);
    
    // TODO: test normalize and normalize_self
    
    assert a.length() == 7f;
    assert a.length2() == 7f * 7f;
    
    assert b0.length() == 9f;
    assert b0.length2() == 9f * 9f;
    
    assert a.distance(&b) == 9f;
    assert a.distance2(&b) == 9f * 9f;
    
    let c = Vec3::new(-2.0f, -1.0f, 1.0f);
    let d = Vec3::new( 1.0f,  0.0f, 0.5f);
    
    assert c.lerp(&d, 0.75f) == Vec3::new(0.250f, -0.250f, 0.625f);
    
    let mut mut_c = c;
    mut_c.lerp_self(&d, &0.75f);
    assert mut_c == c.lerp(&d, 0.75f);
}

#[test]
fn test_Vec4() {
    // assert Vec4::dim == 4;
    
    let a = Vec4 { x: 1f, y: 2f, z: 3f, w: 4f };
    let b = Vec4 { x: 5f, y: 6f, z: 7f, w: 8f };
    let f1 = 1.5f;
    let f2 = 0.5f;
    
    let mut mut_a = a;
    
    assert Vec4::new(1f, 2f, 3f, 4f) == a;
    // assert Vec4::from_value(1f32) == Vec4::new(1f32, 1f32, 1f32, 1f32);
    
    *mut_a.index_mut(0) = 42f;
    *mut_a.index_mut(1) = 43f;
    *mut_a.index_mut(2) = 44f;
    *mut_a.index_mut(3) = 45f;
    assert mut_a == Vec4::new(42f, 43f, 44f, 45f);
    mut_a = a;
    
    mut_a.swap(0, 3);
    assert mut_a[0] == a[3];
    assert mut_a[3] == a[0];
    mut_a = a;
    
    mut_a.swap(1, 2);
    assert mut_a[1] == a[2];
    assert mut_a[2] == a[1];
    mut_a = a;
    
    // assert Vec4::zero()     == Vec4::new(0f, 0f, 0f, 0f);
    // assert Vec4::unit_x()   == Vec4::new(1f, 0f, 0f, 0f);
    // assert Vec4::unit_y()   == Vec4::new(0f, 1f, 0f, 0f);
    // assert Vec4::unit_z()   == Vec4::new(0f, 0f, 1f, 0f);
    // assert Vec4::unit_w()   == Vec4::new(0f, 0f, 0f, 1f);
    // assert Vec4::identity() == Vec4::new(1f, 1f, 1f, 1f);
    
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
    
    assert a.mul_t(f1) == Vec4::new( 1.5f, 3.0f, 4.5f, 6.0f);
    assert a.div_t(f2) == Vec4::new( 2.0f, 4.0f, 6.0f, 8.0f);
    
    assert a.add_v(&b) == Vec4::new( 6f,  8f, 10f, 12f);
    assert a.sub_v(&b) == Vec4::new(-4f, -4f, -4f, -4f);
    
    assert a.dot(&b) == 70f;
    
    mut_a.neg_self();
    assert mut_a == -a;
    mut_a = a;
    
    mut_a.mul_self_t(&f1);
    assert mut_a == a.mul_t(f1);
    mut_a = a;
    
    mut_a.div_self_t(&f2);
    assert mut_a == a.div_t(f2);
    mut_a = a;
    
    mut_a.add_self_v(&b);
    assert mut_a == a.add_v(&b);
    mut_a = a;
    
    mut_a.sub_self_v(&b);
    assert mut_a == a.sub_v(&b);
    // mut_a = a;
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    // assert c.abs()        == Vec4::new( 2.0f,  1.0f, 1.0f, 2.0f);
    // assert c.min(&d)      == Vec4::new(-2.0f, -1.0f, 0.5f, 1.0f);
    // assert c.max(&d)      == Vec4::new( 1.0f,  0.0f, 1.0f, 2.0f);
}

#[test]
fn test_Vec4_euclidean() {
    let a = Vec4::new(1f, 2f, 4f, 10f); // (1, 2, 4, 10, 11) Pythagorean quintuple
    let b0 = Vec4::new(1f, 2f, 8f, 10f); // (1, 2, 8, 10, 13) Pythagorean quintuple
    let b = a.add_v(&b0);
    
    // TODO: test normalize and normalize_self
    
    assert a.length() == 11f;
    assert a.length2() == 11f * 11f;
    
    assert b0.length() == 13f;
    assert b0.length2() == 13f * 13f;
    
    assert a.distance(&b) == 13f;
    assert a.distance2(&b) == 13f * 13f;

    let c = Vec4::new(-2.0f, -1.0f, 1.0f, 2.0f);
    let d = Vec4::new( 1.0f,  0.0f, 0.5f, 1.0f);
    
    assert c.lerp(&d, 0.75f) == Vec4::new(0.250f, -0.250f, 0.625f, 1.250f);
    
    let mut mut_c = c;
    mut_c.lerp_self(&d, &0.75f);
    assert mut_c == c.lerp(&d, 0.75f);
}