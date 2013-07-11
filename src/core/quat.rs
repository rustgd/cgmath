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

use core::Dimensional;
use core::{Mat3, ToMat3};
use core::Vec3;

#[path = "../num_macros.rs"]
mod num_macros;
mod dim_macros;

// GLSL-style type aliases

pub type quat  = Quat<f32>;
pub type dquat = Quat<f64>;

// Rust-style type aliases

pub type Quatf   = Quat<float>;
pub type Quatf32 = Quat<f32>;
pub type Quatf64 = Quat<f64>;

/// A quaternion in scalar/vector form
#[deriving(Clone, Eq)]
pub struct Quat<T> { s: T, v: Vec3<T> }

impl_dimensional!(Quat, T, 4)
impl_swap!(Quat)

pub trait ToQuat<T> {
    pub fn to_quat(&self) -> Quat<T>;
}

impl<T> Quat<T> {
    /// Construct the quaternion from one scalar component and three
    /// imaginary components
    ///
    /// # Arguments
    ///
    /// - `w`: the scalar component
    /// - `xi`: the fist imaginary component
    /// - `yj`: the second imaginary component
    /// - `zk`: the third imaginary component
    #[inline]
    pub fn new(w: T, xi: T, yj: T, zk: T) -> Quat<T> {
        Quat::from_sv(w, Vec3::new(xi, yj, zk))
    }

    /// Construct the quaternion from a scalar and a vector
    ///
    /// # Arguments
    ///
    /// - `s`: the scalar component
    /// - `v`: a vector containing the three imaginary components
    #[inline]
    pub fn from_sv(s: T, v: Vec3<T>) -> Quat<T> {
        Quat { s: s, v: v }
    }
}

impl<T:Clone + Real> Quat<T> {
    /// The multiplicative identity, ie: `q = 1 + 0i + 0j + 0i`
    #[inline]
    pub fn identity() -> Quat<T> {
        Quat::from_sv(one!(T), Vec3::zero())
    }

    /// The additive identity, ie: `q = 0 + 0i + 0j + 0i`
    #[inline]
    pub fn zero() -> Quat<T> {
        Quat::new(zero!(T), zero!(T), zero!(T), zero!(T))
    }

    #[inline]
    pub fn from_angle_x(radians: T) -> Quat<T> {
        Quat::new((radians / two!(T)).cos(), radians.sin(), zero!(T), zero!(T))
    }

    #[inline]
    pub fn from_angle_y(radians: T) -> Quat<T> {
        Quat::new((radians / two!(T)).cos(), zero!(T), radians.sin(), zero!(T))
    }

    #[inline]
    pub fn from_angle_z(radians: T) -> Quat<T> {
        Quat::new((radians / two!(T)).cos(), zero!(T), zero!(T), radians.sin())
    }

    pub fn from_angle_xyz(radians_x: T, radians_y: T, radians_z: T) -> Quat<T> {
        // http://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles#Conversion
        let xdiv2 = radians_x / two!(T);
        let ydiv2 = radians_y / two!(T);
        let zdiv2 = radians_z / two!(T);
        Quat::new(zdiv2.cos() * xdiv2.cos() * ydiv2.cos() + zdiv2.sin() * xdiv2.sin() * ydiv2.sin(),
                  zdiv2.sin() * xdiv2.cos() * ydiv2.cos() - zdiv2.cos() * xdiv2.sin() * ydiv2.sin(),
                  zdiv2.cos() * xdiv2.sin() * ydiv2.cos() + zdiv2.sin() * xdiv2.cos() * ydiv2.sin(),
                  zdiv2.cos() * xdiv2.cos() * ydiv2.sin() - zdiv2.sin() * xdiv2.sin() * ydiv2.cos())
    }

    #[inline]
    pub fn from_angle_axis(radians: T, axis: &Vec3<T>) -> Quat<T> {
        let half = radians / two!(T);
        Quat::from_sv(half.cos(), axis.mul_t(half.sin()))
    }

