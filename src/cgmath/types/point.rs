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

use traits::alg::*;
use traits::util::*;
use types::vector::{Vec2, Vec3};

#[deriving(Eq, Zero, Clone)]
struct Point2<S> { x: S, y: S }

#[deriving(Eq, Zero, Clone)]
struct Point3<S> { x: S, y: S, z: S }

impl<S: Field> Point2<S> {
    #[inline]
    pub fn new(x: S, y: S) -> Point2<S> {
        Point2 { x: x, y: y }
    }
}

impl<S: Field> Point3<S> {
    #[inline]
    pub fn new(x: S, y: S, z: S) -> Point3<S> {
        Point3 { x: x, y: y, z: z }
    }
}

impl_indexable!(Point2<S>, S, [S, ..2])
impl_indexable!(Point3<S>, S, [S, ..3])

impl<S: Clone + Field> Swappable<S, [S, ..2]> for Point2<S>;
impl<S: Clone + Field> Swappable<S, [S, ..3]> for Point3<S>;

impl<S: Clone + Field> Coordinate<S, [S, ..2]> for Point2<S>;
impl<S: Clone + Field> Coordinate<S, [S, ..3]> for Point3<S>;

impl_scalar_binop!(Point2<S>, Mul, mul)
impl_scalar_binop!(Point3<S>, Mul, mul)

impl_scalar_binop!(Point2<S>, Div, div)
impl_scalar_binop!(Point3<S>, Div, div)

impl_scalar_binop!(Point2<S>, Rem, rem)
impl_scalar_binop!(Point3<S>, Rem, rem)

impl<S: Field> ScalarMul<S> for Point2<S>;
impl<S: Field> ScalarMul<S> for Point3<S>;

impl_coordinate_binop!(Point2<S>, Vec2<S>, Point2<S>, Add, add)
impl_coordinate_binop!(Point3<S>, Vec3<S>, Point3<S>, Add, add)

impl_coordinate_binop!(Point2<S>, Point2<S>, Vec2<S>, Sub, sub)
impl_coordinate_binop!(Point3<S>, Point3<S>, Vec3<S>, Sub, sub)

impl<S: Field> AffineSpace<S, Vec2<S>> for Point2<S>;
impl<S: Field> AffineSpace<S, Vec3<S>> for Point3<S>;
