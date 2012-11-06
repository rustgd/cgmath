/**
 * This trait allows for the easy casting between each of thethe built in
 * numeric types, going above and beyond the static 'to_int' function found in
 * the Num trait. I've found that it is especially handy in generic functions
 * when you need to mix floating point and integer values.
 */
trait NumCast {
    static pure fn from<T:NumCast>(n: T) -> self;
    pure fn cast<T:NumCast>() -> T;
    
    pure fn to_u8()    -> u8;
    pure fn to_u16()   -> u16;
    pure fn to_u32()   -> u32;
    pure fn to_u64()   -> u64;
    pure fn to_uint()  ->  uint;
    
    pure fn to_i8()    -> i8;
    pure fn to_i16()   -> i16;
    pure fn to_i32()   -> i32;
    pure fn to_i64()   -> i64;
    pure fn to_int()   -> int;
    
    pure fn to_f32()   -> f32;
    pure fn to_f64()   -> f64;
    pure fn to_float() -> float;
}

#[inline(always)]
pub pure fn cast<T:NumCast, U:NumCast>(n: T) -> U { move n.cast() }

pub impl u8: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u8 { move n.to_u8() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self          }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl u16: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u16 { move n.to_u16() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self          }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl u32: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u32 { move n.to_u32() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self          }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl u64: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u64 { move n.to_u64() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self          }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl uint: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> uint { move n.to_uint() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self          }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl i8: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i8 { move n.to_i8() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self          }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl i16: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i16 { move n.to_i16() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self          }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl i32: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i32 { move n.to_i32() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self          }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl i64: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i64 { move n.to_i64() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self          }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl int: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> int { move n.to_int() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self          }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl f32: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> f32 { move n.to_f32() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self          }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl f64: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> f64 { move n.to_f64() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self          }
    #[inline(always)] pure fn to_float() -> float { self as float }
}

pub impl float: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> float { move n.to_float() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { move from(self) }
    
    #[inline(always)] pure fn to_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn to_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn to_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn to_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn to_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn to_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn to_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn to_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn to_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn to_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn to_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn to_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn to_float() -> float { self          }
}