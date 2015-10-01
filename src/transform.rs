// Copyright 2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
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

use rust_num::{Zero, One};

use approx::ApproxEq;
use matrix::*;
use num::*;
use point::*;
use ray::Ray;
use rotation::*;
use vector::*;

/// A trait representing an [affine
/// transformation](https://en.wikipedia.org/wiki/Affine_transformation) that
/// can be applied to points or vectors. An affine transformation is one which
pub trait Transform<S: BaseNum, V: Vector<S>, P: Point<S, V>>: Sized {
    /// Create an identity transformation. That is, a transformation which
    /// does nothing.
    fn one() -> Self;

    /// Create a transformation that rotates a vector to look at `center` from
    /// `eye`, using `up` for orientation.
    fn look_at(eye: &P, center: &P, up: &V) -> Self;

    /// Transform a vector using this transform.
    fn transform_vector(&self, vec: &V) -> V;

    /// Transform a point using this transform.
    fn transform_point(&self, point: &P) -> P;

    /// Transform a ray using this transform.
    #[inline]
    fn transform_ray(&self, ray: &Ray<S, P,V>) -> Ray<S, P, V> {
        Ray::new(self.transform_point(&ray.origin), self.transform_vector(&ray.direction))
    }

    /// Transform a vector as a point using this transform.
    #[inline]
    fn transform_as_point(&self, vec: &V) -> V {
        self.transform_point(&P::from_vec(vec)).to_vec()
    }

    /// Combine this transform with another, yielding a new transformation
    /// which has the effects of both.
    fn concat(&self, other: &Self) -> Self;

    /// Create a transform that "un-does" this one.
    fn invert(&self) -> Option<Self>;

    /// Combine this transform with another, in-place.
    #[inline]
    fn concat_self(&mut self, other: &Self) {
        *self = Self::concat(self, other);
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

impl<
    S: BaseFloat,
    V: Vector<S>,
    P: Point<S, V>,
    R: Rotation<S, V, P>,
> Transform<S, V, P> for Decomposed<S, V, R> {
    #[inline]
    fn one() -> Decomposed<S, V, R> {
        Decomposed {
            scale: S::one(),
            rot: R::one(),
            disp: V::zero(),
        }
    }

    #[inline]
    fn look_at(eye: &P, center: &P, up: &V) -> Decomposed<S, V, R> {
        let rot = R::look_at(&center.sub_p(eye), up);
        let disp = rot.rotate_vector(&P::origin().sub_p(eye));
        Decomposed {
            scale: S::one(),
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
        if self.scale.approx_eq(&S::zero()) {
            None
        } else {
            let s = S::one() / self.scale;
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

pub trait Transform2<S: BaseNum>: Transform<S, Vector2<S>, Point2<S>> + Into<Matrix3<S>> {}
pub trait Transform3<S: BaseNum>: Transform<S, Vector3<S>, Point3<S>> + Into<Matrix4<S>> {}

impl<
    S: BaseFloat + 'static,
    R: Rotation2<S>,
> From<Decomposed<S, Vector2<S>, R>> for Matrix3<S> {
    fn from(dec: Decomposed<S, Vector2<S>, R>) -> Matrix3<S> {
        let m: Matrix2<_> = dec.rot.into();
        let mut m: Matrix3<_> = m.mul_s(dec.scale).into();
        m.z = dec.disp.extend(S::one());
        m
    }
}

impl<
    S: BaseFloat + 'static,
    R: Rotation3<S>,
> From<Decomposed<S, Vector3<S>, R>> for Matrix4<S> {
    fn from(dec: Decomposed<S, Vector3<S>, R>) -> Matrix4<S> {
        let m: Matrix3<_> = dec.rot.into();
        let mut m: Matrix4<_> = m.mul_s(dec.scale).into();
        m.w = dec.disp.extend(S::one());
        m
    }
}

impl<
    S: BaseFloat + 'static,
    R: Rotation2<S>,
> Transform2<S> for Decomposed<S, Vector2<S>, R> {}

impl<
    S: BaseFloat + 'static,
    R: Rotation3<S>,
> Transform3<S> for Decomposed<S, Vector3<S>, R> {}

impl<
    S: BaseFloat,
    R: fmt::Debug + Rotation3<S>,
> fmt::Debug for Decomposed<S, Vector3<S>, R> {
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
    fn one() -> AffineMatrix3<S> {
       AffineMatrix3 { mat: Matrix4::one() }
    }

    #[inline]
    fn look_at(eye: &Point3<S>, center: &Point3<S>, up: &Vector3<S>) -> AffineMatrix3<S> {
        AffineMatrix3 { mat: Matrix4::look_at(eye, center, up) }
    }

    #[inline]
    fn transform_vector(&self, vec: &Vector3<S>) -> Vector3<S> {
        self.mat.mul_v(&vec.extend(S::zero())).truncate()
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

impl<S: BaseNum> From<AffineMatrix3<S>> for Matrix4<S> {
    #[inline] fn from(aff: AffineMatrix3<S>) -> Matrix4<S> { aff.mat }
}

impl<S: BaseFloat + 'static> Transform3<S> for AffineMatrix3<S> {}

/// A trait that allows extracting components (rotation, translation, scale)
/// from an arbitrary transformations
pub trait ToComponents<S: BaseFloat, V: Vector<S>, P: Point<S, V>, R: Rotation<S, V, P>> {
    /// Extract the (scale, rotation, translation) triple
    fn decompose(&self) -> (V, R, V);
}

pub trait ToComponents2<S: BaseFloat, R: Rotation2<S>>:
    ToComponents<S, Vector2<S>, Point2<S>, R> {}
pub trait ToComponents3<S: BaseFloat, R: Rotation3<S>>:
    ToComponents<S, Vector3<S>, Point3<S>, R> {}

pub trait CompositeTransform<S: BaseFloat, V: Vector<S>, P: Point<S, V>, R: Rotation<S, V, P>>:
    Transform<S, V, P> + ToComponents<S, V, P, R> {}
pub trait CompositeTransform2<S: BaseFloat, R: Rotation2<S>>:
    Transform2<S> + ToComponents2<S, R> {}
pub trait CompositeTransform3<S: BaseFloat, R: Rotation3<S>>:
    Transform3<S> + ToComponents3<S, R> {}

impl<
    S: BaseFloat,
    V: Vector<S> + Clone,
    P: Point<S, V>,
    R: Rotation<S, V, P> + Clone,
> ToComponents<S, V, P, R> for Decomposed<S, V, R> {
    fn decompose(&self) -> (V, R, V) {
        (V::one().mul_s(self.scale), self.rot.clone(), self.disp.clone())
    }
}

impl<
    S: BaseFloat,
    R: Rotation2<S> + Clone,
> ToComponents2<S, R> for Decomposed<S, Vector2<S>, R> {}

impl<
    S: BaseFloat,
    R: Rotation3<S> + Clone,
> ToComponents3<S, R> for Decomposed<S, Vector3<S>, R> {}

impl<
    S: BaseFloat + 'static,
    R: Rotation2<S> + Clone,
> CompositeTransform2<S, R> for Decomposed<S, Vector2<S>, R> {}

impl<
    S: BaseFloat + 'static,
    R: Rotation3<S> + Clone,
> CompositeTransform3<S, R> for Decomposed<S, Vector3<S>, R> {}
