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

//! Column major, square matrix types and traits.

use std::fmt;
use std::num::{Zero, zero, One, one, cast, sqrt};

use angle::{Rad, sin, cos, sin_cos};
use approx::ApproxEq;
use array::{Array, build};
use point::{Point, Point3};
use quaternion::{Quat, ToQuat};
use vector::{Vector, EuclideanVector};
use vector::{Vec2, Vec3, Vec4};
use partial_ord::PartOrdFloat;

/// A 2 x 2, column major matrix
#[deriving(Clone, Eq)]
pub struct Mat2<S> { x: Vec2<S>, y: Vec2<S> }

/// A 3 x 3, column major matrix
#[deriving(Clone, Eq)]
pub struct Mat3<S> { x: Vec3<S>, y: Vec3<S>, z: Vec3<S> }

/// A 4 x 4, column major matrix
#[deriving(Clone, Eq)]
pub struct Mat4<S> { x: Vec4<S>, y: Vec4<S>, z: Vec4<S>, w: Vec4<S> }


impl<S: Primitive> Mat2<S> {
    #[inline]
    pub fn new(c0r0: S, c0r1: S,
               c1r0: S, c1r1: S) -> Mat2<S> {
        Mat2::from_cols(Vec2::new(c0r0, c0r1),
                        Vec2::new(c1r0, c1r1))
    }

    #[inline]
    pub fn from_cols(c0: Vec2<S>, c1: Vec2<S>) -> Mat2<S> {
        Mat2 { x: c0, y: c1 }
    }

    #[inline]
    pub fn from_value(value: S) -> Mat2<S> {
        Mat2::new(value.clone(), zero(),
                  zero(), value.clone())
    }

    #[inline]
    pub fn zero() -> Mat2<S> {
        Mat2::from_value(zero())
    }

    #[inline]
    pub fn identity() -> Mat2<S> {
        Mat2::from_value(one())
    }
}

impl<S: PartOrdFloat<S>> Mat2<S> {
    pub fn look_at(dir: &Vec2<S>, up: &Vec2<S>) -> Mat2<S> {
        //TODO: verify look_at 2D
        Mat2::from_cols(up.clone(), dir.clone()).transpose()
    }

    #[inline]
    pub fn from_angle(theta: Rad<S>) -> Mat2<S> {
        let cos_theta = cos(theta.clone());
        let sin_theta = sin(theta.clone());

        Mat2::new(cos_theta.clone(),  -sin_theta.clone(),
                  sin_theta.clone(),  cos_theta.clone())
    }
}

impl<S: Primitive> Mat3<S> {
    #[inline]
    pub fn new(c0r0:S, c0r1:S, c0r2:S,
               c1r0:S, c1r1:S, c1r2:S,
               c2r0:S, c2r1:S, c2r2:S) -> Mat3<S> {
        Mat3::from_cols(Vec3::new(c0r0, c0r1, c0r2),
                        Vec3::new(c1r0, c1r1, c1r2),
                        Vec3::new(c2r0, c2r1, c2r2))
    }

    #[inline]
    pub fn from_cols(c0: Vec3<S>, c1: Vec3<S>, c2: Vec3<S>) -> Mat3<S> {
        Mat3 { x: c0, y: c1, z: c2 }
    }

    #[inline]
    pub fn from_value(value: S) -> Mat3<S> {
        Mat3::new(value.clone(), zero(), zero(),
                  zero(), value.clone(), zero(),
                  zero(), zero(), value.clone())
    }

    #[inline]
    pub fn zero() -> Mat3<S> {
        Mat3::from_value(zero())
    }

    #[inline]
    pub fn identity() -> Mat3<S> {
        Mat3::from_value(one())
    }
}

