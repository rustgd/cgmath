use gltypes::*;

#[test]
fn test_vec() {
    assert vec2::zero()     == vec2::new(0f32, 0f32);
    assert vec3::zero()     == vec3::new(0f32, 0f32, 0f32);
    assert vec4::zero()     == vec4::new(0f32, 0f32, 0f32, 0f32);
    assert vec2::identity() == vec2::from_value(1f32);
    assert vec3::identity() == vec3::from_value(1f32);
    assert vec4::identity() == vec4::from_value(1f32);
    
    assert dvec2::identity() == dvec2::new(1f64, 1f64);
    assert dvec3::identity() == dvec3::new(1f64, 1f64, 1f64);
    assert dvec4::identity() == dvec4::new(1f64, 1f64, 1f64, 1f64);
    assert dvec2::zero()     == dvec2::new(0f64, 0f64);
    assert dvec3::zero()     == dvec3::new(0f64, 0f64, 0f64);
    assert dvec4::zero()     == dvec4::new(0f64, 0f64, 0f64, 0f64);
    
    assert ivec2::identity() == ivec2::new(1i32, 1i32);
    assert ivec2::identity() == ivec2::new(1i32, 1i32);
    assert ivec3::identity() == ivec3::new(1i32, 1i32, 1i32);
    assert ivec4::identity() == ivec4::new(1i32, 1i32, 1i32, 1i32);
    assert ivec2::zero()     == ivec2::new(0i32, 0i32);
    assert ivec3::zero()     == ivec3::new(0i32, 0i32, 0i32);
    assert ivec4::zero()     == ivec4::new(0i32, 0i32, 0i32, 0i32);
    assert ivec2::identity() == ivec2::new(1, 1);
    assert ivec2::identity() == ivec2::new(1, 1);
    assert ivec3::identity() == ivec3::new(1, 1, 1);
    assert ivec4::identity() == ivec4::new(1, 1, 1, 1);
    assert ivec2::zero()     == ivec2::new(0, 0);
    assert ivec3::zero()     == ivec3::new(0, 0, 0);
    assert ivec4::zero()     == ivec4::new(0, 0, 0, 0);
    
    assert uvec2::identity() == uvec2::new(1u32, 1u32);
    assert uvec2::identity() == uvec2::new(1u32, 1u32);
    assert uvec3::identity() == uvec3::new(1u32, 1u32, 1u32);
    assert uvec4::identity() == uvec4::new(1u32, 1u32, 1u32, 1u32);
    assert uvec2::zero()     == uvec2::new(0u32, 0u32);
    assert uvec3::zero()     == uvec3::new(0u32, 0u32, 0u32);
    assert uvec4::zero()     == uvec4::new(0u32, 0u32, 0u32, 0u32);
    assert uvec2::identity() == uvec2::new(1, 1);
    assert uvec2::identity() == uvec2::new(1, 1);
    assert uvec3::identity() == uvec3::new(1, 1, 1);
    assert uvec4::identity() == uvec4::new(1, 1, 1, 1);
    assert uvec2::zero()     == uvec2::new(0, 0);
    assert uvec3::zero()     == uvec3::new(0, 0, 0);
    assert uvec4::zero()     == uvec4::new(0, 0, 0, 0);
}

#[test]
fn test_mat_nxn() {
    assert mat2x2::identity() == mat2x2::new(1f32, 0f32,
                                             0f32, 1f32);
    assert mat2x2::identity() == mat2x2::from_cols(vec2::new(1f32, 0f32),
                                                   vec2::new(0f32, 1f32));
    assert mat3x3::identity() == mat3x3::new(1f32, 0f32, 0f32,
                                             0f32, 1f32, 0f32,
                                             0f32, 0f32, 1f32);
    assert mat3x3::identity() == mat3x3::from_cols(vec3::new(1f32, 0f32, 0f32),
                                                   vec3::new(0f32, 1f32, 0f32),
                                                   vec3::new(0f32, 0f32, 1f32));
    assert mat4x4::identity() == mat4x4::new(1f32, 0f32, 0f32, 0f32,
                                             0f32, 1f32, 0f32, 0f32,
                                             0f32, 0f32, 1f32, 0f32,
                                             0f32, 0f32, 0f32, 1f32);
    assert mat4x4::identity() == mat4x4::from_cols(vec4::new(1f32, 0f32, 0f32, 0f32),
                                                   vec4::new(0f32, 1f32, 0f32, 0f32),
                                                   vec4::new(0f32, 0f32, 1f32, 0f32),
                                                   vec4::new(0f32, 0f32, 0f32, 1f32));
    assert mat2x2::zero() == mat2x2::from_cols(vec2::zero(),
                                               vec2::zero());
    assert mat3x3::zero() == mat3x3::from_cols(vec3::zero(),
                                               vec3::zero(),
                                               vec3::zero());
    assert mat4x4::zero() == mat4x4::from_cols(vec4::zero(),
                                               vec4::zero(),
                                               vec4::zero(),
                                               vec4::zero());
    
    assert dmat2x2::identity() == dmat2x2::new(1f64, 0f64,
                                               0f64, 1f64);
    assert dmat2x2::identity() == dmat2x2::from_cols(dvec2::new(1f64, 0f64),
                                                     dvec2::new(0f64, 1f64));
    assert dmat3x3::identity() == dmat3x3::new(1f64, 0f64, 0f64,
                                               0f64, 1f64, 0f64,
                                               0f64, 0f64, 1f64);
    assert dmat3x3::identity() == dmat3x3::from_cols(dvec3::new(1f64, 0f64, 0f64),
                                                     dvec3::new(0f64, 1f64, 0f64),
                                                     dvec3::new(0f64, 0f64, 1f64));
    assert dmat4x4::identity() == dmat4x4::new(1f64, 0f64, 0f64, 0f64,
                                               0f64, 1f64, 0f64, 0f64,
                                               0f64, 0f64, 1f64, 0f64,
                                               0f64, 0f64, 0f64, 1f64);
    assert dmat4x4::identity() == dmat4x4::from_cols(dvec4::new(1f64, 0f64, 0f64, 0f64),
                                                     dvec4::new(0f64, 1f64, 0f64, 0f64),
                                                     dvec4::new(0f64, 0f64, 1f64, 0f64),
                                                     dvec4::new(0f64, 0f64, 0f64, 1f64));
    assert dmat2x2::zero() == dmat2x2::from_cols(dvec2::zero(),
                                                 dvec2::zero());
    assert dmat3x3::zero() == dmat3x3::from_cols(dvec3::zero(),
                                                 dvec3::zero(),
                                                 dvec3::zero());
    assert dmat4x4::zero() == dmat4x4::from_cols(dvec4::zero(),
                                                 dvec4::zero(),
                                                 dvec4::zero(),
                                                 dvec4::zero());
}

#[test]
fn test_mat_n() {
    assert mat2::identity() == mat2x2::identity();
    assert mat3::identity() == mat3x3::identity();
    assert mat4::identity() == mat4x4::identity();
    assert mat2::zero()     == mat2x2::zero();
    assert mat3::zero()     == mat3x3::zero();
    assert mat4::zero()     == mat4x4::zero();
    
    assert dmat2::identity() == dmat2x2::identity();
    assert dmat3::identity() == dmat3x3::identity();
    assert dmat4::identity() == dmat4x4::identity();
    assert dmat2::zero()     == dmat2x2::zero();
    assert dmat3::zero()     == dmat3x3::zero();
    assert dmat4::zero()     == dmat4x4::zero();
}