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

use std::cast;

use color::{Channel, ToChannel};
use color::{FloatChannel, ToFloatChannel};
use color::{HSV, ToHSV, RGB, ToRGB, RGBA, ToRGBA};

#[path = "../num_macros.rs"]
mod num_macros;

#[deriving(Clone, Eq)]
pub struct HSVA<T> { h: T, s: T, v: T, a: T }

impl<T> HSVA<T> {
    #[inline]
    pub fn new(h: T, s: T, v: T, a: T) -> HSVA<T> {
        HSVA { h: h, s: s, v: v, a: a }
    }
}

pub trait ToHSVA {
    pub fn to_hsva<U:Clone + FloatChannel>(&self) -> HSVA<U>;
}

impl<C: ToHSV, T:Clone + ToFloatChannel> ToHSVA for (C, T) {
    #[inline]
    pub fn to_hsva<U:Clone + FloatChannel>(&self) -> HSVA<U> {
        match *self {
            (ref c, ref a) => unsafe {
                cast::transmute::<(HSV<U>, U), HSVA<U>>(
                    (c.to_hsv::<U>(), FloatChannel::from(a.clone()))
                )
            }
        }
    }
}

impl<T:Clone + Float + ToChannel> ToRGBA for HSVA<T> {
    #[inline]
    pub fn to_rgba<U:Clone + Channel>(&self) -> RGBA<U> {
        match unsafe {
            cast::transmute::<&HSVA<T>, &(HSV<T>, T)>(self)
        } {
            &(ref c, ref a) => unsafe {
                cast::transmute::<(RGB<U>, U), RGBA<U>>(
                    (c.to_rgb::<U>(), Channel::from(a.clone()))
                )
            },
        }
    }
}
