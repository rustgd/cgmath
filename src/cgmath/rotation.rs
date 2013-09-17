// Copyright 2013 The CGMath Developers. For a full listing of the authors,
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

use std::num::{cast, one};

use angle::{Angle, sin, cos, sin_cos};
use array::Array;
use matrix::Matrix;
use matrix::{Mat2, ToMat2};
use matrix::{Mat3, ToMat3};
use point::{Point2, Point3};
use quaternion::{Quat, ToQuat};
use ray::{Ray2, Ray3};
use vector::{Vector, Vec2, Vec3};

/// A two-dimensional rotation
pub trait Rotation2
<
    S
>
:   Eq
+   ApproxEq<S>
+   ToMat2<S>
+   ToRot2<S>
{
    fn rotate_point2(&self, point: &Point2<S>) -> Point2<S>;
    fn rotate_vec2(&self, vec: &Vec2<S>) -> Vec2<S>;
    fn rotate_ray2(&self, ray: &Ray2<S>) -> Ray2<S>;
    fn concat(&self, other: &Self) -> Self;
    fn concat_self(&mut self, other: &Self);
    fn invert(&self) -> Self;
    fn invert_self(&mut self);
}

/// A three-dimensional rotation
pub trait Rotation3
<
    S
>
:   Eq
+   ApproxEq<S>
+   ToMat3<S>
+   ToRot3<S>
+   ToQuat<S>
{
    fn rotate_point3(&self, point: &Point3<S>) -> Point3<S>;
    fn rotate_vec3(&self, vec: &Vec3<S>) -> Vec3<S>;
    fn rotate_ray3(&self, ray: &Ray3<S>) -> Ray3<S>;
    fn concat(&self, other: &Self) -> Self;
    fn concat_self(&mut self, other: &Self);
    fn invert(&self) -> Self;
    fn invert_self(&mut self);
}

/// A two-dimensional rotation matrix.
///
/// The matrix is guaranteed to be orthogonal, so some operations can be
/// implemented more efficiently than the implementations for `math::Mat2`. To
/// enforce orthogonality at the type level the operations have been restricted
/// to a subeset of those implemented on `Mat2`.
#[deriving(Eq, Clone)]
pub struct Rot2<S> {
    priv mat: Mat2<S>
}

impl<S: Float> Rot2<S> {
    #[inline]
    pub fn as_mat2<'a>(&'a self) -> &'a Mat2<S> { &'a self.mat }
}

pub trait ToRot2<S: Float> {
    fn to_rot2(&self) -> Rot2<S>;
}

impl<S: Float> ToRot2<S> for Rot2<S> {
    #[inline]
    fn to_rot2(&self) -> Rot2<S> { self.clone() }
}

impl<S: Float> ToMat2<S> for Rot2<S> {
    #[inline]
    fn to_mat2(&self) -> Mat2<S> { self.mat.clone() }
}

impl<S: Float> Rotation2<S> for Rot2<S> {
    #[inline]
    fn rotate_point2(&self, _point: &Point2<S>) -> Point2<S> { fail!("Not yet implemented") }

    #[inline]
    fn rotate_vec2(&self, vec: &Vec2<S>) -> Vec2<S> { self.mat.mul_v(vec) }

    #[inline]
    fn rotate_ray2(&self, _ray: &Ray2<S>) -> Ray2<S> { fail!("Not yet implemented") }

    #[inline]
    fn concat(&self, other: &Rot2<S>) -> Rot2<S> { Rot2 { mat: self.mat.mul_m(&other.mat) } }

    #[inline]
    fn concat_self(&mut self, other: &Rot2<S>) { self.mat.mul_self_m(&other.mat); }

    // TODO: we know the matrix is orthogonal, so this could be re-written
    // to be faster
    #[inline]
    fn invert(&self) -> Rot2<S> { Rot2 { mat: self.mat.invert().unwrap() } }

    // TODO: we know the matrix is orthogonal, so this could be re-written
    // to be faster
    #[inline]
    fn invert_self(&mut self) { self.mat.invert_self(); }
}

impl<S: Float> ApproxEq<S> for Rot2<S> {
    #[inline]
    fn approx_epsilon() -> S {
        // TODO: fix this after static methods are fixed in rustc
        fail!(~"Doesn't work!");
    }

