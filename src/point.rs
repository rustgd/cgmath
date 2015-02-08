// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
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
use std::mem;
use std::ops::*;

use approx::ApproxEq;
use array::{Array1, FixedArray};
use num::{BaseNum, BaseFloat, one, zero};
use vector::*;

/// A point in 2-dimensional space.
#[derive(PartialEq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
pub struct Point2<S> { pub x: S, pub y: S }

/// A point in 3-dimensional space.
#[derive(PartialEq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
pub struct Point3<S> { pub x: S, pub y: S, pub z: S }


impl<S: BaseNum> Point2<S> {
    #[inline]
    pub fn new(x: S, y: S) -> Point2<S> {
        Point2 { x: x, y: y }
    }
}

impl<S: BaseNum> Point3<S> {
    #[inline]
    pub fn new(x: S, y: S, z: S) -> Point3<S> {
        Point3 { x: x, y: y, z: z }
    }
}

impl<S: BaseNum> Point3<S> {
    #[inline]
    pub fn from_homogeneous(v: &Vector4<S>) -> Point3<S> {
        let e = v.truncate().mul_s(one::<S>() / v.w);
        Point3::new(e.x, e.y, e.z)  //FIXME
    }

    #[inline]
    pub fn to_homogeneous(&self) -> Vector4<S> {
        Vector4::new(self.x, self.y, self.z, one())
    }
}

/// Specifies the numeric operations for point types.
pub trait Point<S: BaseNum, V: Vector<S>>: Array1<S> + Clone {
    /// Create a point at the origin.
    fn origin() -> Self;

    /// Create a point from a vector.
    fn from_vec(v: &V) -> Self;
    /// Convert a point to a vector.
    fn to_vec(&self) -> V;

    /// Multiply each component by a scalar, returning the new point.
    fn mul_s(&self, s: S) -> Self;
    /// Divide each component by a scalar, returning the new point.
    fn div_s(&self, s: S) -> Self;
    /// Subtract a scalar from each component, returning the new point.
    fn rem_s(&self, s: S) -> Self;

    /// Add a vector to this point, returning the new point.
    fn add_v(&self, v: &V) -> Self;
    /// Subtract another point from this one, returning a new vector.
    fn sub_p(&self, p: &Self) -> V;

    /// Multiply each component by a scalar, in-place.
    fn mul_self_s(&mut self, s: S);
    /// Divide each component by a scalar, in-place.
    fn div_self_s(&mut self, s: S);
    /// Take the remainder of each component by a scalar, in-place.
    fn rem_self_s(&mut self, s: S);

    /// Add a vector to this point, in-place.
    fn add_self_v(&mut self, v: &V);

    /// This is a weird one, but its useful for plane calculations.
    fn dot(&self, v: &V) -> S;

    fn min(&self, p: &Self) -> Self;

    fn max(&self, p: &Self) -> Self;
}

impl<S> FixedArray<[S; 2]> for Point2<S> {
    #[inline]
    fn into_fixed(self) -> [S; 2] {
        match self { Point2 { x, y } => [x, y] }
    }

    #[inline]
    fn as_fixed<'a>(&'a self) -> &'a [S; 2] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn as_mut_fixed<'a>(&'a mut self) -> &'a mut [S; 2] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn from_fixed(_v: [S; 2]) -> Point2<S> {
        // match v { [x, y] => Point2 { x: x, y: y } }
        panic!("Unimplemented, pending a fix for rust-lang/rust#16418");
    }

    #[inline]
    fn from_fixed_ref<'a>(v: &'a [S; 2]) -> &'a Point2<S> {
        unsafe { mem::transmute(v) }
    }

    #[inline]
    fn from_fixed_mut<'a>(v: &'a mut [S; 2]) -> &'a mut Point2<S> {
        unsafe { mem::transmute(v) }
    }
}

impl<S: BaseNum> Index<usize> for Point2<S> {
    type Output = S;
    #[inline]
    fn index<'a>(&'a self, i: &usize) -> &'a S {
        &self.as_fixed()[*i]
    }
}

