// Copyright 2014 The CGMath Developers. For a full listing of the authors,
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

use angle::{Rad, acos};
use approx::ApproxEq;
use array::Array;
use matrix::Matrix;
use matrix::{Mat2, ToMat2};
use matrix::{Mat3, ToMat3};
use point::{Point, Point2, Point3};
use quaternion::{Quat, ToQuat};
use ray::Ray;
use vector::{Vector, Vec2, Vec3};
use partial_ord::{PartOrdPrim, PartOrdFloat};

/// A trait for generic rotation
pub trait Rotation
<
    S: PartOrdPrim,
    Slice,
    V: Vector<S,Slice>,
    P: Point<S,V,Slice>
>
:   Eq
+   ApproxEq<S>
{
    fn identity() -> Self;

    /// Create a rotation to a given direction with an 'up' vector
    fn look_at(dir: &V, up: &V) -> Self;
    
    /// Create a shortest rotation to transform vector 'a' into 'b'.
    /// Both given vectors are assumed to have unit length.
    fn between_vecs(a: &V, b: &V) -> Self;

    fn rotate_vec(&self, vec: &V) -> V;

    #[inline]
    fn rotate_point(&self, point: &P) -> P {
        Point::from_vec( &self.rotate_vec( &point.to_vec() ) )
    }

    #[inline]
    fn rotate_ray(&self, ray: &Ray<P,V>) -> Ray<P,V> {
        Ray::new(   //FIXME: use clone derived from Array
            Array::build(|i| ray.origin.i(i).clone()),
            self.rotate_vec(&ray.direction) )
    }

    fn concat(&self, other: &Self) -> Self;
    fn invert(&self) -> Self;

    #[inline]
    fn concat_self(&mut self, other: &Self) {
        *self = self.concat(other);
    }
    
    #[inline]
    fn invert_self(&mut self) {
        *self = self.invert();
    }
}

/// A two-dimensional rotation
pub trait Rotation2
<
    S
>
:   Rotation<S, [S, ..2], Vec2<S>, Point2<S>>
+   ToMat2<S>
+   ToBasis2<S>
{
    // Create a rotation by a given angle. Thus is a redundant case of both
    // from_axis_angle() and from_euler() for 2D space.
    fn from_angle(theta: Rad<S>) -> Self;
}

/// A three-dimensional rotation
pub trait Rotation3
<
    S: Primitive
>
:   Rotation<S, [S, ..3], Vec3<S>, Point3<S>>
+   ToMat3<S>
+   ToBasis3<S>
+   ToQuat<S>
{
    /// Create a rotation around a given axis.
    fn from_axis_angle(axis: &Vec3<S>, angle: Rad<S>) -> Self;

    /// Create a rotation from a set of euler angles.
    ///
    /// # Parameters
    ///
    /// - `x`: the angular rotation around the `x` axis (pitch).
    /// - `y`: the angular rotation around the `y` axis (yaw).
    /// - `z`: the angular rotation around the `z` axis (roll).
    fn from_euler(x: Rad<S>, y: Rad<S>, z: Rad<S>) -> Self;

    /// Create a rotation matrix from a rotation around the `x` axis (pitch).
    #[inline]
    fn from_angle_x(theta: Rad<S>) -> Self {
        Rotation3::from_axis_angle( &Vec3::unit_x(), theta )
    }

    #[inline]
    fn from_angle_y(theta: Rad<S>) -> Self {
        Rotation3::from_axis_angle( &Vec3::unit_y(), theta )
    }

    #[inline]
    fn from_angle_z(theta: Rad<S>) -> Self {
        Rotation3::from_axis_angle( &Vec3::unit_z(), theta )
    }
}


/// A two-dimensional rotation matrix.
///
/// The matrix is guaranteed to be orthogonal, so some operations can be
/// implemented more efficiently than the implementations for `math::Mat2`. To
/// enforce orthogonality at the type level the operations have been restricted
/// to a subset of those implemented on `Mat2`.
#[deriving(Eq, Clone)]
pub struct Basis2<S> {
    mat: Mat2<S>
}

impl<S: Float> Basis2<S> {
    #[inline]
    pub fn as_mat2<'a>(&'a self) -> &'a Mat2<S> { &'a self.mat }
}

pub trait ToBasis2<S: Float> {
    fn to_rot2(&self) -> Basis2<S>;
}

impl<S: Float> ToBasis2<S> for Basis2<S> {
    #[inline]
    fn to_rot2(&self) -> Basis2<S> { self.clone() }
}

impl<S: Float> ToMat2<S> for Basis2<S> {
    #[inline]
    fn to_mat2(&self) -> Mat2<S> { self.mat.clone() }
}

