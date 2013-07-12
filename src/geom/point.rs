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

use core::{Dimensional, Swap};
use core::{Mat2, Mat3, Quat};
use core::{Vec2, ToVec2, AsVec2};
use core::{Vec3, ToVec3, AsVec3};
use core::{Vec4, ToVec4};
use geom::{Ray2, Ray3};

/// A geometric point
pub trait Point<T, Vec, Ray>: Eq
                            + Add<Vec, Self>
                            + Sub<Self, Vec>
                            + Mul<Vec, Self>
                            + ApproxEq<T>
                            + ToStr {
    pub fn translate(&self, offset: &Vec) -> Self;
    pub fn scale(&self, factor: &Vec) -> Self;
    pub fn distance2(&self, other: &Self) -> T;
    pub fn distance(&self, other: &Self) -> T;
    pub fn direction(&self, other: &Self) -> Vec;
    pub fn ray_to(&self, other: &Self) -> Ray;
}

/// A two-dimensional point
#[deriving(Clone, Eq)]
pub struct Point2<T> { x: T, y: T }

impl_dimensional!(Point2, T, 2)
impl_swap!(Point2)
impl_approx!(Point2 { x, y })

impl<T:Num> Point2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2 { x: x, y: y }
    }

    #[inline]
    pub fn from_vec2(vec: Vec2<T>) -> Point2<T> {
        unsafe { cast::transmute(vec) }
    }

    #[inline]
    pub fn origin() -> Point2<T> {
        Point2::new(zero!(T), zero!(T))
    }
}

impl<T:Clone + Num> ToVec2<T> for Point2<T> {
    #[inline]
    pub fn to_vec2(&self) -> Vec2<T> {
        self.as_vec2().clone()
    }
}

impl<T:Num> AsVec2<T> for Point2<T> {
    #[inline]
    pub fn as_vec2<'a>(&'a self) -> &'a Vec2<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn as_mut_vec2<'a>(&'a mut self) -> &'a mut Vec2<T> {
        unsafe { cast::transmute(self) }
    }
}

impl<T:Clone + Num> ToVec3<T> for Point2<T> {
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
        Point2::from_vec2(mat.mul_v(self.as_vec2()))
    }
}

impl<T:Clone + Float> Point<T, Vec2<T>, Ray2<T>> for Point2<T> {
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
        ((*other) - (*self)).magnitude2()
    }

    /// Returns the scalar distance to the other point
    #[inline]
    pub fn distance(&self, other: &Point2<T>) -> T {
        other.distance2(self).sqrt()
    }

    /// Returns a normalized direction vector pointing to the other point
    #[inline]
    pub fn direction(&self, other: &Point2<T>) -> Vec2<T> {
        ((*other) - (*self)).normalize()
    }

    /// Projects a normalized ray towards the other point
    #[inline]
    pub fn ray_to(&self, other: &Point2<T>) -> Ray2<T> {
        Ray2::new(self.clone(), self.direction(other))
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

impl_dimensional!(Point3, T, 3)
impl_swap!(Point3)
impl_approx!(Point3 { x, y, z })

impl<T:Num> Point3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3 { x: x, y: y, z: z }
    }

    #[inline]
    pub fn from_vec3(vec: Vec3<T>) -> Point3<T> {
        unsafe { cast::transmute(vec) }
    }

    #[inline]
    pub fn origin() -> Point3<T> {
        Point3::new(zero!(T), zero!(T), zero!(T))
    }
}

impl<T:Clone + Num> ToVec3<T> for Point3<T> {
    /// Converts the point to a three-dimensional homogeneous vector:
    /// `[x, y] -> [x, y, 1]`
    #[inline]
    pub fn to_vec3(&self) -> Vec3<T> {
        self.as_vec3().clone()
    }
}

impl<T:Num> AsVec3<T> for Point3<T> {
    #[inline]
    pub fn as_vec3<'a>(&'a self) -> &'a Vec3<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn as_mut_vec3<'a>(&'a mut self) -> &'a mut Vec3<T> {
        unsafe { cast::transmute(self) }
    }
}

impl<T:Clone + Num> ToVec4<T> for Point3<T> {
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
        Point3::from_vec3(quat.mul_v(self.as_vec3()))
    }

    #[inline]
    pub fn rotate_m(&self, mat: &Mat3<T>) -> Point3<T> {
        Point3::from_vec3(mat.mul_v(self.as_vec3()))
    }
}

impl<T:Clone + Float> Point<T, Vec3<T>, Ray3<T>> for Point3<T> {
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
        ((*other) - (*self)).magnitude2()
    }

    /// Returns the scalar distance to the other point
    #[inline]
    pub fn distance(&self, other: &Point3<T>) -> T {
        other.distance2(self).sqrt()
    }

    /// Returns a normalized direction vector pointing to the other point
    #[inline]
    pub fn direction(&self, other: &Point3<T>) -> Vec3<T> {
        ((*other) - (*self)).normalize()
    }

    /// Projects a normalized ray towards the other point
    #[inline]
    pub fn ray_to(&self, other: &Point3<T>) -> Ray3<T> {
        Ray3::new(self.clone(), self.direction(other))
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
