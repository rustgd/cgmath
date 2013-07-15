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

//! Various three-dimensional rotation types that are useful for constructing
//! matricies and quaternions.
//!
//! # Examples
//!
//! ~~~rust
//! Euler::new::<f32>(1.0, 2.0, 0.0).to_mat3()
//! ~~~
//!
//! ~~~rust
//! AxisY::<f32>(0.3).to_quat()
//! ~~~

use math::{Dimensioned, SwapComponents};
use math::{Mat3, ToMat3};
use math::{Quat, ToQuat};
use math::{Vec3, ToVec3, AsVec3};

/// A generic rotation
pub trait Rotation<T>: Eq
                     + ApproxEq<T>
                     + ToMat3<T>
                     + ToQuat<T> {}

/// Euler angles
///
/// # Fields
///
/// - `pitch`: the angular rotation around the `x` axis in radians
/// - `yaw`: the angular rotation around the `y` axis in radians
/// - `roll`: the angular rotation around the `z` axis in radians
#[deriving(Eq, Clone)]
pub struct Euler<T> { pitch: T, yaw: T, roll: T }

impl_dimensioned!(Euler, T, 3)
impl_to_vec!(Euler, 3)
impl_as_vec!(Euler, 3)
impl_swap_components!(Euler)
impl_approx!(Euler { pitch, yaw, roll })

pub trait ToEuler<T> {
    pub fn to_euler(&self) -> Euler<T>;
}

impl<T:Float> Euler<T> {
    #[inline]
    pub fn new(pitch: T, yaw: T, roll: T) -> Euler<T> {
        Euler { pitch: pitch, yaw: yaw, roll: roll }
    }
}

impl<T:Float> ToQuat<T> for Euler<T> {
    pub fn to_quat(&self) -> Quat<T> {
        // http://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles#Conversion
        let xdiv2 = self.pitch / two!(T);
        let ydiv2 = self.yaw / two!(T);
        let zdiv2 = self.roll / two!(T);
        Quat::new(zdiv2.cos() * xdiv2.cos() * ydiv2.cos() + zdiv2.sin() * xdiv2.sin() * ydiv2.sin(),
                  zdiv2.sin() * xdiv2.cos() * ydiv2.cos() - zdiv2.cos() * xdiv2.sin() * ydiv2.sin(),
                  zdiv2.cos() * xdiv2.sin() * ydiv2.cos() + zdiv2.sin() * xdiv2.cos() * ydiv2.sin(),
                  zdiv2.cos() * xdiv2.cos() * ydiv2.sin() - zdiv2.sin() * xdiv2.sin() * ydiv2.cos())
    }
}

impl<T:Float> ToMat3<T> for Euler<T> {
    pub fn to_mat3(&self) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
        let cx = self.pitch.cos();
        let sx = self.pitch.sin();
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();
        let cz = self.roll.cos();
        let sz = self.roll.sin();

        Mat3::new(cy * cz, cy * sz, -sy,
                  -cx * sz + sx * sy * cz, cx * cz + sx * sy * sz, sx * cy,
                  sx * sz + cx * sy * cz, -sx * cz + cx * sy * sz, cx * cy)
    }
}

#[cfg(test)]
mod euler_tests {
    // TODO
}

/// A rotation about an arbitrary axis
///
/// # Fields
///
/// - `axis`: The axis vector about which to rotate.
/// - `angle`: The angle of rotation in radians.
#[deriving(Eq, Clone)]
pub struct AxisAngle<T> {
    axis: Vec3<T>,
    angle: T,
}

impl_approx!(AxisAngle { axis, angle })

pub trait ToAxisAngle<T> {
    pub fn to_axis_angle(&self) -> AxisAngle<T>;
}

impl<T:Float> AxisAngle<T> {
    pub fn new(axis: Vec3<T>, angle: T) -> AxisAngle<T> {
        AxisAngle { axis: axis, angle: angle }
    }
}

impl<T:Float> ToQuat<T> for AxisAngle<T> {
    pub fn to_quat(&self) -> Quat<T> {
        let half = self.angle / two!(T);
        Quat::from_sv(half.cos(), self.axis.mul_t(half.sin()))
    }
}

