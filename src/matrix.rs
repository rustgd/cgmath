// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
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

//! Column major, square matrix types and traits.

use std::fmt;
use std::mem;
use std::ops::*;
use std::ptr;

use rand::{Rand, Rng};

use rust_num::{Zero, One};
use rust_num::traits::cast;

use angle::{Rad, sin, cos, sin_cos};
use approx::ApproxEq;
use array::Array;
use num::{BaseFloat, BaseNum};
use point::{Point, Point3};
use quaternion::Quaternion;
use vector::{Vector, EuclideanVector};
use vector::{Vector2, Vector3, Vector4};

/// A 2 x 2, column major matrix
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Matrix2<S> { pub x: Vector2<S>, pub y: Vector2<S> }

/// A 3 x 3, column major matrix
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Matrix3<S> { pub x: Vector3<S>, pub y: Vector3<S>, pub z: Vector3<S> }

/// A 4 x 4, column major matrix
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Matrix4<S> { pub x: Vector4<S>, pub y: Vector4<S>, pub z: Vector4<S>, pub w: Vector4<S> }


impl<S> Matrix2<S> {
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
}

impl<S: BaseFloat> Matrix2<S> {
    /// Create a transformation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    pub fn look_at(dir: Vector2<S>, up: Vector2<S>) -> Matrix2<S> {
        //TODO: verify look_at 2D
        Matrix2::from_cols(up, dir).transpose()
    }

    #[inline]
    pub fn from_angle(theta: Rad<S>) -> Matrix2<S> {
        let cos_theta = cos(theta.clone());
        let sin_theta = sin(theta.clone());

        Matrix2::new(cos_theta.clone(),  sin_theta.clone(),
                     -sin_theta.clone(), cos_theta.clone())
    }
}

impl<S: Copy + Neg<Output = S>> Matrix2<S> {
    /// Negate this `Matrix2` in-place.
    #[inline]
    pub fn neg_self(&mut self) {
        self[0].neg_self();
        self[1].neg_self();
    }
}

impl<S> Matrix3<S> {
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
}

impl<S: BaseFloat> Matrix3<S> {
    /// Create a transformation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    pub fn look_at(dir: Vector3<S>, up: Vector3<S>) -> Matrix3<S> {
        let dir = dir.normalize();
        let side = up.cross(dir).normalize();
        let up = dir.cross(side).normalize();