    pub fn get_angle_axis(&self) -> (T, Vec3<T>) {
        fail!(~"Not yet implemented.")
    }

    /// The result of multiplying the quaternion a scalar
    #[inline]
    pub fn mul_t(&self, value: T) -> Quat<T> {
        Quat::from_sv(self.s * value, self.v.mul_t(value))
    }

    /// The result of dividing the quaternion a scalar
    #[inline]
    pub fn div_t(&self, value: T) -> Quat<T> {
        Quat::from_sv(self.s / value, self.v.div_t(value))
    }

    /// The result of multiplying the quaternion by a vector
    #[inline]
    pub fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T>  {
        let tmp = self.v.cross(vec).add_v(&vec.mul_t(self.s.clone()));
        self.v.cross(&tmp).mul_t(two!(T)).add_v(vec)
    }

    /// The sum of this quaternion and `other`
    #[inline]
    pub fn add_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1),
                  *self.index(2) + *other.index(2),
                  *self.index(3) + *other.index(3))
    }

    /// The sum of this quaternion and `other`
    #[inline]
    pub fn sub_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1),
                  *self.index(2) - *other.index(2),
                  *self.index(3) - *other.index(3))
    }

    /// The the result of multipliplying the quaternion by `other`
    pub fn mul_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(self.s * other.s - self.v.x * other.v.x - self.v.y * other.v.y - self.v.z * other.v.z,
                  self.s * other.v.x + self.v.x * other.s + self.v.y * other.v.z - self.v.z * other.v.y,
                  self.s * other.v.y + self.v.y * other.s + self.v.z * other.v.x - self.v.x * other.v.z,
                  self.s * other.v.z + self.v.z * other.s + self.v.x * other.v.y - self.v.y * other.v.x)
    }

    /// The dot product of the quaternion and `other`
    #[inline]
    pub fn dot(&self, other: &Quat<T>) -> T {
        self.s * other.s + self.v.dot(&other.v)
    }

    /// The conjugate of the quaternion
    #[inline]
    pub fn conjugate(&self) -> Quat<T> {
        Quat::from_sv(self.s.clone(), -self.v.clone())
    }

    /// The multiplicative inverse of the quaternion
    #[inline]
    pub fn inverse(&self) -> Quat<T> {
        self.conjugate().div_t(self.magnitude2())
    }

    /// The squared magnitude of the quaternion. This is useful for
    /// magnitude comparisons where the exact magnitude does not need to be
    /// calculated.
    #[inline]
    pub fn magnitude2(&self) -> T {
        self.s * self.s + self.v.length2()
    }

    /// The magnitude of the quaternion
    ///
    /// # Performance notes
    ///
    /// For instances where the exact magnitude of the quaternion does not need
    /// to be known, for example for quaternion-quaternion magnitude comparisons,
    /// it is advisable to use the `magnitude2` method instead.
    #[inline]
    pub fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }

    /// The normalized quaternion
    #[inline]
    pub fn normalize(&self) -> Quat<T> {
        self.mul_t(one!(T) / self.magnitude())
    }

    /// Normalised linear interpolation
    ///
    /// # Return value
    ///
    /// The intoperlated quaternion
    pub fn nlerp(&self, other: &Quat<T>, amount: T) -> Quat<T> {
        self.mul_t(one!(T) - amount).add_q(&other.mul_t(amount)).normalize()
    }
}

impl<T:Clone + Num> ToMat3<T> for Quat<T> {
    /// Convert the quaternion to a 3 x 3 rotation matrix
    pub fn to_mat3(&self) -> Mat3<T> {
        let x2 = self.v.x + self.v.x;
        let y2 = self.v.y + self.v.y;
        let z2 = self.v.z + self.v.z;

        let xx2 = x2 * self.v.x;
        let xy2 = x2 * self.v.y;
        let xz2 = x2 * self.v.z;

        let yy2 = y2 * self.v.y;
        let yz2 = y2 * self.v.z;
        let zz2 = z2 * self.v.z;

        let sy2 = y2 * self.s;
        let sz2 = z2 * self.s;
        let sx2 = x2 * self.s;

        let _1: T = one!(T);

        Mat3::new(_1 - yy2 - zz2, xy2 + sz2, xz2 - sy2,
                  xy2 - sz2, _1 - xx2 - zz2, yz2 + sx2,
                  xz2 + sy2, yz2 - sx2, _1 - xx2 - yy2)
    }
}

