/**
 * This trait allows for the easy casting between each of thethe built in
 * numeric types, going above and beyond the static 'to_int' function found in
 * the Num trait. I've found that it is especially handy in generic functions
 * when you need to mix floating point and integer values.
 */
pub trait NumCast {
    static pure fn from<T:NumCast>(n: T) -> self;
    pure fn cast<T:NumCast>(&self) -> T;
    
    static pure fn zero() -> self;
    static pure fn one()  -> self;
    
    pure fn to_u8(&self)    -> u8;
    pure fn to_u16(&self)   -> u16;
    pure fn to_u32(&self)   -> u32;
    pure fn to_u64(&self)   -> u64;
    pure fn to_uint(&self)  -> uint;
    
    pure fn to_i8(&self)    -> i8;
    pure fn to_i16(&self)   -> i16;
    pure fn to_i32(&self)   -> i32;
    pure fn to_i64(&self)   -> i64;
    pure fn to_int(&self)   -> int;
    
    pure fn to_f32(&self)   -> f32;
    pure fn to_f64(&self)   -> f64;
    pure fn to_float(&self) -> float;
}

#[inline(always)]
pub pure fn cast<T:NumCast, U:NumCast>(n: T) -> U { n.cast() }

pub impl u8: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u8 { n.to_u8() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> u8 { 0u8 }
    static pure fn one()  -> u8 { 1u8 }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self          }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl u16: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u16 { n.to_u16() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> u16 { 0u16 }
    static pure fn one()  -> u16 { 1u16 }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self          }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl u32: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u32 { n.to_u32() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> u32 { 0u32 }
    static pure fn one()  -> u32 { 1u32 }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self          }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl u64: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u64 { n.to_u64() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> u64 { 0u64 }
    static pure fn one()  -> u64 { 1u64 }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self          }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl uint: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> uint { n.to_uint() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> uint { 0u }
    static pure fn one()  -> uint { 1u }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self          }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl i8: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i8 { n.to_i8() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> i8 { 0i8 }
    static pure fn one()  -> i8 { 1i8 }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self          }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl i16: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i16 { n.to_i16() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> i16 { 0i16 }
    static pure fn one()  -> i16 { 1i16 }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self          }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl i32: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i32 { n.to_i32() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> i32 { 0i32 }
    static pure fn one()  -> i32 { 1i32 }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self          }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl i64: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i64 { n.to_i64() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> i64 { 0i64 }
    static pure fn one()  -> i64 { 1i64 }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self          }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl int: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> int { n.to_int() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> int { 0 }
    static pure fn one()  -> int { 1 }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self          }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl f32: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> f32 { n.to_f32() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> f32 { 0f32 }
    static pure fn one()  -> f32 { 1f32 }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self          }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl f64: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> f64 { n.to_f64() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> f64 { 0f64 }
    static pure fn one()  -> f64 { 1f64 }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self          }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

pub impl float: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> float { n.to_float() }
    #[inline(always)] pure fn cast<T:NumCast>(&self) -> T { NumCast::from(*self) }
    
    static pure fn zero() -> float { 0f }
    static pure fn one()  -> float { 1f }
    
    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }
    
    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }
    
    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self          }
}