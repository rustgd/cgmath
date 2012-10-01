use omath::mat::*;
use omath::vec::*;

// TODO

#[test]
fn test_Mat2() {
    let a = Mat2 { data: [ Vec2 { data: [ 1f, 3f ] },
                           Vec2 { data: [ 2f, 4f ] } ] };
    let b = Mat2 { data: [ Vec2 { data: [ 2f, 4f ] },
                           Vec2 { data: [ 3f, 5f ] } ] };
    let v1 = Vec2(1f, 2f);
    let f1 = 0.5f;
    
    assert a == Mat2(1f, 3f,
                     2f, 4f);
    
    assert a == Mat2_v(&Vec2(1f, 3f),
                       &Vec2(2f, 4f));
    
    assert a[0] == Vec2(1f, 3f);
    assert a[1] == Vec2(2f, 4f);
    
    assert a.row(0) == Vec2(1f, 2f);
    assert a.row(1) == Vec2(3f, 4f);
    
    assert a.col(0) == Vec2(1f, 3f);
    assert a.col(1) == Vec2(2f, 4f);
    
    assert a.neg() == Mat2(-1f, -3f,
                           -2f, -4f);
    assert -a == a.neg();
    
    assert a.mul_f(f1) == Mat2(0.5f, 1.5f,
                               1.0f, 2.0f);
    assert a.mul_v(&v1) == Vec2(5f, 11f);
    
    assert a.add_m(&b) == Mat2(3f, 7f,
                               5f, 9f);
    assert a.sub_m(&b) == Mat2(-1f, -1f,
                               -1f, -1f);
    assert a.mul_m(&b) == Mat2(10.0, 22.0,
                               13.0, 29.0);
    
    assert a.transpose() == Mat2(1f, 2f,
                                 3f, 4f);
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert mat2_identity.is_identity();
    assert mat2_identity.is_symmetric();
    assert mat2_identity.is_diagonal();
    assert !mat2_identity.is_rotated();
    
    assert !a.is_identity();
    assert !a.is_symmetric();
    assert !a.is_diagonal();
    assert a.is_rotated();
    
    let c = Mat2(2f, 1f,
                 1f, 2f);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
}

#[test]
fn test_Mat3() {
    let a = Mat3 { data: [ Vec3 { data: [ 1f, 4f, 7f ] },
                           Vec3 { data: [ 2f, 5f, 8f ] },
                           Vec3 { data: [ 3f, 6f, 9f ] } ] };
    let b = Mat3 { data: [ Vec3 { data: [ 2f, 5f,  8f ] },
                           Vec3 { data: [ 3f, 6f,  9f ] },
                           Vec3 { data: [ 4f, 7f, 10f ] } ] };
    let v1 = Vec3(1f, 2f, 3f);
    let f1 = 0.5f;
    
    assert a == Mat3(1f, 4f, 7f,
                     2f, 5f, 8f,
                     3f, 6f, 9f);
    
    assert a == Mat3_v(&Vec3(1f, 4f, 7f),
                       &Vec3(2f, 5f, 8f),
                       &Vec3(3f, 6f, 9f));
    
    assert a[0] == Vec3(1f, 4f, 7f);
    assert a[1] == Vec3(2f, 5f, 8f);
    assert a[2] == Vec3(3f, 6f, 9f);
    
    assert a.row(0) == Vec3(1f, 2f, 3f);
    assert a.row(1) == Vec3(4f, 5f, 6f);
    assert a.row(2) == Vec3(7f, 8f, 9f);
    
    assert a.col(0) == Vec3(1f, 4f, 7f);
    assert a.col(1) == Vec3(2f, 5f, 8f);
    assert a.col(2) == Vec3(3f, 6f, 9f);
    
    assert a.neg() == Mat3(-1f, -4f, -7f,
                           -2f, -5f, -8f,
                           -3f, -6f, -9f);
    assert -a == a.neg();
    
    assert a.mul_f(f1) == Mat3(0.5f, 2.0f, 3.5f,
                               1.0f, 2.5f, 4.0f,
                               1.5f, 3.0f, 4.5f);
    assert a.mul_v(&v1) == Vec3(14f, 32f, 50f);
    
    assert a.add_m(&b) == Mat3(3f,  9f, 15f,
                               5f, 11f, 17f,
                               7f, 13f, 19f);
    assert a.sub_m(&b) == Mat3(-1f, -1f, -1f,
                               -1f, -1f, -1f,
                               -1f, -1f, -1f);
    assert a.mul_m(&b) == Mat3(36f,  81f, 126f,
                               42f,  96f, 150f,
                               48f, 111f, 174f);
    
    assert a.transpose() == Mat3(1f, 2f, 3f,
                                 4f, 5f, 6f,
                                 7f, 8f, 9f);
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert mat3_identity.is_identity();
    assert mat3_identity.is_symmetric();
    assert mat3_identity.is_diagonal();
    assert !mat3_identity.is_rotated();
    
    assert !a.is_identity();
    assert !a.is_symmetric();
    assert !a.is_diagonal();
    assert a.is_rotated();
    
    let c = Mat3(3f, 2f, 1f,
                 2f, 3f, 2f,
                 1f, 2f, 3f);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
    
    assert a.to_Mat4() == Mat4(1f, 4f, 7f, 0f,
                               2f, 5f, 8f, 0f,
                               3f, 6f, 9f, 0f,
                               0f, 0f, 0f, 1f);
    
    // to_Quaternion
}

