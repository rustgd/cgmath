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

//! Points are fixed positions in affine space with no length or direction. This
//! disinguishes them from vectors, which have a length and direction, but do
//! not have a fixed position.

use std::num::zero;

use array::*;
use vector::*;

/// A point in 2-dimensional space.
#[deriving(Eq, Zero, Clone)]
struct Point2<S> { x: S, y: S }

/// A point in 2-dimensional space.
#[deriving(Eq, Zero, Clone)]
struct Point3<S> { x: S, y: S, z: S }

approx_eq!(impl<S> Point2<S>)
approx_eq!(impl<S> Point3<S>)

impl<S: Num> Point2<S> {
    #[inline]
    pub fn new(x: S, y: S) -> Point2<S> {
        Point2 { x: x, y: y }
    }

    #[inline]
    pub fn origin() -> Point2<S> { zero() }
}

impl<S: Num> Point3<S> {
    #[inline]
    pub fn new(x: S, y: S, z: S) -> Point3<S> {
        Point3 { x: x, y: y, z: z }
    }

    #[inline]
    pub fn origin() -> Point3<S> { zero() }
}

/// Specifies the numeric operations for point types.
pub trait Point
<
    S: Clone + Num,
    V: Vector<S, Slice>,
    Slice
>
:   Array<S, Slice>
{
    #[inline] fn mul_s(&self, s: S) -> Self { build(|i| self.i(i).mul(&s)) }
    #[inline] fn div_s(&self, s: S) -> Self { build(|i| self.i(i).div(&s)) }
    #[inline] fn rem_s(&self, s: S) -> Self { build(|i| self.i(i).rem(&s)) }

    #[inline] fn add_v(&self, other: &V) -> Self { build(|i| self.i(i).add(other.i(i))) }
    #[inline] fn sub_p(&self, other: &Self) -> V { build(|i| self.i(i).sub(other.i(i))) }

    #[inline] fn mul_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.mul(&s) } }
    #[inline] fn div_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.div(&s) } }
    #[inline] fn rem_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.rem(&s) } }

    #[inline] fn add_self_v(&mut self, other: &V) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.add(b) } }
}

array!(impl<S> Point2<S> -> [S, ..2] _2)
array!(impl<S> Point3<S> -> [S, ..3] _3)

impl<S: Clone + Num + Ord> Point<S, Vec2<S>, [S, ..2]> for Point2<S>;
impl<S: Clone + Num + Ord> Point<S, Vec3<S>, [S, ..3]> for Point3<S>;

impl<S> ToStr for Point2<S> {
    fn to_str(&self) -> ~str {
        fmt!("[%?, %?]", self.x, self.y)
    }
}

impl<S> ToStr for Point3<S> {
    fn to_str(&self) -> ~str {
        fmt!("[%?, %?, %?]", self.x, self.y, self.z)
    }
}
