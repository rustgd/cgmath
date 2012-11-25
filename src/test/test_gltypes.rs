use gltypes::*;

#[test]
fn test_vec() {
    assert vec2::identity() == vec2::from_value(1f32);
    assert vec3::identity() == vec3::from_value(1f32);
    assert vec4::identity() == vec4::from_value(1f32);
    assert vec2::identity() == vec2::new(1f32, 1f32);
    assert vec3::identity() == vec3::new(1f32, 1f32, 1f32);
    assert vec4::identity() == vec4::new(1f32, 1f32, 1f32, 1f32);
    assert vec2::zero()     == vec2::new(0f32, 0f32);
    assert vec3::zero()     == vec3::new(0f32, 0f32, 0f32);
    assert vec4::zero()     == vec4::new(0f32, 0f32, 0f32, 0f32);
    assert vec2::dim()      == 2;
    assert vec3::dim()      == 3;
    assert vec4::dim()      == 4;
    assert vec2::size_of()  == vec2::dim() * 4;
    assert vec3::size_of()  == vec3::dim() * 4;
    assert vec4::size_of()  == vec4::dim() * 4;
    
    assert dvec2::identity() == dvec2::from_value(1f64);
    assert dvec3::identity() == dvec3::from_value(1f64);
    assert dvec4::identity() == dvec4::from_value(1f64);
    assert dvec2::identity() == dvec2::new(1f64, 1f64);
    assert dvec3::identity() == dvec3::new(1f64, 1f64, 1f64);
    assert dvec4::identity() == dvec4::new(1f64, 1f64, 1f64, 1f64);
    assert dvec2::zero()     == dvec2::new(0f64, 0f64);
    assert dvec3::zero()     == dvec3::new(0f64, 0f64, 0f64);
    assert dvec4::zero()     == dvec4::new(0f64, 0f64, 0f64, 0f64);
    assert dvec2::dim()      == 2;
    assert dvec3::dim()      == 3;
    assert dvec4::dim()      == 4;
    assert dvec2::size_of()  == dvec2::dim() * 8;
    assert dvec3::size_of()  == dvec3::dim() * 8;
    assert dvec4::size_of()  == dvec4::dim() * 8;
    
    assert bvec2::dim()      == 2;
    assert bvec3::dim()      == 3;
    assert bvec4::dim()      == 4;
    assert bvec2::size_of()  == bvec2::dim() * 1;
    assert bvec3::size_of()  == bvec3::dim() * 1;
    assert bvec4::size_of()  == bvec4::dim() * 1;
    
    assert ivec2::identity() == ivec2::from_value(1i32);
    assert ivec3::identity() == ivec3::from_value(1i32);
    assert ivec4::identity() == ivec4::from_value(1i32);
    assert ivec2::identity() == ivec2::new(1i32, 1i32);
    assert ivec2::identity() == ivec2::new(1i32, 1i32);
    assert ivec3::identity() == ivec3::new(1i32, 1i32, 1i32);
    assert ivec4::identity() == ivec4::new(1i32, 1i32, 1i32, 1i32);
    assert ivec2::zero()     == ivec2::new(0i32, 0i32);
    assert ivec3::zero()     == ivec3::new(0i32, 0i32, 0i32);
    assert ivec4::zero()     == ivec4::new(0i32, 0i32, 0i32, 0i32);
    assert ivec2::identity() == ivec2::from_value(1);
    assert ivec3::identity() == ivec3::from_value(1);
    assert ivec4::identity() == ivec4::from_value(1);
    assert ivec2::identity() == ivec2::new(1, 1);
    assert ivec2::identity() == ivec2::new(1, 1);
    assert ivec3::identity() == ivec3::new(1, 1, 1);
    assert ivec4::identity() == ivec4::new(1, 1, 1, 1);
    assert ivec2::zero()     == ivec2::new(0, 0);
    assert ivec3::zero()     == ivec3::new(0, 0, 0);
    assert ivec4::zero()     == ivec4::new(0, 0, 0, 0);
    assert ivec2::dim()      == 2;
    assert ivec3::dim()      == 3;
    assert ivec4::dim()      == 4;
    assert ivec2::size_of()  == ivec2::dim() * 4;
    assert ivec3::size_of()  == ivec3::dim() * 4;
    assert ivec4::size_of()  == ivec4::dim() * 4;
    
    assert uvec2::identity() == uvec2::from_value(1u32);
    assert uvec3::identity() == uvec3::from_value(1u32);
    assert uvec4::identity() == uvec4::from_value(1u32);
    assert uvec2::identity() == uvec2::new(1u32, 1u32);
    assert uvec2::identity() == uvec2::new(1u32, 1u32);
    assert uvec3::identity() == uvec3::new(1u32, 1u32, 1u32);
    assert uvec4::identity() == uvec4::new(1u32, 1u32, 1u32, 1u32);
    assert uvec2::zero()     == uvec2::new(0u32, 0u32);
    assert uvec3::zero()     == uvec3::new(0u32, 0u32, 0u32);
    assert uvec4::zero()     == uvec4::new(0u32, 0u32, 0u32, 0u32);
    assert uvec2::identity() == uvec2::from_value(1);
    assert uvec3::identity() == uvec3::from_value(1);
    assert uvec4::identity() == uvec4::from_value(1);
    assert uvec2::identity() == uvec2::new(1, 1);
    assert uvec2::identity() == uvec2::new(1, 1);
    assert uvec3::identity() == uvec3::new(1, 1, 1);
    assert uvec4::identity() == uvec4::new(1, 1, 1, 1);
    assert uvec2::zero()     == uvec2::new(0, 0);
    assert uvec3::zero()     == uvec3::new(0, 0, 0);
    assert uvec4::zero()     == uvec4::new(0, 0, 0, 0);
    assert uvec2::dim()      == 2;
    assert uvec3::dim()      == 3;
    assert uvec4::dim()      == 4;
    assert uvec2::size_of()  == uvec2::dim() * 4;
    assert uvec3::size_of()  == uvec3::dim() * 4;
    assert uvec4::size_of()  == uvec4::dim() * 4;
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
    assert mat2x2::dim() == 2;
    assert mat3x3::dim() == 3;
    assert mat4x4::dim() == 4;
    assert mat2x2::rows() == 2;
    assert mat3x3::rows() == 3;
    assert mat4x4::rows() == 4;
    assert mat2x2::cols() == 2;
    assert mat3x3::cols() == 3;
    assert mat4x4::cols() == 4;
    assert mat2x2::size_of() == mat2x2::rows() * mat2x2::cols() * 4;
    assert mat3x3::size_of() == mat3x3::rows() * mat3x3::cols() * 4;
    assert mat4x4::size_of() == mat4x4::rows() * mat4x4::cols() * 4;
    
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
    assert dmat2x2::dim() == 2;
    assert dmat3x3::dim() == 3;
    assert dmat4x4::dim() == 4;
    assert dmat2x2::rows() == 2;
    assert dmat3x3::rows() == 3;
    assert dmat4x4::rows() == 4;
    assert dmat2x2::cols() == 2;
    assert dmat3x3::cols() == 3;
    assert dmat4x4::cols() == 4;
    assert dmat2x2::size_of() == dmat2x2::rows() * dmat2x2::cols() * 8;
    assert dmat3x3::size_of() == dmat3x3::rows() * dmat3x3::cols() * 8;
    assert dmat4x4::size_of() == dmat4x4::rows() * dmat4x4::cols() * 8;
}

