// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
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
use std::num::{Zero, zero, One, one, cast};

use angle::{Rad, sin, cos, sin_cos};
use approx::ApproxEq;
use array::{Array, build};
use num::{BaseFloat, BaseNum};
use point::{Point, Point3};
use quaternion::{Quaternion, ToQuaternion};
use vector::{Vector, EuclideanVector};
use vector::{Vector2, Vector3, Vector4};

/// A 2 x 2, column major matrix
#[deriving(Clone, Eq)]
pub struct Matrix2<S> { pub x: Vector2<S>, pub y: Vector2<S> }

/// A 3 x 3, column major matrix
#[deriving(Clone, Eq)]
pub struct Matrix3<S> { pub x: Vector3<S>, pub y: Vector3<S>, pub z: Vector3<S> }

/// A 4 x 4, column major matrix
#[deriving(Clone, Eq)]
pub struct Matrix4<S> { pub x: Vector4<S>, pub y: Vector4<S>, pub z: Vector4<S>, pub w: Vector4<S> }


impl<S: BaseNum> Matrix2<S> {
    /// Create a new matrix, providing values for each index.
    #[inline]
    pub fn new(c0r0: S, c0r1: S,
               c1r0: S, c1r1: S) -> Matrix2<S> {
        Matrix2::from_cols(Vector2::new(c0r0, c0r1),
                           Vector2::new(c1r0, c1r1))
    }

    /// Create a new matrix, providing columns.
    #[inline]
    pub fn from_cols(c0: Vector2<S>, c1: Vector2<S>) -> Matrix2<S> {
        Matrix2 { x: c0, y: c1 }
    }

    /// Create a new diagonal matrix, providing a single value to use for each
    /// non-zero index.
    #[inline]
    pub fn from_value(value: S) -> Matrix2<S> {
        Matrix2::new(value.clone(), zero(),
                     zero(), value.clone())
    }

    /// Create a zero matrix (all zeros).
    #[inline]
    pub fn zero() -> Matrix2<S> {
        Matrix2::from_value(zero())
    }

    /// Create an identity matrix (diagonal matrix of ones).
    #[inline]
    pub fn identity() -> Matrix2<S> {
        Matrix2::from_value(one())
    }
}

impl<S: BaseFloat> Matrix2<S> {
    /// Create a transformation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    pub fn look_at(dir: &Vector2<S>, up: &Vector2<S>) -> Matrix2<S> {
        //TODO: verify look_at 2D
        Matrix2::from_cols(up.clone(), dir.clone()).transpose()
    }

    #[inline]
    pub fn from_angle(theta: Rad<S>) -> Matrix2<S> {
        let cos_theta = cos(theta.clone());
        let sin_theta = sin(theta.clone());

        Matrix2::new(cos_theta.clone(), -sin_theta.clone(),
                     sin_theta.clone(),  cos_theta.clone())
    }
}

impl<S: BaseNum> Matrix3<S> {
    /// Create a new matrix, providing values for each index.
    #[inline]
    pub fn new(c0r0:S, c0r1:S, c0r2:S,
               c1r0:S, c1r1:S, c1r2:S,
               c2r0:S, c2r1:S, c2r2:S) -> Matrix3<S> {
        Matrix3::from_cols(Vector3::new(c0r0, c0r1, c0r2),
                           Vector3::new(c1r0, c1r1, c1r2),
                           Vector3::new(c2r0, c2r1, c2r2))
    }

    /// Create a new matrix, providing columns.
    #[inline]
    pub fn from_cols(c0: Vector3<S>, c1: Vector3<S>, c2: Vector3<S>) -> Matrix3<S> {
        Matrix3 { x: c0, y: c1, z: c2 }
    }

    /// Create a new diagonal matrix, providing a single value to use for each
    /// non-zero index.
    #[inline]
    pub fn from_value(value: S) -> Matrix3<S> {
        Matrix3::new(value.clone(), zero(), zero(),
                     zero(), value.clone(), zero(),
                     zero(), zero(), value.clone())
    }

    /// Create a zero matrix (all zeros).
    #[inline]
    pub fn zero() -> Matrix3<S> {
        Matrix3::from_value(zero())
    }

    /// Create an identity matrix (diagonal matrix of ones).
    #[inline]
    pub fn identity() -> Matrix3<S> {
        Matrix3::from_value(one())
    }
}

