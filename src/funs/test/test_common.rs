use common::*;
use angle::{Radians, Degrees};
use vec::{Vec2, Vec3, Vec4};

#[test]
fn test_abs() {
    assert 0.abs()      == 0;
    assert 2.abs()      == 2;
    assert (-2).abs()   == 2;
    assert abs(&0)      == 0;
    assert abs(&2)      == 2;
    assert abs(&-2)     == 2;
    
    assert 0.0.abs()    == 0.0;
    assert 2.5.abs()    == 2.5;
    assert (-2.5).abs() == 2.5;
    assert abs(&0.0)    == 0.0;
    assert abs(&2.5)    == 2.5;
    assert abs(&-2.5)   == 2.5;
    
    assert Radians(0.0).abs()    == Radians(0.0);
    assert Radians(2.5).abs()    == Radians(2.5);
    assert (Radians(-2.5)).abs() == Radians(2.5);
    assert abs(&Radians(0.0))    == Radians(0.0);
    assert abs(&Radians(2.5))    == Radians(2.5);
    assert abs(&Radians(-2.5))   == Radians(2.5);
    
    assert Degrees(0.0).abs()    == Degrees(0.0);
    assert Degrees(2.5).abs()    == Degrees(2.5);
    assert (Degrees(-2.5)).abs() == Degrees(2.5);
    assert abs(&Degrees(0.0))    == Degrees(0.0);
    assert abs(&Degrees(2.5))    == Degrees(2.5);
    assert abs(&Degrees(-2.5))   == Degrees(2.5);
}

#[test]
fn test_sign() {
    assert 0.sign()     == 0;
    assert 2.sign()     == 1;
    assert (-2).sign()  == -1;
    assert sign(&0)     == 0;
    assert sign(&2)     == 1;
    assert sign(&-2)    == -1;
    
    assert 0.0.sign()   == 0.0;
    assert 2.5.sign()   == 1.0;
    assert (-2.5).sign()== -1.0;
    assert sign(&0.0)   == 0.0;
    assert sign(&2.5)   == 1.0;
    assert sign(&-2.5)  == -1.0;
    
    assert Radians(0.0).sign()   == Radians(0.0);
    assert Radians(2.5).sign()   == Radians(1.0);
    assert (Radians(-2.5)).sign()== Radians(-1.0);
    assert sign(&Radians(0.0))   == Radians(0.0);
    assert sign(&Radians(2.5))   == Radians(1.0);
    assert sign(&Radians(-2.5))  == Radians(-1.0);
    
    assert Degrees(0.0).sign()   == Degrees(0.0);
    assert Degrees(2.5).sign()   == Degrees(1.0);
    assert (Degrees(-2.5)).sign()== Degrees(-1.0);
    assert sign(&Degrees(0.0))   == Degrees(0.0);
    assert sign(&Degrees(2.5))   == Degrees(1.0);
    assert sign(&Degrees(-2.5))  == Degrees(-1.0);
}

