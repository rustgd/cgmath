use mat::*;
use vec::*;

// TODO

#[test]
fn test_mat2() {
    let a = Mat2 { x: Vec2 { x: 1.0, y: 3.0 },
                   y: Vec2 { x: 2.0, y: 4.0 } };
    let b = Mat2 { x: Vec2 { x: 2.0, y: 4.0 },
                   y: Vec2 { x: 3.0, y: 5.0 } };
    
    let v1 = vec2::new(1.0, 2.0);
    let f1 = 0.5;
    
    assert a == mat2::new(1.0, 3.0,
                          2.0, 4.0);
    
    assert a == mat2::from_cols(vec2::new(1.0, 3.0),
                                vec2::new(2.0, 4.0));
    
    assert mat2::from_value(4.0) == mat2::new(4.0, 0.0,
                                               0.0, 4.0);
    
    assert a[0] == vec2::new(1.0, 3.0);
    assert a[1] == vec2::new(2.0, 4.0);
    
    assert a.row(0) == vec2::new(1.0, 2.0);
    assert a.row(1) == vec2::new(3.0, 4.0);
    
    assert a.col(0) == vec2::new(1.0, 3.0);
    assert a.col(1) == vec2::new(2.0, 4.0);
    
    assert mat2::identity() == mat2::new(1.0, 0.0,
                                         0.0, 1.0);
    
    assert mat2::zero() == mat2::new(0.0, 0.0,
                                     0.0, 0.0);

    assert a.determinant() == -2.0;
    assert a.trace() == 5.0;
    
    assert a.neg() == mat2::new(-1.0, -3.0,
                                -2.0, -4.0);
    assert -a == a.neg();
    
    assert a.mul_t(f1) == mat2::new(0.5, 1.5,
                                    1.0, 2.0);
    assert a.mul_v(&v1) == vec2::new(5.0, 11.0);
    
    assert a.add_m(&b) == mat2::new(3.0, 7.0,
                                    5.0, 9.0);
    assert a.sub_m(&b) == mat2::new(-1.0, -1.0,
                                    -1.0, -1.0);
    assert a.mul_m(&b) == mat2::new(10.0, 22.0,
                                    13.0, 29.0);
    assert a.dot(&b) == 40.0; 
    
    assert a.transpose() == mat2::new(1.0, 2.0,
                                      3.0, 4.0);

    assert option::unwrap(a.inverse()) == mat2::new(-2.0,  1.5,
                                                     1.0, -0.5);

    assert mat2::new(0.0, 2.0,
                     0.0, 5.0).inverse().is_none();
    
    let ident = mat2::identity();
    
    assert ident.is_identity();
    assert ident.is_symmetric();
    assert ident.is_diagonal();
    assert !ident.is_rotated();
    assert ident.is_invertible();
    
    assert !a.is_identity();
    assert !a.is_symmetric();
    assert !a.is_diagonal();
    assert a.is_rotated();
    assert a.is_invertible();
    
    let c = mat2::new(2.0, 1.0,
                      1.0, 2.0);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
    assert c.is_invertible();
    
    assert mat2::from_value(6.0).is_diagonal();
    
    assert a.to_mat3() == mat3::new(1.0, 3.0, 0.0,
                                    2.0, 4.0, 0.0,
                                    0.0, 0.0, 1.0);
    
    assert a.to_mat4() == mat4::new(1.0, 3.0, 0.0, 0.0,
                                    2.0, 4.0, 0.0, 0.0,
                                    0.0, 0.0, 1.0, 0.0,
                                    0.0, 0.0, 0.0, 1.0);
}

fn test_mat2_mut() {
    let a = Mat2 { x: Vec2 { x: 1.0, y: 3.0 },
                   y: Vec2 { x: 2.0, y: 4.0 } };
    let b = Mat2 { x: Vec2 { x: 2.0, y: 4.0 },
                   y: Vec2 { x: 3.0, y: 5.0 } };
    
    let f1 = 0.5;
    
    let mut mut_a: mat2 = a;
    
    mut_a.swap_cols(0, 1);
    assert mut_a.col(0) == a.col(1);
    assert mut_a.col(1) == a.col(0);
    mut_a = a;
    
    mut_a.swap_rows(0, 1);
    assert mut_a.row(0) == a.row(1);
    assert mut_a.row(1) == a.row(0);
    mut_a = a;
    
    mut_a.set(&b);
    assert mut_a == b;
    mut_a = a;
    
    mut_a.to_identity();
    assert mut_a.is_identity();
    mut_a = a;
    
    mut_a.to_zero();
    assert mut_a == mat2::zero();
    mut_a = a;
    
    mut_a.mul_self_t(f1);
    assert mut_a == a.mul_t(f1);
    mut_a = a;
    
    mut_a.add_self_m(&b);
    assert mut_a == a.add_m(&b);
    mut_a = a;
    
    mut_a.sub_self_m(&b);
    assert mut_a == a.sub_m(&b);
    mut_a = a;
    
    mut_a.invert_self();
    assert mut_a == option::unwrap(a.inverse());
    mut_a = a;
    
    mut_a.transpose_self();
    assert mut_a == a.transpose();
    // mut_a = a;
}

