use core::float::consts::pi;

use angle::*;

#[test]
fn test_radians() {
    assert *Radians(0.0).to_degrees()           == *Degrees(0.0);
    assert *Radians(pi / 4.0).to_degrees()      == *Degrees(45.0);
    assert *Radians(pi / 2.0).to_degrees()      == *Degrees(90.0);
    assert *Radians(pi).to_degrees()            == *Degrees(180.0);
    assert *Radians(2.0 * pi).to_degrees()      == *Degrees(360.0);
    assert *Radians(5.0 * pi).to_degrees()      == *Degrees(900.0);
    assert *Radians(-pi).to_degrees()           == *Degrees(-180.0);
    
    assert *Radians(0.0).to_radians()           == *Radians(0.0);
    assert *Radians(5.0 * pi).to_radians()      == *Radians(5.0 * pi);
    assert *Radians(-pi).to_radians()           == *Radians(-pi);
    
    assert *(Radians(pi) + Radians(pi))         == *Radians(2.0 * pi);
    assert *(Radians(2.0 * pi) - Radians(pi))   == *Radians(pi);
    assert *(Radians(pi) * 2.0)                 == *Radians(2.0 * pi);
    assert *(Radians(pi) / 2.0)                 == *Radians(pi / 2.0);
    assert *(Radians(3.0 * pi) % (2.0 * pi))    == *Radians(pi);
    assert *(-Radians(pi))                      == *Radians(-pi);
}

#[test]
fn test_degrees() {
    assert *Degrees(0.0).to_radians()           == *Radians(0.0)      as float;
    assert *Degrees(45.0).to_radians()          == *Radians(pi / 4.0) as float;
    assert *Degrees(90.0).to_radians()          == *Radians(pi / 2.0) as float;
    assert *Degrees(180.0).to_radians()         == *Radians(pi)       as float;
    assert *Degrees(360.0).to_radians()         == *Radians(2.0 * pi) as float;
    assert *Degrees(900.0).to_radians()         == *Radians(5.0 * pi) as float;
    assert *Degrees(-180.0).to_radians()        == *Radians(-pi)      as float;
    
    assert *Degrees(0.0).to_degrees()           == *Degrees(0.0);
    assert *Degrees(900.0).to_degrees()         == *Degrees(900.0);
    assert *Degrees(-180.0).to_degrees()        == *Degrees(-180.0);
    
    assert *(Degrees(180.0) + Degrees(180.0))   == *Degrees(360.0);
    assert *(Degrees(360.0) - Degrees(180.0))   == *Degrees(180.0);
    assert *(Degrees(360.0) * 2.0)              == *Degrees(720.0);
    assert *(Degrees(180.0) / 2.0)              == *Degrees(90.0);
    assert *(Degrees(540.0) % (360.0))          == *Degrees(180.0);
    assert *(-Degrees(180.0))                   == *Degrees(-180.0);
}