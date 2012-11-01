trait UnSigned {} // Needs trait inheritence to work properly

pub impl u8:   UnSigned {}
pub impl u16:  UnSigned {}
pub impl u32:  UnSigned {}
pub impl u64:  UnSigned {}
pub impl uint: UnSigned {}


trait Signed {} // Needs trait inheritence to work properly

pub impl<T: Neg<T>> T: Signed {}


trait Integer {} // Needs trait inheritence to work properly

pub impl u8:   Integer {}
pub impl u16:  Integer {}
pub impl u32:  Integer {}
pub impl u64:  Integer {}
pub impl uint: Integer {}

pub impl i8:   Integer {}
pub impl i16:  Integer {}
pub impl i32:  Integer {}
pub impl i64:  Integer {}
pub impl int:  Integer {}


trait FloatingPoint {} // Needs trait inheritence to work properly

pub impl f32:   FloatingPoint {}
pub impl f64:   FloatingPoint {}
pub impl float: FloatingPoint {}


pub impl u8    : Add<u8,u8>        { #[inline(always)] pure fn add(rhs: &u8)    -> u8    { self + *rhs } }
pub impl u16   : Add<u16,u16>      { #[inline(always)] pure fn add(rhs: &u16)   -> u16   { self + *rhs } }
pub impl u32   : Add<u32,u32>      { #[inline(always)] pure fn add(rhs: &u32)   -> u32   { self + *rhs } }
pub impl u64   : Add<u64,u64>      { #[inline(always)] pure fn add(rhs: &u64)   -> u64   { self + *rhs } }
pub impl uint  : Add<uint,uint>    { #[inline(always)] pure fn add(rhs: &uint)  -> uint  { self + *rhs } }

pub impl i8    : Add<i8,i8>        { #[inline(always)] pure fn add(rhs: &i8)    -> i8    { self + *rhs } }
pub impl i16   : Add<i16,i16>      { #[inline(always)] pure fn add(rhs: &i16)   -> i16   { self + *rhs } }
pub impl i32   : Add<i32,i32>      { #[inline(always)] pure fn add(rhs: &i32)   -> i32   { self + *rhs } }
pub impl i64   : Add<i64,i64>      { #[inline(always)] pure fn add(rhs: &i64)   -> i64   { self + *rhs } }
pub impl int   : Add<int,int>      { #[inline(always)] pure fn add(rhs: &int)   -> int   { self + *rhs } }

pub impl f32   : Add<f32,f32>      { #[inline(always)] pure fn add(rhs: &f32)   -> f32   { self + *rhs } }
pub impl f64   : Add<f64,f64>      { #[inline(always)] pure fn add(rhs: &f64)   -> f64   { self + *rhs } }
pub impl float : Add<float,float>  { #[inline(always)] pure fn add(rhs: &float) -> float { self + *rhs } }


pub impl u8    : Sub<u8,u8>        { #[inline(always)] pure fn sub(rhs: &u8)    -> u8    { self - *rhs } }
pub impl u16   : Sub<u16,u16>      { #[inline(always)] pure fn sub(rhs: &u16)   -> u16   { self - *rhs } }
pub impl u32   : Sub<u32,u32>      { #[inline(always)] pure fn sub(rhs: &u32)   -> u32   { self - *rhs } }
pub impl u64   : Sub<u64,u64>      { #[inline(always)] pure fn sub(rhs: &u64)   -> u64   { self - *rhs } }
pub impl uint  : Sub<uint,uint>    { #[inline(always)] pure fn sub(rhs: &uint)  -> uint  { self - *rhs } }

pub impl i8    : Sub<i8,i8>        { #[inline(always)] pure fn sub(rhs: &i8)    -> i8    { self - *rhs } }
pub impl i16   : Sub<i16,i16>      { #[inline(always)] pure fn sub(rhs: &i16)   -> i16   { self - *rhs } }
pub impl i32   : Sub<i32,i32>      { #[inline(always)] pure fn sub(rhs: &i32)   -> i32   { self - *rhs } }
pub impl i64   : Sub<i64,i64>      { #[inline(always)] pure fn sub(rhs: &i64)   -> i64   { self - *rhs } }
pub impl int   : Sub<int,int>      { #[inline(always)] pure fn sub(rhs: &int)   -> int   { self - *rhs } }

pub impl f32   : Sub<f32,f32>      { #[inline(always)] pure fn sub(rhs: &f32)   -> f32   { self - *rhs } }
pub impl f64   : Sub<f64,f64>      { #[inline(always)] pure fn sub(rhs: &f64)   -> f64   { self - *rhs } }
pub impl float : Sub<float,float>  { #[inline(always)] pure fn sub(rhs: &float) -> float { self - *rhs } }


pub impl u8    : Mul<u8,u8>        { #[inline(always)] pure fn mul(rhs: &u8)    -> u8    { self * *rhs } }
pub impl u16   : Mul<u16,u16>      { #[inline(always)] pure fn mul(rhs: &u16)   -> u16   { self * *rhs } }
pub impl u32   : Mul<u32,u32>      { #[inline(always)] pure fn mul(rhs: &u32)   -> u32   { self * *rhs } }
pub impl u64   : Mul<u64,u64>      { #[inline(always)] pure fn mul(rhs: &u64)   -> u64   { self * *rhs } }
pub impl uint  : Mul<uint,uint>    { #[inline(always)] pure fn mul(rhs: &uint)  -> uint  { self * *rhs } }

pub impl i8    : Mul<i8,i8>        { #[inline(always)] pure fn mul(rhs: &i8)    -> i8    { self * *rhs } }
pub impl i16   : Mul<i16,i16>      { #[inline(always)] pure fn mul(rhs: &i16)   -> i16   { self * *rhs } }
pub impl i32   : Mul<i32,i32>      { #[inline(always)] pure fn mul(rhs: &i32)   -> i32   { self * *rhs } }
pub impl i64   : Mul<i64,i64>      { #[inline(always)] pure fn mul(rhs: &i64)   -> i64   { self * *rhs } }
pub impl int   : Mul<int,int>      { #[inline(always)] pure fn mul(rhs: &int)   -> int   { self * *rhs } }

pub impl f32   : Mul<f32,f32>      { #[inline(always)] pure fn mul(rhs: &f32)   -> f32   { self * *rhs } }
pub impl f64   : Mul<f64,f64>      { #[inline(always)] pure fn mul(rhs: &f64)   -> f64   { self * *rhs } }
pub impl float : Mul<float,float>  { #[inline(always)] pure fn mul(rhs: &float) -> float { self * *rhs } }


pub impl u8    : Div<u8,u8>        { #[inline(always)] pure fn div(rhs: &u8)    -> u8    { self / *rhs } }
pub impl u16   : Div<u16,u16>      { #[inline(always)] pure fn div(rhs: &u16)   -> u16   { self / *rhs } }
pub impl u32   : Div<u32,u32>      { #[inline(always)] pure fn div(rhs: &u32)   -> u32   { self / *rhs } }
pub impl u64   : Div<u64,u64>      { #[inline(always)] pure fn div(rhs: &u64)   -> u64   { self / *rhs } }
pub impl uint  : Div<uint,uint>    { #[inline(always)] pure fn div(rhs: &uint)  -> uint  { self / *rhs } }

pub impl i8    : Div<i8,i8>        { #[inline(always)] pure fn div(rhs: &i8)    -> i8    { self / *rhs } }
pub impl i16   : Div<i16,i16>      { #[inline(always)] pure fn div(rhs: &i16)   -> i16   { self / *rhs } }
pub impl i32   : Div<i32,i32>      { #[inline(always)] pure fn div(rhs: &i32)   -> i32   { self / *rhs } }
pub impl i64   : Div<i64,i64>      { #[inline(always)] pure fn div(rhs: &i64)   -> i64   { self / *rhs } }
pub impl int   : Div<int,int>      { #[inline(always)] pure fn div(rhs: &int)   -> int   { self / *rhs } }

pub impl f32   : Div<f32,f32>      { #[inline(always)] pure fn div(rhs: &f32)   -> f32   { self / *rhs } }
pub impl f64   : Div<f64,f64>      { #[inline(always)] pure fn div(rhs: &f64)   -> f64   { self / *rhs } }
pub impl float : Div<float,float>  { #[inline(always)] pure fn div(rhs: &float) -> float { self / *rhs } }


pub impl u8    : Modulo<u8,u8>        { #[inline(always)] pure fn modulo(rhs: &u8)    -> u8    { self % *rhs } }
pub impl u16   : Modulo<u16,u16>      { #[inline(always)] pure fn modulo(rhs: &u16)   -> u16   { self % *rhs } }
pub impl u32   : Modulo<u32,u32>      { #[inline(always)] pure fn modulo(rhs: &u32)   -> u32   { self % *rhs } }
pub impl u64   : Modulo<u64,u64>      { #[inline(always)] pure fn modulo(rhs: &u64)   -> u64   { self % *rhs } }
pub impl uint  : Modulo<uint,uint>    { #[inline(always)] pure fn modulo(rhs: &uint)  -> uint  { self % *rhs } }

pub impl i8    : Modulo<i8,i8>        { #[inline(always)] pure fn modulo(rhs: &i8)    -> i8    { self % *rhs } }
pub impl i16   : Modulo<i16,i16>      { #[inline(always)] pure fn modulo(rhs: &i16)   -> i16   { self % *rhs } }
pub impl i32   : Modulo<i32,i32>      { #[inline(always)] pure fn modulo(rhs: &i32)   -> i32   { self % *rhs } }
pub impl i64   : Modulo<i64,i64>      { #[inline(always)] pure fn modulo(rhs: &i64)   -> i64   { self % *rhs } }
pub impl int   : Modulo<int,int>      { #[inline(always)] pure fn modulo(rhs: &int)   -> int   { self % *rhs } }

pub impl f32   : Modulo<f32,f32>      { #[inline(always)] pure fn modulo(rhs: &f32)   -> f32   { self % *rhs } }
pub impl f64   : Modulo<f64,f64>      { #[inline(always)] pure fn modulo(rhs: &f64)   -> f64   { self % *rhs } }
pub impl float : Modulo<float,float>  { #[inline(always)] pure fn modulo(rhs: &float) -> float { self % *rhs } }


pub impl i8    : Neg<i8>    { #[inline(always)] pure fn neg() -> i8    { -self } }
pub impl i16   : Neg<i16>   { #[inline(always)] pure fn neg() -> i16   { -self } }
pub impl i32   : Neg<i32>   { #[inline(always)] pure fn neg() -> i32   { -self } }
pub impl i64   : Neg<i64>   { #[inline(always)] pure fn neg() -> i64   { -self } }
pub impl int   : Neg<int>   { #[inline(always)] pure fn neg() -> int   { -self } }

pub impl f32   : Neg<f32>   { #[inline(always)] pure fn neg() -> f32   { -self } }
pub impl f64   : Neg<f64>   { #[inline(always)] pure fn neg() -> f64   { -self } }
pub impl float : Neg<float> { #[inline(always)] pure fn neg() -> float { -self } }


trait NumCast {
    static pure fn from<T:NumCast>(n: T) -> self;
    pure fn cast<T:NumCast>() -> T;
    
