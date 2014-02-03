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

use std::fmt;
use std::num::{one, zero};

use array::*;
use vector::*;

/// A point in 2-dimensional space.
#[deriving(Eq, Clone, IterBytes)]
pub struct Point2<S> { x: S, y: S }

/// A point in 3-dimensional space.
#[deriving(Eq, Clone, IterBytes)]
pub struct Point3<S> { x: S, y: S, z: S }


impl<S: Num> Point2<S> {
    #[inline]
    pub fn new(x: S, y: S) -> Point2<S> {
        Point2 { x: x, y: y }
    }
}

impl<S: Num> Point3<S> {
    #[inline]
    pub fn new(x: S, y: S, z: S) -> Point3<S> {
        Point3 { x: x, y: y, z: z }
    }
}

impl<S: Clone + Num + Primitive> Point3<S> {
    #[inline]
    pub fn from_homogeneous(v: &Vec4<S>) -> Point3<S> {
        let e = v.truncate().mul_s(one::<S>() / v.w);
        Point3::new(e.x.clone(), e.y.clone(), e.z.clone())  //FIXME
    }

    #[inline]
    pub fn to_homogeneous(&self) -> Vec4<S> {
        Vec4::new(self.x.clone(), self.y.clone(), self.z.clone(), one())
    }
}

/// Specifies the numeric operations for point types.
pub trait Point
<
    S: Primitive,
    V: Vector<S, Slice>,
    Slice
>
:   Array<S, Slice>
{
    #[inline] fn origin() -> Self{ build(|_i| zero::<S>()) }

    #[inline] fn from_vec(v: &V) -> Self { build(|i| v.i(i).clone()) }
    #[inline] fn to_vec(&self) -> V { build(|i| self.i(i).clone()) }

    #[inline] fn mul_s(&self, s: S) -> Self { build(|i| self.i(i).mul(&s)) }
    #[inline] fn div_s(&self, s: S) -> Self { build(|i| self.i(i).div(&s)) }
    #[inline] fn rem_s(&self, s: S) -> Self { build(|i| self.i(i).rem(&s)) }

    #[inline] fn add_v(&self, other: &V) -> Self { build(|i| self.i(i).add(other.i(i))) }
    #[inline] fn sub_p(&self, other: &Self) -> V { build(|i| self.i(i).sub(other.i(i))) }

    #[inline] fn mul_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.mul(&s)) }
    #[inline] fn div_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.div(&s)) }
    #[inline] fn rem_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.rem(&s)) }

    #[inline] fn add_self_v(&mut self, other: &V) { self.each_mut(|i, x| *x = x.add(other.i(i))) }

    /// This is a weird one, but its useful for plane calculations
    #[inline]
    fn dot(&self, v: &V) -> S {
        build::<S, Slice, V>(|i| self.i(i).mul(v.i(i))).comp_add()
    }
}

array!(impl<S> Point2<S> -> [S, ..2] _2)
array!(impl<S> Point3<S> -> [S, ..3] _3)

impl<S: Primitive> Point<S, Vec2<S>, [S, ..2]> for Point2<S> {}
impl<S: Primitive> Point<S, Vec3<S>, [S, ..3]> for Point3<S> {}

impl<S: fmt::Show> ToStr for Point2<S> {
    fn to_str(&self) -> ~str {
        format!("[{}, {}]", self.x, self.y)
    }
}

impl<S: fmt::Show> ToStr for Point3<S> {
    fn to_str(&self) -> ~str {
        format!("[{}, {}, {}]", self.x, self.y, self.z)
    }
}
