use num::types::Number;

/**
 * A color channel
 */
pub trait Channel: Number {
    /**
     * The maximum value used by the channel
     */
    static pure fn max() -> self;
    
    /**
     * Convert a channel to the enclosing type
     *
     * # Example
     *
     * ~~~
     * let chan: f32 = Channel::from(0xFFFFu16);
     * assert chan == 1.0f32;
     * ~~~
     */
    static pure fn from<T:Channel>(val: T) -> self;
    
    pure fn to_channel_u8(&self)    -> u8;
    pure fn to_channel_u16(&self)   -> u16;
    pure fn to_channel_u32(&self)   -> u32;
    pure fn to_channel_u64(&self)   -> u64;
    pure fn to_channel_f32(&self)   -> f32;
    pure fn to_channel_f64(&self)   -> f64;
    pure fn to_channel_float(&self) -> float;
    
    pure fn inverse(&self) -> self;
}

pub impl u8: Channel {
    #[inline(always)] static pure fn max() -> u8 { 0xFF }  // 2^8
    
    #[inline(always)] static pure fn from<T:Channel>(val: T) -> u8 { val.to_channel_u8() }
    
    #[inline(always)] pure fn to_channel_u8(&self)    -> u8    { (*self) }
    #[inline(always)] pure fn to_channel_u16(&self)   -> u16   { (*self as u16 << 8) | (*self) as u16 }
    #[inline(always)] pure fn to_channel_u32(&self)   -> u32   { (self.to_channel_u16() as u32 << 16) | self.to_channel_u16() as u32 }
    #[inline(always)] pure fn to_channel_u64(&self)   -> u64   { (self.to_channel_u32() as u64 << 32) | self.to_channel_u32() as u64 }
    #[inline(always)] pure fn to_channel_f32(&self)   -> f32   { (*self as f32)   / (0xFF as f32)   }
    #[inline(always)] pure fn to_channel_f64(&self)   -> f64   { (*self as f64)   / (0xFF as f64)   }
    #[inline(always)] pure fn to_channel_float(&self) -> float { (*self as float) / (0xFF as float) }
    
    #[inline(always)] pure fn inverse(&self) -> u8 { !(*self) }
}

pub impl u16: Channel {
    #[inline(always)] static pure fn max() -> u16 { 0xFFFF }   // 2^16
    
    #[inline(always)] static pure fn from<T:Channel>(val: T) -> u16 { val.to_channel_u16() }
    
    #[inline(always)] pure fn to_channel_u8(&self)    -> u8    { (*self >> 8) as u8 }    // this is the equivalent of `self/256`. Some folks prefer to do `self/257`
    #[inline(always)] pure fn to_channel_u16(&self)   -> u16   { (*self) }
    #[inline(always)] pure fn to_channel_u32(&self)   -> u32   { (*self as u32 << 16) | (*self) as u32 }
    #[inline(always)] pure fn to_channel_u64(&self)   -> u64   { (self.to_channel_u32() as u64 << 32) | self.to_channel_u32() as u64 }
    #[inline(always)] pure fn to_channel_f32(&self)   -> f32   { (*self) / 0xFFFF as f32   }
    #[inline(always)] pure fn to_channel_f64(&self)   -> f64   { (*self) / 0xFFFF as f64   }
    #[inline(always)] pure fn to_channel_float(&self) -> float { (*self) / 0xFFFF as float }
    
    #[inline(always)] pure fn inverse(&self) -> u16 { !(*self) }
}

pub impl u32: Channel {
    #[inline(always)] static pure fn max() -> u32 { 0xFFFF_FFFF }   // 2^32
    
    #[inline(always)] static pure fn from<T:Channel>(val: T) -> u32 { val.to_channel_u32() }
    
    #[inline(always)] pure fn to_channel_u8(&self)    -> u8    { (*self >> 24) as u8 }
    #[inline(always)] pure fn to_channel_u16(&self)   -> u16   { (*self >> 16) as u16 }
    #[inline(always)] pure fn to_channel_u32(&self)   -> u32   { (*self) }
    #[inline(always)] pure fn to_channel_u64(&self)   -> u64   { (*self as u64 << 32) | (*self) as u64 }
    #[inline(always)] pure fn to_channel_f32(&self)   -> f32   { (*self) / 0xFFFF_FFFF as f32   }
    #[inline(always)] pure fn to_channel_f64(&self)   -> f64   { (*self) / 0xFFFF_FFFF as f64   }
    #[inline(always)] pure fn to_channel_float(&self) -> float { (*self) / 0xFFFF_FFFF as float }
    
    #[inline(always)] pure fn inverse(&self) -> u32 { !(*self) }
}