    pure fn cast_u8()    -> u8;
    pure fn cast_u16()   -> u16;
    pure fn cast_u32()   -> u32;
    pure fn cast_u64()   -> u64;
    pure fn cast_uint()  ->  uint;
    
    pure fn cast_i8()    -> i8;
    pure fn cast_i16()   -> i16;
    pure fn cast_i32()   -> i32;
    pure fn cast_i64()   -> i64;
    pure fn cast_int()   -> int;
    
    pure fn cast_f32()   -> f32;
    pure fn cast_f64()   -> f64;
    pure fn cast_float() -> float;
}

#[inline(always)]
pub pure fn cast<T:NumCast, U:NumCast>(n: T) -> U { n.cast() }

pub impl u8: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u8 { n.cast_u8() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self          }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl u16: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u16 { n.cast_u16() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self          }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl u32: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u32 { n.cast_u32() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self          }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl u64: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> u64 { n.cast_u64() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self          }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl uint: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> uint { n.cast_uint() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self          }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl i8: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i8 { n.cast_i8() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self          }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl i16: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i16 { n.cast_i16() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self          }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl i32: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i32 { n.cast_i32() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self          }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl i64: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> i64 { n.cast_i64() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self          }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl int: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> int { n.cast_int() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self          }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl f32: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> f32 { n.cast_f32() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self          }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl f64: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> f64 { n.cast_f64() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self          }
    #[inline(always)] pure fn cast_float() -> float { self as float }
}

pub impl float: NumCast {
    #[inline(always)] static pure fn from<T:NumCast>(n: T) -> float { n.cast_float() }
    #[inline(always)] pure fn cast<T:NumCast>() -> T { from(self) }
    
    #[inline(always)] pure fn cast_u8()    -> u8    { self as u8    }
    #[inline(always)] pure fn cast_u16()   -> u16   { self as u16   }
    #[inline(always)] pure fn cast_u32()   -> u32   { self as u32   }
    #[inline(always)] pure fn cast_u64()   -> u64   { self as u64   }
    #[inline(always)] pure fn cast_uint()  -> uint  { self as uint  }
    
    #[inline(always)] pure fn cast_i8()    -> i8    { self as i8    }
    #[inline(always)] pure fn cast_i16()   -> i16   { self as i16   }
    #[inline(always)] pure fn cast_i32()   -> i32   { self as i32   }
    #[inline(always)] pure fn cast_i64()   -> i64   { self as i64   }
    #[inline(always)] pure fn cast_int()   -> int   { self as int   }
    
    #[inline(always)] pure fn cast_f32()   -> f32   { self as f32   }
    #[inline(always)] pure fn cast_f64()   -> f64   { self as f64   }
    #[inline(always)] pure fn cast_float() -> float { self          }
}