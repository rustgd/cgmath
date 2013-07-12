// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directionectory of this distribution.
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

use core::{Vec2, Vec3};
use geom::{Point2, Point3};

#[deriving(Clone, Eq)]
pub struct Ray2<T> {
    origin: Point2<T>,
    direction: Vec2<T>,
}

impl_approx!(Ray2 { origin, direction })

impl<T> Ray2<T> {
    #[inline]
    pub fn new(origin: Point2<T>, direction: Vec2<T>) -> Ray2<T> {
        Ray2 { origin: origin, direction: direction }
    }
}


#[deriving(Clone, Eq)]
pub struct Ray3<T> {
    origin: Point3<T>,
    direction: Vec3<T>,
}

impl_approx!(Ray3 { origin, direction })

impl<T> Ray3<T> {
    #[inline]
    pub fn new(origin: Point3<T>, direction: Vec3<T>) -> Ray3<T> {
        Ray3 { origin: origin, direction: direction }
    }
}
