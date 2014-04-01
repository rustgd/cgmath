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

use point::{Point, Point2, Point3};
use vector::{Vector, Vec2, Vec3};

/// A generic ray
#[deriving(Clone, Eq)]
pub struct Ray<P,V>
{
    pub origin: P,
    pub direction: V,
}

impl
<
    S: Primitive,
    Slice,
    V: Vector<S,Slice>,
    P: Point<S,V,Slice>
>  Ray<P,V>
{
    pub fn new(origin: P, direction: V) -> Ray<P,V> {
        Ray { origin:origin, direction:direction }
    }
}

pub type Ray2<S> = Ray<Point2<S>,Vec2<S>>;
pub type Ray3<S> = Ray<Point3<S>,Vec3<S>>;