impl<S: BaseNum> IndexMut<usize> for Point2<S> {
    #[inline]
    fn index_mut<'a>(&'a mut self, i: &usize) -> &'a mut S {
        &mut self.as_mut_fixed()[*i]
    }
}

impl<S: BaseNum> Array1<S> for Point2<S> {
    #[inline]
    fn map<F>(&mut self, mut op: F) -> Point2<S> where F: FnMut(S) -> S {
        self.x = op(self.x);
        self.y = op(self.y);
        *self
    }
}

impl<S: BaseNum> Point<S, Vector2<S>> for Point2<S> {
    #[inline]
    fn origin() -> Point2<S> {
        Point2::new(zero(), zero())
    }

    #[inline]
    fn from_vec(v: &Vector2<S>) -> Point2<S> {
        Point2::new(v.x, v.y)
    }

    #[inline]
    fn to_vec(&self) -> Vector2<S> {
        Vector2::new(self.x,
                     self.y)
    }

    #[inline]
    fn mul_s(&self, s: S) -> Point2<S> {
        Point2::new(self.x * s,
                    self.y * s)
    }

    #[inline]
    fn div_s(&self, s: S) -> Point2<S> {
        Point2::new(self.x / s,
                    self.y / s)
    }

    #[inline]
    fn rem_s(&self, s: S) -> Point2<S> {
        Point2::new(self.x % s,
                    self.y % s)
    }

    #[inline]
    fn add_v(&self, v: &Vector2<S>) -> Point2<S> {
        Point2::new(self.x + v.x,
                    self.y + v.y)
    }

    #[inline]
    fn sub_p(&self, p: &Point2<S>) -> Vector2<S> {
        Vector2::new(self.x - p.x,
                     self.y - p.y)
    }

    #[inline]
    fn mul_self_s(&mut self, s: S) {
        self.x = self.x * s;
        self.y = self.y * s;
    }

    #[inline]
    fn div_self_s(&mut self, s: S) {
        self.x = self.x / s;
        self.y = self.y / s;
    }

    #[inline]
    fn rem_self_s(&mut self, s: S) {
        self.x = self.x % s;
        self.y = self.y % s;
    }

    #[inline]
    fn add_self_v(&mut self, v: &Vector2<S>) {
        self.x = self.x + v.x;
        self.y = self.y + v.y;
    }

    #[inline]
    fn dot(&self, v: &Vector2<S>) -> S {
        self.x * v.x +
        self.y * v.y
    }

    #[inline]
    fn min(&self, p: &Point2<S>) -> Point2<S> {
        Point2::new(self.x.partial_min(p.x),
                    self.y.partial_min(p.y))
    }

    #[inline]
    fn max(&self, p: &Point2<S>) -> Point2<S> {
        Point2::new(self.x.partial_max(p.x),
                    self.y.partial_max(p.y))
    }
}

impl<S: BaseFloat> ApproxEq<S> for Point2<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Point2<S>, epsilon: &S) -> bool {
        self.x.approx_eq_eps(&other.x, epsilon) &&
        self.y.approx_eq_eps(&other.y, epsilon)
    }
}

impl<S> FixedArray<[S; 3]> for Point3<S> {
    #[inline]
    fn into_fixed(self) -> [S; 3] {
        match self { Point3 { x, y, z } => [x, y, z] }
    }

    #[inline]
    fn as_fixed<'a>(&'a self) -> &'a [S; 3] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn as_mut_fixed<'a>(&'a mut self) -> &'a mut [S; 3] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn from_fixed(_v: [S; 3]) -> Point3<S> {
        // match v { [x, y, z] => Point3 { x: x, y: y, z: z } }
        panic!("Unimplemented, pending a fix for rust-lang/rust#16418")
    }

    #[inline]
    fn from_fixed_ref<'a>(v: &'a [S; 3]) -> &'a Point3<S> {
        unsafe { mem::transmute(v) }
    }

    #[inline]
    fn from_fixed_mut<'a>(v: &'a mut [S; 3]) -> &'a mut Point3<S> {
        unsafe { mem::transmute(v) }
    }
}