#[test]
fn test_mat2_fuzzy_eq() {
    assert !mat2::new(0.000001, 0.000001,
                      0.000001, 0.000001).fuzzy_eq(&mat2::zero());
    assert mat2::new(0.0000001, 0.0000001,
                     0.0000001, 0.0000001).fuzzy_eq(&mat2::zero());
}

#[test]
fn test_mat3() {
    let a = Mat3 { x: Vec3 { x: 1.0, y: 4.0, z:  7.0 },
                   y: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                   z: Vec3 { x: 3.0, y: 6.0, z:  9.0 } };
    let b = Mat3 { x: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                   y: Vec3 { x: 3.0, y: 6.0, z:  9.0 },
                   z: Vec3 { x: 4.0, y: 7.0, z: 10.0 } };
    
    let v1 = vec3::new(1.0, 2.0, 3.0);
    let f1 = 0.5;
    
    assert a == mat3::new(1.0, 4.0, 7.0,
                          2.0, 5.0, 8.0,
                          3.0, 6.0, 9.0);
    
    assert a == mat3::from_cols(vec3::new(1.0, 4.0, 7.0),
                                vec3::new(2.0, 5.0, 8.0),
                                vec3::new(3.0, 6.0, 9.0));
    
    assert a[0] == vec3::new(1.0, 4.0, 7.0);
    assert a[1] == vec3::new(2.0, 5.0, 8.0);
    assert a[2] == vec3::new(3.0, 6.0, 9.0);
    
    assert a.row(0) == vec3::new(1.0, 2.0, 3.0);
    assert a.row(1) == vec3::new(4.0, 5.0, 6.0);
    assert a.row(2) == vec3::new(7.0, 8.0, 9.0);
    
    assert a.col(0) == vec3::new(1.0, 4.0, 7.0);
    assert a.col(1) == vec3::new(2.0, 5.0, 8.0);
    assert a.col(2) == vec3::new(3.0, 6.0, 9.0);
    
    assert mat3::identity() == mat3::new(1.0, 0.0, 0.0,
                                         0.0, 1.0, 0.0,
                                         0.0, 0.0, 1.0);
    
    assert mat3::zero() == mat3::new(0.0, 0.0, 0.0,
                                     0.0, 0.0, 0.0,
                                     0.0, 0.0, 0.0);

    assert a.determinant() == 0.0;
    assert a.trace() == 15.0;
    
    assert a.neg() == mat3::new(-1.0, -4.0, -7.0,
                                -2.0, -5.0, -8.0,
                                -3.0, -6.0, -9.0);
    assert -a == a.neg();
    
    assert a.mul_t(f1) == mat3::new(0.5, 2.0, 3.5,
                                    1.0, 2.5, 4.0,
                                    1.5, 3.0, 4.5);
    assert a.mul_v(&v1) == vec3::new(14.0, 32.0, 50.0);
    
    assert a.add_m(&b) == mat3::new(3.0,  9.0, 15.0,
                                    5.0, 11.0, 17.0,
                                    7.0, 13.0, 19.0);
    assert a.sub_m(&b) == mat3::new(-1.0, -1.0, -1.0,
                                    -1.0, -1.0, -1.0,
                                    -1.0, -1.0, -1.0);
    assert a.mul_m(&b) == mat3::new(36.0,  81.0, 126.0,
                                    42.0,  96.0, 150.0,
                                    48.0, 111.0, 174.0);
    assert a.dot(&b) == 330.0;
    
    assert a.transpose() == mat3::new(1.0, 2.0, 3.0,
                                      4.0, 5.0, 6.0,
                                      7.0, 8.0, 9.0);

    assert a.inverse().is_none();
    
    assert option::unwrap(mat3::new(2.0, 4.0, 6.0,
                                    0.0, 2.0, 4.0,
                                    0.0, 0.0, 1.0).inverse())
        == mat3::new(0.5,  -1.0,  1.0,
                     0.0,   0.5, -2.0,
                     0.0,   0.0,  1.0);
    
    let ident: Mat3<float> = Matrix::identity();

    assert option::unwrap(ident.inverse()) == ident;
    
    assert ident.is_identity();
    assert ident.is_symmetric();
    assert ident.is_diagonal();
    assert !ident.is_rotated();
    assert ident.is_invertible();
    
    assert !a.is_identity();
    assert !a.is_symmetric();
    assert !a.is_diagonal();
    assert a.is_rotated();
    assert !a.is_invertible();
    
    let c = mat3::new(3.0, 2.0, 1.0,
                      2.0, 3.0, 2.0,
                      1.0, 2.0, 3.0);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
    assert c.is_invertible();
    
    assert mat3::from_value(6.0).is_diagonal();
    
    assert a.to_mat4() == mat4::new(1.0, 4.0, 7.0, 0.0,
                                    2.0, 5.0, 8.0, 0.0,
                                    3.0, 6.0, 9.0, 0.0,
                                    0.0, 0.0, 0.0, 1.0);
    
    // to_Quaternion
}