impl<S: PartOrdFloat<S>>
Mat3<S> {
    pub fn look_at(dir: &Vec3<S>, up: &Vec3<S>) -> Mat3<S> {
        let dir = dir.normalize();
        let side = up.cross(&dir).normalize();
        let up = dir.cross(&side).normalize();

        Mat3::from_cols(side, up, dir).transpose()
    }

    /// Create a matrix from a rotation around the `x` axis (pitch).
    pub fn from_angle_x(theta: Rad<S>) -> Mat3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = sin_cos(theta);
        Mat3::new(one(), zero(), zero(),
                  zero(), c.clone(), s.clone(),
                  zero(), -s.clone(), c.clone())
    }

    /// Create a matrix from a rotation around the `y` axis (yaw).
    pub fn from_angle_y(theta: Rad<S>) -> Mat3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = sin_cos(theta);
        Mat3::new(c.clone(), zero(), -s.clone(),
                  zero(), one(), zero(),
                  s.clone(), zero(), c.clone())
    }

    /// Create a matrix from a rotation around the `z` axis (roll).
    pub fn from_angle_z(theta: Rad<S>) -> Mat3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = sin_cos(theta);
        Mat3::new(c.clone(), s.clone(), zero(),
                  -s.clone(), c.clone(), zero(),
                  zero(), zero(), one())
    }

    /// Create a matrix from a set of euler angles.
    ///
    /// # Parameters
    ///
    /// - `x`: the angular rotation around the `x` axis (pitch).
    /// - `y`: the angular rotation around the `y` axis (yaw).
    /// - `z`: the angular rotation around the `z` axis (roll).
    pub fn from_euler(x: Rad<S>, y: Rad<S>, z: Rad<S>) -> Mat3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
        let (sx, cx) = sin_cos(x);
        let (sy, cy) = sin_cos(y);
        let (sz, cz) = sin_cos(z);

        Mat3::new(cy * cz, cy * sz, -sy,
                  -cx * sz + sx * sy * cz, cx * cz + sx * sy * sz, sx * cy,
                  sx * sz + cx * sy * cz, -sx * cz + cx * sy * sz, cx * cy)
    }

    /// Create a matrix from a rotation around an arbitrary axis
    pub fn from_axis_angle(axis: &Vec3<S>, angle: Rad<S>) -> Mat3<S> {
        let (s, c) = sin_cos(angle);
        let _1subc = one::<S>() - c;

        Mat3::new(_1subc * axis.x * axis.x + c,
                  _1subc * axis.x * axis.y + s * axis.z,
                  _1subc * axis.x * axis.z - s * axis.y,

                  _1subc * axis.x * axis.y - s * axis.z,
                  _1subc * axis.y * axis.y + c,
                  _1subc * axis.y * axis.z + s * axis.x,

                  _1subc * axis.x * axis.z + s * axis.y,
                  _1subc * axis.y * axis.z - s * axis.x,
                  _1subc * axis.z * axis.z + c)
    }
}

impl<S: Primitive> Mat4<S> {
    #[inline]
    pub fn new(c0r0: S, c0r1: S, c0r2: S, c0r3: S,
               c1r0: S, c1r1: S, c1r2: S, c1r3: S,
               c2r0: S, c2r1: S, c2r2: S, c2r3: S,
               c3r0: S, c3r1: S, c3r2: S, c3r3: S) -> Mat4<S>  {
        Mat4::from_cols(Vec4::new(c0r0, c0r1, c0r2, c0r3),
                        Vec4::new(c1r0, c1r1, c1r2, c1r3),
                        Vec4::new(c2r0, c2r1, c2r2, c2r3),
                        Vec4::new(c3r0, c3r1, c3r2, c3r3))
    }

    #[inline]
    pub fn from_cols(c0: Vec4<S>, c1: Vec4<S>, c2: Vec4<S>, c3: Vec4<S>) -> Mat4<S> {
        Mat4 { x: c0, y: c1, z: c2, w: c3 }
    }

    #[inline]
    pub fn from_value(value: S) -> Mat4<S> {
        Mat4::new(value.clone(), zero(), zero(), zero(),
                  zero(), value.clone(), zero(), zero(),
                  zero(), zero(), value.clone(), zero(),
                  zero(), zero(), zero(), value.clone())
    }

    #[inline]
    pub fn zero() -> Mat4<S> {
        Mat4::from_value(zero())
    }

    #[inline]
    pub fn identity() -> Mat4<S> {
        Mat4::from_value(one())
    }
}

