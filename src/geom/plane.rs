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

use core::{Vec3, Vec4, Mat3};
use geom::{Point, Point3, Ray3};

#[path = "../num_macros.rs"]
mod num_macros;

/// A plane formed from the equation: `Ax + Bx + Cx + D = 0`
///
/// # Fields
///
/// - `n`: the normal of the plane where:
///   - `n.x`: corresponds to `A` in the plane equation
///   - `n.y`: corresponds to `B` in the plane equation
///   - `n.z`: corresponds to `C` in the plane equation
/// - `d`: the distance value, corresponding to `D` in the plane equation
#[deriving(Clone, Eq)]
pub struct Plane3<T> {
    norm: Vec3<T>,
    dist: T,
}

impl<T:Clone + Float> Plane3<T> {
    /// # Arguments
    ///
    /// - `a`: the `x` component of the normal
    /// - `b`: the `y` component of the normal
    /// - `c`: the `z` component of the normal
    /// - `d`: the plane's distance value
    pub fn from_abcd(a: T, b: T, c: T, d: T) -> Plane3<T> {
        Plane3 {
            norm: Vec3::new(a, b, c),
            dist: d,
        }
    }

    /// Construct a plane from a normal vector `n` and a distance `d`
    pub fn from_nd(norm: Vec3<T>, dist: T) -> Plane3<T> {
        Plane3 { norm: norm, dist: dist }
    }

    /// Construct a plane from the components of a four-dimensional vector
    pub fn from_vec4(vec: Vec4<T>) -> Plane3<T> {
        Plane3::from_abcd(vec.x.clone(), vec.y.clone(), vec.z.clone(), vec.w.clone())
    }

    /// Compute the distance from the plane to the point
    pub fn distance(&self, pos: &Point3<T>) -> T {
        self.norm.dot(pos.as_vec()) + self.dist
    }

    /// Computes the point at which `ray` intersects the plane
    pub fn intersection_r(&self, _ray: &Ray3<T>) -> Point3<T> {
        fail!(~"not yet implemented")
    }

    /// Returns `true` if the ray intersects the plane
    pub fn intersects(&self, _ray: &Ray3<T>) -> bool {
        fail!(~"not yet implemented")
    }

    /// Returns `true` if `pos` is located behind the plane - otherwise it returns `false`
    pub fn contains(&self, pos: &Point3<T>) -> bool {
        self.distance(pos) < zero!(T)
    }
}

impl<T:Clone + Float> Plane3<T> {
    /// Constructs a plane that passes through the the three points `a`, `b` and `c`
    pub fn from_3p(a: Point3<T>,
                   b: Point3<T>,
                   c: Point3<T>) -> Option<Plane3<T>> {
        // create two vectors that run parallel to the plane
        let v0 = (b - a);
        let v1 = (c - a);
        // find the vector that is perpendicular to v1 and v2
        let mut norm = v0.cross(&v1);

        if norm.approx_eq(&Vec3::zero()) {
            None
        } else {
            // compute the normal and the distance to the plane
            norm.normalize_self();
            let dist = -a.as_vec().dot(&norm);

            Some(Plane3::from_nd(norm, dist))
        }
    }

    /// Computes the ray created from the two-plane intersection of `self` and `other`
    ///
    /// # Return value
    ///
    /// - `Some(r)`: The ray `r` where the planes intersect.
    /// - `None`: No valid intersection was found. The planes are probably parallel.
    pub fn intersection_2pl(&self, other: &Plane3<T>) -> Option<Ray3<T>> {
        let ray_dir = self.norm.cross(&other.norm);

        if ray_dir.approx_eq(&Vec3::zero::<T>()) {
            None  // the planes are parallel
        } else {
            // The end-point of the ray is at the three-plane intersection between
            // `self`, `other`, and a tempory plane positioned at the origin
            do Plane3::from_nd(ray_dir.clone(), zero!(T)).intersection_3pl(self, other).map |ray_pos| {
                Ray3 {
                    pos: ray_pos.clone(),
                    dir: ray_dir.clone(),
                }
            }
        }
    }

    /// Computes the three-plane intersection between `self`, `other_a` and `other_b`.
    ///
    /// # Return value
    ///
    /// - `Some(p)`: The position vector `p` where the planes intersect.
    /// - `None`:    No valid intersection was found. The normals of the three
    ///              planes are probably coplanar.
    pub fn intersection_3pl(&self, other_a: &Plane3<T>, other_b: &Plane3<T>) -> Option<Point3<T>> {
        let mx = Mat3::new(self.norm.x.clone(), other_a.norm.x.clone(), other_b.norm.x.clone(),
                           self.norm.y.clone(), other_a.norm.y.clone(), other_b.norm.y.clone(),
                           self.norm.z.clone(), other_a.norm.z.clone(), other_b.norm.z.clone());
        do mx.inverse().map |m| {
            Point3::origin() + m.mul_v(&Vec3::new(self.dist.clone(),
                                                  other_a.dist.clone(),
                                                  other_b.dist.clone()))
        }
    }
}

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for Plane3<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &Plane3<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &Plane3<T>, epsilon: &T) -> bool {
        self.norm.approx_eq_eps(&other.norm, epsilon) &&
        self.dist.approx_eq_eps(&other.dist, epsilon)
    }
}

impl<T> ToStr for Plane3<T> {
    pub fn to_str(&self) -> ~str {
        fmt!("%?x + %?y + %?z + %? = 0", self.norm.x, self.norm.y, self.norm.z, self.dist)
    }
}

#[cfg(test)]
mod tests {
    use geom::plane::*;
    use geom::point::*;

    #[test]
    fn test_from_3p() {
        assert_eq!(Plane3::from_3p(Point3::new(5f, 0f,  5f),
                                   Point3::new(5f, 5f,  5f),
                                   Point3::new(5f, 0f, -1f)), Some(Plane3::from_abcd(-1f, 0f, 0f, 5f)));

        assert_eq!(Plane3::from_3p(Point3::new(0f, 5f, -5f),
                                   Point3::new(0f, 5f,  0f),
                                   Point3::new(0f, 5f,  5f)), None);     // The points are parallel
    }

    #[test]
    fn test_plane_intersection_3pl() {
        let p0 = Plane3::from_abcd(1.0,  0.0, 0.0, 1.0);
        let p1 = Plane3::from_abcd(0.0, -1.0, 0.0, 2.0);
        let p2 = Plane3::from_abcd(0.0,  0.0, 1.0, 1.0);

        assert_eq!(p0.intersection_3pl(&p1, &p2), Some(Point3::new(1.0, -2.0, 1.0)));
    }

    #[test]
    fn test_to_str() {
        assert_eq!(Plane3::from_abcd(1.0, 2.0, 3.0, 4.0).to_str(), ~"1x + 2y + 3z + 4 = 0");
    }
}
