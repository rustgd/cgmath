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

use std::cast::transmute;
use std::cmp::ApproxEq;
use std::num::{Zero, One, cast};

use mat::Mat3;
use vec::Vec3;

// FIXME: We can remove this once we have numeric conversions in std
#[inline]
priv fn two<T:Num>() -> T {
    One::one::<T>() + One::one::<T>()
}

/// A quaternion in scalar/vector form
///
/// # Type parameters
///
/// - `T`: The type of the components. Should be a floating point type.
///
/// # Fields
///
/// - `s`: the scalar component
/// - `v`: a vector containing the three imaginary components
#[deriving(Eq)]
pub struct Quat<T> { s: T, v: Vec3<T> }

impl<T> Quat<T> {
    #[inline]
    pub fn index<'a>(&'a self, i: uint) -> &'a T {
        &'a self.as_slice()[i]
    }

    #[inline]
    pub fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [T,..4] {
        unsafe { transmute(self) }
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T,..4] {
        unsafe { transmute(self) }
    }
}

impl<T:Copy> Quat<T> {
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

    #[inline]
    pub fn swap(&mut self, a: uint, b: uint) {
        let tmp = *self.index(a);
        *self.index_mut(a) = *self.index(b);
        *self.index_mut(b) = tmp;
    }

    #[inline]
    pub fn map(&self, f: &fn(&T) -> T) -> Quat<T> {
        Quat::new(f(self.index(0)),
                  f(self.index(1)),
                  f(self.index(2)),
                  f(self.index(3)))
    }
}

impl<T:Copy + Real> Quat<T> {
    /// The multiplicative identity, ie: `q = 1 + 0i + 0j + 0i`
    #[inline]
    pub fn identity() -> Quat<T> {
        Quat::new(One::one(),
                  Zero::zero(),
                  Zero::zero(),
                  Zero::zero())
    }

    /// The additive identity, ie: `q = 0 + 0i + 0j + 0i`
    #[inline]
    pub fn zero() -> Quat<T> {
        Quat::new(Zero::zero(),
                  Zero::zero(),
                  Zero::zero(),
                  Zero::zero())
    }

    #[inline]
    pub fn from_angle_x(radians: T) -> Quat<T> {
        Quat::new((radians / two()).cos(),
                  radians.sin(),
                  Zero::zero(),
                  Zero::zero())
    }

    #[inline]
    pub fn from_angle_y(radians: T) -> Quat<T> {
        Quat::new((radians / two()).cos(),
                  Zero::zero(),
                  radians.sin(),
                  Zero::zero())
    }

    #[inline]
    pub fn from_angle_z(radians: T) -> Quat<T> {
        Quat::new((radians / two()).cos(),
                  Zero::zero(),
                  Zero::zero(),
                  radians.sin())
    }

    pub fn from_angle_xyz(radians_x: T, radians_y: T, radians_z: T) -> Quat<T> {
        // http://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles#Conversion
        let xdiv2 = radians_x / two();
        let ydiv2 = radians_y / two();
        let zdiv2 = radians_z / two();
        Quat::new(zdiv2.cos() * xdiv2.cos() * ydiv2.cos() + zdiv2.sin() * xdiv2.sin() * ydiv2.sin(),
                  zdiv2.sin() * xdiv2.cos() * ydiv2.cos() - zdiv2.cos() * xdiv2.sin() * ydiv2.sin(),
                  zdiv2.cos() * xdiv2.sin() * ydiv2.cos() + zdiv2.sin() * xdiv2.cos() * ydiv2.sin(),
                  zdiv2.cos() * xdiv2.cos() * ydiv2.sin() - zdiv2.sin() * xdiv2.sin() * ydiv2.cos())
    }

    #[inline]
    pub fn from_angle_axis(radians: T, axis: &Vec3<T>) -> Quat<T> {
        let half = radians / two();
        Quat::from_sv(half.cos(), axis.mul_t(half.sin()))
    }

    pub fn get_angle_axis(&self) -> (T, Vec3<T>) {
        fail!(~"Not yet implemented.")
    }

    /// The result of multiplying the quaternion a scalar
    #[inline]
    pub fn mul_t(&self, value: T) -> Quat<T> {
        Quat::new(*self.index(0) * value,
                  *self.index(1) * value,
                  *self.index(2) * value,
                  *self.index(3) * value)
    }

    /// The result of dividing the quaternion a scalar
    #[inline]
    pub fn div_t(&self, value: T) -> Quat<T> {
        Quat::new(*self.index(0) / value,
                  *self.index(1) / value,
                  *self.index(2) / value,
                  *self.index(3) / value)
    }

    /// The result of multiplying the quaternion by a vector
    #[inline]
    pub fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T>  {
        let tmp = self.v.cross(vec).add_v(&vec.mul_t(self.s));
        self.v.cross(&tmp).mul_t(two()).add_v(vec)
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
        Quat::from_sv(self.s, -self.v)
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
        self.mul_t(One::one::<T>() / self.magnitude())
    }

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

        let _1: T = One::one();

        Mat3::new(_1 - yy2 - zz2, xy2 + sz2, xz2 - sy2,
                  xy2 - sz2, _1 - xx2 - zz2, yz2 + sx2,
                  xz2 + sy2, yz2 - sx2, _1 - xx2 - yy2)
    }

    /// Normalised linear interpolation
    ///
    /// # Return value
    ///
    /// The intoperlated quaternion
    pub fn nlerp(&self, other: &Quat<T>, amount: T) -> Quat<T> {
        self.mul_t(One::one::<T>() - amount).add_q(&other.mul_t(amount)).normalize()
    }
}

impl<T:Copy + Float> Neg<Quat<T>> for Quat<T> {
    #[inline]
    pub fn neg(&self) -> Quat<T> {
        Quat::new(-*self.index(0),
                  -*self.index(1),
                  -*self.index(2),
                  -*self.index(3))
    }
}

impl<T:Copy + Float> Quat<T> {
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
        let dot = self.dot(other);
        let dot_threshold = cast(0.9995);

        if dot > dot_threshold {
            self.nlerp(other, amount)                       // if quaternions are close together use `nlerp`
        } else {
            let robust_dot = dot.clamp(&-One::one::<T>(),
                                       &One::one());        // stay within the domain of acos()

            let theta_0 = robust_dot.acos();                // the angle between the quaternions
            let theta = theta_0 * amount;                   // the fraction of theta specified by `amount`

            let q = other.sub_q(&self.mul_t(robust_dot))
                         .normalize();

            self.mul_t(theta.cos())
                .add_q(&q.mul_t(theta.sin()))
        }
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Quat<T> {
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
        self.index(0).approx_eq_eps(other.index(0), epsilon) &&
        self.index(1).approx_eq_eps(other.index(1), epsilon) &&
        self.index(2).approx_eq_eps(other.index(2), epsilon) &&
        self.index(3).approx_eq_eps(other.index(3), epsilon)
    }
}

// GLSL-style type aliases
type quat  = Quat<f32>;
type dquat = Quat<f64>;

// Rust-style type aliases
type Quatf   = Quat<float>;
type Quatf32 = Quat<f32>;
type Quatf64 = Quat<f64>;
