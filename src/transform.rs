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
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
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
            disp: self.rot.rotate_vector(other.disp * self.scale) + self.disp,
        }
    }

    fn inverse_transform(&self) -> Option<Decomposed<P::Diff, R>> {
        if ulps_eq!(self.scale, &P::Scalar::zero()) {
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

impl<S: VectorSpace, R, E: BaseFloat> ApproxEq for Decomposed<S, R>
    where S: ApproxEq<Epsilon = E>, S::Scalar: ApproxEq<Epsilon = E>, R: ApproxEq<Epsilon = E>
{
    type Epsilon = E;

    #[inline]
    fn default_epsilon() -> E {
        E::default_epsilon()
    }

    #[inline]
    fn default_max_relative() -> E {
        E::default_max_relative()
    }

    #[inline]
    fn default_max_ulps() -> u32 {
        E::default_max_ulps()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: E, max_relative: E) -> bool {
        S::Scalar::relative_eq(&self.scale, &other.scale, epsilon, max_relative) &&
        R::relative_eq(&self.rot, &other.rot, epsilon, max_relative) &&
        S::relative_eq(&self.disp, &other.disp, epsilon, max_relative)
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: E, max_ulps: u32) -> bool {
        S::Scalar::ulps_eq(&self.scale, &other.scale, epsilon, max_ulps) &&
        R::ulps_eq(&self.rot, &other.rot, epsilon, max_ulps) &&
        S::ulps_eq(&self.disp, &other.disp, epsilon, max_ulps)
    }
}

#[cfg(feature = "eders")]
#[doc(hidden)]
mod eders_ser {
    use structure::VectorSpace;
    use super::Decomposed;
    use serde::{self, Serialize};

    impl<V: VectorSpace, R> Serialize for Decomposed<V, R>
        where V: Serialize, V::Scalar: Serialize, R: Serialize
    {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer
        {
            let mut state = try!(serializer.serialize_struct("Decomposed", 3));
            try!(serializer.serialize_struct_elt(&mut state, "scale", &self.scale));
            try!(serializer.serialize_struct_elt(&mut state, "rot", &self.rot));
            try!(serializer.serialize_struct_elt(&mut state, "disp", &self.disp));
            serializer.serialize_struct_end(state)
        }
    }
}

#[cfg(feature = "eders")]
#[doc(hidden)]
mod eders_de {
    use structure::VectorSpace;
    use super::Decomposed;
    use serde::{self, Deserialize};
    use std::marker::PhantomData;

    enum DecomposedField {
        Scale,
        Rot,
        Disp,
    }

    impl Deserialize for DecomposedField {
        fn deserialize<D>(deserializer: &mut D) -> Result<DecomposedField, D::Error>
            where D: serde::Deserializer
        {
            struct DecomposedFieldVisitor;

            impl serde::de::Visitor for DecomposedFieldVisitor {
                type Value = DecomposedField;

                fn visit_str<E>(&mut self, value: &str) -> Result<DecomposedField, E>
                    where E: serde::de::Error
                {
                    match value {
                        "scale" => Ok(DecomposedField::Scale),
                        "rot" => Ok(DecomposedField::Rot),
                        "disp" => Ok(DecomposedField::Disp),
                        _ => Err(serde::de::Error::custom("expected scale, rot or disp")),
                    }
                }
            }

            deserializer.deserialize(DecomposedFieldVisitor)
        }
    }

    impl<S: VectorSpace, R> Deserialize for Decomposed<S, R>
        where S: Deserialize, S::Scalar: Deserialize, R: Deserialize
    {
        fn deserialize<D>(deserializer: &mut D) -> Result<Decomposed<S, R>, D::Error>
            where D: serde::de::Deserializer
        {
            const FIELDS: &'static [&'static str] = &["scale", "rot", "disp"];
            deserializer.deserialize_struct("Decomposed", FIELDS, DecomposedVisitor(PhantomData))
        }
    }

    struct DecomposedVisitor<S: VectorSpace, R>(PhantomData<(S, R)>);

    impl<S: VectorSpace, R> serde::de::Visitor for DecomposedVisitor<S, R>
        where S: Deserialize, S::Scalar: Deserialize, R: Deserialize
    {
        type Value = Decomposed<S, R>;

        fn visit_map<V>(&mut self, mut visitor: V) -> Result<Decomposed<S, R>, V::Error>
            where V: serde::de::MapVisitor
        {
            let mut scale = None;
            let mut rot = None;
            let mut disp = None;

            loop {
                match try!(visitor.visit_key()) {
                    Some(DecomposedField::Scale) => { scale = Some(try!(visitor.visit_value())); },
                    Some(DecomposedField::Rot) => { rot = Some(try!(visitor.visit_value())); },
                    Some(DecomposedField::Disp) => { disp = Some(try!(visitor.visit_value())); },
                    _ => { break; },
                }
            }

            let scale = match scale {
                Some(scale) => scale,
                None => try!(visitor.missing_field("scale")),
            };

            let rot = match rot {
                Some(rot) => rot,
                None => try!(visitor.missing_field("rot")),
            };

            let disp = match disp {
                Some(disp) => disp,
                None => try!(visitor.missing_field("disp")),
            };

            try!(visitor.end());

            Ok(Decomposed { scale: scale, rot: rot, disp: disp })
        }
    }
}