        Matrix3::from_cols(side, up, dir).transpose()
    }

    /// Create a matrix from a rotation around the `x` axis (pitch).
    pub fn from_angle_x(theta: Rad<S>) -> Matrix3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = sin_cos(theta);
        Matrix3::new(S::one(), S::zero(), S::zero(),
                     S::zero(), c.clone(), s.clone(),
                     S::zero(), -s.clone(), c.clone())
    }

    /// Create a matrix from a rotation around the `y` axis (yaw).
    pub fn from_angle_y(theta: Rad<S>) -> Matrix3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = sin_cos(theta);
        Matrix3::new(c.clone(), S::zero(), -s.clone(),
                     S::zero(), S::one(), S::zero(),
                     s.clone(), S::zero(), c.clone())
    }

    /// Create a matrix from a rotation around the `z` axis (roll).
    pub fn from_angle_z(theta: Rad<S>) -> Matrix3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = sin_cos(theta);
        Matrix3::new( c.clone(), s.clone(), S::zero(),
                     -s.clone(), c.clone(), S::zero(),
                     S::zero(), S::zero(), S::one())
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
    pub fn from_axis_angle(axis: Vector3<S>, angle: Rad<S>) -> Matrix3<S> {
        let (s, c) = sin_cos(angle);
        let _1subc = S::one() - c;

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

impl<S: Copy + Neg<Output = S>> Matrix3<S> {
    /// Negate this `Matrix3` in-place.
    #[inline]
    pub fn neg_self(&mut self) {
        self[0].neg_self();
        self[1].neg_self();
        self[2].neg_self();
    }
}

impl<S> Matrix4<S> {
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
}

impl<S: BaseNum> Matrix4<S> {
    /// Create a translation matrix from a Vector3
    #[inline]
    pub fn from_translation(v: Vector3<S>) -> Matrix4<S> {
        Matrix4::new(S::one(), S::zero(), S::zero(), S::zero(),
                     S::zero(), S::one(), S::zero(), S::zero(),
                     S::zero(), S::zero(), S::one(), S::zero(),
                     v.x, v.y, v.z, S::one())
    }
}

impl<S: BaseFloat> Matrix4<S> {
    /// Create a transformation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    pub fn look_at(eye: Point3<S>, center: Point3<S>, up: Vector3<S>) -> Matrix4<S> {
        let f = (center - eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        Matrix4::new(s.x.clone(), u.x.clone(), -f.x.clone(), S::zero(),
                     s.y.clone(), u.y.clone(), -f.y.clone(), S::zero(),
                     s.z.clone(), u.z.clone(), -f.z.clone(), S::zero(),
                     -eye.dot(s), -eye.dot(u), eye.dot(f), S::one())
    }
}

impl<S: Copy + Neg<Output = S>> Matrix4<S> {
    /// Negate this `Matrix4` in-place.
    #[inline]
    pub fn neg_self(&mut self) {
        self[0].neg_self();
        self[1].neg_self();
        self[2].neg_self();
        self[3].neg_self();
    }
}

/// A column-major matrix of arbitrary dimensions.
pub trait Matrix  where
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: Index<usize, Output = <Self as Matrix>::Column>,
    Self: IndexMut<usize, Output = <Self as Matrix>::Column>,
    Self: ApproxEq<Epsilon = <Self as Matrix>::Element>,
    // FIXME: blocked by rust-lang/rust#20671
    //
    // for<'a, 'b> &'a Self: Add<&'b Self, Output = Self>,
    // for<'a, 'b> &'a Self: Sub<&'b Self, Output = Self>,
    // for<'a, 'b> &'a Self: Mul<&'b Self, Output = Self>,
    // for<'a, 'b> &'a Self: Mul<&'b V, Output = V>,
    //
    // for<'a> &'a Self: Mul<S, Output = Self>,
    // for<'a> &'a Self: Div<S, Output = Self>,
    // for<'a> &'a Self: Rem<S, Output = Self>,
{
    /// The type of the elements in the matrix.
    type Element: BaseFloat;

    /// The row vector of the matrix.
    type Row: Array<Element = Self::Element>;
    /// The column vector of the matrix.
    type Column: Array<Element = Self::Element>;

    /// The type of the transposed matrix
    type Transpose: Matrix<Element = Self::Element, Row = Self::Column, Column = Self::Row>;

    /// Get the pointer to the first element of the array.
    #[inline]
    fn as_ptr(&self) -> *const Self::Element {
        &self[0][0]
    }

    /// Get a mutable pointer to the first element of the array.
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Element {
        &mut self[0][0]
    }

    /// Replace a column in the array.
    #[inline]
    fn replace_col(&mut self, c: usize, src: Self::Column) -> Self::Column {
        mem::replace(&mut self[c], src)
    }

    /// Get a row from this matrix by-value.
    fn row(&self, r: usize) -> Self::Row;

    /// Swap two rows of this array.
    fn swap_rows(&mut self, a: usize, b: usize);
    /// Swap two columns of this array.
    fn swap_columns(&mut self, a: usize, b: usize);
    /// Swap the values at index `a` and `b`
    fn swap_elements(&mut self, a: (usize, usize), b: (usize, usize));

    /// Create a matrix with all of the elements set to zero.
    fn zero() -> Self;

    /// Multiply the matrix by another matrix,
    fn mul_m(&self, other: &Self::Transpose) -> Self;

    /// Multiply the matrix by a column vector.
    fn mul_v(&self, column: Self::Column) -> Self::Column;

    /// Multiply this matrix by a scalar, returning the new matrix.
    fn mul_s(&self, scalar: Self::Element) -> Self;
    /// Divide this matrix by a scalar, returning the new matrix.
    fn div_s(&self, scalar: Self::Element) -> Self;

    /// Multiply this matrix by a scalar, in-place.
    fn mul_self_s(&mut self, scalar: Self::Element);
    /// Divide this matrix by a scalar, in-place.
    fn div_self_s(&mut self, scalar: Self::Element);

    /// Transpose this matrix, returning a new matrix.
    fn transpose(&self) -> Self::Transpose;
}

/// A column-major major matrix where the rows and column vectors are of the same dimensions.
pub trait SquareMatrix where
    Self: Matrix<
        // FIXME: Can be cleaned up once equality constraints in where clauses are implemented
        Column = <Self as SquareMatrix>::ColumnRow,
        Row = <Self as SquareMatrix>::ColumnRow,
        Transpose = Self,
    >,
{
    // FIXME: Will not be needed once equality constraints in where clauses are implemented
    /// The row/column vector of the matrix.
    ///
    /// This is used to constrain the column and rows to be of the same type in lieu of equality
    /// constraints being implemented for `where` clauses. Once those are added, this type will
    /// likely go away.
    type ColumnRow: Array<Element = Self::Element>;

    /// Create a new diagonal matrix using the supplied value.
    fn from_value(value: Self::Element) -> Self;
    /// Create a matrix from a non-uniform scale
    fn from_diagonal(diagonal: Self::Column) -> Self;

    /// Create a matrix where the each element of the diagonal is equal to one.
    fn one() -> Self;

    /// Add this matrix with another matrix, returning the new metrix.
    fn add_m(&self, m: &Self) -> Self;
    /// Subtract another matrix from this matrix, returning the new matrix.
    fn sub_m(&self, m: &Self) -> Self;

    /// Add this matrix with another matrix, in-place.
    fn add_self_m(&mut self, m: &Self);
    /// Subtract another matrix from this matrix, in-place.
    fn sub_self_m(&mut self, m: &Self);

    /// Multiply this matrix by another matrix, in-place.
    fn mul_self_m(&mut self, m: &Self) { *self = self.mul_m(m); }

    /// Transpose this matrix in-place.
    fn transpose_self(&mut self);
    /// Take the determinant of this matrix.
    fn determinant(&self) -> Self::Element;

    /// Return a vector containing the diagonal of this matrix.
    fn diagonal(&self) -> Self::Column;

    /// Return the trace of this matrix. That is, the sum of the diagonal.
    #[inline]
    fn trace(&self) -> Self::Element { self.diagonal().sum() }

    /// Invert this matrix, returning a new matrix. `m.mul_m(m.invert())` is
    /// the identity matrix. Returns `None` if this matrix is not invertible
    /// (has a determinant of zero).
    #[must_use]
    fn invert(&self) -> Option<Self>;

    /// Invert this matrix in-place.
    #[inline]
    fn invert_self(&mut self) {
        *self = self.invert().expect("Attempted to invert a matrix with zero determinant.");
    }

    /// Test if this matrix is invertible.
    #[inline]
    fn is_invertible(&self) -> bool { ulps_ne!(self.determinant(), Self::Element::zero()) }

    /// Test if this matrix is the identity matrix. That is, it is diagonal
    /// and every element in the diagonal is one.
    #[inline]
    fn is_one(&self) -> bool { ulps_eq!(self, &Self::one()) }

    /// Test if this is a diagonal matrix. That is, every element outside of
    /// the diagonal is 0.
    fn is_diagonal(&self) -> bool;

    /// Test if this matrix is symmetric. That is, it is equal to its
    /// transpose.
    fn is_symmetric(&self) -> bool;
}

impl<S: BaseFloat> Matrix for Matrix2<S> {
    type Element = S;
    type Column = Vector2<S>;
    type Row = Vector2<S>;
    type Transpose = Matrix2<S>;

    #[inline]
    fn row(&self, r: usize) -> Vector2<S> {
        Vector2::new(self[0][r],
                     self[1][r])
    }

    #[inline]
    fn swap_rows(&mut self, a: usize, b: usize) {
        self[0].swap_elements(a, b);
        self[1].swap_elements(a, b);
    }

    #[inline]
    fn swap_columns(&mut self, a: usize, b: usize) {
        unsafe { ptr::swap(&mut self[a], &mut self[b]) };
    }

    #[inline]
    fn swap_elements(&mut self, a: (usize, usize), b: (usize, usize)) {
        let (ac, ar) = a;
        let (bc, br) = b;
        unsafe { ptr::swap(&mut self[ac][ar], &mut self[bc][br]) };
    }

    #[inline]
    fn zero() -> Matrix2<S> {
        Matrix2::new(S::zero(), S::zero(),
                     S::zero(), S::zero())
    }

    #[inline]
    fn mul_m(&self, other: &Matrix2<S>) -> Matrix2<S> { self * other }

    #[inline]
    fn mul_v(&self, v: Vector2<S>) -> Vector2<S> { self * v }

    #[inline]
    fn mul_s(&self, s: S) -> Matrix2<S> { self * s }

    #[inline]
    fn div_s(&self, s: S) -> Matrix2<S> { self / s }

    #[inline]
    fn mul_self_s(&mut self, s: S) {
        self[0].mul_self_s(s);
        self[1].mul_self_s(s);
    }

    #[inline]
    fn div_self_s(&mut self, s: S) {
        self[0].div_self_s(s);
        self[1].div_self_s(s);
    }

    fn transpose(&self) -> Matrix2<S> {
        Matrix2::new(self[0][0], self[1][0],
                     self[0][1], self[1][1])
    }
}

impl<S: BaseFloat> SquareMatrix for Matrix2<S> {
    type ColumnRow = Vector2<S>;

    #[inline]
    fn from_value(value: S) -> Matrix2<S> {
        Matrix2::new(value, S::zero(),
                     S::zero(), value)
    }

    #[inline]
    fn from_diagonal(value: Vector2<S>) -> Matrix2<S> {
        Matrix2::new(value.x, S::zero(),
                     S::zero(), value.y)
    }

    #[inline]
    fn one() -> Matrix2<S> {
        Matrix2::new(S::one(), S::zero(),
                     S::zero(), S::one())
    }

    #[inline]
    fn add_m(&self, m: &Matrix2<S>) -> Matrix2<S> { self + m }

    #[inline]
    fn sub_m(&self, m: &Matrix2<S>) -> Matrix2<S> { self - m }

    #[inline]
    fn add_self_m(&mut self, m: &Matrix2<S>) {
        self[0].add_self_v(m[0]);
        self[1].add_self_v(m[1]);
    }

    #[inline]
    fn sub_self_m(&mut self, m: &Matrix2<S>) {
        self[0].sub_self_v(m[0]);
        self[1].sub_self_v(m[1]);
    }

    #[inline]
    fn transpose_self(&mut self) {
        self.swap_elements((0, 1), (1, 0));
    }

    #[inline]
    fn determinant(&self) -> S {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    #[inline]
    fn diagonal(&self) -> Vector2<S> {
        Vector2::new(self[0][0],
                     self[1][1])
    }

    #[inline]
    fn invert(&self) -> Option<Matrix2<S>> {
        let det = self.determinant();
        if ulps_eq!(det, S::zero()) {
            None
        } else {
            Some(Matrix2::new( self[1][1] / det, -self[0][1] / det,
                              -self[1][0] / det,  self[0][0] / det))
        }
    }

    #[inline]
    fn is_diagonal(&self) -> bool {
        ulps_eq!(self[0][1], S::zero()) &&
        ulps_eq!(self[1][0], S::zero())
    }


    #[inline]
    fn is_symmetric(&self) -> bool {
        ulps_eq!(self[0][1], self[1][0]) &&
        ulps_eq!(self[1][0], self[0][1])
    }
}

impl<S: BaseFloat> Matrix for Matrix3<S> {
    type Element = S;
    type Column = Vector3<S>;
    type Row = Vector3<S>;
    type Transpose = Matrix3<S>;

    #[inline]
    fn row(&self, r: usize) -> Vector3<S> {
        Vector3::new(self[0][r],
                     self[1][r],
                     self[2][r])
    }

    #[inline]
    fn swap_rows(&mut self, a: usize, b: usize) {
        self[0].swap_elements(a, b);
        self[1].swap_elements(a, b);
        self[2].swap_elements(a, b);
    }

    #[inline]
    fn swap_columns(&mut self, a: usize, b: usize) {
        unsafe { ptr::swap(&mut self[a], &mut self[b]) };
    }

    #[inline]
    fn swap_elements(&mut self, a: (usize, usize), b: (usize, usize)) {
        let (ac, ar) = a;
        let (bc, br) = b;
        unsafe { ptr::swap(&mut self[ac][ar], &mut self[bc][br]) };
    }

    #[inline]
    fn zero() -> Matrix3<S> {
        Matrix3::new(S::zero(), S::zero(), S::zero(),
                     S::zero(), S::zero(), S::zero(),
                     S::zero(), S::zero(), S::zero())
    }

    #[inline]
    fn mul_m(&self, other: &Matrix3<S>) -> Matrix3<S> { self * other }

    #[inline]
    fn mul_v(&self, v: Vector3<S>) -> Vector3<S> { self * v}

    #[inline]
    fn mul_s(&self, s: S) -> Matrix3<S> { self * s }

    #[inline]
    fn div_s(&self, s: S) -> Matrix3<S> { self / s }

    #[inline]
    fn mul_self_s(&mut self, s: S) {
        self[0].mul_self_s(s);
        self[1].mul_self_s(s);
        self[2].mul_self_s(s);
    }

    #[inline]
    fn div_self_s(&mut self, s: S) {
        self[0].div_self_s(s);
        self[1].div_self_s(s);
        self[2].div_self_s(s);
    }

    fn transpose(&self) -> Matrix3<S> {
        Matrix3::new(self[0][0], self[1][0], self[2][0],
                     self[0][1], self[1][1], self[2][1],
                     self[0][2], self[1][2], self[2][2])
    }
}

impl<S: BaseFloat> SquareMatrix for Matrix3<S> {
    type ColumnRow = Vector3<S>;

    #[inline]
    fn from_value(value: S) -> Matrix3<S> {
        Matrix3::new(value, S::zero(), S::zero(),
                     S::zero(), value, S::zero(),
                     S::zero(), S::zero(), value)
    }

    #[inline]
    fn from_diagonal(value: Vector3<S>) -> Matrix3<S> {
        Matrix3::new(value.x, S::zero(), S::zero(),
                     S::zero(), value.y, S::zero(),
                     S::zero(), S::zero(), value.z)
    }

    #[inline]
    fn one() -> Matrix3<S> {
        Matrix3::new(S::one(), S::zero(), S::zero(),
                     S::zero(), S::one(), S::zero(),
                     S::zero(), S::zero(), S::one())
    }

    #[inline]
    fn add_m(&self, m: &Matrix3<S>) -> Matrix3<S> { self + m }

    #[inline]
    fn sub_m(&self, m: &Matrix3<S>) -> Matrix3<S> { self - m }

    #[inline]
    fn add_self_m(&mut self, m: &Matrix3<S>) {
        self[0].add_self_v(m[0]);
        self[1].add_self_v(m[1]);
        self[2].add_self_v(m[2]);
    }

    #[inline]
    fn sub_self_m(&mut self, m: &Matrix3<S>) {
        self[0].sub_self_v(m[0]);
        self[1].sub_self_v(m[1]);
        self[2].sub_self_v(m[2]);
    }

    #[inline]
    fn transpose_self(&mut self) {
        self.swap_elements((0, 1), (1, 0));
        self.swap_elements((0, 2), (2, 0));
        self.swap_elements((1, 2), (2, 1));
    }

    fn determinant(&self) -> S {
        self[0][0] * (self[1][1] * self[2][2] - self[2][1] * self[1][2]) -
        self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2]) +
        self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2])
    }

    #[inline]
    fn diagonal(&self) -> Vector3<S> {
        Vector3::new(self[0][0],
                     self[1][1],
                     self[2][2])
    }

    fn invert(&self) -> Option<Matrix3<S>> {
        let det = self.determinant();
        if ulps_eq!(det, S::zero()) { None } else {
            Some(Matrix3::from_cols(self[1].cross(self[2]) / det,
                                    self[2].cross(self[0]) / det,
                                    self[0].cross(self[1]) / det).transpose())
        }
    }

    fn is_diagonal(&self) -> bool {
        ulps_eq!(self[0][1], S::zero()) &&
        ulps_eq!(self[0][2], S::zero()) &&

        ulps_eq!(self[1][0], S::zero()) &&
        ulps_eq!(self[1][2], S::zero()) &&

        ulps_eq!(self[2][0], S::zero()) &&
        ulps_eq!(self[2][1], S::zero())
    }

    fn is_symmetric(&self) -> bool {
        ulps_eq!(self[0][1], self[1][0]) &&
        ulps_eq!(self[0][2], self[2][0]) &&

        ulps_eq!(self[1][0], self[0][1]) &&
        ulps_eq!(self[1][2], self[2][1]) &&

        ulps_eq!(self[2][0], self[0][2]) &&
        ulps_eq!(self[2][1], self[1][2])
    }
}

