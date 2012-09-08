import omath::mat::*;
import omath::vec::*;

// TODO

#[test]
fn test_mat2() {
    let a = mat2 { data: [ vec2 { data: [ 1f, 3f ] },
                           vec2 { data: [ 2f, 4f ] } ] };
    let b = mat2 { data: [ vec2 { data: [ 2f, 4f ] },
                           vec2 { data: [ 3f, 5f ] } ] };
    let v1 = vec2(1f, 2f);
    let f1 = 0.5f;
    
    assert a == mat2(1f, 3f,
                     2f, 4f);
    
    assert a == mat2_v(vec2(1f, 3f),
                       vec2(2f, 4f));
    
    assert a[0] == vec2(1f, 3f);
    assert a[1] == vec2(2f, 4f);
    
    assert a.row(0) == vec2(1f, 2f);
    assert a.row(1) == vec2(3f, 4f);
    
    assert a.col(0) == vec2(1f, 3f);
    assert a.col(1) == vec2(2f, 4f);
    
    assert a.neg() == mat2(-1f, -3f,
                           -2f, -4f);
    assert -a == a.neg();
    
    assert a.mul_f(f1) == mat2(0.5f, 1.5f,
                               1.0f, 2.0f);
    assert a.mul_v(v1) == vec2(5f, 11f);
    
    assert a.add_m(b) == mat2(3f, 7f,
                              5f, 9f);
    assert a.sub_m(b) == mat2(-1f, -1f,
                              -1f, -1f);
    assert a.mul_m(b) == mat2(10.0, 22.0,
                              13.0, 29.0);
    
    assert a.transpose() == mat2(1f, 2f,
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
    
    let c = mat2(2f, 1f,
                 1f, 2f);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
}

#[test]
fn test_mat3() {
    let a = mat3 { data: [ vec3 { data: [ 1f, 4f, 7f ] },
                           vec3 { data: [ 2f, 5f, 8f ] },
                           vec3 { data: [ 3f, 6f, 9f ] } ] };
    let b = mat3 { data: [ vec3 { data: [ 2f, 5f,  8f ] },
                           vec3 { data: [ 3f, 6f,  9f ] },
                           vec3 { data: [ 4f, 7f, 10f ] } ] };
    let v1 = vec3(1f, 2f, 3f);
    let f1 = 0.5f;
    
    assert a == mat3(1f, 4f, 7f,
                     2f, 5f, 8f,
                     3f, 6f, 9f);
    
    assert a == mat3_v(vec3(1f, 4f, 7f),
                       vec3(2f, 5f, 8f),
                       vec3(3f, 6f, 9f));
    
    assert a[0] == vec3(1f, 4f, 7f);
    assert a[1] == vec3(2f, 5f, 8f);
    assert a[2] == vec3(3f, 6f, 9f);
    
    assert a.row(0) == vec3(1f, 2f, 3f);
    assert a.row(1) == vec3(4f, 5f, 6f);
    assert a.row(2) == vec3(7f, 8f, 9f);
    
    assert a.col(0) == vec3(1f, 4f, 7f);
    assert a.col(1) == vec3(2f, 5f, 8f);
    assert a.col(2) == vec3(3f, 6f, 9f);
    
    assert a.neg() == mat3(-1f, -4f, -7f,
                           -2f, -5f, -8f,
                           -3f, -6f, -9f);
    assert -a == a.neg();
    
    assert a.mul_f(f1) == mat3(0.5f, 2.0f, 3.5f,
                               1.0f, 2.5f, 4.0f,
                               1.5f, 3.0f, 4.5f);
    assert a.mul_v(v1) == vec3(14f, 32f, 50f);
    
    assert a.add_m(b) == mat3(3f,  9f, 15f,
                              5f, 11f, 17f,
                              7f, 13f, 19f);
    assert a.sub_m(b) == mat3(-1f, -1f, -1f,
                              -1f, -1f, -1f,
                              -1f, -1f, -1f);
    assert a.mul_m(b) == mat3(36f,  81f, 126f,
                              42f,  96f, 150f,
                              48f, 111f, 174f);
    
    assert a.transpose() == mat3(1f, 2f, 3f,
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
    
    let c = mat3(3f, 2f, 1f,
                 2f, 3f, 2f,
                 1f, 2f, 3f);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
    
    assert a.to_mat4() == mat4(1f, 4f, 7f, 0f,
                               2f, 5f, 8f, 0f,
                               3f, 6f, 9f, 0f,
                               0f, 0f, 0f, 1f);
    
    // to_quaternion
}

#[test]
fn test_mat4() {
    let a = mat4 { data: [ vec4 { data: [ 1f, 5f,  9f, 13f ] },
                           vec4 { data: [ 2f, 6f, 10f, 14f ] },
                           vec4 { data: [ 3f, 7f, 11f, 15f ] },
                           vec4 { data: [ 4f, 8f, 12f, 16f ] } ] };
    let b = mat4 { data: [ vec4 { data: [ 2f, 6f, 10f, 14f ] },
                           vec4 { data: [ 3f, 7f, 11f, 15f ] },
                           vec4 { data: [ 4f, 8f, 12f, 16f ] },
                           vec4 { data: [ 5f, 9f, 13f, 17f ] } ] };
    let v1 = vec4(1f, 2f, 3f, 4f);
    let f1 = 0.5f;
    
    assert a == mat4(1f, 5f,  9f, 13f,
                     2f, 6f, 10f, 14f,
                     3f, 7f, 11f, 15f,
                     4f, 8f, 12f, 16f);
    
    assert a == mat4_v(vec4(1f, 5f,  9f, 13f),
                       vec4(2f, 6f, 10f, 14f),
                       vec4(3f, 7f, 11f, 15f),
                       vec4(4f, 8f, 12f, 16f));
    
    assert a[0] == vec4(1f, 5f,  9f, 13f);
    assert a[1] == vec4(2f, 6f, 10f, 14f);
    assert a[2] == vec4(3f, 7f, 11f, 15f);
    assert a[3] == vec4(4f, 8f, 12f, 16f);
    
    assert a.row(0) == vec4( 1f,  2f,  3f,  4f);
    assert a.row(1) == vec4( 5f,  6f,  7f,  8f);
    assert a.row(2) == vec4( 9f, 10f, 11f, 12f);
    assert a.row(3) == vec4(13f, 14f, 15f, 16f);
    
    assert a.col(0) == vec4(1f, 5f,  9f, 13f);
    assert a.col(1) == vec4(2f, 6f, 10f, 14f);
    assert a.col(2) == vec4(3f, 7f, 11f, 15f);
    assert a.col(3) == vec4(4f, 8f, 12f, 16f);
    
    assert a.neg() == mat4(-1f, -5f,  -9f, -13f,
                           -2f, -6f, -10f, -14f,
                           -3f, -7f, -11f, -15f,
                           -4f, -8f, -12f, -16f);
    assert -a == a.neg();
    
    assert a.mul_f(f1) == mat4(0.5f, 2.5f, 4.5f, 6.5f,
                               1.0f, 3.0f, 5.0f, 7.0f,
                               1.5f, 3.5f, 5.5f, 7.5f,
                               2.0f, 4.0f, 6.0f, 8.0f);
    assert a.mul_v(v1) == vec4(30.0, 70.0, 110.0, 150.0);
    
    assert a.add_m(b) == mat4(3f, 11f, 19f, 27f,
                              5f, 13f, 21f, 29f,
                              7f, 15f, 23f, 31f,
                              9f, 17f, 25f, 33f);
    assert a.sub_m(b) == mat4(-1f, -1f, -1f, -1f,
                              -1f, -1f, -1f, -1f,
                              -1f, -1f, -1f, -1f,
                              -1f, -1f, -1f, -1f);
    assert a.mul_m(b) == mat4(100f, 228f, 356f, 484f,
                              110f, 254f, 398f, 542f,
                              120f, 280f, 440f, 600f,
                              130f, 306f, 482f, 658f);
    
    assert a.transpose() == mat4( 1f,  2f,  3f,  4f,
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
    
    let c = mat4(4f, 3f, 2f, 1f,
                 3f, 4f, 3f, 2f,
                 2f, 3f, 4f, 3f,
                 1f, 2f, 3f, 4f);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
}