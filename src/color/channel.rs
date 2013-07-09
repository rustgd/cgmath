// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// A color channel
pub trait Channel: Num + ToChannel{
    /// Convert a channel to the enclosing type
    ///
    /// # Example
    ///
    /// ~~~
    /// let chan: f32 = Channel::from(0xFFFFu16);
    /// assert chan == 1.0f32;
    /// ~~~
    pub fn from<T: ToChannel>(val: T) -> Self;
}

impl Channel for u8 {
    #[inline] pub fn from<T: ToChannel>(val: T) -> u8 { val.to_channel_u8() }
}

impl Channel for u16 {
    #[inline] pub fn from<T: ToChannel>(val: T) -> u16 { val.to_channel_u16() }
}

impl Channel for u32 {
    #[inline] pub fn from<T: ToChannel>(val: T) -> u32 { val.to_channel_u32() }
}

impl Channel for u64 {
    #[inline] pub fn from<T: ToChannel>(val: T) -> u64 { val.to_channel_u64() }
}

impl Channel for f32 {
    #[inline] pub fn from<T: ToChannel>(val: T) -> f32 { val.to_channel_f32() }
}

impl Channel for f64 {
    #[inline] pub fn from<T: ToChannel>(val: T) -> f64 { val.to_channel_f64() }
}

pub trait ToChannel: ToIntChannel + ToFloatChannel {}

impl ToChannel for u8 {}

impl ToChannel for u16 {}

impl ToChannel for u32 {}

impl ToChannel for u64 {}

impl ToChannel for f32 {}

impl ToChannel for f64 {}

pub trait IntChannel: Int + Channel + ToIntChannel {
    pub fn from<T:ToIntChannel>(val: T) -> Self;
}

impl IntChannel for u8 {
    #[inline] pub fn from<T:ToIntChannel>(val: T) -> u8 { val.to_channel_u8() }
}

impl IntChannel for u16 {
    #[inline] pub fn from<T:ToIntChannel>(val: T) -> u16 { val.to_channel_u16() }
}

impl IntChannel for u32 {
    #[inline] pub fn from<T:ToIntChannel>(val: T) -> u32 { val.to_channel_u32() }
}

impl IntChannel for u64 {
    #[inline] pub fn from<T:ToIntChannel>(val: T) -> u64 { val.to_channel_u64() }
}

pub trait ToIntChannel {
    pub fn to_channel_u8(&self)  -> u8;
    pub fn to_channel_u16(&self) -> u16;
    pub fn to_channel_u32(&self) -> u32;
    pub fn to_channel_u64(&self) -> u64;
}

impl ToIntChannel for u8 {
    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self) }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self as u16 << 8) | (*self) as u16 }
    #[inline] pub fn to_channel_u32(&self) -> u32 { (self.to_channel_u16() as u32 << 16) | self.to_channel_u16() as u32 }
    #[inline] pub fn to_channel_u64(&self) -> u64 { (self.to_channel_u32() as u64 << 32) | self.to_channel_u32() as u64 }
}

impl ToIntChannel for u16 {
    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self >> 8) as u8 } // this is the equivalent of `self/256`. Some folks prefer to do `self/257`
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self) }
    #[inline] pub fn to_channel_u32(&self) -> u32 { (*self as u32 << 16) | (*self) as u32 }
    #[inline] pub fn to_channel_u64(&self) -> u64 { (self.to_channel_u32() as u64 << 32) | self.to_channel_u32() as u64 }
}

impl ToIntChannel for u32 {
    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self >> 24) as u8 }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self >> 16) as u16 }
    #[inline] pub fn to_channel_u32(&self) -> u32 { (*self) }
    #[inline] pub fn to_channel_u64(&self) -> u64 { (*self as u64 << 32) | (*self) as u64 }
}

impl ToIntChannel for u64 {
    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self >> 56) as u8 }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self >> 48) as u16 }
    #[inline] pub fn to_channel_u32(&self) -> u32 { (*self >> 32) as u32 }
    #[inline] pub fn to_channel_u64(&self) -> u64 { (*self) }
}

impl ToIntChannel for f32 {
    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self) * (0xFF_u8    as f32) as u8  }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self) * (0xFFFF_u16 as f32) as u16 }
    #[inline] pub fn to_channel_u32(&self) -> u32 { fail!(~"to_channel_u32 not yet implemented for f32") }
    #[inline] pub fn to_channel_u64(&self) -> u64 { fail!(~"to_channel_u64 not yet implemented for f32") }
}

impl ToIntChannel for f64 {
    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self) * (0xFF_u8    as f64) as u8  }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self) * (0xFFFF_u16 as f64) as u16 }
    #[inline] pub fn to_channel_u32(&self) -> u32 { fail!(~"to_channel_u32 not yet implemented for f64") }
    #[inline] pub fn to_channel_u64(&self) -> u64 { fail!(~"to_channel_u64 not yet implemented for f64") }
}

impl ToIntChannel for float {
    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self) * (0xFF_u8    as float) as u8  }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self) * (0xFFFF_u16 as float) as u16 }
    #[inline] pub fn to_channel_u32(&self) -> u32 { fail!(~"to_channel_u32 not yet implemented for float") }
    #[inline] pub fn to_channel_u64(&self) -> u64 { fail!(~"to_channel_u64 not yet implemented for float") }
}

pub trait FloatChannel: Float + Channel + ToFloatChannel {
    pub fn from<T:ToFloatChannel>(val: T) -> Self;
}

impl FloatChannel for f32 {
    #[inline] pub fn from<T:ToFloatChannel>(val: T) -> f32 { val.to_channel_f32() }
}

impl FloatChannel for f64 {
    #[inline] pub fn from<T:ToFloatChannel>(val: T) -> f64 { val.to_channel_f64() }
}

pub trait ToFloatChannel {
    pub fn to_channel_f32(&self) -> f32;
    pub fn to_channel_f64(&self) -> f64;
}

impl ToFloatChannel for u8 {
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self as f32) / (0xFF as f32) }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self as f64) / (0xFF as f64) }
}

impl ToFloatChannel for u16 {
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self) / 0xFFFF as f32 }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self) / 0xFFFF as f64 }
}

impl ToFloatChannel for u32 {
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self) / 0xFFFF_FFFF as f32 }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self) / 0xFFFF_FFFF as f64 }
}

impl ToFloatChannel for u64 {
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self) / 0xFFFF_FFFF_FFFF_FFFF_u64 as f32 }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self) / 0xFFFF_FFFF_FFFF_FFFF_u64 as f64 }
}

impl ToFloatChannel for f32 {
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self) }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self) as f64 }
}

impl ToFloatChannel for f64 {
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self) as f32 }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self) }
}