impl<S: BaseFloat> Matrix for Matrix4<S> {
    type Element = S;
    type Column = Vector4<S>;
    type Row = Vector4<S>;
    type Transpose = Matrix4<S>;

    #[inline]
    fn row(&self, r: usize) -> Vector4<S> {
        Vector4::new(self[0][r],
                     self[1][r],
                     self[2][r],
                     self[3][r])
    }

    #[inline]
    fn swap_rows(&mut self, a: usize, b: usize) {
        self[0].swap_elements(a, b);
        self[1].swap_elements(a, b);
        self[2].swap_elements(a, b);
        self[3].swap_elements(a, b);
    }

    #[inline]
    fn swap_columns(&mut self, a: usize, b: usize) {
        unsafe { ptr::swap(&mut self[a], &mut self[b]) };
    }

    #[inline]
    fn swap_elements(&mut self, a: (usize, usize), b: (usize, usize)) {
        let (ac, ar) = a;
        let (bc, br) = b;
        unsafe { ptr::swap(&mut self[ac][ar], &mut self[bc][br]) };
    }

    #[inline]
    fn zero() -> Matrix4<S> {
        Matrix4::new(S::zero(), S::zero(), S::zero(), S::zero(),
                     S::zero(), S::zero(), S::zero(), S::zero(),
                     S::zero(), S::zero(), S::zero(), S::zero(),
                     S::zero(), S::zero(), S::zero(), S::zero())
    }

