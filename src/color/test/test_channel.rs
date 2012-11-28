use channel::*;

#[test]
fn test_channel_u8() {
    assert 0x00_u8.to_channel_u8() == 0x00_u8;
    assert 0x30_u8.to_channel_u8() == 0x30_u8;
    assert 0x66_u8.to_channel_u8() == 0x66_u8;
    assert 0xA0_u8.to_channel_u8() == 0xA0_u8;
    assert 0xFF_u8.to_channel_u8() == 0xFF_u8;
    
    assert 0x00_u8.to_channel_u16() == 0x0000_u16;
    assert 0x30_u8.to_channel_u16() == 0x3030_u16;
    assert 0x66_u8.to_channel_u16() == 0x6666_u16;
    assert 0xA0_u8.to_channel_u16() == 0xA0A0_u16;
    assert 0xFF_u8.to_channel_u16() == 0xFFFF_u16;
    
    assert 0x00_u8.to_channel_u32() == 0x0000_0000_u32;
    assert 0x30_u8.to_channel_u32() == 0x3030_3030_u32;
    assert 0x66_u8.to_channel_u32() == 0x6666_6666_u32;
    assert 0xA0_u8.to_channel_u32() == 0xA0A0_A0A0_u32;
    assert 0xFF_u8.to_channel_u32() == 0xFFFF_FFFF_u32;
    
    assert 0x00_u8.to_channel_u64() == 0x0000_0000_0000_0000_u64;
    assert 0x30_u8.to_channel_u64() == 0x3030_3030_3030_3030_u64;
    assert 0x66_u8.to_channel_u64() == 0x6666_6666_6666_6666_u64;
    assert 0xA0_u8.to_channel_u64() == 0xA0A0_A0A0_A0A0_A0A0_u64;
    assert 0xFF_u8.to_channel_u64() == 0xFFFF_FFFF_FFFF_FFFF_u64;
    
    assert 0x00_u8.to_channel_f32() == 0f32;
    assert 0xFF_u8.to_channel_f32() == 1f32;
    
    assert 0x00_u8.to_channel_f64() == 0f64;
    assert 0xFF_u8.to_channel_f64() == 1f64;
    
    assert 0x00_u8.to_channel_float() == 0f;
    assert 0xFF_u8.to_channel_float() == 1f;
    
    
    // Test inverse
    
    assert 0x00_u8.inverse() == 0xFF_u8;
    assert 0x66_u8.inverse() == 0x99_u8;
    assert 0xFF_u8.inverse() == 0x00_u8;
}

#[test]
fn test_channel_u16() {
    assert 0x0000_u16.to_channel_u8() == 0x00_u8;
    assert 0x3300_u16.to_channel_u8() == 0x33_u8;
    assert 0x6666_u16.to_channel_u8() == 0x66_u8;
    assert 0xAA00_u16.to_channel_u8() == 0xAA_u8;
    assert 0xFFFF_u16.to_channel_u8() == 0xFF_u8;
    
    assert 0x0000_u16.to_channel_u16() == 0x0000_u16;
    assert 0x3300_u16.to_channel_u16() == 0x3300_u16;
    assert 0x6666_u16.to_channel_u16() == 0x6666_u16;
    assert 0xAA00_u16.to_channel_u16() == 0xAA00_u16;
    assert 0xFFFF_u16.to_channel_u16() == 0xFFFF_u16;
    
    assert 0x0000_u16.to_channel_u32() == 0x0000_0000_u32;
    assert 0x3300_u16.to_channel_u32() == 0x3300_3300_u32;
    assert 0x6666_u16.to_channel_u32() == 0x6666_6666_u32;
    assert 0xAA00_u16.to_channel_u32() == 0xAA00_AA00_u32;
    assert 0xFFFF_u16.to_channel_u32() == 0xFFFF_FFFF_u32;
    
    assert 0x0000_u16.to_channel_u64() == 0x0000_0000_0000_0000_u64;
    assert 0x3300_u16.to_channel_u64() == 0x3300_3300_3300_3300_u64;
    assert 0x6666_u16.to_channel_u64() == 0x6666_6666_6666_6666_u64;
    assert 0xAA00_u16.to_channel_u64() == 0xAA00_AA00_AA00_AA00_u64;
    assert 0xFFFF_u16.to_channel_u64() == 0xFFFF_FFFF_FFFF_FFFF_u64;
    
    assert 0x0000_u16.to_channel_f32() == 0f32;
    assert 0xFFFF_u16.to_channel_f32() == 1f32;
    
    assert 0x0000_u16.to_channel_f64() == 0f64;
    assert 0xFFFF_u16.to_channel_f64() == 1f64;
    
    assert 0x0000_u16.to_channel_float() == 0f;
    assert 0xFFFF_u16.to_channel_float() == 1f;
    
    
    // Test inverse
    
    assert 0x0000_u16.inverse() == 0xFFFF_u16;
    assert 0x6666_u16.inverse() == 0x9999_u16;
    assert 0xFFFF_u16.inverse() == 0x0000_u16;
}

