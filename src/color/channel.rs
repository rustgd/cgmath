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

pub trait Channel: Num {
    pub fn from<T:Channel>(chan: T) -> Self;

    pub fn to_channel_u8(&self)  -> u8;
    pub fn to_channel_u16(&self) -> u16;
    pub fn to_channel_f32(&self) -> f32;
    pub fn to_channel_f64(&self) -> f64;
}

impl Channel for u8 {
    #[inline] pub fn from<T:Channel>(chan: T) -> u8 { chan.to_channel_u8() }

    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self) }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self as u16 << 8) | (*self) as u16 }
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self as f32) / (0xFF as f32) }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self as f64) / (0xFF as f64) }
}

impl Channel for u16 {
    #[inline] pub fn from<T:Channel>(chan: T) -> u16 { chan.to_channel_u16() }

    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self >> 8) as u8 }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self) }
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self) / 0xFFFF as f32 }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self) / 0xFFFF as f64 }
}

impl Channel for f32 {
    #[inline] pub fn from<T:Channel>(chan: T) -> f32 { chan.to_channel_f32() }

    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self) * (0xFF_u8    as f32) as u8  }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self) * (0xFFFF_u16 as f32) as u16 }
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self) }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self) as f64 }
}

impl Channel for f64 {
    #[inline] pub fn from<T:Channel>(chan: T) -> f64 { chan.to_channel_f64() }

    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self) * (0xFF_u8    as f64) as u8  }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self) * (0xFFFF_u16 as f64) as u16 }
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self) as f32 }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self) }
}