#[test]
fn test_min() {
    assert 1u.min(&2u)      == 1u;
    assert 1u8.min(&2u8)    == 1u8;
    assert 1u16.min(&2u16)  == 1u16;
    assert 1u32.min(&2u32)  == 1u32;
    assert 1u64.min(&2u64)  == 1u64;
    assert 1.min(&2)        == 1;
    assert 1i8.min(&2i8)    == 1i8;
    assert 1i16.min(&2i16)  == 1i16;
    assert 1i32.min(&2i32)  == 1i32;
    assert 1i64.min(&2i64)  == 1i64;
    assert 1f.min(&2f)      == 1f;
    assert 1f32.min(&2f32)  == 1f32;
    assert 1f64.min(&2f64)  == 1f64;
    
    assert 2u.min(&1u)      == 1u;
    assert 2u8.min(&1u8)    == 1u8;
    assert 2u16.min(&1u16)  == 1u16;
    assert 2u32.min(&1u32)  == 1u32;
    assert 2u64.min(&1u64)  == 1u64;
    assert 2.min(&1)        == 1;
    assert 2i8.min(&1i8)    == 1i8;
    assert 2i16.min(&1i16)  == 1i16;
    assert 2i32.min(&1i32)  == 1i32;
    assert 2i64.min(&1i64)  == 1i64;
    assert 2f.min(&1f)      == 1f;
    assert 2f32.min(&1f32)  == 1f32;
    assert 2f64.min(&1f64)  == 1f64;
    
    assert min(&1u, &2u)        == 1u;
    assert min(&1u8, &2u8)      == 1u8;
    assert min(&1u16, &2u16)    == 1u16;
    assert min(&1u32, &2u32)    == 1u32;
    assert min(&1u64, &2u64)    == 1u64;
    assert min(&1, &2)          == 1;
    assert min(&1i8, &2i8)      == 1i8;
    assert min(&1i16, &2i16)    == 1i16;
    assert min(&1i32, &2i32)    == 1i32;
    assert min(&1i64, &2i64)    == 1i64;
    assert min(&1f, &2f)        == 1f;
    assert min(&1f32, &2f32)    == 1f32;
    assert min(&1f64, &2f64)    == 1f64;
    
    assert min(&2u, &1u)        == 1u;
    assert min(&2u8,  &1u8)     == 1u8;
    assert min(&2u16, &1u16)    == 1u16;
    assert min(&2u32, &1u32)    == 1u32;
    assert min(&2u64, &1u64)    == 1u64;
    assert min(&2, &1)          == 1;
    assert min(&2i8, &1i8)      == 1i8;
    assert min(&2i16, &1i16)    == 1i16;
    assert min(&2i32, &1i32)    == 1i32;
    assert min(&2i64, &1i64)    == 1i64;
    assert min(&2f, &1f)        == 1f;
    assert min(&2f32, &1f32)    == 1f32;
    assert min(&2f64, &1f64)    == 1f64;
    
    assert Radians(1).min(&Radians(2))   == Radians(1);
    assert Radians(2).min(&Radians(1))   == Radians(1);
    assert min(&Radians(1), &Radians(2)) == Radians(1);
    assert min(&Radians(2), &Radians(1)) == Radians(1);
    
    assert Degrees(1).min(&Degrees(2))   == Degrees(1);
    assert Degrees(2).min(&Degrees(1))   == Degrees(1);
    assert min(&Degrees(1), &Degrees(2)) == Degrees(1);
    assert min(&Degrees(2), &Degrees(1)) == Degrees(1);
    
    assert min(&Vec2::new(1, 2),        &Vec2::new(2, 1))       == Vec2::new(1, 1);
    assert min(&Vec3::new(1, 2, 3),     &Vec3::new(3, 2, 1))    == Vec3::new(1, 2, 1);
    assert min(&Vec4::new(1, 2, 3, 4),  &Vec4::new(4, 3, 2, 1)) == Vec4::new(1, 2, 2, 1);
    
    assert min(&Vec2::new(2, 1),        &Vec2::new(1, 2))       == Vec2::new(1, 1);
    assert min(&Vec3::new(3, 2, 1),     &Vec3::new(1, 2, 3))    == Vec3::new(1, 2, 1);
    assert min(&Vec4::new(4, 3, 2, 1),  &Vec4::new(1, 2, 3, 4)) == Vec4::new(1, 2, 2, 1);
}