impl<S: BaseFloat>
Matrix3<S> {
    /// Create a transformation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    pub fn look_at(dir: &Vector3<S>, up: &Vector3<S>) -> Matrix3<S> {
        let dir = dir.normalize();
        let side = up.cross(&dir).normalize();
        let up = dir.cross(&side).normalize();

        Matrix3::from_cols(side, up, dir).transpose()
    }

    /// Create a matrix from a rotation around the `x` axis (pitch).
    pub fn from_angle_x(theta: Rad<S>) -> Matrix3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = sin_cos(theta);
        Matrix3::new( one(),     zero(),    zero(),
                     zero(),  c.clone(), s.clone(),
                     zero(), -s.clone(), c.clone())
    }

    /// Create a matrix from a rotation around the `y` axis (yaw).
    pub fn from_angle_y(theta: Rad<S>) -> Matrix3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = sin_cos(theta);
        Matrix3::new(c.clone(), zero(), -s.clone(),
                        zero(),  one(),     zero(),
                     s.clone(), zero(),  c.clone())
    }

    /// Create a matrix from a rotation around the `z` axis (roll).
    pub fn from_angle_z(theta: Rad<S>) -> Matrix3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = sin_cos(theta);
        Matrix3::new( c.clone(), s.clone(), zero(),
                     -s.clone(), c.clone(), zero(),
                         zero(),    zero(),  one())
    }

    /// Create a matrix from a set of euler angles.
    ///
    /// # Parameters
    ///
    /// - `x`: the angular rotation around the `x` axis (pitch).
    /// - `y`: the angular rotation around the `y` axis (yaw).
    /// - `z`: the angular rotation around the `z` axis (roll).
    pub fn from_euler(x: Rad<S>, y: Rad<S>, z: Rad<S>) -> Matrix3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
        let (sx, cx) = sin_cos(x);
        let (sy, cy) = sin_cos(y);
        let (sz, cz) = sin_cos(z);

        Matrix3::new(                cy * cz,                 cy * sz,     -sy,
                     -cx * sz + sx * sy * cz,  cx * cz + sx * sy * sz, sx * cy,
                      sx * sz + cx * sy * cz, -sx * cz + cx * sy * sz, cx * cy)
    }

    /// Create a matrix from a rotation around an arbitrary axis
    pub fn from_axis_angle(axis: &Vector3<S>, angle: Rad<S>) -> Matrix3<S> {
        let (s, c) = sin_cos(angle);
        let _1subc = one::<S>() - c;

        Matrix3::new(_1subc * axis.x * axis.x + c,
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

impl<S: BaseNum> Matrix4<S> {
    /// Create a new matrix, providing values for each index.
    #[inline]
    pub fn new(c0r0: S, c0r1: S, c0r2: S, c0r3: S,
               c1r0: S, c1r1: S, c1r2: S, c1r3: S,
               c2r0: S, c2r1: S, c2r2: S, c2r3: S,
               c3r0: S, c3r1: S, c3r2: S, c3r3: S) -> Matrix4<S>  {
        Matrix4::from_cols(Vector4::new(c0r0, c0r1, c0r2, c0r3),
                           Vector4::new(c1r0, c1r1, c1r2, c1r3),
                           Vector4::new(c2r0, c2r1, c2r2, c2r3),
                           Vector4::new(c3r0, c3r1, c3r2, c3r3))
    }

    /// Create a new matrix, providing columns.
    #[inline]
    pub fn from_cols(c0: Vector4<S>, c1: Vector4<S>, c2: Vector4<S>, c3: Vector4<S>) -> Matrix4<S> {
        Matrix4 { x: c0, y: c1, z: c2, w: c3 }
    }

    /// Create a new diagonal matrix, providing a single value to use for each
    /// non-zero index.
    #[inline]
    pub fn from_value(value: S) -> Matrix4<S> {
        Matrix4::new(value.clone(),        zero(),        zero(),        zero(),
                            zero(), value.clone(),        zero(),        zero(),
                            zero(),        zero(), value.clone(),        zero(),
                            zero(),        zero(),        zero(), value.clone())
    }

    /// Create a zero matrix (all zeros).
    #[inline]
    pub fn zero() -> Matrix4<S> {
        Matrix4::from_value(zero())
    }

    /// Create an identity matrix (diagonal matrix of ones).
    #[inline]
    pub fn identity() -> Matrix4<S> {
        Matrix4::from_value(one())
    }
}

impl<S: BaseFloat>
Matrix4<S> {
    /// Create a transformation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    pub fn look_at(eye: &Point3<S>, center: &Point3<S>, up: &Vector3<S>) -> Matrix4<S> {
        let f = center.sub_p(eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(&f);

        Matrix4::new( s.x.clone(),  u.x.clone(), -f.x.clone(), zero(),
                      s.y.clone(),  u.y.clone(), -f.y.clone(), zero(),
                      s.z.clone(),  u.z.clone(), -f.z.clone(), zero(),
                     -eye.dot(&s), -eye.dot(&u),  eye.dot(&f),  one())
    }
}

array!(impl<S> Matrix2<S> -> [Vector2<S>, ..2] _2)
array!(impl<S> Matrix3<S> -> [Vector3<S>, ..3] _3)
array!(impl<S> Matrix4<S> -> [Vector4<S>, ..4] _4)

pub trait Matrix
<
    S: BaseFloat, Slice,
    V: Clone + Vector<S, VSlice> + Array<S, VSlice>, VSlice
>
:   Array<V, Slice>
+   Neg<Self>
+   Zero + One
+   ApproxEq<S>
{
    /// Get a shared reference to a column of this matrix.
    #[inline]
    fn c<'a>(&'a self, c: uint) -> &'a V { self.i(c) }

    /// Get a mutable reference to a column of this matrix.
    #[inline]
    fn mut_c<'a>(&'a mut self, c: uint) -> &'a mut V { self.mut_i(c) }

    /// Swap two columns of this matrix.
    #[inline]
    fn swap_c(&mut self, a: uint, b: uint) {
        let tmp = self.c(a).clone();
        *self.mut_c(a) = self.c(b).clone();
        *self.mut_c(b) = tmp;
    }

    /// Get a row from this matrix.
    ///
    /// Since matrixes in cgmath are stored column-major, this cannot return a
    /// reference. It creates a new copy of the row instead.
    #[inline]
    fn r(&self, r: uint) -> V {
        build(|i| self.i(i).i(r).clone())
    }

    /// Swap two rows of this matrix.
    #[inline]
    fn swap_r(&mut self, a: uint, b: uint) {
        self.each_mut(|_, c| c.swap(a, b))
    }

    /// Return a shared reference to the element at column `c` and row `r`.
    #[inline]
    fn cr<'a>(&'a self, c: uint, r: uint) -> &'a S { self.i(c).i(r) }

    /// Return a mutable reference to the element at column `c` and row `r`.
    #[inline]
    fn mut_cr<'a>(&'a mut self, c: uint, r: uint) -> &'a mut S {
        self.mut_i(c).mut_i(r)
    }

    /// Swap the values at index `a` and `b`
    #[inline]
    fn swap_cr(&mut self, a: (uint, uint), b: (uint, uint)) {
        let (ca, ra) = a;
        let (cb, rb) = b;
        let tmp = self.cr(ca, ra).clone();
        *self.mut_cr(ca, ra) = self.cr(cb, rb).clone();
        *self.mut_cr(cb, rb) = tmp;
    }

    /// Negate this matrix in-place (multiply by scalar -1).
    #[inline] fn neg_self(&mut self) { self.each_mut(|_, x| *x = x.neg()) }

    /// Multiply this matrix by a scalar, returning the new matrix.
    #[inline] fn mul_s(&self, s: S) -> Self { build(|i| self.c(i).mul_s(s.clone())) }
    /// Divide this matrix by a scalar, returning the new matrix.
    #[inline] fn div_s(&self, s: S) -> Self { build(|i| self.c(i).div_s(s.clone())) }
    /// Take the remainder of this matrix by a scalar, returning the new
    /// matrix.
    #[inline] fn rem_s(&self, s: S) -> Self { build(|i| self.c(i).rem_s(s.clone())) }

    /// Add this matrix with another matrix, returning the new metrix.
    #[inline] fn add_m(&self, other: &Self) -> Self { build(|i| self.i(i).add_v(other.i(i))) }
    /// Subtract another matrix from this matrix, returning the new matrix.
    #[inline] fn sub_m(&self, other: &Self) -> Self { build(|i| self.i(i).sub_v(other.i(i))) }

    /// Multiplay a vector by this matrix, returning a new vector.
    #[inline] fn mul_v(&self, v: &V) -> V { build(|i| self.r(i).dot(v)) }

    /// Multiply this matrix by another matrix, returning the new matrix.
    fn mul_m(&self, other: &Self) -> Self;

    /// Multiply this matrix by a scalar, in-place.
    #[inline] fn mul_self_s(&mut self, s: S) { self.each_mut(|_, c| *c = c.mul_s(s.clone())) }
    /// Divide this matrix by a scalar, in-place.
    #[inline] fn div_self_s(&mut self, s: S) { self.each_mut(|_, c| *c = c.div_s(s.clone())) }
    /// Take the remainder of this matrix, in-place.
    #[inline] fn rem_self_s(&mut self, s: S) { self.each_mut(|_, c| *c = c.rem_s(s.clone())) }

    /// Add this matrix with another matrix, in-place.
    #[inline] fn add_self_m(&mut self, other: &Self) { self.each_mut(|i, c| *c = c.add_v(other.c(i))) }
    /// Subtract another matrix from this matrix, in-place.
    #[inline] fn sub_self_m(&mut self, other: &Self) { self.each_mut(|i, c| *c = c.sub_v(other.c(i))) }

    /// Multiply this matrix by another matrix, in-place.
    #[inline] fn mul_self_m(&mut self, other: &Self) { *self = self.mul_m(other); }

    /// Transpose this matrix, returning a new matrix.
    fn transpose(&self) -> Self;
    /// Transpose this matrix in-place.
    fn transpose_self(&mut self);
    /// Take the determinant of this matrix.
    fn determinant(&self) -> S;

    /// Return a vector containing the diagonal of this matrix.
    #[inline]
    fn diagonal(&self) -> V { build(|i| self.cr(i, i).clone()) }

    /// Return the trace of this matrix. That is, the sum of the diagonal.
    #[inline]
    fn trace(&self) -> S { self.diagonal().comp_add() }

    /// Invert this matrix, returning a new matrix. `m.mul_m(m.invert())` is
    /// the identity matrix. Returns `None` if this matrix is not invertible
    /// (has a determinant of zero).
    fn invert(&self) -> Option<Self>;

    /// Invert this matrix in-place.
    #[inline]
    fn invert_self(&mut self) {
        *self = self.invert().expect("Attempted to invert a matrix with zero determinant.");
    }

    /// Test if this matrix is invertible.
    #[inline]
    fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&zero())
    }

    /// Test if this matrix is the identity matrix. That is, it is diagonal
    /// and every element in the diagonal is one.
    #[inline]
    fn is_identity(&self) -> bool {
        self.approx_eq(&one())
    }

    /// Test if this is a diagonal matrix. That is, every element outside of
    /// the diagonal is 0.
    fn is_diagonal(&self) -> bool;

    /// Test if this matrix is symmetric. That is, it is equal to its
    /// transpose.
    fn is_symmetric(&self) -> bool;
}

