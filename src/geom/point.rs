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

use std::cast;

use core::{Vec2, Vec3};

/// A geometric point
pub trait Point<T,V>: Eq + ApproxEq<T> + ToStr {
    pub fn from_vec(vec: V) -> Self;
    pub fn as_vec<'a>(&'a self) -> &'a V;
    pub fn as_mut_vec<'a>(&'a mut self) -> &'a mut V;

    pub fn translate(&self, offset: &V) -> Self;
    pub fn distance(&self, other: &Self) -> T;
}

/// A two-dimensional point
#[deriving(Clone, Eq)]
pub struct Point2<T> { x: T, y: T }

impl<T> Point2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2 { x: x, y: y }
    }
}

impl<T:Clone + Float> Point<T,Vec2<T>> for Point2<T> {
    #[inline]
    pub fn from_vec(vec: Vec2<T>) -> Point2<T> {
        unsafe { cast::transmute(vec) }
    }

    #[inline]
    pub fn as_vec<'a>(&'a self) -> &'a Vec2<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn as_mut_vec<'a>(&'a mut self) -> &'a mut Vec2<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn translate(&self, offset: &Vec2<T>) -> Point2<T> {
        Point::from_vec(self.as_vec().add_v(offset))
    }

    #[inline]
    pub fn distance(&self, other: &Point2<T>) -> T {
        self.as_vec().distance(other.as_vec())
    }
}

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for Point2<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &Point2<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &Point2<T>, epsilon: &T) -> bool {
        self.x.approx_eq_eps(&other.x, epsilon) &&
        self.y.approx_eq_eps(&other.y, epsilon)
    }
}

impl<T> ToStr for Point2<T> {
    pub fn to_str(&self) -> ~str {
        fmt!("[%?, %?]", self.x, self.y)
    }
}

#[cfg(test)]
mod test_point2 {
    use geom::point::*;

    #[test]
    fn test_to_str() {
        assert_eq!(Point2::new(1, 2).to_str(), ~"[1, 2]");
    }
}

/// A three-dimensional point
#[deriving(Clone, Eq)]
pub struct Point3<T> { x: T, y: T, z: T }

impl<T> Point3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3 { x: x, y: y, z: z }
    }
}

impl<T:Clone + Float> Point<T,Vec3<T>> for Point3<T> {
    #[inline]
    pub fn from_vec(vec: Vec3<T>) -> Point3<T> {
        unsafe { cast::transmute(vec) }
    }

    #[inline]
    pub fn as_vec<'a>(&'a self) -> &'a Vec3<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn as_mut_vec<'a>(&'a mut self) -> &'a mut Vec3<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn translate(&self, offset: &Vec3<T>) -> Point3<T> {
        Point::from_vec(self.as_vec().add_v(offset))
    }

    #[inline]
    pub fn distance(&self, other: &Point3<T>) -> T {
        self.as_vec().distance(other.as_vec())
    }
}

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for Point3<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &Point3<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &Point3<T>, epsilon: &T) -> bool {
        self.x.approx_eq_eps(&other.x, epsilon) &&
        self.y.approx_eq_eps(&other.y, epsilon) &&
        self.z.approx_eq_eps(&other.z, epsilon)
    }
}

impl<T> ToStr for Point3<T> {
    pub fn to_str(&self) -> ~str {
        fmt!("[%?, %?, %?]", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test_point3 {
    use geom::point::*;

    #[test]
    fn test_to_str() {
        assert_eq!(Point3::new(1, 2, 3).to_str(), ~"[1, 2, 3]");
    }
}