#[test]
fn test_max() {
    assert 1u.max(&2u)      == 2u;
    assert 1u8.max(&2u8)    == 2u8;
    assert 1u16.max(&2u16)  == 2u16;
    assert 1u32.max(&2u32)  == 2u32;
    assert 1u64.max(&2u64)  == 2u64;
    assert 1.max(&2)        == 2;
    assert 1i8.max(&2i8)    == 2i8;
    assert 1i16.max(&2i16)  == 2i16;
    assert 1i32.max(&2i32)  == 2i32;
    assert 1i64.max(&2i64)  == 2i64;
    assert 1f.max(&2f)      == 2f;
    assert 1f32.max(&2f32)  == 2f32;
    assert 1f64.max(&2f64)  == 2f64;
    
    assert 2u.max(&1u)      == 2u;
    assert 2u8.max(&1u8)    == 2u8;
    assert 2u16.max(&1u16)  == 2u16;
    assert 2u32.max(&1u32)  == 2u32;
    assert 2u64.max(&1u64)  == 2u64;
    assert 2.max(&1)        == 2;
    assert 2i8.max(&1i8)    == 2i8;
    assert 2i16.max(&1i16)  == 2i16;
    assert 2i32.max(&1i32)  == 2i32;
    assert 2i64.max(&1i64)  == 2i64;
    assert 2f.max(&1f)      == 2f;
    assert 2f32.max(&1f32)  == 2f32;
    assert 2f64.max(&1f64)  == 2f64;
    
    
    assert max(&1u, &2u)        == 2u;
    assert max(&1u8, &2u8)      == 2u8;
    assert max(&1u16, &2u16)    == 2u16;
    assert max(&1u32, &2u32)    == 2u32;
    assert max(&1u64, &2u64)    == 2u64;
    assert max(&1, &2)          == 2;
    assert max(&1i8, &2i8)      == 2i8;
    assert max(&1i16, &2i16)    == 2i16;
    assert max(&1i32, &2i32)    == 2i32;
    assert max(&1i64, &2i64)    == 2i64;
    assert max(&1f, &2f)        == 2f;
    assert max(&1f32, &2f32)    == 2f32;
    assert max(&1f64, &2f64)    == 2f64;
    
    
    assert max(&2u, &1u)        == 2u;
    assert max(&2u8,  &1u8)     == 2u8;
    assert max(&2u16, &1u16)    == 2u16;
    assert max(&2u32, &1u32)    == 2u32;
    assert max(&2u64, &1u64)    == 2u64;
    assert max(&2, &1)          == 2;
    assert max(&2i8, &1i8)      == 2i8;
    assert max(&2i16, &1i16)    == 2i16;
    assert max(&2i32, &1i32)    == 2i32;
    assert max(&2i64, &1i64)    == 2i64;
    assert max(&2f, &1f)        == 2f;
    assert max(&2f32, &1f32)    == 2f32;
    assert max(&2f64, &1f64)    == 2f64;
    
    assert Radians(1).max(&Radians(2))   == Radians(2);
    assert Radians(2).max(&Radians(1))   == Radians(2);
    assert max(&Radians(1), &Radians(2)) == Radians(2);
    assert max(&Radians(2), &Radians(1)) == Radians(2);
    
    assert Degrees(1).max(&Degrees(2))   == Degrees(2);
    assert Degrees(2).max(&Degrees(1))   == Degrees(2);
    assert max(&Degrees(1), &Degrees(2)) == Degrees(2);
    assert max(&Degrees(2), &Degrees(1)) == Degrees(2);
    
    assert max(&Vec2::new(1, 2),        &Vec2::new(2, 1))       == Vec2::new(2, 2);
    assert max(&Vec3::new(1, 2, 3),     &Vec3::new(3, 2, 1))    == Vec3::new(3, 2, 3);
    assert max(&Vec4::new(1, 2, 3, 4),  &Vec4::new(4, 3, 2, 1)) == Vec4::new(4, 3, 3, 4);
    
    assert max(&Vec2::new(2, 1),        &Vec2::new(1, 2))       == Vec2::new(2, 2);
    assert max(&Vec3::new(3, 2, 1),     &Vec3::new(1, 2, 3))    == Vec3::new(3, 2, 3);
    assert max(&Vec4::new(4, 3, 2, 1),  &Vec4::new(1, 2, 3, 4)) == Vec4::new(4, 3, 3, 4);
}

#[test]
fn test_clamp() {
    
}

#[test]
fn test_clampv() {
    
}