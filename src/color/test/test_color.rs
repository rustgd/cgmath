use angle::*;
use color::*;

#[test]
fn test_color_rgb() {
    // TODO
    assert RGB::new::<u8>(0xA0, 0xA0, 0xA0).to_rgb_u8()  == RGB::new(0xA0, 0xA0, 0xA0);
    assert RGB::new::<u8>(0xA0, 0xA0, 0xA0).to_rgb_u16() == RGB::new(0xA0A0, 0xA0A0, 0xA0A0);
    assert RGB::new::<u8>(0xA0, 0xA0, 0xA0).to_rgb_u32() == RGB::new(0xA0A0_A0A0, 0xA0A0_A0A0, 0xA0A0_A0A0);
    assert RGB::new::<u8>(0xA0, 0xA0, 0xA0).to_rgb_u64() == RGB::new(0xA0A0_A0A0_A0A0_A0A0, 0xA0A0_A0A0_A0A0_A0A0, 0xA0A0_A0A0_A0A0_A0A0);
    
    // assert RGB::new::<u8>(0xFF, 0xFF, 0xFF).to_hsv_f32() == HSV::new(Degrees(0.0), 0.0, 1.0); // FIXME: causes an ICE
    // RGB::new::<u8>(0xFF, 0xFF, 0xFF).to_hsv_f32(); // FIXME: causes an ICE as well :(
}

#[test]
fn test_color_rgba() {
    // TODO
}

#[test]
fn test_color_hsv() {
    // TODO
}

#[test]
fn test_color_hsva() {
    // TODO
}