    #[inline]
    fn approx_eq(&self, other: &Rot2<S>) -> bool {
        self.mat.approx_eq(&other.mat)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Rot2<S>, approx_epsilon: &S) -> bool {
        self.mat.approx_eq_eps(&other.mat, approx_epsilon)
    }
}

/// A three-dimensional rotation matrix.
///
/// The matrix is guaranteed to be orthogonal, so some operations, specifically
/// inversion, can be implemented more efficiently than the implementations for
/// `math::Mat3`. To ensure orthogonality is maintained, the operations have
/// been restricted to a subeset of those implemented on `Mat3`.
#[deriving(Eq, Clone)]
pub struct Rot3<S> {
    priv mat: Mat3<S>
}

impl<S: Float> Rot3<S> {
    #[inline]
    pub fn look_at(dir: &Vec3<S>, up: &Vec3<S>) -> Rot3<S> {
        Rot3 { mat: Mat3::look_at(dir, up) }
    }

    #[inline]
    pub fn as_mat3<'a>(&'a self) -> &'a Mat3<S> { &'a self.mat }
}

pub trait ToRot3<S: Float> {
    fn to_rot3(&self) -> Rot3<S>;
}

impl<S: Float> ToRot3<S> for Rot3<S> {
    #[inline]
    fn to_rot3(&self) -> Rot3<S> { self.clone() }
}

impl<S: Float> ToMat3<S> for Rot3<S> {
    #[inline]
    fn to_mat3(&self) -> Mat3<S> { self.mat.clone() }
}

impl<S: Float> ToQuat<S> for Rot3<S> {
    #[inline]
    fn to_quat(&self) -> Quat<S> { self.mat.to_quat() }
}

impl<S: Float> Rotation3<S> for Rot3<S> {
    #[inline]
    fn rotate_point3(&self, _point: &Point3<S>) -> Point3<S> { fail!("Not yet implemented") }

    #[inline]
    fn rotate_vec3(&self, vec: &Vec3<S>) -> Vec3<S> { self.mat.mul_v(vec) }

    #[inline]
    fn rotate_ray3(&self, _ray: &Ray3<S>) -> Ray3<S> { fail!("Not yet implemented") }

    #[inline]
    fn concat(&self, other: &Rot3<S>) -> Rot3<S> { Rot3 { mat: self.mat.mul_m(&other.mat) } }

    #[inline]
    fn concat_self(&mut self, other: &Rot3<S>) { self.mat.mul_self_m(&other.mat); }

    // TODO: we know the matrix is orthogonal, so this could be re-written
    // to be faster
    #[inline]
    fn invert(&self) -> Rot3<S> { Rot3 { mat: self.mat.invert().unwrap() } }

    // TODO: we know the matrix is orthogonal, so this could be re-written
    // to be faster
    #[inline]
    fn invert_self(&mut self) { self.mat.invert_self(); }
}

impl<S: Float> ApproxEq<S> for Rot3<S> {
    #[inline]
    fn approx_epsilon() -> S {
        // TODO: fix this after static methods are fixed in rustc
        fail!(~"Doesn't work!");
    }

    #[inline]
    fn approx_eq(&self, other: &Rot3<S>) -> bool {
        self.mat.approx_eq(&other.mat)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Rot3<S>, approx_epsilon: &S) -> bool {
        self.mat.approx_eq_eps(&other.mat, approx_epsilon)
    }
}

// Quaternion Rotation impls

impl<S: Float> ToRot3<S> for Quat<S> {
    #[inline]
    fn to_rot3(&self) -> Rot3<S> { Rot3 { mat: self.to_mat3() } }
}

impl<S: Float> ToQuat<S> for Quat<S> {
    #[inline]
    fn to_quat(&self) -> Quat<S> { self.clone() }
}

impl<S: Float> Rotation3<S> for Quat<S> {
    #[inline]
    fn rotate_point3(&self, _point: &Point3<S>) -> Point3<S> { fail!("Not yet implemented") }

    #[inline]
    fn rotate_vec3(&self, vec: &Vec3<S>) -> Vec3<S> { self.mul_v(vec) }

    #[inline]
    fn rotate_ray3(&self, _ray: &Ray3<S>) -> Ray3<S> { fail!("Not yet implemented") }