impl<T:Clone + Float> Neg<Quat<T>> for Quat<T> {
    #[inline]
    pub fn neg(&self) -> Quat<T> {
        Quat::from_sv(-self.s, -self.v)
    }
}

impl<T:Clone + Float> Quat<T> {
    #[inline]
    pub fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> Quat<T> {
        Mat3::look_at(dir, up).to_quat()
    }

    #[inline]
    pub fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Quat<T> {
        Mat3::from_axes(x, y, z).to_quat()
    }

    /// Spherical Linear Intoperlation
    ///
    /// Perform a spherical linear interpolation between the quaternion and
    /// `other`. Both quaternions should be normalized first.
    ///
    /// # Return value
    ///
    /// The intoperlated quaternion
    ///
    /// # Performance notes
    ///
    /// The `acos` operation used in `slerp` is an expensive operation, so unless
    /// your quarternions a far away from each other it's generally more advisable
    /// to use `nlerp` when you know your rotations are going to be small.
    ///
    /// - [Understanding Slerp, Then Not Using It]
    ///   (http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/)
    /// - [Arcsynthesis OpenGL tutorial]
    ///   (http://www.arcsynthesis.org/gltut/Positioning/Tut08%20Interpolation.html)
    pub fn slerp(&self, other: &Quat<T>, amount: T) -> Quat<T> {
        use std::num::cast;

        let dot = self.dot(other);
        let dot_threshold = cast(0.9995);

        // if quaternions are close together use `nlerp`
        if dot > dot_threshold {
            self.nlerp(other, amount)
        } else {
            // stay within the domain of acos()
            let robust_dot = dot.clamp(&-one!(T), &one!(T));

            let theta_0 = robust_dot.acos();    // the angle between the quaternions
            let theta = theta_0 * amount;       // the fraction of theta specified by `amount`

            let q = other.sub_q(&self.mul_t(robust_dot))
                         .normalize();

            self.mul_t(theta.cos())
                .add_q(&q.mul_t(theta.sin()))
        }
    }
}

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for Quat<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &Quat<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &Quat<T>, epsilon: &T) -> bool {
        self.s.approx_eq_eps(&other.s, epsilon) &&
        self.v.approx_eq_eps(&other.v, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use core::mat::*;
    use core::quat::*;
    use core::vec::*;

    #[test]
    fn test_from_angle_axis() {
        let v = Vec3::new(1f, 0f, 0f);

        let q = Quat::from_angle_axis((-45f).to_radians(), &Vec3::new(0f, 0f, -1f));

        // http://www.wolframalpha.com/input/?i={1,0}+rotate+-45+degrees
        assert_approx_eq!(q.mul_v(&v), Vec3::new(1f/2f.sqrt(), 1f/2f.sqrt(), 0f));
        assert_eq!(q.mul_v(&v).length(), v.length());
        assert_approx_eq!(q.to_mat3(), Mat3::new( 1f/2f.sqrt(), 1f/2f.sqrt(), 0f,
                                                 -1f/2f.sqrt(), 1f/2f.sqrt(), 0f,
                                                            0f,           0f, 1f));
    }

    #[test]
    fn test_approx_eq() {
        assert!(!Quat::new::<float>(0.000001, 0.000001, 0.000001, 0.000001)
                .approx_eq(&Quat::new::<float>(0.0, 0.0, 0.0, 0.0)));
        assert!(Quat::new::<float>(0.0000001, 0.0000001, 0.0000001, 0.0000001)
                .approx_eq(&Quat::new::<float>(0.0, 0.0, 0.0, 0.0)));
    }
}
