use num::cast::{NumCast, cast};

pub trait Channel {
    static pure fn channel_max() -> self;
    
    static pure fn from_channel<T:Channel>(val: T) -> self;
    pure fn convert_channel<T:Channel>() -> T;
    
    pure fn to_channel_u8()    -> u8;
    pure fn to_channel_u16()   -> u16;
    pure fn to_channel_u32()   -> u32;
    pure fn to_channel_u64()   -> u64;
    pure fn to_channel_f32()   -> f32;
    pure fn to_channel_f64()   -> f64;
    pure fn to_channel_float() -> float;
    
    pure fn inverse() -> self;
}

pub pure fn convert_channel<T:Channel, U:Channel>(val: T) -> U { val.convert_channel() }

pub impl u8: Channel {
    static pure fn channel_max() -> u8 { 0xFF }  // 2^8
    
    static pure fn from_channel<T:Channel>(val: T) -> u8 { val.to_channel_u8() }
    pure fn convert_channel<T:Channel>() -> T { from_channel(self) }
    
    pure fn to_channel_u8()    -> u8    { self }
    pure fn to_channel_u16()   -> u16   { (self as u16 << 8) | self as u16 }
    pure fn to_channel_u32()   -> u32   { (self.to_channel_u16() as u32 << 16) | self.to_channel_u16() as u32 }
    pure fn to_channel_u64()   -> u64   { (self.to_channel_u32() as u64 << 32) | self.to_channel_u32() as u64 }
    pure fn to_channel_f32()   -> f32   { (self as f32)   / (0xFF as f32)   }
    pure fn to_channel_f64()   -> f64   { (self as f64)   / (0xFF as f64)   }
    pure fn to_channel_float() -> float { (self as float) / (0xFF as float) }
    
    pure fn inverse() -> u8 { !self }
}

pub impl u16: Channel {
    static pure fn channel_max() -> u16 { 0xFFFF }   // 2^16
    
    static pure fn from_channel<T:Channel>(val: T) -> u16 { val.to_channel_u16() }
    pure fn convert_channel<T:Channel>() -> T { from_channel(self) }
    
    pure fn to_channel_u8()    -> u8    { self / 0x100 as u8 }
    pure fn to_channel_u16()   -> u16   { self }
    pure fn to_channel_u32()   -> u32   { (self as u32 << 16) | self as u32 }
    pure fn to_channel_u64()   -> u64   { (self.to_channel_u32() as u64 << 32) | self.to_channel_u32() as u64 }
    pure fn to_channel_f32()   -> f32   { self / 0xFFFF as f32   }
    pure fn to_channel_f64()   -> f64   { self / 0xFFFF as f64   }
    pure fn to_channel_float() -> float { self / 0xFFFF as float }
    
    pure fn inverse() -> u16 { !self }
}

pub impl u32: Channel {
    static pure fn channel_max() -> u32 { 0xFFFF_FFFF }   // 2^32
    
    static pure fn from_channel<T:Channel>(val: T) -> u32 { val.to_channel_u32() }
    pure fn convert_channel<T:Channel>() -> T { from_channel(self) }
    
    pure fn to_channel_u8()    -> u8    { self / 0x1_0000_00 as u8 }
    pure fn to_channel_u16()   -> u16   { self / 0x1_0000 as u16 }
    pure fn to_channel_u32()   -> u32   { self }
    pure fn to_channel_u64()   -> u64   { (self as u64 << 32) | self as u64 }
    pure fn to_channel_f32()   -> f32   { self / 0xFFFF_FFFF as f32   }
    pure fn to_channel_f64()   -> f64   { self / 0xFFFF_FFFF as f64   }
    pure fn to_channel_float() -> float { self / 0xFFFF_FFFF as float }
    
    pure fn inverse() -> u32 { !self }
}

pub impl u64: Channel {
    static pure fn channel_max() -> u64 { 0xFFFF_FFFF_FFFF_FFFF_u64 }   // 2^64
    
