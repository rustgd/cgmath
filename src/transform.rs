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

use structure::*;

use approx::ApproxEq;
use matrix::{Matrix2, Matrix3, Matrix4};
use num::{BaseFloat, BaseNum};
use point::{Point2, Point3};
use rotation::*;
use vector::{Vector2, Vector3};

/// A trait representing an [affine
/// transformation](https://en.wikipedia.org/wiki/Affine_transformation) that
/// can be applied to points or vectors. An affine transformation is one which
pub trait Transform<P: EuclideanSpace>: Sized {
    /// Create an identity transformation. That is, a transformation which
    /// does nothing.
    fn one() -> Self;

    /// Create a transformation that rotates a vector to look at `center` from
    /// `eye`, using `up` for orientation.
    fn look_at(eye: P, center: P, up: P::Diff) -> Self;

    /// Transform a vector using this transform.
    fn transform_vector(&self, vec: P::Diff) -> P::Diff;

    /// Transform a point using this transform.
    fn transform_point(&self, point: P) -> P;

    /// Combine this transform with another, yielding a new transformation
    /// which has the effects of both.
    fn concat(&self, other: &Self) -> Self;

    /// Create a transform that "un-does" this one.
    fn inverse_transform(&self) -> Option<Self>;

    /// Combine this transform with another, in-place.
    #[inline]
    fn concat_self(&mut self, other: &Self) {
        *self = Self::concat(self, other);
    }
}

/// A generic transformation consisting of a rotation,
/// displacement vector and scale amount.
#[derive(Copy, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Decomposed<V: VectorSpace, R> {
    pub scale: V::Scalar,
    pub rot: R,
    pub disp: V,
}

impl<P: EuclideanSpace, R: Rotation<P>> Transform<P> for Decomposed<P::Diff, R> where
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    <P as EuclideanSpace>::Scalar: BaseFloat,
    // FIXME: Investigate why this is needed!
    <P as EuclideanSpace>::Diff: VectorSpace,
{
    #[inline]
    fn one() -> Decomposed<P::Diff, R> {
        Decomposed {
            scale: P::Scalar::one(),
            rot: R::one(),
            disp: P::Diff::zero(),
        }
    }

    #[inline]
    fn look_at(eye: P, center: P, up: P::Diff) -> Decomposed<P::Diff, R> {
        let rot = R::look_at(center - eye, up);
        let disp = rot.rotate_vector(P::origin() - eye);
        Decomposed {
            scale: P::Scalar::one(),
            rot: rot,
            disp: disp,
        }
    }

    #[inline]
    fn transform_vector(&self, vec: P::Diff) -> P::Diff {
        self.rot.rotate_vector(vec * self.scale)
    }

    #[inline]
    fn transform_point(&self, point: P) -> P {
        self.rot.rotate_point(point * self.scale) + self.disp
    }

    fn concat(&self, other: &Decomposed<P::Diff, R>) -> Decomposed<P::Diff, R> {
        Decomposed {
            scale: self.scale * other.scale,
            rot: self.rot * other.rot,
            disp: self.disp + other.disp,
        }
    }

    fn inverse_transform(&self) -> Option<Decomposed<P::Diff, R>> {
        if self.scale.approx_eq(&P::Scalar::zero()) {
            None
        } else {
            let s = P::Scalar::one() / self.scale;
            let r = self.rot.invert();
            let d = r.rotate_vector(self.disp.clone()) * -s;
            Some(Decomposed {
                scale: s,
                rot: r,
                disp: d,
            })
        }
    }
}

pub trait Transform2<S: BaseNum>: Transform<Point2<S>> + Into<Matrix3<S>> {}
pub trait Transform3<S: BaseNum>: Transform<Point3<S>> + Into<Matrix4<S>> {}

impl<S: BaseFloat, R: Rotation2<S>> From<Decomposed<Vector2<S>, R>> for Matrix3<S> {
    fn from(dec: Decomposed<Vector2<S>, R>) -> Matrix3<S> {
        let m: Matrix2<_> = dec.rot.into();
        let mut m: Matrix3<_> = (&m * dec.scale).into();
        m.z = dec.disp.extend(S::one());
        m
    }
}

impl<S: BaseFloat, R: Rotation3<S>> From<Decomposed<Vector3<S>, R>> for Matrix4<S> {
    fn from(dec: Decomposed<Vector3<S>, R>) -> Matrix4<S> {
        let m: Matrix3<_> = dec.rot.into();
        let mut m: Matrix4<_> = (&m * dec.scale).into();
        m.w = dec.disp.extend(S::one());
        m
    }
}

impl<S: BaseFloat, R: Rotation2<S>> Transform2<S> for Decomposed<Vector2<S>, R> {}

impl<S: BaseFloat, R: Rotation3<S>> Transform3<S> for Decomposed<Vector3<S>, R> {}
