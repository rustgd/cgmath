// Copyright 2013 The OMath Developers. For a full listing of the authors,
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

use traits::alg::*;
use traits::util::*;
use types::vector::{Vec2, Vec3};

#[deriving(Eq, Zero, Clone)]
struct Point2<S> { x: S, y: S }

#[deriving(Eq, Zero, Clone)]
struct Point3<S> { x: S, y: S, z: S }

impl<S> Point2<S> {
    #[inline]
    pub fn new(x: S, y: S) -> Point2<S> {
        Point2 { x: x, y: y }
    }
}

impl<S> Point3<S> {
    #[inline]
    pub fn new(x: S, y: S, z: S) -> Point3<S> {
        Point3 { x: x, y: y, z: z }
    }
}

// Operator impls

impl<S:Ring> Mul<S, Point2<S>> for Point2<S> { #[inline(always)] fn mul(&self, s: &S) -> Point2<S> { self.map(|x| x.mul(s)) }  }
impl<S:Ring> Mul<S, Point3<S>> for Point3<S> { #[inline(always)] fn mul(&self, s: &S) -> Point3<S> { self.map(|x| x.mul(s)) }  }

impl<S:Ring> Div<S, Point2<S>> for Point2<S> { #[inline(always)] fn div(&self, s: &S) -> Point2<S> { self.map(|x| x.div(s)) }  }
impl<S:Ring> Div<S, Point3<S>> for Point3<S> { #[inline(always)] fn div(&self, s: &S) -> Point3<S> { self.map(|x| x.div(s)) }  }

impl<S:Ring> Rem<S, Point2<S>> for Point2<S> { #[inline(always)] fn rem(&self, s: &S) -> Point2<S> { self.map(|x| x.rem(s)) }  }
impl<S:Ring> Rem<S, Point3<S>> for Point3<S> { #[inline(always)] fn rem(&self, s: &S) -> Point3<S> { self.map(|x| x.rem(s)) }  }

impl<S:Ring> Add<Vec2<S>, Point2<S>> for Point2<S> { #[inline(always)] fn add(&self, other: &Vec2<S>) -> Point2<S> { self.bimap(other, |a, b| a.add(b)) } }
impl<S:Ring> Add<Vec3<S>, Point3<S>> for Point3<S> { #[inline(always)] fn add(&self, other: &Vec3<S>) -> Point3<S> { self.bimap(other, |a, b| a.add(b)) } }

impl<S:Ring> Sub<Point2<S>, Vec2<S>> for Point2<S> { #[inline(always)] fn sub(&self, other: &Point2<S>) -> Vec2<S> { self.bimap(other, |a, b| a.sub(b)) } }
impl<S:Ring> Sub<Point3<S>, Vec3<S>> for Point3<S> { #[inline(always)] fn sub(&self, other: &Point3<S>) -> Vec3<S> { self.bimap(other, |a, b| a.sub(b)) } }

// Trait impls

impl_indexable!(Point2<T>, [T, ..2])
impl_indexable!(Point3<T>, [T, ..3])

impl<S: Clone> Swappable<S, [S, ..2]> for Point2<S>;
impl<S: Clone> Swappable<S, [S, ..3]> for Point3<S>;

impl<S: Clone> Coordinate<S, [S, ..2]> for Point2<S>;
impl<S: Clone> Coordinate<S, [S, ..3]> for Point3<S>;

impl<S: Ring> ScalarMul<S> for Point2<S>;
impl<S: Ring> ScalarMul<S> for Point3<S>;

impl<S: Ring> AffineSpace<S, Vec2<S>> for Point2<S>;
impl<S: Ring> AffineSpace<S, Vec3<S>> for Point3<S>;
