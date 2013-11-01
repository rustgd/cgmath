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
use point::{Point,Point3};
use ray::Ray;
use rotation::Rotation3;
use vector::{Vector,Vec3};

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

/// A transformation in three dimensions consisting of a rotation,
/// displacement vector and scale amount.
pub struct Transform3<S, R> {
    rot: R,
    disp: Vec3<S>,
    scale: S,
}

impl<S: Float, R: Rotation3<S>> Transform3<S, R> {
    #[inline]
    pub fn new(rot: R, disp: Vec3<S>, scale: S) -> Transform3<S, R> {
        Transform3 { rot: rot, disp: disp, scale: scale }
    }
}

impl <S: Float, R: Rotation3<S>> Transform<S, [S, .. 3], Vec3<S>, Point3<S>> for Transform3<S,R>   {
    #[inline]
    fn transform_vec(&self, vec: &Vec3<S>) -> Vec3<S>   {
        self.rot.rotate_vec3( &vec.mul_s( self.scale.clone() ))
    }

    #[inline]
    fn transform_point(&self, point: &Point3<S>) -> Point3<S>   {
        self.rot.rotate_point3( &point.mul_s( self.scale.clone() )).add_v( &self.disp )
    }
}
