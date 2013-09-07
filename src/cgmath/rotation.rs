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

use angle::Angle;
use array::Array;
use matrix::{Mat2, ToMat2};
use matrix::{Mat3, ToMat3};
use point::{Point2, Point3};
use quaternion::ToQuat;
use ray::{Ray2, Ray3};
use vector::{Vec2, Vec3};

/// A two-dimensional rotation
pub trait Rotation2
<
    S
>
:   Eq
+   ApproxEq<S>
+   Neg<Self>
+   Add<Self, Self>
+   Sub<Self, Self>
+   ToMat2<S>
+   ToRot2<S>
{
    fn rotate_point2(&self, point: Point2<S>) -> Point2<S>;
    fn rotate_vec2(&self, vec: &Vec2<S>) -> Vec2<S>;
    fn rotate_ray2(&self, ray: &Ray2<S>) -> Ray2<S>;
}

/// A three-dimensional rotation
pub trait Rotation3
<
    S
>
:   Eq
+   ApproxEq<S>
+   Neg<Self>
+   Add<Self, Self>
+   Sub<Self, Self>
+   ToMat3<S>
+   ToRot3<S>
+   ToQuat<S>
{
    fn rotate_point3(&self, point: &Point3<S>) -> Point3<S>;
    fn rotate_vec3(&self, vec: &Vec3<S>) -> Vec3<S>;
    fn rotate_ray3(&self, ray: &Ray3<S>) -> Ray3<S>;
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

pub trait ToRot2<S: Clone + Float> {
    fn to_rot2(&self) -> Rot2<S>;
}

/// A three-dimensional rotation matrix.
///
/// The matrix is guaranteed to be orthogonal, so some operations, specifically
/// inversion, can be implemented more efficiently than the implementations for
/// `math::Mat3`. To enforce orthogonality at the type level the operations have
/// been restricted to a subeset of those implemented on `Mat3`.
#[deriving(Eq, Clone)]
pub struct Rot3<S> {
    priv mat: Mat3<S>
}

pub trait ToRot3<S: Clone + Float> {
    fn to_rot3(&self) -> Rot3<S>;
}

/// Euler angles
///
/// Whilst Euler angles are easier to visualise, and more intuitive to specify,
/// they are not reccomended for general use because they are prone to gimble
/// lock.
///
/// # Fields
///
/// - `x`: the angular rotation around the `x` axis (pitch)
/// - `y`: the angular rotation around the `y` axis (yaw)
/// - `z`: the angular rotation around the `z` axis (roll)
#[deriving(Eq, Clone)]
pub struct Euler<A> { x: A, y: A, z: A }

array!(impl<A> Euler<A> -> [A, ..3] _3)

pub trait ToEuler<A> {
    fn to_euler(&self) -> Euler<A>;
}

impl<S: Clone + Float, A: Angle<S>> Euler<A> {
    #[inline]
    pub fn new(x: A, y: A, z: A) -> Euler<A> {
        Euler { x: x, y: y, z: z }
    }
}

/// A rotation about an arbitrary axis
#[deriving(Eq, Clone)]
pub struct AxisAngle<S, A> {
    axis: Vec3<S>,
    angle: A,
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

impl<S: Clone + Float, A: Angle<S>> Neg<AngleX<A>> for AngleX<A> { #[inline] fn neg(&self) -> AngleX<A> { AngleX(-**self) } }
impl<S: Clone + Float, A: Angle<S>> Neg<AngleY<A>> for AngleY<A> { #[inline] fn neg(&self) -> AngleY<A> { AngleY(-**self) } }
impl<S: Clone + Float, A: Angle<S>> Neg<AngleZ<A>> for AngleZ<A> { #[inline] fn neg(&self) -> AngleZ<A> { AngleZ(-**self) } }

impl<S: Clone + Float, A: Angle<S>> Add<AngleX<A>, AngleX<A>> for AngleX<A> { #[inline] fn add(&self, other: &AngleX<A>) -> AngleX<A> { AngleX((**self).add_a(*other.clone())) } }
impl<S: Clone + Float, A: Angle<S>> Add<AngleY<A>, AngleY<A>> for AngleY<A> { #[inline] fn add(&self, other: &AngleY<A>) -> AngleY<A> { AngleY((**self).add_a(*other.clone())) } }
impl<S: Clone + Float, A: Angle<S>> Add<AngleZ<A>, AngleZ<A>> for AngleZ<A> { #[inline] fn add(&self, other: &AngleZ<A>) -> AngleZ<A> { AngleZ((**self).add_a(*other.clone())) } }
impl<S: Clone + Float, A: Angle<S>> Sub<AngleX<A>, AngleX<A>> for AngleX<A> { #[inline] fn sub(&self, other: &AngleX<A>) -> AngleX<A> { AngleX((**self).sub_a(*other.clone())) } }
impl<S: Clone + Float, A: Angle<S>> Sub<AngleY<A>, AngleY<A>> for AngleY<A> { #[inline] fn sub(&self, other: &AngleY<A>) -> AngleY<A> { AngleY((**self).sub_a(*other.clone())) } }
impl<S: Clone + Float, A: Angle<S>> Sub<AngleZ<A>, AngleZ<A>> for AngleZ<A> { #[inline] fn sub(&self, other: &AngleZ<A>) -> AngleZ<A> { AngleZ((**self).sub_a(*other.clone())) } }
