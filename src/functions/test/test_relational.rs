use vector::*;
use relational::*;

#[test]
fn test_BooleanVec2_trait() {
    let tf = Vec2::new(true, false);
    let ff = Vec2::new(false, false);
    let tt = Vec2::new(true, true);
    
    assert tf.any() == true;
    assert tf.all() == false;
    assert tf.not().exact_eq(&Vec2::new(false, true));
    
    assert ff.any() == false;
    assert ff.all() == false;
    assert ff.not().exact_eq(&Vec2::new(true, true));
    
    assert tt.any() == true;
    assert tt.all() == true;
    assert tt.not().exact_eq(&Vec2::new(false, false));
}

#[test]
fn test_BooleanVec3_trait() {
    let tft = Vec3::new(true, false, true);
    let fff = Vec3::new(false, false, false);
    let ttt = Vec3::new(true, true, true);
    
    assert tft.any() == true;
    assert tft.all() == false;
    assert tft.not().exact_eq(&Vec3::new(false, true, false));
    
    assert fff.any() == false;
    assert fff.all() == false;
    assert fff.not().exact_eq(&Vec3::new(true, true, true));
    
    assert ttt.any() == true;
    assert ttt.all() == true;
    assert ttt.not().exact_eq(&Vec3::new(false, false, false));
}

#[test]
fn test_BooleanVec4_trait() {
    let tftf = Vec4::new(true, false, true, false);
    let ffff = Vec4::new(false, false, false, false);
    let tttt = Vec4::new(true, true, true, true);
    
    assert tftf.any() == true;
    assert tftf.all() == false;
    assert tftf.not().exact_eq(&Vec4::new(false, true, false, true));
    
    assert ffff.any() == false;
    assert ffff.all() == false;
    assert ffff.not().exact_eq(&Vec4::new(true, true, true, true));
    
    assert tttt.any() == true;
    assert tttt.all() == true;
    assert tttt.not().exact_eq(&Vec4::new(false, false, false, false));
}

#[test]
fn test_BooleanVec_fns() {
    let tf = Vec2::new(true, false);
    let ftf = Vec3::new(false, true, false);
    let tftf = Vec4::new(true, false, true, false);
    
    assert any(&tf) == true;
    assert all(&ftf) == false;
    assert not(&tftf).exact_eq(&Vec4::new(false, true, false, true));
}