    #[inline]
    fn concat(&self, other: &Quat<S>) -> Quat<S> { self.mul_q(other) }

    #[inline]
    fn concat_self(&mut self, other: &Quat<S>) { self.mul_self_q(other); }

    #[inline]
    fn invert(&self) -> Quat<S> { self.conjugate().div_s(self.magnitude2()) }

    #[inline]
    fn invert_self(&mut self) { *self = self.invert() }
}

/// Euler angles
///
/// # Fields
///
/// - `x`: the angular rotation around the `x` axis (pitch)
/// - `y`: the angular rotation around the `y` axis (yaw)
/// - `z`: the angular rotation around the `z` axis (roll)
///
/// # Notes
///
/// Whilst Euler angles are more intuitive to specify than quaternions,
/// they are not reccomended for general use because they are prone to gimble
/// lock.
#[deriving(Eq, Clone)]
pub struct Euler<A> { x: A, y: A, z: A }

array!(impl<A> Euler<A> -> [A, ..3] _3)

impl<S: Float, A: Angle<S>> Euler<A> {
    #[inline]
    pub fn new(x: A, y: A, z: A) -> Euler<A> {
        Euler { x: x, y: y, z: z }
    }
}

pub trait ToEuler<A> {
    fn to_euler(&self) -> Euler<A>;
}

impl<S: Float, A: Angle<S>> ToMat3<S> for Euler<A> {
    fn to_mat3(&self) -> Mat3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
        let (sx, cx) = sin_cos(self.x.clone());
        let (sy, cy) = sin_cos(self.y.clone());
        let (sz, cz) = sin_cos(self.z.clone());

        Mat3::new(cy * cz, cy * sz, -sy,
                  -cx * sz + sx * sy * cz, cx * cz + sx * sy * sz, sx * cy,
                  sx * sz + cx * sy * cz, -sx * cz + cx * sy * sz, cx * cy)
    }
}

impl<S: Float, A: Angle<S>> ToRot3<S> for Euler<A> {
    #[inline]
    fn to_rot3(&self) -> Rot3<S> {
        Rot3 { mat: self.to_mat3() }
    }
}

impl<S: Float, A: Angle<S>> ToQuat<S> for Euler<A> {
    fn to_quat(&self) -> Quat<S> {
        // http://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles#Conversion
        let (sx2, cx2) = sin_cos(self.x.div_s(cast(2)));
        let (sy2, cy2) = sin_cos(self.y.div_s(cast(2)));
        let (sz2, cz2) = sin_cos(self.z.div_s(cast(2)));

        Quat::new(cz2 * cx2 * cy2 + sz2 * sx2 * sy2,
                  sz2 * cx2 * cy2 - cz2 * sx2 * sy2,
                  cz2 * sx2 * cy2 + sz2 * cx2 * sy2,
                  cz2 * cx2 * sy2 - sz2 * sx2 * cy2)
    }
}

impl<S: Float, A: Angle<S>> Rotation3<S> for Euler<A> {
    #[inline]
    fn rotate_point3(&self, _point: &Point3<S>) -> Point3<S> { fail!("Not yet implemented") }

    #[inline]
    fn rotate_vec3(&self, _vec: &Vec3<S>) -> Vec3<S> { fail!("Not yet implemented"); }

    #[inline]
    fn rotate_ray3(&self, _ray: &Ray3<S>) -> Ray3<S> { fail!("Not yet implemented") }

    #[inline]
    fn concat(&self, _other: &Euler<A>) -> Euler<A> { fail!("Not yet implemented") }

    #[inline]
    fn concat_self(&mut self, _other: &Euler<A>) { fail!("Not yet implemented"); }

    #[inline]
    fn invert(&self) -> Euler<A> { fail!("Not yet implemented") }

    #[inline]
    fn invert_self(&mut self) { fail!("Not yet implemented"); }
}

impl<S: Float, A: Angle<S>> ApproxEq<S> for Euler<A> {
    #[inline]
    fn approx_epsilon() -> S {
        // TODO: fix this after static methods are fixed in rustc
        fail!(~"Doesn't work!");
    }