fn test_mat3_mut() {
    let a = Mat3 { x: Vec3 { x: 1.0, y: 4.0, z:  7.0 },
                   y: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                   z: Vec3 { x: 3.0, y: 6.0, z:  9.0 } };
    let b = Mat3 { x: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                   y: Vec3 { x: 3.0, y: 6.0, z:  9.0 },
                   z: Vec3 { x: 4.0, y: 7.0, z: 10.0 } };
    let c = Mat3 { x: Vec3 { x: 2.0, y: 4.0, z:  6.0 },
                   y: Vec3 { x: 0.0, y: 2.0, z:  4.0 },
                   z: Vec3 { x: 0.0, y: 0.0, z:  1.0 } };
    
    let f1 = 0.5;
    
    let mut mut_a: mat3 = a;
    let mut mut_c: mat3 = c;
    
    mut_a.swap_cols(0, 2);
    assert mut_a.col(0) == a.col(2);
    assert mut_a.col(2) == a.col(0);
    mut_a = a;
    
    mut_a.swap_cols(1, 2);
    assert mut_a.col(1) == a.col(2);
    assert mut_a.col(2) == a.col(1);
    mut_a = a;
    
    mut_a.swap_rows(0, 2);
    assert mut_a.row(0) == a.row(2);
    assert mut_a.row(2) == a.row(0);
    mut_a = a;
    
    mut_a.swap_rows(1, 2);
    assert mut_a.row(1) == a.row(2);
    assert mut_a.row(2) == a.row(1);
    mut_a = a;
    
    mut_a.set(&b);
    assert mut_a == b;
    mut_a = a;
    
    mut_a.to_identity();
    assert mut_a.is_identity();
    mut_a = a;
    
    mut_a.to_zero();
    assert mut_a == mat3::zero();
    mut_a = a;
    
    mut_a.mul_self_t(f1);
    assert mut_a == a.mul_t(f1);
    mut_a = a;
    
    mut_a.add_self_m(&b);
    assert mut_a == a.add_m(&b);
    mut_a = a;
    
    mut_a.sub_self_m(&b);
    assert mut_a == a.sub_m(&b);
    mut_a = a;
    
    mut_c.invert_self();
    assert mut_c == option::unwrap(c.inverse());
    // mut_c = c;
    
    mut_a.transpose_self();
    assert mut_a == a.transpose();
    // mut_a = a;
}

#[test]
fn test_mat3_fuzzy_eq() {
    assert !mat3::new(0.000001, 0.000001, 0.000001,
                      0.000001, 0.000001, 0.000001,
                      0.000001, 0.000001, 0.000001).fuzzy_eq(&mat3::zero());
    assert mat3::new(0.0000001, 0.0000001, 0.0000001,
                     0.0000001, 0.0000001, 0.0000001,
                     0.0000001, 0.0000001, 0.0000001).fuzzy_eq(&mat3::zero());
}