    #[inline]
    fn mul_m(&self, other: &Matrix4<S>) -> Matrix4<S> { self * other }

    #[inline]
    fn mul_v(&self, v: Vector4<S>) -> Vector4<S> { self * v }

    #[inline]
    fn mul_s(&self, s: S) -> Matrix4<S> { self * s }

    #[inline]
    fn div_s(&self, s: S) -> Matrix4<S> { self / s }

    #[inline]
    fn mul_self_s(&mut self, s: S) {
        self[0].mul_self_s(s);
        self[1].mul_self_s(s);
        self[2].mul_self_s(s);
        self[3].mul_self_s(s);
    }

    #[inline]
    fn div_self_s(&mut self, s: S) {
        self[0].div_self_s(s);
        self[1].div_self_s(s);
        self[2].div_self_s(s);
        self[3].div_self_s(s);
    }

    fn transpose(&self) -> Matrix4<S> {
        Matrix4::new(self[0][0], self[1][0], self[2][0], self[3][0],
                     self[0][1], self[1][1], self[2][1], self[3][1],
                     self[0][2], self[1][2], self[2][2], self[3][2],
                     self[0][3], self[1][3], self[2][3], self[3][3])
    }
}

impl<S: BaseFloat> SquareMatrix for Matrix4<S> {
    type ColumnRow = Vector4<S>;