    #[inline]
    fn approx_eq(&self, other: &Euler<A>) -> bool {
        self.x.approx_eq(&other.x) &&
        self.y.approx_eq(&other.y) &&
        self.z.approx_eq(&other.z)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Euler<A>, approx_epsilon: &S) -> bool {
        self.x.approx_eq_eps(&other.x, approx_epsilon) &&
        self.y.approx_eq_eps(&other.y, approx_epsilon) &&
        self.z.approx_eq_eps(&other.z, approx_epsilon)
    }
}

/// A rotation about an arbitrary axis
#[deriving(Eq, Clone)]
pub struct AxisAngle<S, A> { v: Vec3<S>, a: A }

impl<S: Float, A: Angle<S>> AxisAngle<S, A> {
    #[inline]
    pub fn new(v: Vec3<S>, a: A) -> AxisAngle<S, A> {
        AxisAngle { v: v, a: a }
    }
}

impl<S: Float, A: Angle<S>> ToMat3<S> for AxisAngle<S, A> {
    fn to_mat3(&self) -> Mat3<S> {
        let (s, c) = sin_cos(self.a.clone());
        let _1subc = one::<S>() - c;

        Mat3::new(_1subc * self.v.x * self.v.x + c,
                  _1subc * self.v.x * self.v.y + s * self.v.z,
                  _1subc * self.v.x * self.v.z - s * self.v.y,

                  _1subc * self.v.x * self.v.y - s * self.v.z,
                  _1subc * self.v.y * self.v.y + c,
                  _1subc * self.v.y * self.v.z + s * self.v.x,

                  _1subc * self.v.x * self.v.z + s * self.v.y,
                  _1subc * self.v.y * self.v.z - s * self.v.x,
                  _1subc * self.v.z * self.v.z + c)
    }
}

impl<S: Float, A: Angle<S>> ToRot3<S> for AxisAngle<S, A> {
    #[inline]
    fn to_rot3(&self) -> Rot3<S> {
        Rot3 { mat: self.to_mat3() }
    }
}

impl<S: Float, A: Angle<S>> ToQuat<S> for AxisAngle<S, A> {
    fn to_quat(&self) -> Quat<S> {
        let half = self.a.div_s(cast(2));
        Quat::from_sv(
            cos(half.clone()),
            self.v.mul_s(sin(half.clone()))
        )
    }
}

impl<S: Float, A: Angle<S>> Rotation3<S> for AxisAngle<S, A> {
    #[inline]
    fn rotate_point3(&self, _point: &Point3<S>) -> Point3<S> { fail!("Not yet implemented") }

    #[inline]
    fn rotate_vec3(&self, _vec: &Vec3<S>) -> Vec3<S> { fail!("Not yet implemented"); }

    #[inline]
    fn rotate_ray3(&self, _ray: &Ray3<S>) -> Ray3<S> { fail!("Not yet implemented") }

    #[inline]
    fn concat(&self, _other: &AxisAngle<S, A>) -> AxisAngle<S, A> { fail!("Not yet implemented") }

    #[inline]
    fn concat_self(&mut self, _other: &AxisAngle<S, A>) { fail!("Not yet implemented"); }

    #[inline]
    fn invert(&self) -> AxisAngle<S, A> { fail!("Not yet implemented") }

    #[inline]
    fn invert_self(&mut self) { fail!("Not yet implemented"); }
}

impl<S: Float, A: Angle<S>> ApproxEq<S> for AxisAngle<S, A> {
    #[inline]
    fn approx_epsilon() -> S {
        // TODO: fix this after static methods are fixed in rustc
        fail!(~"Doesn't work!");
    }

    #[inline]
    fn approx_eq(&self, other: &AxisAngle<S, A>) -> bool {
        self.v.approx_eq(&other.v) &&
        self.a.approx_eq(&other.a)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &AxisAngle<S, A>, approx_epsilon: &S) -> bool {
        self.v.approx_eq_eps(&other.v, approx_epsilon) &&
        self.a.approx_eq_eps(&other.a, approx_epsilon)
    }
}

/// An angle around the X axis (pitch).
#[deriving(Eq, Ord, Clone)]
pub struct AngleX<A>(A);

/// An angle around the X axis (yaw).
#[deriving(Eq, Ord, Clone)]
pub struct AngleY<A>(A);

/// An angle around the Z axis (roll).
#[deriving(Eq, Ord, Clone)]
pub struct AngleZ<A>(A);