#[test]
fn test_mat4() {
    let a: mat4 = Mat4 { x: Vec4 { x: 1.0, y: 5.0, z:  9.0, w: 13.0 },
                         y: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                         z: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                         w: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 } };
    let b: mat4 = Mat4 { x: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                         y: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                         z: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 },
                         w: Vec4 { x: 5.0, y: 9.0, z: 13.0, w: 17.0 } };
    let c: mat4 = Mat4 { x: Vec4 { x: 3.0, y: 2.0, z:  1.0, w:  1.0 },
                         y: Vec4 { x: 2.0, y: 3.0, z:  2.0, w:  2.0 },
                         z: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  3.0 },
                         w: Vec4 { x: 0.0, y: 1.0, z:  1.0, w:  0.0 } };
    
    let v1 = vec4::new(1.0, 2.0, 3.0, 4.0);
    let f1 = 0.5;
    
    assert a == mat4::new(1.0, 5.0,  9.0, 13.0,
                          2.0, 6.0, 10.0, 14.0,
                          3.0, 7.0, 11.0, 15.0,
                          4.0, 8.0, 12.0, 16.0);
    
    assert a == mat4::from_cols(vec4::new(1.0, 5.0,  9.0, 13.0),
                                vec4::new(2.0, 6.0, 10.0, 14.0),
                                vec4::new(3.0, 7.0, 11.0, 15.0),
                                vec4::new(4.0, 8.0, 12.0, 16.0));
    
    assert mat4::from_value(4.0) == mat4::new(4.0, 0.0, 0.0, 0.0,
                                              0.0, 4.0, 0.0, 0.0,
                                              0.0, 0.0, 4.0, 0.0,
                                              0.0, 0.0, 0.0, 4.0);
    
    assert a[0] == vec4::new(1.0, 5.0,  9.0, 13.0);
    assert a[1] == vec4::new(2.0, 6.0, 10.0, 14.0);
    assert a[2] == vec4::new(3.0, 7.0, 11.0, 15.0);
    assert a[3] == vec4::new(4.0, 8.0, 12.0, 16.0);
    
    assert a.row(0) == vec4::new( 1.0,  2.0,  3.0,  4.0);
    assert a.row(1) == vec4::new( 5.0,  6.0,  7.0,  8.0);
    assert a.row(2) == vec4::new( 9.0, 10.0, 11.0, 12.0);
    assert a.row(3) == vec4::new(13.0, 14.0, 15.0, 16.0);
    
    assert a.col(0) == vec4::new(1.0, 5.0,  9.0, 13.0);
    assert a.col(1) == vec4::new(2.0, 6.0, 10.0, 14.0);
    assert a.col(2) == vec4::new(3.0, 7.0, 11.0, 15.0);
    assert a.col(3) == vec4::new(4.0, 8.0, 12.0, 16.0);
    
    assert mat4::identity() == mat4::new(1.0, 0.0, 0.0, 0.0,
                                         0.0, 1.0, 0.0, 0.0,
                                         0.0, 0.0, 1.0, 0.0,
                                         0.0, 0.0, 0.0, 1.0);
    
    assert mat4::zero() == mat4::new(0.0, 0.0, 0.0, 0.0,
                                     0.0, 0.0, 0.0, 0.0,
                                     0.0, 0.0, 0.0, 0.0,
                                     0.0, 0.0, 0.0, 0.0);

    assert a.determinant() == 0.0;
    assert a.trace() == 34.0;
    
    assert a.neg() == mat4::new(-1.0, -5.0,  -9.0, -13.0,
                                -2.0, -6.0, -10.0, -14.0,
                                -3.0, -7.0, -11.0, -15.0,
                                -4.0, -8.0, -12.0, -16.0);
    assert -a == a.neg();
    
    assert a.mul_t(f1) == mat4::new(0.5, 2.5, 4.5, 6.5,
                                    1.0, 3.0, 5.0, 7.0,
                                    1.5, 3.5, 5.5, 7.5,
                                    2.0, 4.0, 6.0, 8.0);
    assert a.mul_v(&v1) == vec4::new(30.0, 70.0, 110.0, 150.0);
    
    assert a.add_m(&b) == mat4::new(3.0, 11.0, 19.0, 27.0,
                                    5.0, 13.0, 21.0, 29.0,
                                    7.0, 15.0, 23.0, 31.0,
                                    9.0, 17.0, 25.0, 33.0);
    assert a.sub_m(&b) == mat4::new(-1.0, -1.0, -1.0, -1.0,
                                    -1.0, -1.0, -1.0, -1.0,
                                    -1.0, -1.0, -1.0, -1.0,
                                    -1.0, -1.0, -1.0, -1.0);
    assert a.mul_m(&b) == mat4::new(100.0, 228.0, 356.0, 484.0,
                                    110.0, 254.0, 398.0, 542.0,
                                    120.0, 280.0, 440.0, 600.0,
                                    130.0, 306.0, 482.0, 658.0);
    assert a.dot(&b) == 1632.0;
    
    assert a.transpose() == mat4::new( 1.0,  2.0,  3.0,  4.0,
                                       5.0,  6.0,  7.0,  8.0,
                                       9.0, 10.0, 11.0, 12.0,
                                      13.0, 14.0, 15.0, 16.0);
    
    assert c.inverse().unwrap()
        .fuzzy_eq(&mat4::new( 5.0, -4.0,  1.0,  0.0,
                             -4.0,  8.0, -4.0,  0.0,
                              4.0, -8.0,  4.0,  8.0,
                             -3.0,  4.0,  1.0, -8.0).mul_t(0.125));
    
    let ident = mat4::identity();

    assert ident.inverse().unwrap() == ident;
    
    assert ident.is_identity();
    assert ident.is_symmetric();
    assert ident.is_diagonal();
    assert !ident.is_rotated();
    assert ident.is_invertible();
    
    assert !a.is_identity();
    assert !a.is_symmetric();
    assert !a.is_diagonal();
    assert a.is_rotated();
    assert !a.is_invertible();
    
    let c = mat4::new(4.0, 3.0, 2.0, 1.0,
                      3.0, 4.0, 3.0, 2.0,
                      2.0, 3.0, 4.0, 3.0,
                      1.0, 2.0, 3.0, 4.0);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
    assert c.is_invertible();
    
    assert mat4::from_value(6.0).is_diagonal();
}