impl<S: PartOrdFloat<S>>
Mat4<S> {
    pub fn look_at(eye: &Point3<S>, center: &Point3<S>, up: &Vec3<S>) -> Mat4<S> {
        let f = center.sub_p(eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(&f);

        Mat4::new(s.x.clone(), u.x.clone(), -f.x.clone(), zero(),
                  s.y.clone(), u.y.clone(), -f.y.clone(), zero(),
                  s.z.clone(), u.z.clone(), -f.z.clone(), zero(),
                  -eye.dot(&s), -eye.dot(&u), eye.dot(&f), one())
    }
}

array!(impl<S> Mat2<S> -> [Vec2<S>, ..2] _2)
array!(impl<S> Mat3<S> -> [Vec3<S>, ..3] _3)
array!(impl<S> Mat4<S> -> [Vec4<S>, ..4] _4)

pub trait Matrix
<
    S: PartOrdFloat<S>, Slice,
    V: Clone + Vector<S, VSlice> + Array<S, VSlice>, VSlice
>
:   Array<V, Slice>
+   Neg<Self>
+   Zero + One
+   ApproxEq<S>
{
    #[inline]
    fn c<'a>(&'a self, c: uint) -> &'a V { self.i(c) }

    #[inline]
    fn mut_c<'a>(&'a mut self, c: uint) -> &'a mut V { self.mut_i(c) }

    #[inline]
    fn swap_c(&mut self, a: uint, b: uint) {
        let tmp = self.c(a).clone();
        *self.mut_c(a) = self.c(b).clone();
        *self.mut_c(b) = tmp;
    }

    #[inline]
    fn r(&self, r: uint) -> V {
        build(|i| self.i(i).i(r).clone())
    }

    #[inline]
    fn swap_r(&mut self, a: uint, b: uint) {
        self.each_mut(|_, c| c.swap(a, b))
    }

    #[inline]
    fn cr<'a>(&'a self, c: uint, r: uint) -> &'a S { self.i(c).i(r) }

    #[inline]
    fn mut_cr<'a>(&'a mut self, c: uint, r: uint) -> &'a mut S {
        self.mut_i(c).mut_i(r)
    }

    #[inline]
    fn swap_cr(&mut self, a: (uint, uint), b: (uint, uint)) {
        let (ca, ra) = a;
        let (cb, rb) = b;
        let tmp = self.cr(ca, ra).clone();
        *self.mut_cr(ca, ra) = self.cr(cb, rb).clone();
        *self.mut_cr(cb, rb) = tmp;
    }

    // fn swap_cr(&mut self, (ca, ra): (uint, uint), (cb, rb): (uint, uint)) {
    //     let tmp = self.cr(ca, ra).clone();
    //     *self.mut_cr(ca, ra) = self.cr(cb, rb).clone();
    //     *self.mut_cr(cb, rb) = tmp;
    // }

    #[inline] fn neg_self(&mut self) { self.each_mut(|_, x| *x = x.neg()) }

    #[inline] fn mul_s(&self, s: S) -> Self { build(|i| self.i(i).mul_s(s.clone())) }
    #[inline] fn div_s(&self, s: S) -> Self { build(|i| self.i(i).div_s(s.clone())) }
    #[inline] fn rem_s(&self, s: S) -> Self { build(|i| self.i(i).rem_s(s.clone())) }

    #[inline] fn add_m(&self, other: &Self) -> Self { build(|i| self.i(i).add_v(other.i(i))) }
    #[inline] fn sub_m(&self, other: &Self) -> Self { build(|i| self.i(i).sub_v(other.i(i))) }

    #[inline] fn mul_v(&self, v: &V) -> V { build(|i| self.r(i).dot(v)) }

    fn mul_m(&self, other: &Self) -> Self;

    #[inline] fn mul_self_s(&mut self, s: S) { self.each_mut(|_, c| *c = c.mul_s(s.clone())) }
    #[inline] fn div_self_s(&mut self, s: S) { self.each_mut(|_, c| *c = c.div_s(s.clone())) }
    #[inline] fn rem_self_s(&mut self, s: S) { self.each_mut(|_, c| *c = c.rem_s(s.clone())) }

    #[inline] fn add_self_m(&mut self, other: &Self) { self.each_mut(|i, c| *c = c.add_v(other.c(i))) }
    #[inline] fn sub_self_m(&mut self, other: &Self) { self.each_mut(|i, c| *c = c.sub_v(other.c(i))) }

    #[inline] fn mul_self_m(&mut self, other: &Self) { *self = self.mul_m(other); }

    fn transpose(&self) -> Self;
    fn transpose_self(&mut self);
    fn determinant(&self) -> S;

    #[inline]
    fn diagonal(&self) -> V { build(|i| self.cr(i, i).clone()) }

    #[inline]
    fn trace(&self) -> S { self.diagonal().comp_add() }

    fn invert(&self) -> Option<Self>;

    #[inline]
    fn invert_self(&mut self) {
        *self = self.invert().expect("Attempted to invert a matrix with zero determinant.");
    }

    #[inline]
    fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&zero())
    }

    #[inline]
    fn is_identity(&self) -> bool {
        self.approx_eq(&one())
    }

    #[inline]
    fn is_rotated(&self) -> bool {
        !self.approx_eq(&one())
    }

    fn is_diagonal(&self) -> bool;
    fn is_symmetric(&self) -> bool;
}

