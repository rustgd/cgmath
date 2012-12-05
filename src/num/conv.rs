/**
 * Wrapper methods for numeric type casts
 *
 * Note: although the `from` and `cast` medods are still inclueded, they will
 * be phased out in favour of the `num::kinds::Number::from<T:Number(n: T)`
 * static constructor method. I feel that this will be far clearer than using
 * the `cast` function everywhere.
 */
pub trait NumConv {
    // TODO: the plan is to remove the `from` and `cast` methods and use the 
    // `num::kinds::Number::from<T:Number(n: T)` method instead
    static pure fn from<T:NumConv>(n: T) -> self;
    pure fn cast<T:NumConv>(&self) -> T;
    
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
pub pure fn cast<T:NumConv, U:NumConv>(n: T) -> U { n.cast() }

pub impl u8: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> u8 { n.to_u8() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl u16: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> u16 { n.to_u16() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl u32: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> u32 { n.to_u32() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl u64: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> u64 { n.to_u64() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl uint: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> uint { n.to_uint() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl i8: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> i8 { n.to_i8() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl i16: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> i16 { n.to_i16() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl i32: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> i32 { n.to_i32() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl i64: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> i64 { n.to_i64() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl int: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> int { n.to_int() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl f32: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> f32 { n.to_f32() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl f64: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> f64 { n.to_f64() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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

pub impl float: NumConv {
    #[inline(always)] static pure fn from<T:NumConv>(n: T) -> float { n.to_float() }
    #[inline(always)] pure fn cast<T:NumConv>(&self) -> T { NumConv::from(*self) }
    
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