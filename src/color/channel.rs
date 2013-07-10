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
    priv fn from<T:Channel>(chan: T) -> Self;
    pub fn to_channel<T:Channel>(&self) -> T;
    pub fn to_channel_u8(&self)  -> u8;
    pub fn to_channel_u16(&self) -> u16;
    pub fn to_channel_f32(&self) -> f32;
    pub fn to_channel_f64(&self) -> f64;
}

impl Channel for u8 {
    #[inline] priv fn from<T:Channel>(chan: T) -> u8 { chan.to_channel_u8() }
    #[inlune] pub fn to_channel<T:Channel>(&self) -> T { Channel::from(*self) }
    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self) }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self as u16 << 8) | (*self) as u16 }
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self as f32) / (0xFF as f32) }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self as f64) / (0xFF as f64) }
}

impl Channel for u16 {
    #[inline] priv fn from<T:Channel>(chan: T) -> u16 { chan.to_channel_u16() }
    #[inlune] pub fn to_channel<T:Channel>(&self) -> T { Channel::from(*self) }
    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self >> 8) as u8 }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self) }
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self) / 0xFFFF as f32 }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self) / 0xFFFF as f64 }
}

impl Channel for f32 {
    #[inline] priv fn from<T:Channel>(chan: T) -> f32 { chan.to_channel_f32() }
    #[inlune] pub fn to_channel<T:Channel>(&self) -> T { Channel::from(*self) }
    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self) * (0xFF_u8 as f32) as u8 }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self) * (0xFFFF_u16 as f32) as u16 }
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self) }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self) as f64 }
}

impl Channel for f64 {
    #[inline] priv fn from<T:Channel>(chan: T) -> f64 { chan.to_channel_f64() }
    #[inlune] pub fn to_channel<T:Channel>(&self) -> T { Channel::from(*self) }
    #[inline] pub fn to_channel_u8(&self)  -> u8  { (*self) * (0xFF_u8 as f64) as u8 }
    #[inline] pub fn to_channel_u16(&self) -> u16 { (*self) * (0xFFFF_u16 as f64) as u16 }
    #[inline] pub fn to_channel_f32(&self) -> f32 { (*self) as f32 }
    #[inline] pub fn to_channel_f64(&self) -> f64 { (*self) }
}

#[cfg(test)]
mod tests {
    use color::Channel;

    #[test]
    fn test_channel_u8() {
        assert_eq!(0x00_u8.to_channel_u8(), 0x00_u8);
        assert_eq!(0x30_u8.to_channel_u8(), 0x30_u8);
        assert_eq!(0x66_u8.to_channel_u8(), 0x66_u8);
        assert_eq!(0xA0_u8.to_channel_u8(), 0xA0_u8);
        assert_eq!(0xFF_u8.to_channel_u8(), 0xFF_u8);

        assert_eq!(0x00_u8.to_channel_u16(), 0x0000_u16);
        assert_eq!(0x30_u8.to_channel_u16(), 0x3030_u16);
        assert_eq!(0x66_u8.to_channel_u16(), 0x6666_u16);
        assert_eq!(0xA0_u8.to_channel_u16(), 0xA0A0_u16);
        assert_eq!(0xFF_u8.to_channel_u16(), 0xFFFF_u16);

        assert_eq!(0x00_u8.to_channel_f32(), 0f32);
        assert_eq!(0xFF_u8.to_channel_f32(), 1f32);

        assert_eq!(0x00_u8.to_channel_f64(), 0f64);
        assert_eq!(0xFF_u8.to_channel_f64(), 1f64);
    }

    #[test]
    fn test_channel_u16() {
        assert_eq!(0x0000_u16.to_channel_u8(), 0x00_u8);
        assert_eq!(0x3300_u16.to_channel_u8(), 0x33_u8);
        assert_eq!(0x6666_u16.to_channel_u8(), 0x66_u8);
        assert_eq!(0xAA00_u16.to_channel_u8(), 0xAA_u8);
        assert_eq!(0xFFFF_u16.to_channel_u8(), 0xFF_u8);

        assert_eq!(0x0000_u16.to_channel_u16(), 0x0000_u16);
        assert_eq!(0x3300_u16.to_channel_u16(), 0x3300_u16);
        assert_eq!(0x6666_u16.to_channel_u16(), 0x6666_u16);
        assert_eq!(0xAA00_u16.to_channel_u16(), 0xAA00_u16);
        assert_eq!(0xFFFF_u16.to_channel_u16(), 0xFFFF_u16);

        assert_eq!(0x0000_u16.to_channel_f32(), 0f32);
        assert_eq!(0xFFFF_u16.to_channel_f32(), 1f32);

        assert_eq!(0x0000_u16.to_channel_f64(), 0f64);
        assert_eq!(0xFFFF_u16.to_channel_f64(), 1f64);
    }

    #[test]
    fn test_channel_f32() {
        assert_eq!(0.00f32.to_channel_u8(), 0x00);
        assert_eq!(0.25f32.to_channel_u8(), 0x3F);
        assert_eq!(0.50f32.to_channel_u8(), 0x7F);
        assert_eq!(0.75f32.to_channel_u8(), 0xBF);
        assert_eq!(1.00f32.to_channel_u8(), 0xFF);

        assert_eq!(0.00f32.to_channel_u16(), 0x0000);
        assert_eq!(0.25f32.to_channel_u16(), 0x3FFF);
        assert_eq!(0.50f32.to_channel_u16(), 0x7FFF);
        assert_eq!(0.75f32.to_channel_u16(), 0xBFFF);
        assert_eq!(1.00f32.to_channel_u16(), 0xFFFF);

        assert_eq!(0.00f32.to_channel_f32(), 0.00f32);
        assert_eq!(1.00f32.to_channel_f32(), 1.00f32);

        assert_eq!(0.00f32.to_channel_f64(), 0.00f64);
        assert_eq!(1.00f32.to_channel_f64(), 1.00f64);
    }

    #[test]
    fn test_channel_f64() {
        assert_eq!(0.00f64.to_channel_u8(), 0x00);
        assert_eq!(0.25f64.to_channel_u8(), 0x3F);
        assert_eq!(0.50f64.to_channel_u8(), 0x7F);
        assert_eq!(0.75f64.to_channel_u8(), 0xBF);
        assert_eq!(1.00f64.to_channel_u8(), 0xFF);

        assert_eq!(0.00f64.to_channel_u16(), 0x0000);
        assert_eq!(0.25f64.to_channel_u16(), 0x3FFF);
        assert_eq!(0.50f64.to_channel_u16(), 0x7FFF);
        assert_eq!(0.75f64.to_channel_u16(), 0xBFFF);
        assert_eq!(1.00f64.to_channel_u16(), 0xFFFF);

        assert_eq!(0.00f64.to_channel_f32(), 0.00f32);
        assert_eq!(1.00f64.to_channel_f32(), 1.00f32);

        assert_eq!(0.00f64.to_channel_f64(), 0.00f64);
        assert_eq!(1.00f64.to_channel_f64(), 1.00f64);
    }
}