impl<S: PartOrdFloat<S>> Add<Mat2<S>, Mat2<S>> for Mat2<S> { #[inline] fn add(&self, other: &Mat2<S>) -> Mat2<S> { build(|i| self.i(i).add_v(other.i(i))) } }
impl<S: PartOrdFloat<S>> Add<Mat3<S>, Mat3<S>> for Mat3<S> { #[inline] fn add(&self, other: &Mat3<S>) -> Mat3<S> { build(|i| self.i(i).add_v(other.i(i))) } }
impl<S: PartOrdFloat<S>> Add<Mat4<S>, Mat4<S>> for Mat4<S> { #[inline] fn add(&self, other: &Mat4<S>) -> Mat4<S> { build(|i| self.i(i).add_v(other.i(i))) } }

impl<S: PartOrdFloat<S>> Sub<Mat2<S>, Mat2<S>> for Mat2<S> { #[inline] fn sub(&self, other: &Mat2<S>) -> Mat2<S> { build(|i| self.i(i).sub_v(other.i(i))) } }
impl<S: PartOrdFloat<S>> Sub<Mat3<S>, Mat3<S>> for Mat3<S> { #[inline] fn sub(&self, other: &Mat3<S>) -> Mat3<S> { build(|i| self.i(i).sub_v(other.i(i))) } }
impl<S: PartOrdFloat<S>> Sub<Mat4<S>, Mat4<S>> for Mat4<S> { #[inline] fn sub(&self, other: &Mat4<S>) -> Mat4<S> { build(|i| self.i(i).sub_v(other.i(i))) } }

impl<S: PartOrdFloat<S>> Neg<Mat2<S>> for Mat2<S> { #[inline] fn neg(&self) -> Mat2<S> { build(|i| self.i(i).neg()) } }
impl<S: PartOrdFloat<S>> Neg<Mat3<S>> for Mat3<S> { #[inline] fn neg(&self) -> Mat3<S> { build(|i| self.i(i).neg()) } }
impl<S: PartOrdFloat<S>> Neg<Mat4<S>> for Mat4<S> { #[inline] fn neg(&self) -> Mat4<S> { build(|i| self.i(i).neg()) } }

impl<S: PartOrdFloat<S>> Zero for Mat2<S> { #[inline] fn zero() -> Mat2<S> { Mat2::zero() } #[inline] fn is_zero(&self) -> bool { *self == zero() } }
impl<S: PartOrdFloat<S>> Zero for Mat3<S> { #[inline] fn zero() -> Mat3<S> { Mat3::zero() } #[inline] fn is_zero(&self) -> bool { *self == zero() } }
impl<S: PartOrdFloat<S>> Zero for Mat4<S> { #[inline] fn zero() -> Mat4<S> { Mat4::zero() } #[inline] fn is_zero(&self) -> bool { *self == zero() } }

impl<S: PartOrdFloat<S>> Mul<Mat2<S>, Mat2<S>> for Mat2<S> { #[inline] fn mul(&self, other: &Mat2<S>) -> Mat2<S> { build(|i| self.i(i).mul_v(other.i(i))) } }
impl<S: PartOrdFloat<S>> Mul<Mat3<S>, Mat3<S>> for Mat3<S> { #[inline] fn mul(&self, other: &Mat3<S>) -> Mat3<S> { build(|i| self.i(i).mul_v(other.i(i))) } }
impl<S: PartOrdFloat<S>> Mul<Mat4<S>, Mat4<S>> for Mat4<S> { #[inline] fn mul(&self, other: &Mat4<S>) -> Mat4<S> { build(|i| self.i(i).mul_v(other.i(i))) } }

impl<S: PartOrdFloat<S>> One for Mat2<S> { #[inline] fn one() -> Mat2<S> { Mat2::identity() } }
impl<S: PartOrdFloat<S>> One for Mat3<S> { #[inline] fn one() -> Mat3<S> { Mat3::identity() } }
impl<S: PartOrdFloat<S>> One for Mat4<S> { #[inline] fn one() -> Mat4<S> { Mat4::identity() } }

impl<S: PartOrdFloat<S>>
Matrix<S, [Vec2<S>, ..2], Vec2<S>, [S, ..2]>
for Mat2<S>
{
    fn mul_m(&self, other: &Mat2<S>) -> Mat2<S> {
        Mat2::new(self.r(0).dot(other.c(0)), self.r(1).dot(other.c(0)),
                  self.r(0).dot(other.c(1)), self.r(1).dot(other.c(1)))
    }

    fn transpose(&self) -> Mat2<S> {
        Mat2::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(),
                  self.cr(0, 1).clone(), self.cr(1, 1).clone())
    }