impl<S: PartOrdFloat<S>>
Rotation<S, [S, ..2], Vec2<S>, Point2<S>> for Basis2<S> {
    #[inline]
    fn identity() -> Basis2<S> { Basis2{ mat: Mat2::identity() } }

    #[inline]
    fn look_at(dir: &Vec2<S>, up: &Vec2<S>) -> Basis2<S> {
        Basis2 { mat: Mat2::look_at(dir, up) }
    }

    #[inline]
    fn between_vecs(a: &Vec2<S>, b: &Vec2<S>) -> Basis2<S> {
        Rotation2::from_angle( acos(a.dot(b)) )
    }
    
    #[inline]
    fn rotate_vec(&self, vec: &Vec2<S>) -> Vec2<S> { self.mat.mul_v(vec) }

    #[inline]
    fn concat(&self, other: &Basis2<S>) -> Basis2<S> { Basis2 { mat: self.mat.mul_m(&other.mat) } }

    #[inline]
    fn concat_self(&mut self, other: &Basis2<S>) { self.mat.mul_self_m(&other.mat); }

    // TODO: we know the matrix is orthogonal, so this could be re-written
    // to be faster
    #[inline]
    fn invert(&self) -> Basis2<S> { Basis2 { mat: self.mat.invert().unwrap() } }

    // TODO: we know the matrix is orthogonal, so this could be re-written
    // to be faster
    #[inline]
    fn invert_self(&mut self) { self.mat.invert_self(); }
}

impl<S: PartOrdFloat<S>>
ApproxEq<S> for Basis2<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Basis2<S>, epsilon: &S) -> bool {
        self.mat.approx_eq_eps(&other.mat, epsilon)
    }
}

impl<S: PartOrdFloat<S>>
Rotation2<S> for Basis2<S> {
    fn from_angle(theta: Rad<S>) -> Basis2<S> { Basis2 { mat: Mat2::from_angle(theta) } }
}

/// A three-dimensional rotation matrix.
///
/// The matrix is guaranteed to be orthogonal, so some operations, specifically
/// inversion, can be implemented more efficiently than the implementations for
/// `math::Mat3`. To ensure orthogonality is maintained, the operations have
/// been restricted to a subeset of those implemented on `Mat3`.
#[deriving(Eq, Clone)]
pub struct Basis3<S> {
    mat: Mat3<S>
}

impl<S: PartOrdFloat<S>>
Basis3<S> {
    #[inline]
    pub fn from_quat(quat: &Quat<S>) -> Basis3<S> { Basis3 { mat: quat.to_mat3() } }
    
    #[inline]
    pub fn as_mat3<'a>(&'a self) -> &'a Mat3<S> { &'a self.mat }
}

pub trait ToBasis3<S: Float> {
    fn to_rot3(&self) -> Basis3<S>;
}

impl<S: Float> ToBasis3<S> for Basis3<S> {
    #[inline]
    fn to_rot3(&self) -> Basis3<S> { self.clone() }
}

impl<S: Float> ToMat3<S> for Basis3<S> {
    #[inline]
    fn to_mat3(&self) -> Mat3<S> { self.mat.clone() }
}

impl<S: PartOrdFloat<S>>
ToQuat<S> for Basis3<S> {
    #[inline]
    fn to_quat(&self) -> Quat<S> { self.mat.to_quat() }
}

impl<S: PartOrdFloat<S>>
Rotation<S, [S, ..3], Vec3<S>, Point3<S>> for Basis3<S> {
    #[inline]
    fn identity() -> Basis3<S> { Basis3{ mat: Mat3::identity() } }

    #[inline]
    fn look_at(dir: &Vec3<S>, up: &Vec3<S>) -> Basis3<S> {
        Basis3 { mat: Mat3::look_at(dir, up) }
    }

    #[inline]
    fn between_vecs(a: &Vec3<S>, b: &Vec3<S>) -> Basis3<S> {
        let q: Quat<S> = Rotation::between_vecs(a, b);
        q.to_rot3()
    }
    
    #[inline]
    fn rotate_vec(&self, vec: &Vec3<S>) -> Vec3<S> { self.mat.mul_v(vec) }

    #[inline]
    fn concat(&self, other: &Basis3<S>) -> Basis3<S> { Basis3 { mat: self.mat.mul_m(&other.mat) } }

    #[inline]
    fn concat_self(&mut self, other: &Basis3<S>) { self.mat.mul_self_m(&other.mat); }

    // TODO: we know the matrix is orthogonal, so this could be re-written
    // to be faster
    #[inline]
    fn invert(&self) -> Basis3<S> { Basis3 { mat: self.mat.invert().unwrap() } }

    // TODO: we know the matrix is orthogonal, so this could be re-written
    // to be faster
    #[inline]
    fn invert_self(&mut self) { self.mat.invert_self(); }
}

impl<S: Float + ApproxEq<S>>
ApproxEq<S> for Basis3<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Basis3<S>, epsilon: &S) -> bool {
        self.mat.approx_eq_eps(&other.mat, epsilon)
    }
}

impl<S: PartOrdFloat<S>>
Rotation3<S> for Basis3<S> {
    fn from_axis_angle(axis: &Vec3<S>, angle: Rad<S>) -> Basis3<S> {
        Basis3 { mat: Mat3::from_axis_angle(axis, angle) }
    }

    fn from_euler(x: Rad<S>, y: Rad<S>, z: Rad<S>) -> Basis3<S> {
        Basis3 { mat: Mat3::from_euler(x, y ,z) }
    }

    fn from_angle_x(theta: Rad<S>) -> Basis3<S> {
        Basis3 { mat: Mat3::from_angle_x(theta) }
    }

    fn from_angle_y(theta: Rad<S>) -> Basis3<S> {
        Basis3 { mat: Mat3::from_angle_y(theta) }
    }

    fn from_angle_z(theta: Rad<S>) -> Basis3<S> {
        Basis3 { mat: Mat3::from_angle_z(theta) }
    }
}