#[test]
fn test_mat_n() {
    assert mat2::identity() == mat2x2::identity();
    assert mat3::identity() == mat3x3::identity();
    assert mat4::identity() == mat4x4::identity();
    assert mat2::zero()     == mat2x2::zero();
    assert mat3::zero()     == mat3x3::zero();
    assert mat4::zero()     == mat4x4::zero();
    assert mat2::dim()      == 2;
    assert mat3::dim()      == 3;
    assert mat4::dim()      == 4;
    assert mat2::rows()     == 2;
    assert mat3::rows()     == 3;
    assert mat4::rows()     == 4;
    assert mat2::cols()     == 2;
    assert mat3::cols()     == 3;
    assert mat4::cols()     == 4;
    assert mat2::size_of()  == mat2::rows() * mat2::cols() * 4;
    assert mat3::size_of()  == mat3::rows() * mat3::cols() * 4;
    assert mat4::size_of()  == mat4::rows() * mat4::cols() * 4;
    
    assert dmat2::identity() == dmat2x2::identity();
    assert dmat3::identity() == dmat3x3::identity();
    assert dmat4::identity() == dmat4x4::identity();
    assert dmat2::zero()     == dmat2x2::zero();
    assert dmat3::zero()     == dmat3x3::zero();
    assert dmat4::zero()     == dmat4x4::zero();
    assert dmat2::dim()      == 2;
    assert dmat3::dim()      == 3;
    assert dmat4::dim()      == 4;
    assert dmat2::rows()     == 2;
    assert dmat3::rows()     == 3;
    assert dmat4::rows()     == 4;
    assert dmat2::cols()     == 2;
    assert dmat3::cols()     == 3;
    assert dmat4::cols()     == 4;
    assert dmat2::size_of()  == dmat2::rows() * dmat2::cols() * 8;
    assert dmat3::size_of()  == dmat3::rows() * dmat3::cols() * 8;
    assert dmat4::size_of()  == dmat4::rows() * dmat4::cols() * 8;
}