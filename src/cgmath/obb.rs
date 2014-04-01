// Copyright 2013 The CGMath Developers. For a full listing of the authors,
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

//! Oriented bounding boxes

use point::{Point2, Point3};
use vector::{Vec2, Vec3};

#[deriving(Clone, Eq)]
pub struct Obb2<S> {
    pub center: Point2<S>,
    pub axis: Vec2<S>,
    pub extents: Vec2<S>,
}

#[deriving(Clone, Eq)]
pub struct Obb3<S> {
    pub center: Point3<S>,
    pub axis: Vec3<S>,
    pub extents: Vec3<S>,
}