    #[inline]
    fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
    }

    #[inline]
    fn determinant(&self) -> S {
        *self.cr(0, 0) * *self.cr(1, 1) - *self.cr(1, 0) * *self.cr(0, 1)
    }

    #[inline]
    fn invert(&self) -> Option<Mat2<S>> {
        let det = self.determinant();
        if det.approx_eq(&zero()) {
            None
        } else {
            Some(Mat2::new( *self.cr(1, 1) / det, -*self.cr(0, 1) / det,
                           -*self.cr(1, 0) / det,  *self.cr(0, 0) / det))
        }
    }

    #[inline]
    fn is_diagonal(&self) -> bool {
        self.cr(0, 1).approx_eq(&zero()) &&
        self.cr(1, 0).approx_eq(&zero())
    }


    #[inline]
    fn is_symmetric(&self) -> bool {
        self.cr(0, 1).approx_eq(self.cr(1, 0)) &&
        self.cr(1, 0).approx_eq(self.cr(0, 1))
    }
}

impl<S: PartOrdFloat<S>>
Matrix<S, [Vec3<S>, ..3], Vec3<S>, [S, ..3]>
for Mat3<S>
{
    fn mul_m(&self, other: &Mat3<S>) -> Mat3<S> {
        Mat3::new(self.r(0).dot(other.c(0)),self.r(1).dot(other.c(0)),self.r(2).dot(other.c(0)),
                  self.r(0).dot(other.c(1)),self.r(1).dot(other.c(1)),self.r(2).dot(other.c(1)),
                  self.r(0).dot(other.c(2)),self.r(1).dot(other.c(2)),self.r(2).dot(other.c(2)))
    }

    fn transpose(&self) -> Mat3<S> {
        Mat3::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(), self.cr(2, 0).clone(),
                  self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(2, 1).clone(),
                  self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(2, 2).clone())
    }

    #[inline]
    fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
        self.swap_cr((0, 2), (2, 0));
        self.swap_cr((1, 2), (2, 1));
    }

    fn determinant(&self) -> S {
        *self.cr(0, 0) * (*self.cr(1, 1) * *self.cr(2, 2) - *self.cr(2, 1) * *self.cr(1, 2)) -
        *self.cr(1, 0) * (*self.cr(0, 1) * *self.cr(2, 2) - *self.cr(2, 1) * *self.cr(0, 2)) +
        *self.cr(2, 0) * (*self.cr(0, 1) * *self.cr(1, 2) - *self.cr(1, 1) * *self.cr(0, 2))
    }

    fn invert(&self) -> Option<Mat3<S>> {
        let det = self.determinant();
        if det.approx_eq(&zero()) { None } else {
            Some(Mat3::from_cols(self.c(1).cross(self.c(2)).div_s(det.clone()),
                                 self.c(2).cross(self.c(0)).div_s(det.clone()),
                                 self.c(0).cross(self.c(1)).div_s(det.clone())).transpose())
        }
    }

