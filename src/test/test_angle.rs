use core::float::consts::pi;

use angle::*;
use mat::Mat4;
use vec::{Vec3, Vec4};

#[test]
fn test_radians() {
    assert Radians(0.0).to_degrees()            == Degrees(0.0);
    assert Radians(pi / 4.0).to_degrees()       == Degrees(45.0);
    assert Radians(pi / 2.0).to_degrees()       == Degrees(90.0);
    assert Radians(pi).to_degrees()             == Degrees(180.0);
    assert Radians(2.0 * pi).to_degrees()       == Degrees(360.0);
    assert Radians(5.0 * pi).to_degrees()       == Degrees(900.0);
    assert Radians(-pi).to_degrees()            == Degrees(-180.0);
    
    assert Radians(0.0).to_radians()            == Radians(0.0);
    assert Radians(5.0 * pi).to_radians()       == Radians(5.0 * pi);
    assert Radians(-pi).to_radians()            == Radians(-pi);
    
    assert Radians(pi).wrap()                   == Radians(pi);
    assert Radians(3.0 * pi).wrap()             == Radians(pi);
    assert Radians(2.0 * pi).wrap()             == Radians(0.0);
    assert Radians(-pi).wrap()                  == Radians(-pi);   // FIXME: should be in the domain of 0 to two_pi
    assert Radians(-3.0 * pi).wrap()            == Radians(-pi);   // FIXME: should be in the domain of 0 to two_pi
    assert Radians(-2.0 * pi).wrap()            == Radians(0.0);
    
    assert Radians(0.0).opposite()              == Radians(pi as float);
    assert Radians(pi / 2.0).opposite()         == Radians((pi / 2.0) * 3.0);
    assert Radians(pi * 3.0).opposite()         == Radians(0.0);
    assert Radians(-2.0 * pi).opposite()        == Radians(-pi);   // FIXME: should be in the domain of 0 to two_pi
    
    assert Radians(pi) + Radians(pi)            == Radians(2.0 * pi);
    assert Radians(2.0 * pi) - Radians(pi)      == Radians(pi);
    assert Radians(pi) * 2.0                    == Radians(2.0 * pi);
    assert Radians(pi) / 2.0                    == Radians(pi / 2.0);
    assert Radians(3.0 * pi) % (2.0 * pi)       == Radians(pi);
    assert -Radians(pi)                         == Radians(-pi);
    
    assert fmt!("%s", Radians(1).to_str())      == ~"1 rad";
}

#[test]
fn test_degrees() {
    assert Degrees(0.0).to_radians()            == Radians(0.0);
    assert Degrees(45.0).to_radians()           == Radians(pi / 4.0 as float);  // Rust can't infer these types
    assert Degrees(90.0).to_radians()           == Radians(pi / 2.0 as float);
    assert Degrees(180.0).to_radians()          == Radians(pi       as float);
    assert Degrees(360.0).to_radians()          == Radians(2.0 * pi as float);
    assert Degrees(900.0).to_radians()          == Radians(5.0 * pi as float);
    assert Degrees(-180.0).to_radians()         == Radians(-pi      as float);
    
    assert Degrees(0.0).to_degrees()            == Degrees(0.0);
    assert Degrees(900.0).to_degrees()          == Degrees(900.0);
    assert Degrees(-180.0).to_degrees()         == Degrees(-180.0);
    
    assert Degrees(90.0).wrap()                 == Degrees(90.0);
    assert Degrees(450.0).wrap()                == Degrees(90.0);
    assert Degrees(360.0).wrap()                == Degrees(0.0);
    assert Degrees(-90.0).wrap()                == Degrees(-90.0);   // FIXME: should be in the domain of 0 to 360
    assert Degrees(-450.0).wrap()               == Degrees(-90.0);   // FIXME: should be in the domain of 0 to 360
    assert Degrees(-360.0).wrap()               == Degrees(0.0);
    
    assert Degrees(0.0).opposite()              == Degrees(180.0);
    assert Degrees(90.0).opposite()             == Degrees(270.0);
    assert Degrees(540.0).opposite()            == Degrees(0.0);
    assert Degrees(-360.0).opposite()           == Degrees(-180.0);   // FIXME: should be in the domain of 0 to 360
    
    assert Degrees(180.0) + Degrees(180.0)      == Degrees(360.0);
    assert Degrees(360.0) - Degrees(180.0)      == Degrees(180.0);
    assert Degrees(360.0) * 2.0                 == Degrees(720.0);
    assert Degrees(180.0) / 2.0                 == Degrees(90.0);
    assert Degrees(540.0) % (360.0)             == Degrees(180.0);
    assert -Degrees(180.0)                      == Degrees(-180.0);
    
    assert fmt!("%s", Degrees(180.0).to_str())  == ~"180°";
}



#[test]
fn test_rotation() {
    {
        let pos = Vec4::new(1.0, 0.0, 0.0, 1.0);   // the position to transform
        let rot = Rotation {
            theta: Degrees(180.0).to_radians(),
            axis:  Vec3::new(0.0, 0.0, 1.0),       // unit_z
        };
        
        let newpos = rot.to_mat4().mul_v(&pos);
        let expected_pos = Vec4::new(-1.0, 0.0, 0.0, 1.0);
        
        assert newpos == expected_pos;
    }
    {
        let pos = Vec4::new(4f32, 0f32, 0f32, 1f32);
        
        let rot_a = Rotation {
            theta: Degrees(90f32).to_radians(),
            axis:  Vec3::new(0f32, 1f32, 0f32),     // unit_y
        };
        
        let rot_b = Rotation {
            theta: Degrees(90f32).to_radians(),
            axis:  -Vec3::new(0f32, 1f32, 0f32),    // -unit_y
        };
        
        let newpos_a = rot_a.to_mat4().mul_v(&pos);
        let newpos_b = rot_b.to_mat4().mul_v(&pos);
        
        let expected_pos_a = Vec4::new(0f32, 0f32, -4f32, 1f32);
        let expected_pos_b = Vec4::new(0f32, 0f32,  4f32, 1f32);
        
        assert newpos_a == expected_pos_a;
        assert newpos_b == expected_pos_b;
    }
    
    // TODO: test to_quat
}