#[test]
fn test_Mat4() {
    let a = Mat4 { data: [ Vec4 { data: [ 1f, 5f,  9f, 13f ] },
                           Vec4 { data: [ 2f, 6f, 10f, 14f ] },
                           Vec4 { data: [ 3f, 7f, 11f, 15f ] },
                           Vec4 { data: [ 4f, 8f, 12f, 16f ] } ] };
    let b = Mat4 { data: [ Vec4 { data: [ 2f, 6f, 10f, 14f ] },
                           Vec4 { data: [ 3f, 7f, 11f, 15f ] },
                           Vec4 { data: [ 4f, 8f, 12f, 16f ] },
                           Vec4 { data: [ 5f, 9f, 13f, 17f ] } ] };
    let v1 = Vec4(1f, 2f, 3f, 4f);
    let f1 = 0.5f;
    
    assert a == Mat4(1f, 5f,  9f, 13f,
                     2f, 6f, 10f, 14f,
                     3f, 7f, 11f, 15f,
                     4f, 8f, 12f, 16f);
    
    assert a == Mat4_v(&Vec4(1f, 5f,  9f, 13f),
                       &Vec4(2f, 6f, 10f, 14f),
                       &Vec4(3f, 7f, 11f, 15f),
                       &Vec4(4f, 8f, 12f, 16f));
    
    assert a[0] == Vec4(1f, 5f,  9f, 13f);
    assert a[1] == Vec4(2f, 6f, 10f, 14f);
    assert a[2] == Vec4(3f, 7f, 11f, 15f);
    assert a[3] == Vec4(4f, 8f, 12f, 16f);
    
    assert a.row(0) == Vec4( 1f,  2f,  3f,  4f);
    assert a.row(1) == Vec4( 5f,  6f,  7f,  8f);
    assert a.row(2) == Vec4( 9f, 10f, 11f, 12f);
    assert a.row(3) == Vec4(13f, 14f, 15f, 16f);
    
    assert a.col(0) == Vec4(1f, 5f,  9f, 13f);
    assert a.col(1) == Vec4(2f, 6f, 10f, 14f);
    assert a.col(2) == Vec4(3f, 7f, 11f, 15f);
    assert a.col(3) == Vec4(4f, 8f, 12f, 16f);
    
    assert a.neg() == Mat4(-1f, -5f,  -9f, -13f,
                           -2f, -6f, -10f, -14f,
                           -3f, -7f, -11f, -15f,
                           -4f, -8f, -12f, -16f);
    assert -a == a.neg();
    
    assert a.mul_f(f1) == Mat4(0.5f, 2.5f, 4.5f, 6.5f,
                               1.0f, 3.0f, 5.0f, 7.0f,
                               1.5f, 3.5f, 5.5f, 7.5f,
                               2.0f, 4.0f, 6.0f, 8.0f);
    assert a.mul_v(&v1) == Vec4(30.0, 70.0, 110.0, 150.0);
    
    assert a.add_m(&b) == Mat4(3f, 11f, 19f, 27f,
                               5f, 13f, 21f, 29f,
                               7f, 15f, 23f, 31f,
                               9f, 17f, 25f, 33f);
    assert a.sub_m(&b) == Mat4(-1f, -1f, -1f, -1f,
                               -1f, -1f, -1f, -1f,
                               -1f, -1f, -1f, -1f,
                               -1f, -1f, -1f, -1f);
    assert a.mul_m(&b) == Mat4(100f, 228f, 356f, 484f,
                               110f, 254f, 398f, 542f,
                               120f, 280f, 440f, 600f,
                               130f, 306f, 482f, 658f);
    
    assert a.transpose() == Mat4( 1f,  2f,  3f,  4f,
                                  5f,  6f,  7f,  8f,
                                  9f, 10f, 11f, 12f,
                                 13f, 14f, 15f, 16f);
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert mat4_identity.is_identity();
    assert mat4_identity.is_symmetric();
    assert mat4_identity.is_diagonal();
    assert !mat4_identity.is_rotated();
    
    assert !a.is_identity();
    assert !a.is_symmetric();
    assert !a.is_diagonal();
    assert a.is_rotated();
    
    let c = Mat4(4f, 3f, 2f, 1f,
                 3f, 4f, 3f, 2f,
                 2f, 3f, 4f, 3f,
                 1f, 2f, 3f, 4f);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
}