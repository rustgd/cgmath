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

use std::fmt;

use approx::ApproxEq;
use matrix::{Matrix, Matrix4, ToMatrix4};
use num::{BaseNum, BaseFloat, zero, one};
use point::{Point, Point3};
use ray::Ray;
use rotation::{Rotation, Rotation3};
use vector::{Vector, Vector3};

/// A trait representing an [affine
/// transformation](https://en.wikipedia.org/wiki/Affine_transformation) that
/// can be applied to points or vectors. An affine transformation is one which
pub trait Transform<S: BaseNum, V: Vector<S>, P: Point<S,V>>: Sized {
    /// Create an identity transformation. That is, a transformation which
    /// does nothing.
    fn identity() -> Self;

    /// Create a transformation that rotates a vector to look at `center` from
    /// `eye`, using `up` for orientation.
    fn look_at(eye: &P, center: &P, up: &V) -> Self;

    /// Transform a vector using this transform.
    fn transform_vector(&self, vec: &V) -> V;

    /// Transform a point using this transform.
    fn transform_point(&self, point: &P) -> P;

    /// Transform a ray using this transform.
    #[inline]
    fn transform_ray(&self, ray: &Ray<P,V>) -> Ray<P,V> {
        Ray::new(self.transform_point(&ray.origin), self.transform_vector(&ray.direction))
    }

    /// Transform a vector as a point using this transform.
    #[inline]
    fn transform_as_point(&self, vec: &V) -> V {
        self.transform_point(&Point::from_vec(vec)).to_vec()
    }

    /// Combine this transform with another, yielding a new transformation
    /// which has the effects of both.
    fn concat(&self, other: &Self) -> Self;

    /// Create a transform that "un-does" this one.
    fn invert(&self) -> Option<Self>;

    /// Combine this transform with another, in-place.
    #[inline]
    fn concat_self(&mut self, other: &Self) {
        *self = Transform::concat(self, other);
    }

    /// Invert this transform in-place, failing if the transformation is not
    /// invertible.
    #[inline]
    fn invert_self(&mut self) {
        *self = self.invert().unwrap()
    }
}

/// A generic transformation consisting of a rotation,
/// displacement vector and scale amount.
#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct Decomposed<S, V, R> {
    pub scale: S,
    pub rot: R,
    pub disp: V,
}

impl<S: BaseFloat, V: Vector<S>, P: Point<S, V>, R: Rotation<S, V, P>> Transform<S, V, P> for Decomposed<S, V, R> {
    #[inline]
    fn identity() -> Decomposed<S, V, R> {
        Decomposed {
            scale: one(),
            rot: Rotation::identity(),
            disp: zero(),
        }
    }

    #[inline]
    fn look_at(eye: &P, center: &P, up: &V) -> Decomposed<S, V, R> {
        let origin: P = Point::origin();
        let rot: R = Rotation::look_at(&center.sub_p(eye), up);
        let disp: V = rot.rotate_vector(&origin.sub_p(eye));
        Decomposed {
            scale: one(),
            rot: rot,
            disp: disp,
        }
    }

    #[inline]
    fn transform_vector(&self, vec: &V) -> V {
        self.rot.rotate_vector(&vec.mul_s(self.scale.clone()))
    }

    #[inline]
    fn transform_point(&self, point: &P) -> P {
        self.rot.rotate_point(&point.mul_s(self.scale.clone())).add_v(&self.disp)
    }

    fn concat(&self, other: &Decomposed<S, V, R>) -> Decomposed<S, V, R> {
        Decomposed {
            scale: self.scale * other.scale,
            rot: self.rot.concat(&other.rot),
            disp: self.transform_as_point(&other.disp),
        }
    }

    fn invert(&self) -> Option<Decomposed<S, V, R>> {
        if self.scale.approx_eq(&zero()) {
            None
        } else {
            let s = one::<S>() / self.scale;
            let r = self.rot.invert();
            let d = r.rotate_vector(&self.disp).mul_s(-s);
            Some(Decomposed {
                scale: s,
                rot: r,
                disp: d,
            })
        }
    }
}

pub trait Transform3<S>: Transform<S, Vector3<S>, Point3<S>>+ ToMatrix4<S> {}

impl<S: BaseFloat + 'static, R: Rotation3<S>> ToMatrix4<S> for Decomposed<S, Vector3<S>, R> {
    fn to_matrix4(&self) -> Matrix4<S> {
        let mut m = self.rot.to_matrix3().mul_s(self.scale.clone()).to_matrix4();
        m.w = self.disp.extend(one());
        m
    }
}

impl<S: BaseFloat, R: Rotation3<S>> Transform3<S> for Decomposed<S,Vector3<S>,R> where S: 'static {}

impl<S: BaseFloat, R: fmt::Debug + Rotation3<S>> fmt::Debug for Decomposed<S,Vector3<S>,R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(scale({:?}), rot({:?}), disp{:?})",
            self.scale, self.rot, self.disp)
    }
}

/// A homogeneous transformation matrix.
#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct AffineMatrix3<S> {
    pub mat: Matrix4<S>,
}

impl<S: BaseFloat + 'static> Transform<S, Vector3<S>, Point3<S>> for AffineMatrix3<S> {
    #[inline]
    fn identity() -> AffineMatrix3<S> {
       AffineMatrix3 { mat: Matrix4::identity() }
    }

    #[inline]
    fn look_at(eye: &Point3<S>, center: &Point3<S>, up: &Vector3<S>) -> AffineMatrix3<S> {
        AffineMatrix3 { mat: Matrix4::look_at(eye, center, up) }
    }

    #[inline]
    fn transform_vector(&self, vec: &Vector3<S>) -> Vector3<S> {
        self.mat.mul_v(&vec.extend(zero())).truncate()
    }

    #[inline]
    fn transform_point(&self, point: &Point3<S>) -> Point3<S> {
        Point3::from_homogeneous(&self.mat.mul_v(&point.to_homogeneous()))
    }

    #[inline]
    fn concat(&self, other: &AffineMatrix3<S>) -> AffineMatrix3<S> {
        AffineMatrix3 { mat: self.mat.mul_m(&other.mat) }
    }

    #[inline]
    fn invert(&self) -> Option<AffineMatrix3<S>> {
        self.mat.invert().map(|m| AffineMatrix3{ mat: m })
    }
}

impl<S: BaseNum> ToMatrix4<S> for AffineMatrix3<S> {
    #[inline] fn to_matrix4(&self) -> Matrix4<S> { self.mat.clone() }
}

impl<S: BaseFloat> Transform3<S> for AffineMatrix3<S> where S: 'static {}
