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

//! Axis-aligned bounding boxes

use math::*;

#[deriving(Clone, Eq)]
pub struct AABB2<T> {
    center: Point2<T>,
    size: Vec2<T>,
}

impl_approx!(AABB2 { center, size })

impl<T> AABB2<T> {
    #[inline]
    pub fn new(center: Point2<T>, size: Vec2<T>) -> AABB2<T> {
        AABB2 { center: center, size: size }
    }
}

impl<T:Clone + Float> AABB2<T> {
    pub fn from_bounds(mn: Point2<T>, mx: Point2<T>) -> AABB2<T> {
        AABB2 {
            center: Point2::from_vec2(
                mn.as_vec2().add_v(mx.as_vec2()).div_s(two!(T))
            ),
            size: mx - mn,
        }
    }
}

#[deriving(Clone, Eq)]
pub struct AABB3<T> {
    center: Point3<T>,
    size: Vec3<T>,
}

impl_approx!(AABB3 { center, size })

impl<T> AABB3<T> {
    #[inline]
    pub fn new(center: Point3<T>, size: Vec3<T>) -> AABB3<T> {
        AABB3 { center: center, size: size }
    }
}

impl<T:Clone + Float> AABB3<T> {
    pub fn from_bounds(mn: Point3<T>, mx: Point3<T>) -> AABB3<T> {
        AABB3 {
            center: Point3::from_vec3(
                mn.as_vec3().add_v(mx.as_vec3()).div_s(two!(T))
            ),
            size: mx - mn,
        }
    }
}