    #[inline]
    fn from_value(value: S) -> Matrix4<S> {
        Matrix4::new(value, S::zero(), S::zero(), S::zero(),
                     S::zero(), value, S::zero(), S::zero(),
                     S::zero(), S::zero(), value, S::zero(),
                     S::zero(), S::zero(), S::zero(), value)
    }

    #[inline]
    fn from_diagonal(value: Vector4<S>) -> Matrix4<S> {
        Matrix4::new(value.x, S::zero(), S::zero(), S::zero(),
                     S::zero(), value.y, S::zero(), S::zero(),
                     S::zero(), S::zero(), value.z, S::zero(),
                     S::zero(), S::zero(), S::zero(), value.w)
    }

    #[inline]
    fn one() -> Matrix4<S> {
        Matrix4::new(S::one(), S::zero(), S::zero(), S::zero(),
                     S::zero(), S::one(), S::zero(), S::zero(),
                     S::zero(), S::zero(), S::one(), S::zero(),
                     S::zero(), S::zero(), S::zero(), S::one())
    }

    #[inline]
    fn add_m(&self, m: &Matrix4<S>) -> Matrix4<S> { self + m }

    #[inline]
    fn sub_m(&self, m: &Matrix4<S>) -> Matrix4<S> { self - m }

    #[inline]
    fn add_self_m(&mut self, m: &Matrix4<S>) {
        self[0].add_self_v(m[0]);
        self[1].add_self_v(m[1]);
        self[2].add_self_v(m[2]);
        self[3].add_self_v(m[3]);
    }

    #[inline]
    fn sub_self_m(&mut self, m: &Matrix4<S>) {
        self[0].sub_self_v(m[0]);
        self[1].sub_self_v(m[1]);
        self[2].sub_self_v(m[2]);
        self[3].sub_self_v(m[3]);
    }

    fn transpose_self(&mut self) {
        self.swap_elements((0, 1), (1, 0));
        self.swap_elements((0, 2), (2, 0));
        self.swap_elements((0, 3), (3, 0));
        self.swap_elements((1, 2), (2, 1));
        self.swap_elements((1, 3), (3, 1));
        self.swap_elements((2, 3), (3, 2));
    }

    fn determinant(&self) -> S {
        let m0 = Matrix3::new(self[1][1], self[2][1], self[3][1],
                              self[1][2], self[2][2], self[3][2],
                              self[1][3], self[2][3], self[3][3]);
        let m1 = Matrix3::new(self[0][1], self[2][1], self[3][1],
                              self[0][2], self[2][2], self[3][2],
                              self[0][3], self[2][3], self[3][3]);
        let m2 = Matrix3::new(self[0][1], self[1][1], self[3][1],
                              self[0][2], self[1][2], self[3][2],
                              self[0][3], self[1][3], self[3][3]);
        let m3 = Matrix3::new(self[0][1], self[1][1], self[2][1],
                              self[0][2], self[1][2], self[2][2],
                              self[0][3], self[1][3], self[2][3]);

        self[0][0] * m0.determinant() -
        self[1][0] * m1.determinant() +
        self[2][0] * m2.determinant() -
        self[3][0] * m3.determinant()
    }

    #[inline]
    fn diagonal(&self) -> Vector4<S> {
        Vector4::new(self[0][0],
                     self[1][1],
                     self[2][2],
                     self[3][3])
    }

    fn invert(&self) -> Option<Matrix4<S>> {
        let det = self.determinant();
        if ulps_eq!(det, S::zero()) { None } else {
            let inv_det = S::one() / det;
            let t = self.transpose();
            let cf = |i, j| {
                let mat = match i {
                    0 => Matrix3::from_cols(t.y.truncate_n(j), t.z.truncate_n(j), t.w.truncate_n(j)),
                    1 => Matrix3::from_cols(t.x.truncate_n(j), t.z.truncate_n(j), t.w.truncate_n(j)),
                    2 => Matrix3::from_cols(t.x.truncate_n(j), t.y.truncate_n(j), t.w.truncate_n(j)),
                    3 => Matrix3::from_cols(t.x.truncate_n(j), t.y.truncate_n(j), t.z.truncate_n(j)),
                    _ => panic!("out of range"),
                };
                let sign = if (i + j) & 1 == 1 { -S::one() } else { S::one() };
                mat.determinant() * sign * inv_det
            };

            Some(Matrix4::new(cf(0, 0), cf(0, 1), cf(0, 2), cf(0, 3),
                              cf(1, 0), cf(1, 1), cf(1, 2), cf(1, 3),
                              cf(2, 0), cf(2, 1), cf(2, 2), cf(2, 3),
                              cf(3, 0), cf(3, 1), cf(3, 2), cf(3, 3)))
        }
    }

    fn is_diagonal(&self) -> bool {
        ulps_eq!(self[0][1], S::zero()) &&
        ulps_eq!(self[0][2], S::zero()) &&
        ulps_eq!(self[0][3], S::zero()) &&

        ulps_eq!(self[1][0], S::zero()) &&
        ulps_eq!(self[1][2], S::zero()) &&
        ulps_eq!(self[1][3], S::zero()) &&

        ulps_eq!(self[2][0], S::zero()) &&
        ulps_eq!(self[2][1], S::zero()) &&
        ulps_eq!(self[2][3], S::zero()) &&

        ulps_eq!(self[3][0], S::zero()) &&
        ulps_eq!(self[3][1], S::zero()) &&
        ulps_eq!(self[3][2], S::zero())
    }

    fn is_symmetric(&self) -> bool {
        ulps_eq!(self[0][1], self[1][0]) &&
        ulps_eq!(self[0][2], self[2][0]) &&
        ulps_eq!(self[0][3], self[3][0]) &&

        ulps_eq!(self[1][0], self[0][1]) &&
        ulps_eq!(self[1][2], self[2][1]) &&
        ulps_eq!(self[1][3], self[3][1]) &&

        ulps_eq!(self[2][0], self[0][2]) &&
        ulps_eq!(self[2][1], self[1][2]) &&
        ulps_eq!(self[2][3], self[3][2]) &&

        ulps_eq!(self[3][0], self[0][3]) &&
        ulps_eq!(self[3][1], self[1][3]) &&
        ulps_eq!(self[3][2], self[2][3])
    }
}