impl<S: BaseFloat> Add<Matrix2<S>, Matrix2<S>> for Matrix2<S> { #[inline] fn add(&self, other: &Matrix2<S>) -> Matrix2<S> { self.add_m(other) } }
impl<S: BaseFloat> Add<Matrix3<S>, Matrix3<S>> for Matrix3<S> { #[inline] fn add(&self, other: &Matrix3<S>) -> Matrix3<S> { self.add_m(other) } }
impl<S: BaseFloat> Add<Matrix4<S>, Matrix4<S>> for Matrix4<S> { #[inline] fn add(&self, other: &Matrix4<S>) -> Matrix4<S> { self.add_m(other) } }

impl<S: BaseFloat> Sub<Matrix2<S>, Matrix2<S>> for Matrix2<S> { #[inline] fn sub(&self, other: &Matrix2<S>) -> Matrix2<S> { self.sub_m(other) } }
impl<S: BaseFloat> Sub<Matrix3<S>, Matrix3<S>> for Matrix3<S> { #[inline] fn sub(&self, other: &Matrix3<S>) -> Matrix3<S> { self.sub_m(other) } }
impl<S: BaseFloat> Sub<Matrix4<S>, Matrix4<S>> for Matrix4<S> { #[inline] fn sub(&self, other: &Matrix4<S>) -> Matrix4<S> { self.sub_m(other) } }

impl<S: BaseFloat> Neg<Matrix2<S>> for Matrix2<S> { #[inline] fn neg(&self) -> Matrix2<S> { build(|i| self.i(i).neg()) } }
impl<S: BaseFloat> Neg<Matrix3<S>> for Matrix3<S> { #[inline] fn neg(&self) -> Matrix3<S> { build(|i| self.i(i).neg()) } }
impl<S: BaseFloat> Neg<Matrix4<S>> for Matrix4<S> { #[inline] fn neg(&self) -> Matrix4<S> { build(|i| self.i(i).neg()) } }

impl<S: BaseFloat> Zero for Matrix2<S> { #[inline] fn zero() -> Matrix2<S> { Matrix2::zero() } #[inline] fn is_zero(&self) -> bool { *self == zero() } }
impl<S: BaseFloat> Zero for Matrix3<S> { #[inline] fn zero() -> Matrix3<S> { Matrix3::zero() } #[inline] fn is_zero(&self) -> bool { *self == zero() } }
impl<S: BaseFloat> Zero for Matrix4<S> { #[inline] fn zero() -> Matrix4<S> { Matrix4::zero() } #[inline] fn is_zero(&self) -> bool { *self == zero() } }

impl<S: BaseFloat> Mul<Matrix2<S>, Matrix2<S>> for Matrix2<S> { #[inline] fn mul(&self, other: &Matrix2<S>) -> Matrix2<S> { self.mul_m(other) } }
impl<S: BaseFloat> Mul<Matrix3<S>, Matrix3<S>> for Matrix3<S> { #[inline] fn mul(&self, other: &Matrix3<S>) -> Matrix3<S> { self.mul_m(other) } }
impl<S: BaseFloat> Mul<Matrix4<S>, Matrix4<S>> for Matrix4<S> { #[inline] fn mul(&self, other: &Matrix4<S>) -> Matrix4<S> { self.mul_m(other) } }

impl<S: BaseFloat> One for Matrix2<S> { #[inline] fn one() -> Matrix2<S> { Matrix2::identity() } }
impl<S: BaseFloat> One for Matrix3<S> { #[inline] fn one() -> Matrix3<S> { Matrix3::identity() } }
impl<S: BaseFloat> One for Matrix4<S> { #[inline] fn one() -> Matrix4<S> { Matrix4::identity() } }

impl<S: BaseFloat>
Matrix<S, [Vector2<S>, ..2], Vector2<S>, [S, ..2]>
for Matrix2<S>
{
    fn mul_m(&self, other: &Matrix2<S>) -> Matrix2<S> {
        Matrix2::new(self.r(0).dot(other.c(0)), self.r(1).dot(other.c(0)),
                     self.r(0).dot(other.c(1)), self.r(1).dot(other.c(1)))
    }

    fn transpose(&self) -> Matrix2<S> {
        Matrix2::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(),
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
    fn invert(&self) -> Option<Matrix2<S>> {
        let det = self.determinant();
        if det.approx_eq(&zero()) {
            None
        } else {
            Some(Matrix2::new( *self.cr(1, 1) / det, -*self.cr(0, 1) / det,
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

impl<S: BaseFloat>
Matrix<S, [Vector3<S>, ..3], Vector3<S>, [S, ..3]>
for Matrix3<S>
{
    fn mul_m(&self, other: &Matrix3<S>) -> Matrix3<S> {
        Matrix3::new(self.r(0).dot(other.c(0)),self.r(1).dot(other.c(0)),self.r(2).dot(other.c(0)),
                     self.r(0).dot(other.c(1)),self.r(1).dot(other.c(1)),self.r(2).dot(other.c(1)),
                     self.r(0).dot(other.c(2)),self.r(1).dot(other.c(2)),self.r(2).dot(other.c(2)))
    }

    fn transpose(&self) -> Matrix3<S> {
        Matrix3::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(), self.cr(2, 0).clone(),
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

    fn invert(&self) -> Option<Matrix3<S>> {
        let det = self.determinant();
        if det.approx_eq(&zero()) { None } else {
            Some(Matrix3::from_cols(self.c(1).cross(self.c(2)).div_s(det.clone()),
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
macro_rules! dot_matrix4(
    ($A:expr, $B:expr, $I:expr, $J:expr) => (
        (*$A.cr(0, $I)) * (*$B.cr($J, 0)) +
        (*$A.cr(1, $I)) * (*$B.cr($J, 1)) +
        (*$A.cr(2, $I)) * (*$B.cr($J, 2)) +
        (*$A.cr(3, $I)) * (*$B.cr($J, 3))
))

impl<S: BaseFloat>
Matrix<S, [Vector4<S>, ..4], Vector4<S>, [S, ..4]>
for Matrix4<S>
{
    fn mul_m(&self, other: &Matrix4<S>) -> Matrix4<S> {
        Matrix4::new(dot_matrix4!(self, other, 0, 0), dot_matrix4!(self, other, 1, 0), dot_matrix4!(self, other, 2, 0), dot_matrix4!(self, other, 3, 0),
                     dot_matrix4!(self, other, 0, 1), dot_matrix4!(self, other, 1, 1), dot_matrix4!(self, other, 2, 1), dot_matrix4!(self, other, 3, 1),
                     dot_matrix4!(self, other, 0, 2), dot_matrix4!(self, other, 1, 2), dot_matrix4!(self, other, 2, 2), dot_matrix4!(self, other, 3, 2),
                     dot_matrix4!(self, other, 0, 3), dot_matrix4!(self, other, 1, 3), dot_matrix4!(self, other, 2, 3), dot_matrix4!(self, other, 3, 3))
    }

    fn transpose(&self) -> Matrix4<S> {
        Matrix4::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(), self.cr(2, 0).clone(), self.cr(3, 0).clone(),
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
        let m0 = Matrix3::new(self.cr(1, 1).clone(), self.cr(2, 1).clone(), self.cr(3, 1).clone(),
                              self.cr(1, 2).clone(), self.cr(2, 2).clone(), self.cr(3, 2).clone(),
                              self.cr(1, 3).clone(), self.cr(2, 3).clone(), self.cr(3, 3).clone());
        let m1 = Matrix3::new(self.cr(0, 1).clone(), self.cr(2, 1).clone(), self.cr(3, 1).clone(),
                              self.cr(0, 2).clone(), self.cr(2, 2).clone(), self.cr(3, 2).clone(),
                              self.cr(0, 3).clone(), self.cr(2, 3).clone(), self.cr(3, 3).clone());
        let m2 = Matrix3::new(self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(3, 1).clone(),
                              self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(3, 2).clone(),
                              self.cr(0, 3).clone(), self.cr(1, 3).clone(), self.cr(3, 3).clone());
        let m3 = Matrix3::new(self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(2, 1).clone(),
                              self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(2, 2).clone(),
                              self.cr(0, 3).clone(), self.cr(1, 3).clone(), self.cr(2, 3).clone());

        *self.cr(0, 0) * m0.determinant() -
        *self.cr(1, 0) * m1.determinant() +
        *self.cr(2, 0) * m2.determinant() -
        *self.cr(3, 0) * m3.determinant()
    }

    fn invert(&self) -> Option<Matrix4<S>> {
        if self.is_invertible() {
            // Gauss Jordan Elimination with partial pivoting
            // So take this matrix ('mat') augmented with the identity ('ident'),
            // and essentially reduce [mat|ident]

            let mut mat = self.clone();
            let mut ident = Matrix4::identity();

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

/// Represents types which can be converted to a Matrix2
pub trait ToMatrix2<S: BaseNum> {
    /// Convert this value to a Matrix2
    fn to_matrix2(&self) -> Matrix2<S>;
}

/// Represents types which can be converted to a Matrix3
pub trait ToMatrix3<S: BaseNum> {
    /// Convert this value to a Matrix3
    fn to_matrix3(&self) -> Matrix3<S>;
}

/// Represents types which can be converted to a Matrix4
pub trait ToMatrix4<S: BaseNum> {
    /// Convert this value to a Matrix4
    fn to_matrix4(&self) -> Matrix4<S>;
}

impl<S: BaseFloat>
ToMatrix3<S> for Matrix2<S> {
    /// Clone the elements of a 2-dimensional matrix into the top-left corner
    /// of a 3-dimensional identity matrix.
    fn to_matrix3(&self) -> Matrix3<S> {
        Matrix3::new(self.cr(0, 0).clone(), self.cr(0, 1).clone(), zero(),
                     self.cr(1, 0).clone(), self.cr(1, 1).clone(), zero(),
                                    zero(),                zero(),  one())
    }
}

impl<S: BaseFloat>
ToMatrix4<S> for Matrix2<S> {
    /// Clone the elements of a 2-dimensional matrix into the top-left corner
    /// of a 4-dimensional identity matrix.
    fn to_matrix4(&self) -> Matrix4<S> {
        Matrix4::new(self.cr(0, 0).clone(), self.cr(0, 1).clone(), zero(), zero(),
                     self.cr(1, 0).clone(), self.cr(1, 1).clone(), zero(), zero(),
                                    zero(),                zero(),  one(), zero(),
                                    zero(),                zero(), zero(),  one())
    }
}

impl<S: BaseFloat>
ToMatrix4<S> for Matrix3<S> {
    /// Clone the elements of a 3-dimensional matrix into the top-left corner
    /// of a 4-dimensional identity matrix.
    fn to_matrix4(&self) -> Matrix4<S> {
        Matrix4::new(self.cr(0, 0).clone(), self.cr(0, 1).clone(), self.cr(0, 2).clone(), zero(),
                     self.cr(1, 0).clone(), self.cr(1, 1).clone(), self.cr(1, 2).clone(), zero(),
                     self.cr(2, 0).clone(), self.cr(2, 1).clone(), self.cr(2, 2).clone(), zero(),
                                    zero(),                zero(),                zero(),  one())
    }
}

impl<S: BaseFloat>
ToQuaternion<S> for Matrix3<S> {
    /// Convert the matrix to a quaternion
    fn to_quaternion(&self) -> Quaternion<S> {
        // http://www.cs.ucr.edu/~vbz/resources/quatut.pdf
        let trace = self.trace();
        let half: S = cast(0.5).unwrap();
        match () {
            () if trace >= zero::<S>() => {
                let s = (one::<S>() + trace).sqrt();
                let w = half * s;
                let s = half / s;
                let x = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                let y = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                let z = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                Quaternion::new(w, x, y, z)
            }
            () if (*self.cr(0, 0) > *self.cr(1, 1)) && (*self.cr(0, 0) > *self.cr(2, 2)) => {
                let s = (half + (*self.cr(0, 0) - *self.cr(1, 1) - *self.cr(2, 2))).sqrt();
                let w = half * s;
                let s = half / s;
                let x = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                let y = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                let z = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                Quaternion::new(w, x, y, z)
            }
            () if *self.cr(1, 1) > *self.cr(2, 2) => {
                let s = (half + (*self.cr(1, 1) - *self.cr(0, 0) - *self.cr(2, 2))).sqrt();
                let w = half * s;
                let s = half / s;
                let x = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                let y = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                let z = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                Quaternion::new(w, x, y, z)
            }
            () => {
                let s = (half + (*self.cr(2, 2) - *self.cr(0, 0) - *self.cr(1, 1))).sqrt();
                let w = half * s;
                let s = half / s;
                let x = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                let y = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                let z = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                Quaternion::new(w, x, y, z)
            }
        }
    }
}

impl<S: BaseFloat> fmt::Show for Matrix2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{}, {}], [{}, {}]]",
                self.cr(0, 0), self.cr(0, 1),
                self.cr(1, 0), self.cr(1, 1))
    }
}

impl<S: BaseFloat> fmt::Show for Matrix3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]]",
                self.cr(0, 0), self.cr(0, 1), self.cr(0, 2),
                self.cr(1, 0), self.cr(1, 1), self.cr(1, 2),
                self.cr(2, 0), self.cr(2, 1), self.cr(2, 2))
    }
}

impl<S: BaseFloat> fmt::Show for Matrix4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}]]",
                self.cr(0, 0), self.cr(0, 1), self.cr(0, 2), self.cr(0, 3),
                self.cr(1, 0), self.cr(1, 1), self.cr(1, 2), self.cr(1, 3),
                self.cr(2, 0), self.cr(2, 1), self.cr(2, 2), self.cr(2, 3),
                self.cr(3, 0), self.cr(3, 1), self.cr(3, 2), self.cr(3, 3))
    }
}