    static pure fn from_channel<T:Channel>(val: T) -> u64 { val.to_channel_u64() }
    pure fn convert_channel<T:Channel>() -> T { from_channel(self) }
    
    pure fn to_channel_u8()    -> u8    { self / 0x1_0000_0000_0000_00 as u8 }
    pure fn to_channel_u16()   -> u16   { self / 0x1_0000_0000_0000 as u16 }
    pure fn to_channel_u32()   -> u32   { self / 0x1_0000_0000 as u32 }
    pure fn to_channel_u64()   -> u64   { self }
    pure fn to_channel_f32()   -> f32   { self / 0xFFFF_FFFF_FFFF_FFFF_u64 as f32   }
    pure fn to_channel_f64()   -> f64   { self / 0xFFFF_FFFF_FFFF_FFFF_u64 as f64   }
    pure fn to_channel_float() -> float { self / 0xFFFF_FFFF_FFFF_FFFF_u64 as float }
    
    pure fn inverse() -> u64 { !self }
}

pub impl f32: Channel {
    static pure fn channel_max() -> f32 { 1f32 }
    
    static pure fn from_channel<T:Channel>(val: T) -> f32 { val.to_channel_f32() }
    pure fn convert_channel<T:Channel>() -> T { from_channel(self) }
    
    
    pure fn to_channel_u8()    -> u8    { self * (0xFFu8    as f32) as u8  }
    pure fn to_channel_u16()   -> u16   { self * (0xFFFFu16 as f32) as u16 }
    pure fn to_channel_u32()   -> u32   { fail(~"to_channel_u32 not yet implemented for f32") }
    pure fn to_channel_u64()   -> u64   { fail(~"to_channel_u64 not yet implemented for f32") }
    pure fn to_channel_f32()   -> f32   { self }
    pure fn to_channel_f64()   -> f64   { self as f64 }
    pure fn to_channel_float() -> float { self as float }
    
    pure fn inverse() -> f32 { 1.0 - self }
}

pub impl f64: Channel {
    static pure fn channel_max() -> f64 { 1f64 }
    
    static pure fn from_channel<T:Channel>(val: T) -> f64 { val.to_channel_f64() }
    pure fn convert_channel<T:Channel>() -> T { from_channel(self) }
    
    pure fn to_channel_u8()    -> u8    { self * (0xFFu8    as f64) as u8  }
    pure fn to_channel_u16()   -> u16   { self * (0xFFFFu16 as f64) as u16 }
    pure fn to_channel_u32()   -> u32   { fail(~"to_channel_u32 not yet implemented for f64") }
    pure fn to_channel_u64()   -> u64   { fail(~"to_channel_u64 not yet implemented for f64") }
    pure fn to_channel_f32()   -> f32   { self as f32 }
    pure fn to_channel_f64()   -> f64   { self }
    pure fn to_channel_float() -> float { self as float }
    
    pure fn inverse() -> f64 { 1.0 - self }
}

pub impl float: Channel {
    static pure fn channel_max() -> float { 1f }
    
    static pure fn from_channel<T:Channel>(val: T) -> float { val.to_channel_float() }
    pure fn convert_channel<T:Channel>() -> T { from_channel(self) }
    
    pure fn to_channel_u8()    -> u8    { self * (0xFFu8    as float) as u8  }
    pure fn to_channel_u16()   -> u16   { self * (0xFFFFu16 as float) as u16 }
    pure fn to_channel_u32()   -> u32   { fail(~"to_channel_u32 not yet implemented for float") }
    pure fn to_channel_u64()   -> u64   { fail(~"to_channel_u64 not yet implemented for float") }
    pure fn to_channel_f32()   -> f32   { self as f32 }
    pure fn to_channel_f64()   -> f64   { self as f64 }
    pure fn to_channel_float() -> float { self }
    
    pure fn inverse() -> float { 1.0 - self }
}