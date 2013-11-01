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

use std::num;

use matrix::{Matrix, Mat4, ToMat4};
use point::{Point, Point3};
use ray::Ray;
use rotation::{Rotation, Rotation3};
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

pub trait Transform3<S>
: Transform<S, [S, ..3], Vec3<S>, Point3<S>>
+ ToMat4<S>
{}

impl<S: Float + Clone, R: Rotation3<S>>
ToMat4<S> for Decomposed<S, Vec3<S>, R> {
    fn to_mat4(&self) -> Mat4<S>   {
        let mut m = self.rot.to_mat3().mul_s( self.scale.clone() ).to_mat4();
        m.w = self.disp.extend( num::one() );
        m
    }
}

impl<S: Float, R: Rotation3<S>>
Transform3<S> for Decomposed<S,Vec3<S>,R>   {}

/// A homogeneous transformation matrix.
pub struct AffineMatrix3<S> {
    mat: Mat4<S>,
}

impl<S : Clone + Float>
Transform<S, [S, ..3], Vec3<S>, Point3<S>> for AffineMatrix3<S>  {
    #[inline]
    fn transform_vec(&self, vec: &Vec3<S>) -> Vec3<S>  {
        self.mat.mul_v( &vec.extend(num::zero()) ).truncate()
    }

    #[inline]
    fn transform_point(&self, point: &Point3<S>) -> Point3<S>   {
        Point3::from_homogeneous( &self.mat.mul_v( &point.to_homogeneous() ))
    }   
}

impl<S: Clone + Primitive>
ToMat4<S> for AffineMatrix3<S>  {
    #[inline] fn to_mat4(&self) -> Mat4<S>    { self.mat.clone() }
}



/// A transformation in three dimensions consisting of a rotation,
/// displacement vector and scale amount.
pub struct Transform3D<S>( Decomposed<S,Vec3<S>,Quat<S>> );

impl<S: Float> Transform3D<S> {
    #[inline]
    pub fn new(scale: S, rot: Quat<S>, disp: Vec3<S>) -> Transform3D<S> {
       Transform3D( Decomposed { scale: scale, rot: rot, disp: disp })
    }
}

