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

use std::cast;

use math::{Dimensioned, SwapComponents};
use math::{Mat, NumMat, FloatMat};
use math::{Mat3, ToMat3};
use math::{Mat4, ToMat4};
use math::{Quat, ToQuat};
use math::{Vec3, ToVec3, AsVec3};

use math::{Point3, Ray3};

/// A generic rotation
pub trait Rotation<T>: Eq
                     + ApproxEq<T>
                     + ToMat3<T>
                     + ToMat4<T>
                     + ToQuat<T> {
    pub fn rotate_point3(&self, point: Point3<T>) -> Point3<T>;
    pub fn rotate_vec3(&self, vec: Vec3<T>) -> Point3<T>;
    pub fn rotate_ray3(&self, vec: Ray3<T>) -> Ray3<T>;
}

impl<T:Float> Rotation<T> for Quat<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: Vec3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _vec: Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }
}

/// A rotation matrix
#[deriving(Eq, Clone)]
pub struct RotationMat<T> {
    priv mat: Mat3<T>
}

impl_dimensioned!(RotationMat, Vec3<T>, 3)

impl<T> RotationMat<T> {
    #[inline]
    pub fn as_mat3<'a>(&'a self) -> & 'a Mat3<T> {
        unsafe { cast::transmute(self) }
    }
}

impl<T:Float> Rotation<T> for RotationMat<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: Vec3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _vec: Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }
}

impl<T:Clone + Float> ToQuat<T> for RotationMat<T> {
    #[inline] pub fn to_quat(&self) -> Quat<T> { self.mat.to_quat() }
}

impl<T:Clone + Float> ToMat3<T> for RotationMat<T> {
    #[inline] pub fn to_mat3(&self) -> Mat3<T> { self.mat.clone() }
}

impl<T:Clone + Float> ToMat4<T> for RotationMat<T> {
    #[inline] pub fn to_mat4(&self) -> Mat4<T> { self.mat.to_mat4() }
}

impl<T:Clone> Mat<T,Vec3<T>,[Vec3<T>,..3]> for RotationMat<T> {
    #[inline]
    pub fn col<'a>(&'a self, i: uint) -> &'a Vec3<T> { self.mat.col(i) }

    #[inline]
    pub fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut Vec3<T> { self.mat.col_mut(i) }

    #[inline]
    pub fn elem<'a>(&'a self, col: uint, row: uint) -> &'a T { self.mat.elem(col, row) }

    #[inline]
    pub fn elem_mut<'a>(&'a mut self, col: uint, row: uint) -> &'a mut T { self.mat.elem_mut(col, row) }

    #[inline]
    pub fn swap_cols(&mut self, a: uint, b: uint) { self.mat.swap_cols(a, b) }

    #[inline]
    pub fn row(&self, i: uint) -> Vec3<T> { self.mat.row(i) }

    #[inline]
    pub fn swap_rows(&mut self, a: uint, b: uint) { self.mat.swap_rows(a, b) }

    #[inline]
    pub fn swap_elem(&mut self, a: (uint, uint), b: (uint, uint)) { self.mat.swap_elem(a, b) }

    #[inline]
    pub fn transpose(&self) -> RotationMat<T> { RotationMat { mat: self.mat.transpose() } }

    #[inline]
    pub fn transpose_self(&mut self) { self.mat.transpose_self() }

}

impl<T:Num> RotationMat<T> {
    #[inline]
    pub fn identity() -> RotationMat<T> {
        RotationMat { mat: Mat3::identity() }
    }

    #[inline]
    pub fn zero() -> RotationMat<T> {
        RotationMat { mat: Mat3::zero() }
    }
}

impl<T:Clone + Num> RotationMat<T> {
    #[inline]
    pub fn from_value(value: T) -> RotationMat<T> {
        RotationMat { mat: Mat3::from_value(value) }
    }
}

impl<T:Clone + Num> NumMat<T,Vec3<T>,[Vec3<T>,..3]> for RotationMat<T> {
    #[inline]
    pub fn mul_t(&self, value: T) -> RotationMat<T> {
        RotationMat { mat: self.mat.mul_t(value) }
    }

    #[inline]
    pub fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T> { self.mat.mul_v(vec) }

    #[inline]
    pub fn add_m(&self, other: &RotationMat<T>) -> RotationMat<T> {
        RotationMat { mat: self.mat.add_m(&other.mat) }
    }

    #[inline]
    pub fn sub_m(&self, other: &RotationMat<T>) -> RotationMat<T> {
        RotationMat { mat: self.mat.sub_m(&other.mat) }
    }

    #[inline]
    pub fn mul_m(&self, other: &RotationMat<T>) -> RotationMat<T> {
        RotationMat { mat: self.mat.mul_m(&other.mat) }
    }

    #[inline]
    pub fn mul_self_t(&mut self, value: T) { self.mat.mul_self_t(value) }

    #[inline]
    pub fn add_self_m(&mut self, other: &RotationMat<T>) { self.mat.add_self_m(&other.mat) }

    #[inline]
    pub fn sub_self_m(&mut self, other: &RotationMat<T>) { self.mat.sub_self_m(&other.mat) }

    #[inline]
    pub fn dot(&self, other: &RotationMat<T>) -> T { self.mat.dot(&other.mat) }

    #[inline]
    pub fn determinant(&self) -> T { self.mat.determinant() }

    #[inline]
    pub fn trace(&self) -> T { self.mat.trace() }