impl<S: BaseNum> Index<usize> for Point3<S> {
    type Output = S;

    #[inline]
    fn index<'a>(&'a self, i: &usize) -> &'a S {
        &self.as_fixed()[*i]
    }
}

impl<S: BaseNum> IndexMut<usize> for Point3<S> {
    #[inline]
    fn index_mut<'a>(&'a mut self, i: &usize) -> &'a mut S {
        &mut self.as_mut_fixed()[*i]
    }
}

impl<S: BaseNum> Array1<S> for Point3<S> {
    #[inline]
    fn map<F>(&mut self, mut op: F) -> Point3<S> where F: FnMut(S) -> S {
        self.x = op(self.x);
        self.y = op(self.y);
        self.z = op(self.z);
        *self
    }
}

impl<S: BaseNum> Point<S, Vector3<S>> for Point3<S> {
    #[inline]
    fn origin() -> Point3<S> {
        Point3::new(zero(), zero(), zero())
    }

    #[inline]
    fn from_vec(v: &Vector3<S>) -> Point3<S> {
        Point3::new(v.x, v.y, v.z)
    }

    #[inline]
    fn to_vec(&self) -> Vector3<S> {
        Vector3::new(self.x,
                     self.y,
                     self.z)
    }

    #[inline]
    fn mul_s(&self, s: S) -> Point3<S> {
        Point3::new(self.x * s,
                    self.y * s,
                    self.z * s)
    }

    #[inline]
    fn div_s(&self, s: S) -> Point3<S> {
        Point3::new(self.x / s,
                    self.y / s,
                    self.z / s)
    }

    #[inline]
    fn rem_s(&self, s: S) -> Point3<S> {
        Point3::new(self.x % s,
                    self.y % s,
                    self.z % s)
    }

    #[inline]
    fn add_v(&self, v: &Vector3<S>) -> Point3<S> {
        Point3::new(self.x + v.x,
                    self.y + v.y,
                    self.z + v.z)
    }

    #[inline]
    fn sub_p(&self, p: &Point3<S>) -> Vector3<S> {
        Vector3::new(self.x - p.x,
                     self.y - p.y,
                     self.z - p.z)
    }

    #[inline]
    fn mul_self_s(&mut self, s: S) {
        self.x = self.x * s;
        self.y = self.y * s;
        self.z = self.z * s;
    }

    #[inline]
    fn div_self_s(&mut self, s: S) {
        self.x = self.x / s;
        self.y = self.y / s;
        self.z = self.z / s;
    }

    #[inline]
    fn rem_self_s(&mut self, s: S) {
        self.x = self.x % s;
        self.y = self.y % s;
        self.z = self.z % s;
    }

    #[inline]
    fn add_self_v(&mut self, v: &Vector3<S>) {
        self.x = self.x + v.x;
        self.y = self.y + v.y;
        self.z = self.z + v.z;
    }

    #[inline]
    fn dot(&self, v: &Vector3<S>) -> S {
        self.x * v.x +
        self.y * v.y +
        self.z * v.z
    }

    #[inline]
    fn min(&self, p: &Point3<S>) -> Point3<S> {
        Point3::new(self.x.partial_min(p.x),
                    self.y.partial_min(p.y),
                    self.z.partial_min(p.z))
    }

    #[inline]
    fn max(&self, p: &Point3<S>) -> Point3<S> {
        Point3::new(self.x.partial_max(p.x),
                    self.y.partial_max(p.y),
                    self.z.partial_max(p.z))
    }
}

impl<S: BaseFloat> ApproxEq<S> for Point3<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Point3<S>, epsilon: &S) -> bool {
        self.x.approx_eq_eps(&other.x, epsilon) &&
        self.y.approx_eq_eps(&other.y, epsilon) &&
        self.z.approx_eq_eps(&other.z, epsilon)
    }
}

impl<S: BaseNum> fmt::Debug for Point2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?}]", self.x, self.y)
    }
}

impl<S: BaseNum> fmt::Debug for Point3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?}, {:?}]", self.x, self.y, self.z)
    }
}
