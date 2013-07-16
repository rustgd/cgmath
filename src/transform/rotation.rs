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

//! Various three-dimensional rotation types and impls.
//!
//! Some of these are more useful for constructing matricies and quaternions.
//! than for general use. For example due to issues with gimble lock, it is
//! not reccomended that Euler rotations be used for translations, but
//! they _are_ useful for intuitively specifying rotations.
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

use math::*;

use math::{Point3, Ray3};

/// A two-dimensional rotation
pub trait Rotation2<T>: Eq
                      + ApproxEq<T>
                      + ToMat2<T> {
    pub fn rotate_point2(&self, point: Point2<T>) -> Point2<T>;
    pub fn rotate_vec2(&self, vec: &Vec2<T>) -> Vec2<T>;
    pub fn rotate_ray2(&self, ray: &Ray2<T>) -> Ray2<T>;
    pub fn to_rotation_mat2(&self) -> RotationMat2<T>;
}

/// A three-dimensional rotation
pub trait Rotation3<T>: Eq
                     + ApproxEq<T>
                     + ToMat3<T>
                     + ToMat4<T>
                     + ToQuat<T> {
    pub fn rotate_point3(&self, point: Point3<T>) -> Point3<T>;
    pub fn rotate_vec3(&self, vec: &Vec3<T>) -> Vec3<T>;
    pub fn rotate_ray3(&self, ray: &Ray3<T>) -> Ray3<T>;
    pub fn to_rotation_mat3(&self) -> RotationMat3<T>;
}

/// A two-dimensional rotation matrix.
///
/// The matrix is guaranteed to be orthogonal, so some operations can be
/// implemented more efficiently than the implementations for `math::Mat2`. To
/// enforce orthogonality at the type level the operations have been restricted
/// to a subeset of those implemented on `Mat3`.
#[deriving(Eq, Clone)]
pub struct RotationMat2<T> {
    priv mat: Mat2<T>
}

impl<T> RotationMat2<T> {
    #[inline]
    pub fn as_mat2<'a>(&'a self) -> & 'a Mat2<T> {
        unsafe { cast::transmute(self) }
    }
}

impl<T:Clone + Float> Rotation2<T> for RotationMat2<T> {
    pub fn rotate_point2(&self, point: Point2<T>) -> Point2<T> {
        Point2::from_vec2(self.mat.mul_v(point.as_vec2()))
    }

    pub fn rotate_vec2(&self, vec: &Vec2<T>) -> Vec2<T> {
        self.mat.mul_v(vec)
    }

    pub fn rotate_ray2(&self, _ray: &Ray2<T>) -> Ray2<T> {
        fail!("Not yet implemented.")
    }

    #[inline]
    pub fn to_rotation_mat2(&self) -> RotationMat2<T> {
        RotationMat2 { mat: self.to_mat2() }
    }
}

impl<T:Clone + Float> ToMat2<T> for RotationMat2<T> {
    #[inline] pub fn to_mat2(&self) -> Mat2<T> { self.mat.clone() }
}

impl<T:Num> RotationMat2<T> {
    #[inline]
    pub fn identity() -> RotationMat2<T> {
        RotationMat2 { mat: Mat2::identity() }
    }

    #[inline]
    pub fn zero() -> RotationMat2<T> {
        RotationMat2 { mat: Mat2::zero() }
    }
}

impl<T:Clone + Num> Neg<RotationMat2<T>> for RotationMat2<T> {
    #[inline]
    pub fn neg(&self) -> RotationMat2<T> {
        RotationMat2 { mat: -self.mat }
    }
}

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for RotationMat2<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &RotationMat2<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &RotationMat2<T>, epsilon: &T) -> bool {
        self.mat.approx_eq_eps(&other.mat, epsilon)
    }
}


impl<T:Clone + Float> Rotation3<T> for Quat<T> {
    pub fn rotate_point3(&self, point: Point3<T>) -> Point3<T> {
        Point3::from_vec3(self.mul_v(point.as_vec3()))
    }

    pub fn rotate_vec3(&self, vec: &Vec3<T>) -> Vec3<T> {
        self.mul_v(vec)
    }

    pub fn rotate_ray3(&self, _ray: &Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }

    #[inline]
    pub fn to_rotation_mat3(&self) -> RotationMat3<T> {
        RotationMat3 { mat: self.to_mat3() }
    }
}

/// A three-dimensional rotation matrix.
///
/// The matrix is guaranteed to be orthogonal, so some operations, specifically
/// inversion, can be implemented more efficiently than the implementations for
/// `math::Mat3`. To enforce orthogonality at the type level the operations have
/// been restricted to a subeset of those implemented on `Mat3`.
#[deriving(Eq, Clone)]
pub struct RotationMat3<T> {
    priv mat: Mat3<T>
}