#[test]
fn test_channel_u32() {
    assert 0x0000_0000_u32.to_channel_u8() == 0x00_u8;
    assert 0x3333_0000_u32.to_channel_u8() == 0x33_u8;
    assert 0x6666_6666_u32.to_channel_u8() == 0x66_u8;
    assert 0xAAAA_0000_u32.to_channel_u8() == 0xAA_u8;
    assert 0xFFFF_FFFF_u32.to_channel_u8() == 0xFF_u8;
    
    assert 0x0000_0000_u32.to_channel_u16() == 0x0000_u16;
    assert 0x3333_0000_u32.to_channel_u16() == 0x3333_u16;
    assert 0x6666_6666_u32.to_channel_u16() == 0x6666_u16;
    assert 0xAAAA_0000_u32.to_channel_u16() == 0xAAAA_u16;
    assert 0xFFFF_FFFF_u32.to_channel_u16() == 0xFFFF_u16;
    
    assert 0x0000_0000_u32.to_channel_u32() == 0x0000_0000_u32;
    assert 0x3333_0000_u32.to_channel_u32() == 0x3333_0000_u32;
    assert 0x6666_6666_u32.to_channel_u32() == 0x6666_6666_u32;
    assert 0xAAAA_0000_u32.to_channel_u32() == 0xAAAA_0000_u32;
    assert 0xFFFF_FFFF_u32.to_channel_u32() == 0xFFFF_FFFF_u32;
    
    assert 0x0000_0000_u32.to_channel_u64() == 0x0000_0000_0000_0000_u64;
    assert 0x3333_0000_u32.to_channel_u64() == 0x3333_0000_3333_0000_u64;
    assert 0x6666_6666_u32.to_channel_u64() == 0x6666_6666_6666_6666_u64;
    assert 0xAAAA_0000_u32.to_channel_u64() == 0xAAAA_0000_AAAA_0000_u64;
    assert 0xFFFF_FFFF_u32.to_channel_u64() == 0xFFFF_FFFF_FFFF_FFFF_u64;
    
    assert 0x0000_0000_u32.to_channel_f32() == 0f32;
    assert 0xFFFF_FFFF_u32.to_channel_f32() == 1f32;
    
    assert 0x0000_0000_u32.to_channel_f64() == 0f64;
    assert 0xFFFF_FFFF_u32.to_channel_f64() == 1f64;
    
    assert 0x0000_0000_u32.to_channel_float() == 0f;
    assert 0xFFFF_FFFF_u32.to_channel_float() == 1f;
    
    
    // Test inverse
    
    assert 0x0000_0000_u32.inverse() == 0xFFFF_FFFF_u32;
    assert 0x6666_6666_u32.inverse() == 0x9999_9999_u32;
    assert 0xFFFF_FFFF_u32.inverse() == 0x0000_0000_u32;
}

#[test]
fn test_channel_u64() {
    assert 0x0000_0000_0000_0000_u64.to_channel_u8() == 0x00_u8;
    assert 0x3333_3333_0000_0000_u64.to_channel_u8() == 0x33_u8;    // FIXME: color shift?
    assert 0x6666_6666_6666_6666_u64.to_channel_u8() == 0x66_u8;
    assert 0xAAAA_AAAA_0000_0000_u64.to_channel_u8() == 0xAA_u8;    // FIXME: color shift?
    assert 0xFFFF_FFFF_FFFF_FFFF_u64.to_channel_u8() == 0xFF_u8;
    
    assert 0x0000_0000_0000_0000_u64.to_channel_u16() == 0x0000_u16;
    assert 0x3333_3333_0000_0000_u64.to_channel_u16() == 0x3333_u16;    // FIXME: color shift?
    assert 0x6666_6666_6666_6666_u64.to_channel_u16() == 0x6666_u16;
    assert 0xAAAA_AAAA_0000_0000_u64.to_channel_u16() == 0xAAAA_u16;    // FIXME: color shift?
    assert 0xFFFF_FFFF_FFFF_FFFF_u64.to_channel_u16() == 0xFFFF_u16;
    
    assert 0x0000_0000_0000_0000_u64.to_channel_u32() == 0x0000_0000_u32;
    assert 0x3333_3333_0000_0000_u64.to_channel_u32() == 0x3333_3333_u32;    // FIXME: color shift?
    assert 0x6666_6666_6666_6666_u64.to_channel_u32() == 0x6666_6666_u32;
    assert 0xAAAA_AAAA_0000_0000_u64.to_channel_u32() == 0xAAAA_AAAA_u32;    // FIXME: color shift?
    assert 0xFFFF_FFFF_FFFF_FFFF_u64.to_channel_u32() == 0xFFFF_FFFF_u32;
    
    assert 0x0000_0000_0000_0000_u64.to_channel_u64() == 0x0000_0000_0000_0000_u64;
    assert 0x3333_3333_0000_0000_u64.to_channel_u64() == 0x3333_3333_0000_0000_u64;
    assert 0x6666_6666_6666_6666_u64.to_channel_u64() == 0x6666_6666_6666_6666_u64;
    assert 0xAAAA_AAAA_0000_0000_u64.to_channel_u64() == 0xAAAA_AAAA_0000_0000_u64;
    assert 0xFFFF_FFFF_FFFF_FFFF_u64.to_channel_u64() == 0xFFFF_FFFF_FFFF_FFFF_u64;
    
    assert 0x0000_0000_0000_0000_u64.to_channel_f32() == 0f32;
    assert 0xFFFF_FFFF_FFFF_FFFF_u64.to_channel_f32() == 1f32;
    
    assert 0x0000_0000_0000_0000_u64.to_channel_f64() == 0f64;
    assert 0xFFFF_FFFF_FFFF_FFFF_u64.to_channel_f64() == 1f64;
    
    assert 0x0000_0000_0000_0000_u64.to_channel_float() == 0f;
    assert 0xFFFF_FFFF_FFFF_FFFF_u64.to_channel_float() == 1f;
    
    
    // Test inverse
    
    assert 0x0000_0000_0000_0000_u64.inverse() == 0xFFFF_FFFF_FFFF_FFFF_u64;
    assert 0x6666_6666_6666_6666_u64.inverse() == 0x9999_9999_9999_9999_u64;
    assert 0xFFFF_FFFF_FFFF_FFFF_u64.inverse() == 0x0000_0000_0000_0000_u64;
}

