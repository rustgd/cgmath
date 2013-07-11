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

//! Coordinate vectors for positional data
//!
//! These types differ from the vector types implemented in `core::vec` because
//! they describe coordinates in geometric space and not a magnitude and a
//! direction. All positional data throughout the library uses these point
//! types, which allows for a clear, self-documenting API.

use std::cast;

use core::{Mat2, Mat3, Quat, Vec2, Vec3, Vec4};

#[path = "../num_macros.rs"]
mod num_macros;

/// A geometric point
pub trait Point<T, Vec>: Eq
                       + Add<Vec, Self>
                       + Sub<Self, Vec>
                       + Mul<Vec, Self>
                       + ApproxEq<T>
                       + ToStr {
    pub fn as_vec<'a>(&'a self) -> &'a Vec;
    pub fn as_mut_vec<'a>(&'a mut self) -> &'a mut Vec;

    pub fn translate(&self, offset: &Vec) -> Self;
    pub fn scale(&self, factor: &Vec) -> Self;
    pub fn distance2(&self, other: &Self) -> T;
    pub fn distance(&self, other: &Self) -> T;
    pub fn direction(&self, other: &Self) -> Vec;
}

/// A two-dimensional point
#[deriving(Clone, Eq)]
pub struct Point2<T> { x: T, y: T }

impl<T:Num> Point2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2 { x: x, y: y }
    }

    #[inline]
    pub fn from_vec(vec: Vec2<T>) -> Point2<T> {
        unsafe { cast::transmute(vec) }
    }

    #[inline]
    pub fn origin() -> Point2<T> {
        Point2::new(zero!(T), zero!(T))
    }
}

impl<T:Clone + Num> Point2<T> {
    /// Converts the point to a three-dimensional homogeneous vector:
    /// `[x, y] -> [x, y, 1]`
    #[inline]
    pub fn to_vec3(&self) -> Vec3<T> {
        Vec3::new((*self).x.clone(),
                  (*self).y.clone(),
                  one!(T))
    }
}

impl<T:Clone + Float> Point2<T> {
    #[inline]
    pub fn rotate_t(&self, radians: &T) -> Point2<T> {
        Point2::new((*self).x.cos() * (*radians),
                    (*self).y.sin() * (*radians))
    }

    #[inline]
    pub fn rotate_m(&self, mat: &Mat2<T>) -> Point2<T> {
        Point2::from_vec(mat.mul_v(self.as_vec()))
    }
}

impl<T:Clone + Float> Point<T, Vec2<T>> for Point2<T> {
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
        (*self) + (*offset)
    }

    #[inline]
    pub fn scale(&self, factor: &Vec2<T>) -> Point2<T> {
        (*self) * (*factor)
    }

    #[inline]
    pub fn distance2(&self, other: &Point2<T>) -> T {
        ((*other) - (*self)).length2()
    }

    #[inline]
    pub fn distance(&self, other: &Point2<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline]
    pub fn direction(&self, other: &Point2<T>) -> Vec2<T> {
        ((*other) - (*self)).normalize()
    }
}

impl<T:Num> Add<Vec2<T>, Point2<T>> for Point2<T> {
    fn add(&self, other: &Vec2<T>) -> Point2<T> {
        Point2::new((*self).x + (*other).x,
                    (*self).y + (*other).y)
    }
}

impl<T:Num> Sub<Point2<T>, Vec2<T>> for Point2<T> {
    fn sub(&self, other: &Point2<T>) -> Vec2<T> {
        Vec2::new((*self).x - (*other).x,
                  (*self).y - (*other).y)
    }
}

impl<T:Num> Mul<Vec2<T>, Point2<T>> for Point2<T> {
    fn mul(&self, scale: &Vec2<T>) -> Point2<T> {
        Point2::new((*self).x * (*scale).x,
                    (*self).y * (*scale).y)
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

impl<T:Num> Point3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3 { x: x, y: y, z: z }
    }

    #[inline]
    pub fn from_vec(vec: Vec3<T>) -> Point3<T> {
        unsafe { cast::transmute(vec) }
    }

    #[inline]
    pub fn origin() -> Point3<T> {
        Point3::new(zero!(T), zero!(T), zero!(T))
    }
}

impl<T:Clone + Num> Point3<T> {
    /// Converts the point to a four-dimensional homogeneous vector:
    /// `[x, y, z] -> [x, y, z, 1]`
    #[inline]
    pub fn to_vec4(&self) -> Vec4<T> {
        Vec4::new((*self).x.clone(),
                  (*self).y.clone(),
                  (*self).z.clone(),
                  one!(T))
    }
}

impl<T:Clone + Float> Point3<T> {
    #[inline]
    pub fn rotate_q(&self, quat: &Quat<T>) -> Point3<T> {
        Point3::from_vec(quat.mul_v(self.as_vec()))
    }

    #[inline]
    pub fn rotate_m(&self, mat: &Mat3<T>) -> Point3<T> {
        Point3::from_vec(mat.mul_v(self.as_vec()))
    }
}

impl<T:Clone + Float> Point<T, Vec3<T>> for Point3<T> {
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
        (*self) + (*offset)
    }

    #[inline]
    pub fn scale(&self, factor: &Vec3<T>) -> Point3<T> {
        (*self) * (*factor)
    }

    #[inline]
    pub fn distance2(&self, other: &Point3<T>) -> T {
        ((*other) - (*self)).length2()
    }

    #[inline]
    pub fn distance(&self, other: &Point3<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline]
    pub fn direction(&self, other: &Point3<T>) -> Vec3<T> {
        ((*other) - (*self)).normalize()
    }
}

impl<T:Num> Add<Vec3<T>, Point3<T>> for Point3<T> {
    fn add(&self, other: &Vec3<T>) -> Point3<T> {
        Point3::new((*self).x + (*other).x,
                    (*self).y + (*other).y,
                    (*self).z + (*other).z)
    }
}

impl<T:Num> Sub<Point3<T>, Vec3<T>> for Point3<T> {
    fn sub(&self, other: &Point3<T>) -> Vec3<T> {
        Vec3::new((*self).x - (*other).x,
                  (*self).y - (*other).y,
                  (*self).z - (*other).z)
    }
}

impl<T:Num> Mul<Vec3<T>, Point3<T>> for Point3<T> {
    fn mul(&self, scale: &Vec3<T>) -> Point3<T> {
        Point3::new((*self).x * (*scale).x,
                    (*self).y * (*scale).y,
                    (*self).z * (*scale).z)
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
