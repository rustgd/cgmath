// Copyright 2013 The CGMath Developers. For a full listing of the authors,
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

use point::{Point2, Point3};
use vector::{Vec2, Vec3};

#[deriving(Clone, Eq)]
pub struct Ray2<S> {
    origin: Point2<S>,
    direction: Vec2<S>,
}

impl<S> Ray2<S> {
    /// Creates a new ray from a position coordinate and a direction vector
    #[inline]
    pub fn new(origin: Point2<S>, direction: Vec2<S>) -> Ray2<S> {
        Ray2 { origin: origin, direction: direction }
    }
}

#[deriving(Clone, Eq)]
pub struct Ray3<S> {
    origin: Point3<S>,
    direction: Vec3<S>,
}

impl<S> Ray3<S> {
    /// Creates a new ray from a position coordinate and a direction vector
    #[inline]
    pub fn new(origin: Point3<S>, direction: Vec3<S>) -> Ray3<S> {
        Ray3 { origin: origin, direction: direction }
    }
}