    fn is_diagonal(&self) -> bool {
        self.cr(0, 1).approx_eq(&zero()) &&
        self.cr(0, 2).approx_eq(&zero()) &&

        self.cr(1, 0).approx_eq(&zero()) &&
        self.cr(1, 2).approx_eq(&zero()) &&

        self.cr(2, 0).approx_eq(&zero()) &&
        self.cr(2, 1).approx_eq(&zero())
    }

    fn is_symmetric(&self) -> bool {
        self.cr(0, 1).approx_eq(self.cr(1, 0)) &&
        self.cr(0, 2).approx_eq(self.cr(2, 0)) &&

        self.cr(1, 0).approx_eq(self.cr(0, 1)) &&
        self.cr(1, 2).approx_eq(self.cr(2, 1)) &&

        self.cr(2, 0).approx_eq(self.cr(0, 2)) &&
        self.cr(2, 1).approx_eq(self.cr(1, 2))
    }
}

// Using self.r(0).dot(other.c(0)) like the other matrix multiplies
// causes the LLVM to miss identical loads and multiplies. This optimization
// causes the code to be auto vectorized properly increasing the performance
// around ~4 times.
macro_rules! dot_mat4(
    ($A:expr, $B:expr, $I:expr, $J:expr) => (
        (*$A.cr(0, $I)) * (*$B.cr($J, 0)) +
        (*$A.cr(1, $I)) * (*$B.cr($J, 1)) +
        (*$A.cr(2, $I)) * (*$B.cr($J, 2)) +
        (*$A.cr(3, $I)) * (*$B.cr($J, 3))
))

impl<S: PartOrdFloat<S>>
Matrix<S, [Vec4<S>, ..4], Vec4<S>, [S, ..4]>
for Mat4<S>
{
    fn mul_m(&self, other: &Mat4<S>) -> Mat4<S> {
        Mat4::new(dot_mat4!(self, other, 0, 0), dot_mat4!(self, other, 1, 0), dot_mat4!(self, other, 2, 0), dot_mat4!(self, other, 3, 0),
                  dot_mat4!(self, other, 0, 1), dot_mat4!(self, other, 1, 1), dot_mat4!(self, other, 2, 1), dot_mat4!(self, other, 3, 1),
                  dot_mat4!(self, other, 0, 2), dot_mat4!(self, other, 1, 2), dot_mat4!(self, other, 2, 2), dot_mat4!(self, other, 3, 2),
                  dot_mat4!(self, other, 0, 3), dot_mat4!(self, other, 1, 3), dot_mat4!(self, other, 2, 3), dot_mat4!(self, other, 3, 3))
    }

    fn transpose(&self) -> Mat4<S> {
        Mat4::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(), self.cr(2, 0).clone(), self.cr(3, 0).clone(),
                  self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(2, 1).clone(), self.cr(3, 1).clone(),
                  self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(2, 2).clone(), self.cr(3, 2).clone(),
                  self.cr(0, 3).clone(), self.cr(1, 3).clone(), self.cr(2, 3).clone(), self.cr(3, 3).clone())
    }

    fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
        self.swap_cr((0, 2), (2, 0));
        self.swap_cr((0, 3), (3, 0));
        self.swap_cr((1, 2), (2, 1));
        self.swap_cr((1, 3), (3, 1));
        self.swap_cr((2, 3), (3, 2));
    }

    fn determinant(&self) -> S {
        let m0 = Mat3::new(self.cr(1, 1).clone(), self.cr(2, 1).clone(), self.cr(3, 1).clone(),
                           self.cr(1, 2).clone(), self.cr(2, 2).clone(), self.cr(3, 2).clone(),
                           self.cr(1, 3).clone(), self.cr(2, 3).clone(), self.cr(3, 3).clone());
        let m1 = Mat3::new(self.cr(0, 1).clone(), self.cr(2, 1).clone(), self.cr(3, 1).clone(),
                           self.cr(0, 2).clone(), self.cr(2, 2).clone(), self.cr(3, 2).clone(),
                           self.cr(0, 3).clone(), self.cr(2, 3).clone(), self.cr(3, 3).clone());
        let m2 = Mat3::new(self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(3, 1).clone(),
                           self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(3, 2).clone(),
                           self.cr(0, 3).clone(), self.cr(1, 3).clone(), self.cr(3, 3).clone());
        let m3 = Mat3::new(self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(2, 1).clone(),
                           self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(2, 2).clone(),
                           self.cr(0, 3).clone(), self.cr(1, 3).clone(), self.cr(2, 3).clone());

        *self.cr(0, 0) * m0.determinant() -
        *self.cr(1, 0) * m1.determinant() +
        *self.cr(2, 0) * m2.determinant() -
        *self.cr(3, 0) * m3.determinant()
    }