impl<T> RotationMat3<T> {
    #[inline]
    pub fn as_mat3<'a>(&'a self) -> & 'a Mat3<T> {
        unsafe { cast::transmute(self) }
    }
}

impl<T:Clone + Float> Rotation3<T> for RotationMat3<T> {
    pub fn rotate_point3(&self, point: Point3<T>) -> Point3<T> {
        Point3::from_vec3(self.mat.mul_v(point.as_vec3()))
    }

    pub fn rotate_vec3(&self, vec: &Vec3<T>) -> Vec3<T> {
        self.mat.mul_v(vec)
    }

    pub fn rotate_ray3(&self, _ray: &Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }

    #[inline]
    pub fn to_rotation_mat3(&self) -> RotationMat3<T> {
        RotationMat3 { mat: self.to_mat3() }
    }
}

impl<T:Clone + Float> ToQuat<T> for RotationMat3<T> {
    #[inline] pub fn to_quat(&self) -> Quat<T> { self.mat.to_quat() }
}

impl<T:Clone + Float> ToMat3<T> for RotationMat3<T> {
    #[inline] pub fn to_mat3(&self) -> Mat3<T> { self.mat.clone() }
}

impl<T:Clone + Float> ToMat4<T> for RotationMat3<T> {
    #[inline] pub fn to_mat4(&self) -> Mat4<T> { self.mat.to_mat4() }
}

impl<T:Num> RotationMat3<T> {
    #[inline]
    pub fn identity() -> RotationMat3<T> {
        RotationMat3 { mat: Mat3::identity() }
    }

    #[inline]
    pub fn zero() -> RotationMat3<T> {
        RotationMat3 { mat: Mat3::zero() }
    }
}

impl<T:Clone + Num> Neg<RotationMat3<T>> for RotationMat3<T> {
    #[inline]
    pub fn neg(&self) -> RotationMat3<T> {
        RotationMat3 { mat: -self.mat }
    }
}

impl<T:Float> RotationMat3<T> {
    pub fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> RotationMat3<T> {
        RotationMat3 { mat: Mat3::look_at(dir, up) }
    }
}

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for RotationMat3<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &RotationMat3<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &RotationMat3<T>, epsilon: &T) -> bool {
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

impl<T:Clone + Float> Rotation3<T> for Euler<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: &Vec3<T>) -> Vec3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _ray: &Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }

    #[inline]
    pub fn to_rotation_mat3(&self) -> RotationMat3<T> {
        RotationMat3 { mat: self.to_mat3() }
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

impl<T:Float> Rotation3<T> for AxisAngle<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: &Vec3<T>) -> Vec3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _ray: &Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }

    #[inline]
    pub fn to_rotation_mat3(&self) -> RotationMat3<T> {
        RotationMat3 { mat: self.to_mat3() }
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
    use math::*;
    use transform::*;

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

impl<T:Clone + Float> Rotation3<T> for AngleX<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: &Vec3<T>) -> Vec3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _ray: &Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }

    #[inline]
    pub fn to_rotation_mat3(&self) -> RotationMat3<T> {
        RotationMat3 { mat: self.to_mat3() }
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

impl<T:Clone + Float> Rotation3<T> for AngleY<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: &Vec3<T>) -> Vec3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _ray: &Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }

    #[inline]
    pub fn to_rotation_mat3(&self) -> RotationMat3<T> {
        RotationMat3 { mat: self.to_mat3() }
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

impl<T:Float> Rotation2<T> for AngleZ<T> {
    pub fn rotate_point2(&self, _point: Point2<T>) -> Point2<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec2(&self, _vec: &Vec2<T>) -> Vec2<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray2(&self, _ray: &Ray2<T>) -> Ray2<T> {
        fail!("Not yet implemented.")
    }

    #[inline]
    pub fn to_rotation_mat2(&self) -> RotationMat2<T> {
        fail!("Not yet implemented.")
    }
}

impl<T:Clone + Float> Rotation3<T> for AngleZ<T> {
    pub fn rotate_point3(&self, _point: Point3<T>) -> Point3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_vec3(&self, _vec: &Vec3<T>) -> Vec3<T> {
        fail!("Not yet implemented.")
    }

    pub fn rotate_ray3(&self, _ray: &Ray3<T>) -> Ray3<T> {
        fail!("Not yet implemented.")
    }

    #[inline]
    pub fn to_rotation_mat3(&self) -> RotationMat3<T> {
        RotationMat3 { mat: self.to_mat3() }
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
