import std::cmp::FuzzyEq;
import om3d::vec::*;

// TODO

#[test]
fn test_vec2() {
    let a = vec2 { data: [ 1f, 2f ] };
    let b = vec2 { data: [ 3f, 4f ] };
    let f1 = 1.5f;
    let f2 = 0.5f;
    
    assert a == vec2(1f, 2f);
    
    assert vec2_zero()     == vec2(0f, 0f);
    assert vec2_unit_x()   == vec2(1f, 0f);
    assert vec2_unit_y()   == vec2(0f, 1f);
    assert vec2_identity() == vec2(1f, 1f);
    
    assert a[0] == 1f;
    assert a[1] == 2f;
    assert a.x() == 1f;
    assert a.y() == 2f;
    
    assert -a      == vec2(-1f, -2f);
    assert a.neg() == vec2(-1f, -2f);
    
    assert a.add_f(f1) == vec2( 2.5f, 3.5f);
    assert a.sub_f(f1) == vec2(-0.5f, 0.5f);
    assert a.mul_f(f1) == vec2( 1.5f, 3.0f);
    assert a.div_f(f2) == vec2( 2.0f, 4.0f);
    
    assert a.add_v(b) == vec2( 4f,  6f);
    assert a.sub_v(b) == vec2(-2f, -2f);
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert a.magnitude2().fuzzy_eq(&5f);
    assert a.magnitude().fuzzy_eq(&2.236068f);
    
    let c = vec2(-2.0f, -1.0f);
    let d = vec2( 1.0f,  0.0f);
    let f3 = 0.75f;
    
    assert c.lerp(d, f3) == vec2(0.250f, -0.250f);
    assert c.abs() == vec2(2.0f, 1.0f);
    
    // TODO min, max
}

#[test]
fn test_vec3() {
    let a = vec3 { data: [ 1f, 2f, 3f ] };
    let b = vec3 { data: [ 4f, 5f, 6f ] };
    let f1 = 1.5f;
    let f2 = 0.5f;
    
    assert a == vec3(1f, 2f, 3f);
    
    assert vec3_zero()     == vec3(0f, 0f, 0f);
    assert vec3_unit_x()   == vec3(1f, 0f, 0f);
    assert vec3_unit_y()   == vec3(0f, 1f, 0f);
    assert vec3_unit_z()   == vec3(0f, 0f, 1f);
    assert vec3_identity() == vec3(1f, 1f, 1f);
    
    assert a[0] == 1f;
    assert a[1] == 2f;
    assert a[2] == 3f;
    assert a.x() == 1f;
    assert a.y() == 2f;
    assert a.z() == 3f;
    
    assert a.cross(b) == vec3(-3f, 6f, -3f);
    
    assert -a      == vec3(-1f, -2f, -3f);
    assert a.neg() == vec3(-1f, -2f, -3f);
    
    assert a.add_f(f1) == vec3( 2.5f, 3.5f, 4.5f);
    assert a.sub_f(f1) == vec3(-0.5f, 0.5f, 1.5f);
    assert a.mul_f(f1) == vec3( 1.5f, 3.0f, 4.5f);
    assert a.div_f(f2) == vec3( 2.0f, 4.0f, 6.0f);
    
    assert a.add_v(b) == vec3( 5f,  7f,  9f);
    assert a.sub_v(b) == vec3(-3f, -3f, -3f);
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert a.magnitude2().fuzzy_eq(&14f);
    assert a.magnitude().fuzzy_eq(&3.74165738677f);
    
    let c = vec3(-2.0f, -1.0f, 1.0f);
    let d = vec3( 1.0f,  0.0f, 0.5f);
    let f3 = 0.75f;
    
    assert c.lerp(d, f3) == vec3(0.250f, -0.250f, 0.625f);
    assert c.abs() == vec3(2.0f, 1.0f, 1.0f);
    
    // TODO min, max
}

#[test]
fn test_vec4() {
    let a = vec4 { data: [ 1f, 2f, 3f, 4f ] };
    let b = vec4 { data: [ 5f, 6f, 7f, 8f ] };
    let f1 = 1.5f;
    let f2 = 0.5f;
    
    assert a == vec4(1f, 2f, 3f, 4f);
    
    assert vec4_zero()     == vec4(0f, 0f, 0f, 0f);
    assert vec4_unit_x()   == vec4(1f, 0f, 0f, 0f);
    assert vec4_unit_y()   == vec4(0f, 1f, 0f, 0f);
    assert vec4_unit_z()   == vec4(0f, 0f, 1f, 0f);
    assert vec4_unit_w()   == vec4(0f, 0f, 0f, 1f);
    assert vec4_identity() == vec4(1f, 1f, 1f, 1f);
    
    assert a[0] == 1f;
    assert a[1] == 2f;
    assert a[2] == 3f;
    assert a[3] == 4f;
    assert a.x() == 1f;
    assert a.y() == 2f;
    assert a.z() == 3f;
    assert a.w() == 4f;
    
    assert -a      == vec4(-1f, -2f, -3f, -4f);
    assert a.neg() == vec4(-1f, -2f, -3f, -4f);
    
    assert a.add_f(f1) == vec4( 2.5f, 3.5f, 4.5f, 5.5f);
    assert a.sub_f(f1) == vec4(-0.5f, 0.5f, 1.5f, 2.5f);
    assert a.mul_f(f1) == vec4( 1.5f, 3.0f, 4.5f, 6.0f);
    assert a.div_f(f2) == vec4( 2.0f, 4.0f, 6.0f, 8.0f);
    
    assert a.add_v(b) == vec4( 6f,  8f, 10f, 12f);
    assert a.sub_v(b) == vec4(-4f, -4f, -4f, -4f);
    
    assert a.dot(b) == 70f;
    
    // exact_eq
    // fuzzy_eq
    // eq
    
    assert a.magnitude2().fuzzy_eq(&30f);
    assert a.magnitude().fuzzy_eq(&5.477226f);
    
    let c = vec4(-2.0f, -1.0f, 1.0f, 2.0f);
    let d = vec4( 1.0f,  0.0f, 0.5f, 1.0f);
    let f3 = 0.75f;
    
    assert c.lerp(d, f3) == vec4(0.250f, -0.250f, 0.625f, 1.250f);
    assert c.abs() == vec4(2.0f, 1.0f, 1.0f, 2.0f);
    
    // TODO min, max
}