pub impl u64: Channel {
    #[inline(always)] static pure fn max() -> u64 { 0xFFFF_FFFF_FFFF_FFFF_u64 }   // 2^64
    
    #[inline(always)] static pure fn from<T:Channel>(val: T) -> u64 { val.to_channel_u64() }
    
    #[inline(always)] pure fn to_channel_u8(&self)    -> u8    { (*self >> 56) as u8 }
    #[inline(always)] pure fn to_channel_u16(&self)   -> u16   { (*self >> 48) as u16 }
    #[inline(always)] pure fn to_channel_u32(&self)   -> u32   { (*self >> 32) as u32 }
    #[inline(always)] pure fn to_channel_u64(&self)   -> u64   { (*self) }
    #[inline(always)] pure fn to_channel_f32(&self)   -> f32   { (*self) / 0xFFFF_FFFF_FFFF_FFFF_u64 as f32   }
    #[inline(always)] pure fn to_channel_f64(&self)   -> f64   { (*self) / 0xFFFF_FFFF_FFFF_FFFF_u64 as f64   }
    #[inline(always)] pure fn to_channel_float(&self) -> float { (*self) / 0xFFFF_FFFF_FFFF_FFFF_u64 as float }
    
    #[inline(always)] pure fn inverse(&self) -> u64 { !(*self) }
}

pub impl f32: Channel {
    #[inline(always)] static pure fn max() -> f32 { 1f32 }
    
    #[inline(always)] static pure fn from<T:Channel>(val: T) -> f32 { val.to_channel_f32() }
    
    
    #[inline(always)] pure fn to_channel_u8(&self)    -> u8    { (*self) * (0xFF_u8    as f32) as u8  }
    #[inline(always)] pure fn to_channel_u16(&self)   -> u16   { (*self) * (0xFFFF_u16 as f32) as u16 }
    #[inline(always)] pure fn to_channel_u32(&self)   -> u32   { fail(~"to_channel_u32 not yet implemented for f32") }
    #[inline(always)] pure fn to_channel_u64(&self)   -> u64   { fail(~"to_channel_u64 not yet implemented for f32") }
    #[inline(always)] pure fn to_channel_f32(&self)   -> f32   { (*self) }
    #[inline(always)] pure fn to_channel_f64(&self)   -> f64   { (*self) as f64 }
    #[inline(always)] pure fn to_channel_float(&self) -> float { (*self) as float }
    
    #[inline(always)] pure fn inverse(&self) -> f32 { 1f32 - (*self) }
}

pub impl f64: Channel {
    #[inline(always)] static pure fn max() -> f64 { 1f64 }
    
    #[inline(always)] static pure fn from<T:Channel>(val: T) -> f64 { val.to_channel_f64() }
    
    #[inline(always)] pure fn to_channel_u8(&self)    -> u8    { (*self) * (0xFF_u8    as f64) as u8  }
    #[inline(always)] pure fn to_channel_u16(&self)   -> u16   { (*self) * (0xFFFF_u16 as f64) as u16 }
    #[inline(always)] pure fn to_channel_u32(&self)   -> u32   { fail(~"to_channel_u32 not yet implemented for f64") }
    #[inline(always)] pure fn to_channel_u64(&self)   -> u64   { fail(~"to_channel_u64 not yet implemented for f64") }
    #[inline(always)] pure fn to_channel_f32(&self)   -> f32   { (*self) as f32 }
    #[inline(always)] pure fn to_channel_f64(&self)   -> f64   { (*self) }
    #[inline(always)] pure fn to_channel_float(&self) -> float { (*self) as float }
    
    #[inline(always)] pure fn inverse(&self) -> f64 { 1f64 - (*self) }
}

pub impl float: Channel {
    #[inline(always)] static pure fn max() -> float { 1f }
    
    #[inline(always)] static pure fn from<T:Channel>(val: T) -> float { val.to_channel_float() }
    
    #[inline(always)] pure fn to_channel_u8(&self)    -> u8    { (*self) * (0xFF_u8    as float) as u8  }
    #[inline(always)] pure fn to_channel_u16(&self)   -> u16   { (*self) * (0xFFFF_u16 as float) as u16 }
    #[inline(always)] pure fn to_channel_u32(&self)   -> u32   { fail(~"to_channel_u32 not yet implemented for float") }
    #[inline(always)] pure fn to_channel_u64(&self)   -> u64   { fail(~"to_channel_u64 not yet implemented for float") }
    #[inline(always)] pure fn to_channel_f32(&self)   -> f32   { (*self) as f32 }
    #[inline(always)] pure fn to_channel_f64(&self)   -> f64   { (*self) as f64 }
    #[inline(always)] pure fn to_channel_float(&self) -> float { (*self) }
    
    #[inline(always)] pure fn inverse(&self) -> float { 1f - (*self) }
}