    #[inline]
    pub fn to_identity(&mut self) { self.mat.to_identity() }

    #[inline]
    pub fn to_zero(&mut self) { self.mat.to_zero() }
}

impl<T:Clone + Num> Neg<RotationMat<T>> for RotationMat<T> {
    #[inline]
    pub fn neg(&self) -> RotationMat<T> {
        RotationMat { mat: -self.mat }
    }
}

impl<T:Float> RotationMat<T> {
    pub fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> RotationMat<T> {
        RotationMat { mat: Mat3::look_at(dir, up) }
    }
}

impl<T:Clone + Float> FloatMat<T,Vec3<T>,[Vec3<T>,..3]> for RotationMat<T> {
    #[inline]
    pub fn inverse(&self) -> Option<RotationMat<T>> {
        unsafe { cast::transmute(self.mat.inverse()) }
    }

    #[inline]
    pub fn invert_self(&mut self) { self.mat.invert_self() }

    #[inline]
    pub fn is_identity(&self) -> bool { self.mat.is_identity() }

    #[inline]
    pub fn is_diagonal(&self) -> bool { self.mat.is_diagonal() }

    #[inline]
    pub fn is_rotated(&self) -> bool { self.mat.is_rotated() }

    #[inline]
    pub fn is_symmetric(&self) -> bool { self.mat.is_symmetric() }

    #[inline]
    pub fn is_invertible(&self) -> bool { self.mat.is_invertible() }
}

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for RotationMat<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &RotationMat<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &RotationMat<T>, epsilon: &T) -> bool {
        self.mat.approx_eq_eps(&other.mat, epsilon)
    }
}

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

impl<T:Float> Rotation<T> for Euler<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: Vec3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _vec: Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
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

impl<T:Float> ToMat4<T> for Euler<T> {
    pub fn to_mat4(&self) -> Mat4<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
        let cx = self.pitch.cos();
        let sx = self.pitch.sin();
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();
        let cz = self.roll.cos();
        let sz = self.roll.sin();

        Mat4::new(cy * cz, cy * sz, -sy, zero!(T),
                  -cx * sz + sx * sy * cz, cx * cz + sx * sy * sz, sx * cy, zero!(T),
                  sx * sz + cx * sy * cz, -sx * cz + cx * sy * sz, cx * cy, zero!(T),
                  zero!(T), zero!(T), zero!(T), one!(T))
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

impl<T:Float> Rotation<T> for AxisAngle<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: Vec3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _vec: Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
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

impl<T:Float> ToMat4<T> for AxisAngle<T> {
    pub fn to_mat4(&self) -> Mat4<T> {
        let c = self.angle.cos();
        let s = self.angle.sin();
        let _1_c = one!(T) - c;

        Mat4::new(_1_c * self.axis.x * self.axis.x + c,
                  _1_c * self.axis.x * self.axis.y + s * self.axis.z,
                  _1_c * self.axis.x * self.axis.z - s * self.axis.y,
                  zero!(T),

                  _1_c * self.axis.x * self.axis.y - s * self.axis.z,
                  _1_c * self.axis.y * self.axis.y + c,
                  _1_c * self.axis.y * self.axis.z + s * self.axis.x,
                  zero!(T),

                  _1_c * self.axis.x * self.axis.z + s * self.axis.y,
                  _1_c * self.axis.y * self.axis.z - s * self.axis.x,
                  _1_c * self.axis.z * self.axis.z + c,
                  zero!(T),

                  zero!(T), zero!(T), zero!(T), one!(T))
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

impl<T:Float> Rotation<T> for AngleX<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: Vec3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _vec: Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }
}

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

impl<T:Clone + Float> ToMat4<T> for AngleX<T> {
    pub fn to_mat4(&self) -> Mat4<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = (**self).cos();
        let sin_theta = (**self).sin();

        Mat4::new(one!(T), zero!(T), zero!(T), zero!(T),
                  zero!(T), cos_theta.clone(), sin_theta.clone(), zero!(T),
                  zero!(T), -sin_theta.clone(), cos_theta.clone(), zero!(T),
                  zero!(T), zero!(T), zero!(T), one!(T))
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

impl<T:Float> Rotation<T> for AngleY<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: Vec3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _vec: Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }
}

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

impl<T:Clone + Float> ToMat4<T> for AngleY<T> {
    pub fn to_mat4(&self) -> Mat4<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = (**self).cos();
        let sin_theta = (**self).sin();

        Mat4::new(cos_theta.clone(), zero!(T), -sin_theta.clone(), zero!(T),
                  zero!(T), one!(T), zero!(T), zero!(T),
                  sin_theta.clone(), zero!(T), cos_theta.clone(), zero!(T),
                  zero!(T), zero!(T), zero!(T), one!(T))
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

impl<T:Float> Rotation<T> for AngleZ<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: Vec3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _vec: Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }
}

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

impl<T:Clone + Float> ToMat4<T> for AngleZ<T> {
    pub fn to_mat4(&self) -> Mat4<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = (**self).cos();
        let sin_theta = (**self).sin();

        Mat4::new(cos_theta.clone(), sin_theta.clone(), zero!(T), zero!(T),
                  -sin_theta.clone(), cos_theta.clone(), zero!(T), zero!(T),
                  zero!(T), zero!(T), one!(T), zero!(T),
                  zero!(T), zero!(T), zero!(T), one!(T))
    }
}

#[cfg(test)]
mod angle_z_tests {
    // TODO
}