#[test]
fn test_channel_f32() {
    assert 0.00f32.to_channel_u8() == 0x00;
    assert 0.25f32.to_channel_u8() == 0x3F;
    assert 0.50f32.to_channel_u8() == 0x7F;
    assert 0.75f32.to_channel_u8() == 0xBF;
    assert 1.00f32.to_channel_u8() == 0xFF;
    
    assert 0.00f32.to_channel_u16() == 0x0000;
    assert 0.25f32.to_channel_u16() == 0x3FFF;
    assert 0.50f32.to_channel_u16() == 0x7FFF;
    assert 0.75f32.to_channel_u16() == 0xBFFF;
    assert 1.00f32.to_channel_u16() == 0xFFFF;
    
    // TODO: test to_channel_u32()
    
    // TODO: test to_channel_u64()
    
    // TODO: test to_channel_f32()
    
    // TODO: test to_channel_f64()
    
    // TODO: test to_channel_float()
    
    
    // Test inverse
    
    assert 0.00f32.inverse() == 1.00f32;
    assert 0.25f32.inverse() == 0.75f32;
    assert 0.50f32.inverse() == 0.50f32;
    assert 0.75f32.inverse() == 0.25f32;
    assert 1.00f32.inverse() == 0.00f32;
}

#[test]
fn test_channel_f64() {
    assert 0.00f64.to_channel_u8() == 0x00;
    assert 0.25f64.to_channel_u8() == 0x3F;
    assert 0.50f64.to_channel_u8() == 0x7F;
    assert 0.75f64.to_channel_u8() == 0xBF;
    assert 1.00f64.to_channel_u8() == 0xFF;
    
    assert 0.00f64.to_channel_u16() == 0x0000;
    assert 0.25f64.to_channel_u16() == 0x3FFF;
    assert 0.50f64.to_channel_u16() == 0x7FFF;
    assert 0.75f64.to_channel_u16() == 0xBFFF;
    assert 1.00f64.to_channel_u16() == 0xFFFF;
    
    // TODO: test to_channel_u32()
    
    // TODO: test to_channel_u64()
    
    // TODO: test to_channel_f32()
    
    // TODO: test to_channel_f64()
    
    // TODO: test to_channel_float()
    
    
    // Test inverse
    
    assert 0.00f64.inverse() == 1.00f64;
    assert 0.25f64.inverse() == 0.75f64;
    assert 0.50f64.inverse() == 0.50f64;
    assert 0.75f64.inverse() == 0.25f64;
    assert 1.00f64.inverse() == 0.00f64;
}

#[test]
fn test_channel_float() {
    assert 0.00f.to_channel_u8() == 0x00;
    assert 0.25f.to_channel_u8() == 0x3F;
    assert 0.50f.to_channel_u8() == 0x7F;
    assert 0.75f.to_channel_u8() == 0xBF;
    assert 1.00f.to_channel_u8() == 0xFF;
    
    assert 0.00f.to_channel_u16() == 0x0000;
    assert 0.25f.to_channel_u16() == 0x3FFF;
    assert 0.50f.to_channel_u16() == 0x7FFF;
    assert 0.75f.to_channel_u16() == 0xBFFF;
    assert 1.00f.to_channel_u16() == 0xFFFF;
    
    // TODO: test to_channel_u32()
    
    // TODO: test to_channel_u64()
    
    // TODO: test to_channel_f32()
    
    // TODO: test to_channel_f64()
    
    // TODO: test to_channel_float()
    
    
    // Test inverse
    
    assert 0.00f.inverse() == 1.00f;
    assert 0.25f.inverse() == 0.75f;
    assert 0.50f.inverse() == 0.50f;
    assert 0.75f.inverse() == 0.25f;
    assert 1.00f.inverse() == 0.00f;
}