impl<T:Float> ToMat3<T> for AxisAngle<T> {
    pub fn to_mat3(&self) -> Mat3<T> {
        let c = self.angle.cos();
        let s = self.angle.sin();
        let _1_c = one!(T) - c;

        Mat3::new(_1_c * self.axis.x * self.axis.x + c,
                  _1_c * self.axis.x * self.axis.y + s * self.axis.z,
                  _1_c * self.axis.x * self.axis.z - s * self.axis.y,

                  _1_c * self.axis.x * self.axis.y - s * self.axis.z,
                  _1_c * self.axis.y * self.axis.y + c,
                  _1_c * self.axis.y * self.axis.z + s * self.axis.x,

                  _1_c * self.axis.x * self.axis.z + s * self.axis.y,
                  _1_c * self.axis.y * self.axis.z - s * self.axis.x,
                  _1_c * self.axis.z * self.axis.z + c)
    }
}

#[cfg(test)]
mod axis_angle_tests {
    use math::mat::*;
    use math::quat::*;
    use math::rotation::*;
    use math::vec::*;

    #[test]
    fn test_to_quat() {
        let v = Vec3::new(1f, 0f, 0f);

        let q = AxisAngle::new(Vec3::new(0f, 0f, -1f), (-45f).to_radians()).to_quat();

        // http://www.wolframalpha.com/input/?i={1,0}+rotate+-45+degrees
        assert_approx_eq!(q.mul_v(&v), Vec3::new(1f/2f.sqrt(), 1f/2f.sqrt(), 0f));
        assert_eq!(q.mul_v(&v).magnitude(), v.magnitude());
        assert_approx_eq!(q.to_mat3(), Mat3::new( 1f/2f.sqrt(), 1f/2f.sqrt(), 0f,
                                                 -1f/2f.sqrt(), 1f/2f.sqrt(), 0f,
                                                            0f,           0f, 1f));
    }
}

/// An angle around the X axis (pitch), in radians.
#[deriving(Eq, Clone)]
pub struct AngleX<T>(T);

impl_approx!(AngleX)

impl<T:Float> ToQuat<T> for AngleX<T> {
    pub fn to_quat(&self) -> Quat<T> {
        Quat::new(((**self) / two!(T)).cos(), (**self).sin(), zero!(T), zero!(T))
    }
}

impl<T:Clone + Float> ToMat3<T> for AngleX<T> {
    pub fn to_mat3(&self) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = (**self).cos();
        let sin_theta = (**self).sin();

        Mat3::new(one!(T), zero!(T), zero!(T),
                  zero!(T), cos_theta.clone(), sin_theta.clone(),
                  zero!(T), -sin_theta.clone(), cos_theta.clone())
    }
}

#[cfg(test)]
mod angle_x_tests {
    // TODO
}

/// An angle around the X axis (yaw), in radians.
#[deriving(Eq, Clone)]
pub struct AngleY<T>(T);

impl_approx!(AngleY)

impl<T:Float> ToQuat<T> for AngleY<T> {
    pub fn to_quat(&self) -> Quat<T> {
        Quat::new(((**self) / two!(T)).cos(), zero!(T), (**self).sin(), zero!(T))
    }
}

impl<T:Clone + Float> ToMat3<T> for AngleY<T> {
    pub fn to_mat3(&self) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = (**self).cos();
        let sin_theta = (**self).sin();

        Mat3::new(cos_theta.clone(), zero!(T), -sin_theta.clone(),
                  zero!(T), one!(T), zero!(T),
                  sin_theta.clone(), zero!(T), cos_theta.clone())
    }
}

#[cfg(test)]
mod angle_y_tests {
    // TODO
}

/// An angle around the Z axis (roll), in radians.
#[deriving(Eq, Clone)]
pub struct AngleZ<T>(T);

impl_approx!(AngleZ)

impl<T:Float> ToQuat<T> for AngleZ<T> {
    pub fn to_quat(&self) -> Quat<T> {
        Quat::new(((**self) / two!(T)).cos(), zero!(T), zero!(T), (**self).sin())
    }
}

impl<T:Clone + Float> ToMat3<T> for AngleZ<T> {
    pub fn to_mat3(&self) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = (**self).cos();
        let sin_theta = (**self).sin();

        Mat3::new(cos_theta.clone(), sin_theta.clone(), zero!(T),
                  -sin_theta.clone(), cos_theta.clone(), zero!(T),
                  zero!(T), zero!(T), one!(T))
    }
}

#[cfg(test)]
mod angle_z_tests {
    // TODO
}