    fn invert(&self) -> Option<Mat4<S>> {
        if self.is_invertible() {
            // Gauss Jordan Elimination with partial pivoting
            // So take this matrix ('mat') augmented with the identity ('ident'),
            // and essentially reduce [mat|ident]

            let mut mat = self.clone();
            let mut ident = Mat4::identity();

            for j in range(0u, 4u) {
                // Find largest element in col j
                let mut i1 = j;
                for i in range(j + 1, 4) {
                    if mat.cr(j, i).abs() > mat.cr(j, i1).abs() {
                        i1 = i;
                    }
                }

                // Swap columns i1 and j in mat and ident to
                // put pivot on diagonal
                mat.swap_c(i1, j);
                ident.swap_c(i1, j);

                // Scale col j to have a unit diagonal
                *ident.mut_c(j) = ident.c(j).div_s(mat.cr(j, j).clone());
                *mat.mut_c(j) = mat.c(j).div_s(mat.cr(j, j).clone());

                // Eliminate off-diagonal elems in col j of 'mat',
                // doing identical ops to 'ident'
                for i in range(0u, 4u) {
                    if i != j {
                        let ij_mul_aij = ident.c(j).mul_s(mat.cr(i, j).clone());
                        let aj_mul_aij = mat.c(j).mul_s(mat.cr(i, j).clone());
                        *ident.mut_c(i) = ident.c(i).sub_v(&ij_mul_aij);
                        *mat.mut_c(i) = mat.c(i).sub_v(&aj_mul_aij);
                    }
                }
            }
            Some(ident)
        } else {
            None
        }
    }

    fn is_diagonal(&self) -> bool {
        self.cr(0, 1).approx_eq(&zero()) &&
        self.cr(0, 2).approx_eq(&zero()) &&
        self.cr(0, 3).approx_eq(&zero()) &&

        self.cr(1, 0).approx_eq(&zero()) &&
        self.cr(1, 2).approx_eq(&zero()) &&
        self.cr(1, 3).approx_eq(&zero()) &&

        self.cr(2, 0).approx_eq(&zero()) &&
        self.cr(2, 1).approx_eq(&zero()) &&
        self.cr(2, 3).approx_eq(&zero()) &&

        self.cr(3, 0).approx_eq(&zero()) &&
        self.cr(3, 1).approx_eq(&zero()) &&
        self.cr(3, 2).approx_eq(&zero())
    }