fn test_mat4_mut() {
    let a = Mat4 { x: Vec4 { x: 1.0, y: 5.0, z:  9.0, w: 13.0 },
                   y: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                   z: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                   w: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 } };
    let b = Mat4 { x: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                   y: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                   z: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 },
                   w: Vec4 { x: 5.0, y: 9.0, z: 13.0, w: 17.0 } };
    let c = Mat4 { x: Vec4 { x: 3.0, y: 2.0, z:  1.0, w:  1.0 },
                   y: Vec4 { x: 2.0, y: 3.0, z:  2.0, w:  2.0 },
                   z: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  3.0 },
                   w: Vec4 { x: 0.0, y: 1.0, z:  1.0, w:  0.0 } };
    
    let f1 = 0.5;
    
    let mut mut_a: mat4 = a;
    let mut mut_c: mat4 = c;
    
    mut_a.swap_cols(0, 3);
    assert mut_a.col(0) == a.col(3);
    assert mut_a.col(3) == a.col(0);
    mut_a = a;
    
    mut_a.swap_cols(1, 2);
    assert mut_a.col(1) == a.col(2);
    assert mut_a.col(2) == a.col(1);
    mut_a = a;
    
    mut_a.swap_rows(0, 3);
    assert mut_a.row(0) == a.row(3);
    assert mut_a.row(3) == a.row(0);
    mut_a = a;
    
    mut_a.swap_rows(1, 2);
    assert mut_a.row(1) == a.row(2);
    assert mut_a.row(2) == a.row(1);
    mut_a = a;
    
    mut_a.set(&b);
    assert mut_a == b;
    mut_a = a;
    
    mut_a.to_identity();
    assert mut_a.is_identity();
    mut_a = a;
    
    mut_a.to_zero();
    assert mut_a == mat4::zero();
    mut_a = a;
    
    mut_a.mul_self_t(f1);
    assert mut_a == a.mul_t(f1);
    mut_a = a;
    
    mut_a.add_self_m(&b);
    assert mut_a == a.add_m(&b);
    mut_a = a;
    
    mut_a.sub_self_m(&b);
    assert mut_a == a.sub_m(&b);
    mut_a = a;
    
    mut_c.invert_self();
    assert mut_c == option::unwrap(c.inverse());
    // mut_c = c;
    
    mut_a.transpose_self();
    assert mut_a == a.transpose();
    // mut_a = a;
}

#[test]
fn test_mat4_fuzzy_eq() {
    assert !mat4::new(0.000001, 0.000001, 0.000001, 0.000001,
                      0.000001, 0.000001, 0.000001, 0.000001,
                      0.000001, 0.000001, 0.000001, 0.000001,
                      0.000001, 0.000001, 0.000001, 0.000001).fuzzy_eq(&mat4::zero());
    assert mat4::new(0.0000001, 0.0000001, 0.0000001, 0.0000001,
                     0.0000001, 0.0000001, 0.0000001, 0.0000001,
                     0.0000001, 0.0000001, 0.0000001, 0.0000001,
                     0.0000001, 0.0000001, 0.0000001, 0.0000001).fuzzy_eq(&mat4::zero());
}