macro_rules! impl_approx_eq {
    ($MatrixN:ident { $($field:ident),+ }) => {
        impl<S: BaseFloat> ApproxEq for $MatrixN<S> {
            type Epsilon = S;

            #[inline]
            fn default_epsilon() -> S { S::default_epsilon() }

            #[inline]
            fn default_max_relative() -> S { S::default_max_relative() }

            #[inline]
            fn default_max_ulps() -> u32 { S::default_max_ulps() }

            #[inline]
            fn relative_eq(&self, other: &$MatrixN<S>, epsilon: S, max_relative: S) -> bool {
                $(relative_eq!(&self.$field, &other.$field, epsilon = epsilon, max_relative = max_relative))&&+
            }

            #[inline]
            fn ulps_eq(&self, other: &$MatrixN<S>, epsilon: S, max_ulps: u32) -> bool {
                $(ulps_eq!(&self.$field, &other.$field, epsilon = epsilon, max_ulps = max_ulps))&&+
            }
        }
    }
}

impl_approx_eq!(Matrix2 { x, y });
impl_approx_eq!(Matrix3 { x, y, z });
impl_approx_eq!(Matrix4 { x, y, z, w });

impl<S: BaseFloat> Neg for Matrix2<S> {
    type Output = Matrix2<S>;

    #[inline]
    fn neg(self) -> Matrix2<S> {
        Matrix2::from_cols(-self.x, -self.y)
    }
}

impl<S: BaseFloat> Neg for Matrix3<S> {
    type Output = Matrix3<S>;

    #[inline]
    fn neg(self) -> Matrix3<S> {
        Matrix3::from_cols(-self.x, -self.y, -self.z)
    }
}

impl<S: BaseFloat> Neg for Matrix4<S> {
    type Output = Matrix4<S>;

    #[inline]
    fn neg(self) -> Matrix4<S> {
        Matrix4::from_cols(-self.x, -self.y, -self.z, -self.w)
    }
}

macro_rules! impl_scalar_binary_operator {
    ($Binop:ident :: $binop:ident, $MatrixN:ident { $($field:ident),+ }) => {
        impl<'a, S: BaseFloat> $Binop<S> for &'a $MatrixN<S> {
            type Output = $MatrixN<S>;

            #[inline]
            fn $binop(self, s: S) -> $MatrixN<S> {
                $MatrixN { $($field: self.$field.$binop(s)),+ }
            }
        }
    }
}

impl_scalar_binary_operator!(Mul::mul, Matrix2 { x, y });
impl_scalar_binary_operator!(Mul::mul, Matrix3 { x, y, z });
impl_scalar_binary_operator!(Mul::mul, Matrix4 { x, y, z, w });
impl_scalar_binary_operator!(Div::div, Matrix2 { x, y });
impl_scalar_binary_operator!(Div::div, Matrix3 { x, y, z });
impl_scalar_binary_operator!(Div::div, Matrix4 { x, y, z, w });
impl_scalar_binary_operator!(Rem::rem, Matrix2 { x, y });
impl_scalar_binary_operator!(Rem::rem, Matrix3 { x, y, z });
impl_scalar_binary_operator!(Rem::rem, Matrix4 { x, y, z, w });

macro_rules! impl_binary_operator {
    ($Binop:ident :: $binop:ident, $MatrixN:ident { $($field:ident),+ }) => {
        impl<'a, 'b, S: BaseFloat> $Binop<&'a $MatrixN<S>> for &'b $MatrixN<S> {
            type Output = $MatrixN<S>;

            #[inline]
            fn $binop(self, other: &'a $MatrixN<S>) -> $MatrixN<S> {
                $MatrixN { $($field: self.$field.$binop(other.$field)),+ }
            }
        }
    }
}

impl_binary_operator!(Add::add, Matrix2 { x, y });
impl_binary_operator!(Add::add, Matrix3 { x, y, z });
impl_binary_operator!(Add::add, Matrix4 { x, y, z, w });
impl_binary_operator!(Sub::sub, Matrix2 { x, y });
impl_binary_operator!(Sub::sub, Matrix3 { x, y, z });
impl_binary_operator!(Sub::sub, Matrix4 { x, y, z, w });

macro_rules! impl_vector_mul_operators {
    ($MatrixN:ident, $VectorN:ident { $($row_index:expr),+ }) => {
        impl<'a, S: BaseFloat> Mul<$VectorN<S>> for &'a $MatrixN<S> {
            type Output = $VectorN<S>;

            fn mul(self, v: $VectorN<S>) -> $VectorN<S> {
                $VectorN::new($(self.row($row_index).dot(v)),+)
            }
        }

        impl<'a, 'b, S: BaseFloat> Mul<&'a $VectorN<S>> for &'b $MatrixN<S> {
            type Output = $VectorN<S>;

            fn mul(self, v: &'a $VectorN<S>) -> $VectorN<S> {
                $VectorN::new($(self.row($row_index).dot(*v)),+)
            }
        }
    }
}

impl_vector_mul_operators!(Matrix2, Vector2 { 0, 1 });
impl_vector_mul_operators!(Matrix3, Vector3 { 0, 1, 2 });
impl_vector_mul_operators!(Matrix4, Vector4 { 0, 1, 2, 3 });

impl<'a, 'b, S: BaseFloat> Mul<&'a Matrix2<S>> for &'b Matrix2<S> {
    type Output = Matrix2<S>;

    fn mul(self, other: &'a Matrix2<S>) -> Matrix2<S> {
        Matrix2::new(self.row(0).dot(other[0]), self.row(1).dot(other[0]),
                     self.row(0).dot(other[1]), self.row(1).dot(other[1]))
    }
}

impl<'a, 'b, S: BaseFloat> Mul<&'a Matrix3<S>> for &'b Matrix3<S> {
    type Output = Matrix3<S>;

