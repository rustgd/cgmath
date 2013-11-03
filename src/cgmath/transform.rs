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

use matrix::Mat4;
use point::Point;
use ray::Ray;
use rotation::Rotation;
use quaternion::Quat;
use vector::{Vector, Vec3};

/// A trait of affine transformation, that can be applied to points or vectors
pub trait Transform
<
    S: Primitive,
    Slice,
    V: Vector<S,Slice>,
    P: Point<S,V,Slice>
>
{
    fn transform_vec(&self, vec: &V) -> V;
    fn transform_point(&self, point: &P) -> P;

    #[inline]
    fn transform_ray(&self, ray: &Ray<P,V>) -> Ray<P,V>    {
        Ray::new( self.transform_point(&ray.origin), self.transform_vec(&ray.direction) )
    }
}

/// A homogeneous transformation matrix.
pub struct AffineMatrix3<S> {
    mat: Mat4<S>,
}

/// A generic transformation consisting of a rotation,
/// displacement vector and scale amount.
pub struct Decomposed<S,V,R>    {
    scale: S,
    rot: R,
    disp: V,
}

impl
<
    S: Float,
    Slice,
    V: Vector<S, Slice>,
    P: Point<S, V, Slice>,
    R: Rotation<S, Slice, V, P>
>
Transform<S, Slice, V, P> for Decomposed<S,V,R>    {
    #[inline]
    fn transform_vec(&self, vec: &V) -> V   {
        self.rot.rotate_vec( &vec.mul_s( self.scale.clone() ))
    }

    #[inline]
    fn transform_point(&self, point: &P) -> P   {
        self.rot.rotate_point( &point.mul_s( self.scale.clone() )).add_v( &self.disp )
    }
}

/// A transformation in three dimensions consisting of a rotation,
/// displacement vector and scale amount.
pub struct Transform3<S>( Decomposed<S,Vec3<S>,Quat<S>> );

impl<S: Float> Transform3<S> {
    #[inline]
    pub fn new(scale: S, rot: Quat<S>, disp: Vec3<S>) -> Transform3<S> {
       Transform3( Decomposed { scale: scale, rot: rot, disp: disp })
    }
}