    fn is_symmetric(&self) -> bool {
        self.cr(0, 1).approx_eq(self.cr(1, 0)) &&
        self.cr(0, 2).approx_eq(self.cr(2, 0)) &&
        self.cr(0, 3).approx_eq(self.cr(3, 0)) &&

        self.cr(1, 0).approx_eq(self.cr(0, 1)) &&
        self.cr(1, 2).approx_eq(self.cr(2, 1)) &&
        self.cr(1, 3).approx_eq(self.cr(3, 1)) &&

        self.cr(2, 0).approx_eq(self.cr(0, 2)) &&
        self.cr(2, 1).approx_eq(self.cr(1, 2)) &&
        self.cr(2, 3).approx_eq(self.cr(3, 2)) &&

        self.cr(3, 0).approx_eq(self.cr(0, 3)) &&
        self.cr(3, 1).approx_eq(self.cr(1, 3)) &&
        self.cr(3, 2).approx_eq(self.cr(2, 3))
    }
}

// Conversion traits
pub trait ToMat2<S: Primitive> { fn to_mat2(&self) -> Mat2<S>; }
pub trait ToMat3<S: Primitive> { fn to_mat3(&self) -> Mat3<S>; }
pub trait ToMat4<S: Primitive> { fn to_mat4(&self) -> Mat4<S>; }

impl<S: PartOrdFloat<S>>
ToMat3<S> for Mat2<S> {
    /// Clone the elements of a 2-dimensional matrix into the top corner of a
    /// 3-dimensional identity matrix.
    fn to_mat3(&self) -> Mat3<S> {
        Mat3::new(self.cr(0, 0).clone(), self.cr(0, 1).clone(), zero(),
                  self.cr(1, 0).clone(), self.cr(1, 1).clone(), zero(),
                  zero(), zero(), one())
    }
}

impl<S: PartOrdFloat<S>>
ToMat4<S> for Mat2<S> {
    /// Clone the elements of a 2-dimensional matrix into the top corner of a
    /// 4-dimensional identity matrix.
    fn to_mat4(&self) -> Mat4<S> {
        Mat4::new(self.cr(0, 0).clone(), self.cr(0, 1).clone(), zero(), zero(),
                  self.cr(1, 0).clone(), self.cr(1, 1).clone(), zero(), zero(),
                  zero(), zero(), one(), zero(),
                  zero(), zero(), zero(), one())
    }
}

impl<S: PartOrdFloat<S>>
ToMat4<S> for Mat3<S> {
    /// Clone the elements of a 3-dimensional matrix into the top corner of a
    /// 4-dimensional identity matrix.
    fn to_mat4(&self) -> Mat4<S> {
        Mat4::new(self.cr(0, 0).clone(), self.cr(0, 1).clone(), self.cr(0, 2).clone(), zero(),
                  self.cr(1, 0).clone(), self.cr(1, 1).clone(), self.cr(1, 2).clone(), zero(),
                  self.cr(2, 0).clone(), self.cr(2, 1).clone(), self.cr(2, 2).clone(), zero(),
                  zero(), zero(), zero(), one())
    }
}

impl<S: PartOrdFloat<S>>
ToQuat<S> for Mat3<S> {
    /// Convert the matrix to a quaternion
    fn to_quat(&self) -> Quat<S> {
        // http://www.cs.ucr.edu/~vbz/resources/quatut.pdf
        let trace = self.trace();
        let half: S = cast(0.5).unwrap();
        match () {
            () if trace >= zero::<S>() => {
                let s = sqrt(one::<S>() + trace);
                let w = half * s;
                let s = half / s;
                let x = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                let y = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                let z = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                Quat::new(w, x, y, z)
            }
            () if (*self.cr(0, 0) > *self.cr(1, 1)) && (*self.cr(0, 0) > *self.cr(2, 2)) => {
                let s = sqrt(half + (*self.cr(0, 0) - *self.cr(1, 1) - *self.cr(2, 2)));
                let w = half * s;
                let s = half / s;
                let x = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                let y = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                let z = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                Quat::new(w, x, y, z)
            }
            () if *self.cr(1, 1) > *self.cr(2, 2) => {
                let s = sqrt(half + (*self.cr(1, 1) - *self.cr(0, 0) - *self.cr(2, 2)));
                let w = half * s;
                let s = half / s;
                let x = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                let y = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                let z = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                Quat::new(w, x, y, z)
            }
            () => {
                let s = sqrt(half + (*self.cr(2, 2) - *self.cr(0, 0) - *self.cr(1, 1)));
                let w = half * s;
                let s = half / s;
                let x = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                let y = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                let z = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                Quat::new(w, x, y, z)
            }
        }
    }
}

impl<S: PartOrdFloat<S> + fmt::Show> fmt::Show for Mat2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "[[{}, {}], [{}, {}]]",
                self.cr(0, 0), self.cr(0, 1),
                self.cr(1, 0), self.cr(1, 1))
    }
}

impl<S: PartOrdFloat<S> + fmt::Show> fmt::Show for Mat3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "[[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]]",
                self.cr(0, 0), self.cr(0, 1), self.cr(0, 2),
                self.cr(1, 0), self.cr(1, 1), self.cr(1, 2),
                self.cr(2, 0), self.cr(2, 1), self.cr(2, 2))
    }
}

impl<S: PartOrdFloat<S> + fmt::Show> fmt::Show for Mat4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "[[{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}]]",
                self.cr(0, 0), self.cr(0, 1), self.cr(0, 2), self.cr(0, 3),
                self.cr(1, 0), self.cr(1, 1), self.cr(1, 2), self.cr(1, 3),
                self.cr(2, 0), self.cr(2, 1), self.cr(2, 2), self.cr(2, 3),
                self.cr(3, 0), self.cr(3, 1), self.cr(3, 2), self.cr(3, 3))
    }
}