    fn mul(self, other: &'a Matrix3<S>) -> Matrix3<S> {
        Matrix3::new(self.row(0).dot(other[0]),self.row(1).dot(other[0]),self.row(2).dot(other[0]),
                     self.row(0).dot(other[1]),self.row(1).dot(other[1]),self.row(2).dot(other[1]),
                     self.row(0).dot(other[2]),self.row(1).dot(other[2]),self.row(2).dot(other[2]))
    }
}

impl<'a, 'b, S: BaseFloat> Mul<&'a Matrix4<S>> for &'b Matrix4<S> {
    type Output = Matrix4<S>;

    fn mul(self, other: &'a Matrix4<S>) -> Matrix4<S> {
        // Using self.row(0).dot(other[0]) like the other matrix multiplies
        // causes the LLVM to miss identical loads and multiplies. This optimization
        // causes the code to be auto vectorized properly increasing the performance
        // around ~4 times.
        macro_rules! dot_matrix4 {
            ($A:expr, $B:expr, $I:expr, $J:expr) => {
                ($A[0][$I]) * ($B[$J][0]) +
                ($A[1][$I]) * ($B[$J][1]) +
                ($A[2][$I]) * ($B[$J][2]) +
                ($A[3][$I]) * ($B[$J][3])
            };
        };

        Matrix4::new(dot_matrix4!(self, other, 0, 0), dot_matrix4!(self, other, 1, 0), dot_matrix4!(self, other, 2, 0), dot_matrix4!(self, other, 3, 0),
                     dot_matrix4!(self, other, 0, 1), dot_matrix4!(self, other, 1, 1), dot_matrix4!(self, other, 2, 1), dot_matrix4!(self, other, 3, 1),
                     dot_matrix4!(self, other, 0, 2), dot_matrix4!(self, other, 1, 2), dot_matrix4!(self, other, 2, 2), dot_matrix4!(self, other, 3, 2),
                     dot_matrix4!(self, other, 0, 3), dot_matrix4!(self, other, 1, 3), dot_matrix4!(self, other, 2, 3), dot_matrix4!(self, other, 3, 3))

    }
}

macro_rules! index_operators {
    ($MatrixN:ident<$S:ident>, $n:expr, $Output:ty, $I:ty) => {
        impl<$S> Index<$I> for $MatrixN<$S> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&'a self, i: $I) -> &'a $Output {
                let v: &[[$S; $n]; $n] = self.as_ref();
                From::from(&v[i])
            }
        }

        impl<$S> IndexMut<$I> for $MatrixN<$S> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: $I) -> &'a mut $Output {
                let v: &mut [[$S; $n]; $n] = self.as_mut();
                From::from(&mut v[i])
            }
        }
    }
}

index_operators!(Matrix2<S>, 2, Vector2<S>, usize);
index_operators!(Matrix3<S>, 3, Vector3<S>, usize);
index_operators!(Matrix4<S>, 4, Vector4<S>, usize);
// index_operators!(Matrix2<S>, 2, [Vector2<S>], Range<usize>);
// index_operators!(Matrix3<S>, 3, [Vector3<S>], Range<usize>);
// index_operators!(Matrix4<S>, 4, [Vector4<S>], Range<usize>);
// index_operators!(Matrix2<S>, 2, [Vector2<S>], RangeTo<usize>);
// index_operators!(Matrix3<S>, 3, [Vector3<S>], RangeTo<usize>);
// index_operators!(Matrix4<S>, 4, [Vector4<S>], RangeTo<usize>);
// index_operators!(Matrix2<S>, 2, [Vector2<S>], RangeFrom<usize>);
// index_operators!(Matrix3<S>, 3, [Vector3<S>], RangeFrom<usize>);
// index_operators!(Matrix4<S>, 4, [Vector4<S>], RangeFrom<usize>);
// index_operators!(Matrix2<S>, 2, [Vector2<S>], RangeFull);
// index_operators!(Matrix3<S>, 3, [Vector3<S>], RangeFull);
// index_operators!(Matrix4<S>, 4, [Vector4<S>], RangeFull);

macro_rules! fixed_array_conversions {
    ($MatrixN:ident <$S:ident> { $($field:ident : $index:expr),+ }, $n:expr) => {
        impl<$S> Into<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn into(self) -> [[$S; $n]; $n] {
                match self { $MatrixN { $($field),+ } => [$($field.into()),+] }
            }
        }

        impl<$S> AsRef<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn as_ref(&self) -> &[[$S; $n]; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [[$S; $n]; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S: Copy> From<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn from(m: [[$S; $n]; $n]) -> $MatrixN<$S> {
                // We need to use a copy here because we can't pattern match on arrays yet
                $MatrixN { $($field: From::from(m[$index])),+ }
            }
        }

        impl<'a, $S> From<&'a [[$S; $n]; $n]> for &'a $MatrixN<$S> {
            #[inline]
            fn from(m: &'a [[$S; $n]; $n]) -> &'a $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }

        impl<'a, $S> From<&'a mut [[$S; $n]; $n]> for &'a mut $MatrixN<$S> {
            #[inline]
            fn from(m: &'a mut [[$S; $n]; $n]) -> &'a mut $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }

        // impl<$S> Into<[$S; ($n * $n)]> for $MatrixN<$S> {
        //     #[inline]
        //     fn into(self) -> [[$S; $n]; $n] {
        //         // TODO: Not sure how to implement this...
        //         unimplemented!()
        //     }
        // }

        impl<$S> AsRef<[$S; ($n * $n)]> for $MatrixN<$S> {
            #[inline]
            fn as_ref(&self) -> &[$S; ($n * $n)] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[$S; ($n * $n)]> for $MatrixN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$S; ($n * $n)] {
                unsafe { mem::transmute(self) }
            }
        }

        // impl<$S> From<[$S; ($n * $n)]> for $MatrixN<$S> {
        //     #[inline]
        //     fn from(m: [$S; ($n * $n)]) -> $MatrixN<$S> {
        //         // TODO: Not sure how to implement this...
        //         unimplemented!()
        //     }
        // }

        impl<'a, $S> From<&'a [$S; ($n * $n)]> for &'a $MatrixN<$S> {
            #[inline]
            fn from(m: &'a [$S; ($n * $n)]) -> &'a $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }

        impl<'a, $S> From<&'a mut [$S; ($n * $n)]> for &'a mut $MatrixN<$S> {
            #[inline]
            fn from(m: &'a mut [$S; ($n * $n)]) -> &'a mut $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }
    }
}

fixed_array_conversions!(Matrix2<S> { x:0, y:1 }, 2);
fixed_array_conversions!(Matrix3<S> { x:0, y:1, z:2 }, 3);
fixed_array_conversions!(Matrix4<S> { x:0, y:1, z:2, w:3 }, 4);

impl<S: BaseFloat> From<Matrix2<S>> for Matrix3<S> {
    /// Clone the elements of a 2-dimensional matrix into the top-left corner
    /// of a 3-dimensional identity matrix.
    fn from(m: Matrix2<S>) -> Matrix3<S> {
        Matrix3::new(m[0][0], m[0][1], S::zero(),
                     m[1][0], m[1][1], S::zero(),
                     S::zero(), S::zero(), S::one())
    }
}

impl<S: BaseFloat> From<Matrix2<S>> for Matrix4<S> {
    /// Clone the elements of a 2-dimensional matrix into the top-left corner
    /// of a 4-dimensional identity matrix.
    fn from(m: Matrix2<S>) -> Matrix4<S> {
        Matrix4::new(m[0][0], m[0][1], S::zero(), S::zero(),
                     m[1][0], m[1][1], S::zero(), S::zero(),
                     S::zero(), S::zero(), S::one(), S::zero(),
                     S::zero(), S::zero(), S::zero(), S::one())
    }
}

impl<S: BaseFloat> From<Matrix3<S>> for Matrix4<S> {
    /// Clone the elements of a 3-dimensional matrix into the top-left corner
    /// of a 4-dimensional identity matrix.
    fn from(m: Matrix3<S>) -> Matrix4<S> {
        Matrix4::new(m[0][0], m[0][1], m[0][2], S::zero(),
                     m[1][0], m[1][1], m[1][2], S::zero(),
                     m[2][0], m[2][1], m[2][2], S::zero(),
                     S::zero(), S::zero(), S::zero(), S::one())
    }
}

impl<S: BaseFloat> From<Matrix3<S>> for Quaternion<S> {
    /// Convert the matrix to a quaternion
    fn from(mat: Matrix3<S>) -> Quaternion<S> {
        // http://www.cs.ucr.edu/~vbz/resources/quatut.pdf
        let trace = mat.trace();
        let half: S = cast(0.5f64).unwrap();

        if trace >= S::zero() {
            let s = (S::one() + trace).sqrt();
            let w = half * s;
            let s = half / s;
            let x = (mat[1][2] - mat[2][1]) * s;
            let y = (mat[2][0] - mat[0][2]) * s;
            let z = (mat[0][1] - mat[1][0]) * s;
            Quaternion::new(w, x, y, z)
        } else if (mat[0][0] > mat[1][1]) && (mat[0][0] > mat[2][2]) {
            let s = (half + (mat[0][0] - mat[1][1] - mat[2][2])).sqrt();
            let w = half * s;
            let s = half / s;
            let x = (mat[0][1] - mat[1][0]) * s;
            let y = (mat[2][0] - mat[0][2]) * s;
            let z = (mat[1][2] - mat[2][1]) * s;
            Quaternion::new(w, x, y, z)
        } else if mat[1][1] > mat[2][2] {
            let s = (half + (mat[1][1] - mat[0][0] - mat[2][2])).sqrt();
            let w = half * s;
            let s = half / s;
            let x = (mat[0][1] - mat[1][0]) * s;
            let y = (mat[1][2] - mat[2][1]) * s;
            let z = (mat[2][0] - mat[0][2]) * s;
            Quaternion::new(w, x, y, z)
        } else {
            let s = (half + (mat[2][2] - mat[0][0] - mat[1][1])).sqrt();
            let w = half * s;
            let s = half / s;
            let x = (mat[2][0] - mat[0][2]) * s;
            let y = (mat[1][2] - mat[2][1]) * s;
            let z = (mat[0][1] - mat[1][0]) * s;
            Quaternion::new(w, x, y, z)
        }
    }
}

impl<S: BaseNum> fmt::Debug for Matrix2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{:?}, {:?}], [{:?}, {:?}]]",
                self[0][0], self[0][1],
                self[1][0], self[1][1])
    }
}

impl<S: BaseNum> fmt::Debug for Matrix3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{:?}, {:?}, {:?}], [{:?}, {:?}, {:?}], [{:?}, {:?}, {:?}]]",
                self[0][0], self[0][1], self[0][2],
                self[1][0], self[1][1], self[1][2],
                self[2][0], self[2][1], self[2][2])
    }
}

impl<S: BaseNum> fmt::Debug for Matrix4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{:?}, {:?}, {:?}, {:?}], [{:?}, {:?}, {:?}, {:?}], [{:?}, {:?}, {:?}, {:?}], [{:?}, {:?}, {:?}, {:?}]]",
                self[0][0], self[0][1], self[0][2], self[0][3],
                self[1][0], self[1][1], self[1][2], self[1][3],
                self[2][0], self[2][1], self[2][2], self[2][3],
                self[3][0], self[3][1], self[3][2], self[3][3])
    }
}

impl<S: BaseFloat + Rand> Rand for Matrix2<S> {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Matrix2<S> {
        Matrix2{ x: rng.gen(), y: rng.gen() }
    }
}

impl<S: BaseFloat + Rand> Rand for Matrix3<S> {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Matrix3<S> {
        Matrix3{ x: rng.gen(), y: rng.gen(), z: rng.gen() }
    }
}

impl<S: BaseFloat + Rand> Rand for Matrix4<S> {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Matrix4<S> {
        Matrix4{ x: rng.gen(), y: rng.gen(), z: rng.gen(), w: rng.gen() }
    }
}
