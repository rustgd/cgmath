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
use std::mem;
use std::num::cast;
use std::ops::*;

use angle::{Rad, sin, cos, sin_cos};
use approx::ApproxEq;
use array::{Array1, Array2, FixedArray};
use num::{BaseFloat, BaseNum, Zero, zero, One, one};
use point::{Point, Point3};
use quaternion::{Quaternion, ToQuaternion};
use vector::{Vector, EuclideanVector};
use vector::{Vector2, Vector3, Vector4};

/// A 2 x 2, column major matrix
#[derive_Rand]
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Matrix2<S> { pub x: Vector2<S>, pub y: Vector2<S> }

/// A 3 x 3, column major matrix
#[derive_Rand]
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Matrix3<S> { pub x: Vector3<S>, pub y: Vector3<S>, pub z: Vector3<S> }

/// A 4 x 4, column major matrix
#[derive_Rand]
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
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

impl<S: BaseFloat + 'static> Matrix2<S> {
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

        Matrix2::new(cos_theta.clone(),  sin_theta.clone(),
                     -sin_theta.clone(), cos_theta.clone())
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

impl<S: BaseFloat + 'static>
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

    /// Create a matrix from a non-uniform scale
    pub fn from_diagonal(value: &Vector3<S>) -> Matrix3<S> {
        Matrix3::new(value.x, zero(),  zero(),
                     zero(),  value.y, zero(),
                     zero(),  zero(),  value.z)
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

    /// Create a translation matrix from a Vector3
    #[inline]
    pub fn from_translation(v: &Vector3<S>) -> Matrix4<S> {
        Matrix4::new(one(),  zero(), zero(), zero(),
                     zero(), one(),  zero(), zero(),
                     zero(), zero(), one(),  zero(),
                     v.x,    v.y,    v.z,    one())
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

pub trait Matrix<S: BaseFloat, V: Clone + Vector<S>>: Array2<V, V, S>
                                                    + Neg
                                                    + Zero + One
                                                    + ApproxEq<S>
                                                    + Sized {
    /// Multiply this matrix by a scalar, returning the new matrix.
    fn mul_s(&self, s: S) -> Self;
    /// Divide this matrix by a scalar, returning the new matrix.
    fn div_s(&self, s: S) -> Self;
    /// Take the remainder of this matrix by a scalar, returning the new
    /// matrix.
    fn rem_s(&self, s: S) -> Self;

    /// Add this matrix with another matrix, returning the new metrix.
    fn add_m(&self, m: &Self) -> Self;
    /// Subtract another matrix from this matrix, returning the new matrix.
    fn sub_m(&self, m: &Self) -> Self;

    /// Multiplay a vector by this matrix, returning a new vector.
    fn mul_v(&self, v: &V) -> V;

    /// Multiply this matrix by another matrix, returning the new matrix.
    fn mul_m(&self, m: &Self) -> Self;

    /// Negate this matrix in-place (multiply by scalar -1).
    fn neg_self(&mut self);

    /// Multiply this matrix by a scalar, in-place.
    fn mul_self_s(&mut self, s: S);
    /// Divide this matrix by a scalar, in-place.
    fn div_self_s(&mut self, s: S);
    /// Take the remainder of this matrix, in-place.
    fn rem_self_s(&mut self, s: S);

    /// Add this matrix with another matrix, in-place.
    fn add_self_m(&mut self, m: &Self);
    /// Subtract another matrix from this matrix, in-place.
    fn sub_self_m(&mut self, m: &Self);

    /// Multiply this matrix by another matrix, in-place.
    #[inline]
    fn mul_self_m(&mut self, m: &Self) { *self = self.mul_m(m); }

    /// Transpose this matrix, returning a new matrix.
    fn transpose(&self) -> Self;
    /// Transpose this matrix in-place.
    fn transpose_self(&mut self);
    /// Take the determinant of this matrix.
    fn determinant(&self) -> S;

    /// Return a vector containing the diagonal of this matrix.
    fn diagonal(&self) -> V;

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
    fn is_invertible(&self) -> bool { !self.determinant().approx_eq(&zero()) }

    /// Test if this matrix is the identity matrix. That is, it is diagonal
    /// and every element in the diagonal is one.
    #[inline]
    fn is_identity(&self) -> bool { self.approx_eq(&one()) }

    /// Test if this is a diagonal matrix. That is, every element outside of
    /// the diagonal is 0.
    fn is_diagonal(&self) -> bool;

    /// Test if this matrix is symmetric. That is, it is equal to its
    /// transpose.
    fn is_symmetric(&self) -> bool;
}

impl<S: BaseFloat + 'static> Add for Matrix2<S> {
    type Output = Matrix2<S>;

    #[inline]
    fn add(self, other: Matrix2<S>) -> Matrix2<S> { self.add_m(&other) }
}

impl<S: BaseFloat + 'static> Add for Matrix3<S> {
    type Output = Matrix3<S>;

    #[inline]
    fn add(self, other: Matrix3<S>) -> Matrix3<S> { self.add_m(&other) }
}

impl<S: BaseFloat + 'static> Add for Matrix4<S> {
    type Output = Matrix4<S>;

    #[inline]
    fn add(self, other: Matrix4<S>) -> Matrix4<S> { self.add_m(&other) }
}

impl<S: BaseFloat + 'static> Sub for Matrix2<S> {
    type Output = Matrix2<S>;

    #[inline]
    fn sub(self, other: Matrix2<S>) -> Matrix2<S> { self.sub_m(&other) }
}

impl<S: BaseFloat + 'static> Sub for Matrix3<S> {
    type Output = Matrix3<S>;

    #[inline]
    fn sub(self, other: Matrix3<S>) -> Matrix3<S> { self.sub_m(&other) }
}

impl<S: BaseFloat + 'static> Sub for Matrix4<S> {
    type Output = Matrix4<S>;

    #[inline]
    fn sub(self, other: Matrix4<S>) -> Matrix4<S> { self.sub_m(&other) }
}

impl<S: BaseFloat> Neg for Matrix2<S> {
    type Output = Matrix2<S>;

    #[inline]
    fn neg(self) -> Matrix2<S> { Matrix2::from_cols(self[0].neg(), self[1].neg()) }
}

impl<S: BaseFloat> Neg for Matrix3<S> {
    type Output = Matrix3<S>;

    #[inline]
    fn neg(self) -> Matrix3<S> { Matrix3::from_cols(self[0].neg(), self[1].neg(), self[2].neg()) }
}

impl<S: BaseFloat> Neg for Matrix4<S> {
    type Output = Matrix4<S>;

    #[inline]
    fn neg(self) -> Matrix4<S> { Matrix4::from_cols(self[0].neg(), self[1].neg(), self[2].neg(), self[3].neg()) }
}

impl<S: BaseFloat> Zero for Matrix2<S> {
    #[inline]
    fn zero() -> Matrix2<S> { Matrix2::zero() }
    #[inline]
    fn is_zero(&self) -> bool{ *self == zero() }
}

impl<S: BaseFloat> Zero for Matrix3<S> {
    #[inline]
    fn zero() -> Matrix3<S> { Matrix3::zero() }
    #[inline]
    fn is_zero(&self) -> bool{ *self == zero() }
}

impl<S: BaseFloat> Zero for Matrix4<S> {
    #[inline]
    fn zero() -> Matrix4<S> { Matrix4::zero() }
    #[inline]
    fn is_zero(&self) -> bool{ *self == zero() }
}

impl<S: BaseFloat + 'static> Mul for Matrix2<S> {
    type Output = Matrix2<S>;

    #[inline]
    fn mul(self, other: Matrix2<S>) -> Matrix2<S> { self.mul_m(&other) }
}

impl<S: BaseFloat + 'static> Mul for Matrix3<S> {
    type Output = Matrix3<S>;

    #[inline]
    fn mul(self, other: Matrix3<S>) -> Matrix3<S> { self.mul_m(&other) }
}

impl<S: BaseFloat + 'static> Mul for Matrix4<S> {
    type Output = Matrix4<S>;

    #[inline]
    fn mul(self, other: Matrix4<S>) -> Matrix4<S> { self.mul_m(&other) }
}

impl<S: BaseFloat> One for Matrix2<S> {
    #[inline]
    fn one() -> Matrix2<S> { Matrix2::identity() }
}
impl<S: BaseFloat> One for Matrix3<S> {
    #[inline]
    fn one() -> Matrix3<S> { Matrix3::identity() }
}
impl<S: BaseFloat> One for Matrix4<S> {
    #[inline] fn one() -> Matrix4<S> { Matrix4::identity() }
}

impl<S> FixedArray<[[S; 2]; 2]> for Matrix2<S> {
    #[inline]
    fn into_fixed(self) -> [[S; 2]; 2] {
        match self {
            Matrix2 { x, y } => [
                x.into_fixed(),
                y.into_fixed(),
            ],
        }
    }

    #[inline]
    fn as_fixed<'a>(&'a self) -> &'a [[S; 2]; 2] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn as_mut_fixed<'a>(&'a mut self) -> &'a mut [[S; 2]; 2] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn from_fixed(_v: [[S; 2]; 2]) -> Matrix2<S> {
        // match v {
        //     [x, y] => Matrix2 {
        //         x: FixedArray::from_fixed(x),
        //         y: FixedArray::from_fixed(y),
        //     },
        // }
        panic!("Unimplemented, pending a fix for rust-lang/rust#16418");
    }

    #[inline]
    fn from_fixed_ref<'a>(v: &'a [[S; 2]; 2]) -> &'a Matrix2<S> {
        unsafe { mem::transmute(v) }
    }

    #[inline]
    fn from_fixed_mut<'a>(v: &'a mut [[S; 2]; 2]) -> &'a mut Matrix2<S> {
        unsafe { mem::transmute(v) }
    }
}

impl<S> Index<usize> for Matrix2<S> {
    type Output =  Vector2<S>;

    #[inline]
    fn index<'a>(&'a self, i: &usize) -> &'a Vector2<S> {
        FixedArray::from_fixed_ref(&self.as_fixed()[*i])
    }
}

impl<S> IndexMut<usize> for Matrix2<S> {
    #[inline]
    fn index_mut<'a>(&'a mut self, i: &usize) -> &'a mut Vector2<S> {
        FixedArray::from_fixed_mut(&mut self.as_mut_fixed()[*i])
    }
}

impl<S: Copy + 'static> Array2<Vector2<S>, Vector2<S>, S> for Matrix2<S> {
    #[inline]
    fn row(&self, r: usize) -> Vector2<S> {
        Vector2::new(self[0][r],
                     self[1][r])
    }

    #[inline]
    fn swap_rows(&mut self, a: usize, b: usize) {
        (&mut self[0]).swap_elems(a, b);
        (&mut self[1]).swap_elems(a, b);
    }

    #[inline]
    fn map<F>(&mut self, mut op: F) -> Matrix2<S> where F: FnMut(&Vector2<S>) -> Vector2<S> {
        self.x = op(&self.x);
        self.y = op(&self.y);
        *self
    }
}

impl<S> FixedArray<[[S; 3]; 3]> for Matrix3<S> {
    #[inline]
    fn into_fixed(self) -> [[S; 3]; 3] {
        match self {
            Matrix3 { x, y, z } => [
                x.into_fixed(),
                y.into_fixed(),
                z.into_fixed(),
            ],
        }
    }

    #[inline]
    fn as_fixed<'a>(&'a self) -> &'a [[S; 3]; 3] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn as_mut_fixed<'a>(&'a mut self) -> &'a mut [[S; 3]; 3] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn from_fixed(_v: [[S; 3]; 3]) -> Matrix3<S> {
        // match v {
        //     [x, y, z] => Matrix3 {
        //         x: FixedArray::from_fixed(x),
        //         y: FixedArray::from_fixed(y),
        //         z: FixedArray::from_fixed(z),
        //     },
        // }
        panic!("Unimplemented, pending a fix for rust-lang/rust#16418")
    }

    #[inline]
    fn from_fixed_ref<'a>(v: &'a [[S; 3]; 3]) -> &'a Matrix3<S> {
        unsafe { mem::transmute(v) }
    }

    #[inline]
    fn from_fixed_mut<'a>(v: &'a mut [[S; 3]; 3]) -> &'a mut Matrix3<S> {
        unsafe { mem::transmute(v) }
    }
}

impl<S> Index<usize> for Matrix3<S> {
    type Output = Vector3<S>;

    #[inline]
    fn index<'a>(&'a self, i: &usize) -> &'a Vector3<S> {
        FixedArray::from_fixed_ref(&self.as_fixed()[*i])
    }
}

impl<S> IndexMut<usize> for Matrix3<S> {
    #[inline]
    fn index_mut<'a>(&'a mut self, i: &usize) -> &'a mut Vector3<S> {
        FixedArray::from_fixed_mut(&mut self.as_mut_fixed()[*i])
    }
}

impl<S: Copy + 'static> Array2<Vector3<S>, Vector3<S>, S> for Matrix3<S> {
    #[inline]
    fn row(&self, r: usize) -> Vector3<S> {
        Vector3::new(self[0][r],
                     self[1][r],
                     self[2][r])
    }

    #[inline]
    fn swap_rows(&mut self, a: usize, b: usize) {
        (&mut self[0]).swap_elems(a, b);
        (&mut self[1]).swap_elems(a, b);
        (&mut self[2]).swap_elems(a, b);
    }

    #[inline]
    fn map<F>(&mut self, mut op: F) -> Matrix3<S> where F: FnMut(&Vector3<S>) -> Vector3<S> {
        self.x = op(&self.x);
        self.y = op(&self.y);
        self.z = op(&self.z);
        *self
    }
}

impl<S> FixedArray<[[S; 4]; 4]> for Matrix4<S> {
    #[inline]
    fn into_fixed(self) -> [[S; 4]; 4] {
        match self {
            Matrix4 { x, y, z, w } => [
                x.into_fixed(),
                y.into_fixed(),
                z.into_fixed(),
                w.into_fixed(),
            ],
        }
    }

    #[inline]
    fn as_fixed<'a>(&'a self) -> &'a [[S; 4]; 4] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn as_mut_fixed<'a>(&'a mut self) -> &'a mut [[S; 4]; 4] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn from_fixed(_v: [[S; 4]; 4]) -> Matrix4<S> {
        // match v {
        //     [x, y, z, w] => Matrix4 {
        //         x: FixedArray::from_fixed(x),
        //         y: FixedArray::from_fixed(y),
        //         z: FixedArray::from_fixed(z),
        //         w: FixedArray::from_fixed(w),
        //     },
        // }
        panic!("Unimplemented, pending a fix for rust-lang/rust#16418")
    }

    #[inline]
    fn from_fixed_ref<'a>(v: &'a [[S; 4]; 4]) -> &'a Matrix4<S> {
        unsafe { mem::transmute(v) }
    }

    #[inline]
    fn from_fixed_mut<'a>(v: &'a mut [[S; 4]; 4]) -> &'a mut Matrix4<S> {
        unsafe { mem::transmute(v) }
    }
}

impl<S> Index<usize> for Matrix4<S> {
    type Output = Vector4<S>;

    #[inline]
    fn index<'a>(&'a self, i: &usize) -> &'a Vector4<S> {
        FixedArray::from_fixed_ref(&self.as_fixed()[*i])
    }
}

impl<S> IndexMut<usize> for Matrix4<S> {
    #[inline]
    fn index_mut<'a>(&'a mut self, i: &usize) -> &'a mut Vector4<S> {
        FixedArray::from_fixed_mut(&mut self.as_mut_fixed()[*i])
    }
}

impl<S: Copy + 'static> Array2<Vector4<S>, Vector4<S>, S> for Matrix4<S> {
    #[inline]
    fn row(&self, r: usize) -> Vector4<S> {
        Vector4::new(self[0][r],
                     self[1][r],
                     self[2][r],
                     self[3][r])
    }

    #[inline]
    fn swap_rows(&mut self, a: usize, b: usize) {
        (&mut self[0]).swap_elems(a, b);
        (&mut self[1]).swap_elems(a, b);
        (&mut self[2]).swap_elems(a, b);
        (&mut self[3]).swap_elems(a, b);
    }

    #[inline]
    fn map<F>(&mut self, mut op: F) -> Matrix4<S> where F: FnMut(&Vector4<S>) -> Vector4<S> {
        self.x = op(&self.x);
        self.y = op(&self.y);
        self.z = op(&self.z);
        self.w = op(&self.w);
        *self
    }
}

impl<S: BaseFloat + 'static> Matrix<S, Vector2<S>> for Matrix2<S> {
    #[inline]
    fn mul_s(&self, s: S) -> Matrix2<S> {
        Matrix2::from_cols(self[0].mul_s(s),
                           self[1].mul_s(s))
    }

    #[inline]
    fn div_s(&self, s: S) -> Matrix2<S> {
        Matrix2::from_cols(self[0].div_s(s),
                           self[1].div_s(s))
    }

    #[inline]
    fn rem_s(&self, s: S) -> Matrix2<S> {
        Matrix2::from_cols(self[0].rem_s(s),
                           self[1].rem_s(s))
    }

    #[inline]
    fn add_m(&self, m: &Matrix2<S>) -> Matrix2<S> {
        Matrix2::from_cols(self[0].add_v(&m[0]),
                           self[1].add_v(&m[1]))
    }

    #[inline]
    fn sub_m(&self, m: &Matrix2<S>) -> Matrix2<S> {
        Matrix2::from_cols(self[0].sub_v(&m[0]),
                           self[1].sub_v(&m[1]))
    }

    #[inline]
    fn mul_v(&self, v: &Vector2<S>) -> Vector2<S> {
        Vector2::new(self.row(0).dot(v),
                     self.row(1).dot(v))
    }

    fn mul_m(&self, other: &Matrix2<S>) -> Matrix2<S> {
        Matrix2::new(self.row(0).dot(&other[0]), self.row(1).dot(&other[0]),
                     self.row(0).dot(&other[1]), self.row(1).dot(&other[1]))
    }

    #[inline]
    fn neg_self(&mut self) {
        (&mut self[0]).neg_self();
        (&mut self[1]).neg_self();
    }

    #[inline]
    fn mul_self_s(&mut self, s: S) {
        (&mut self[0]).mul_self_s(s);
        (&mut self[1]).mul_self_s(s);
    }

    #[inline]
    fn div_self_s(&mut self, s: S) {
        (&mut self[0]).div_self_s(s);
        (&mut self[1]).div_self_s(s);
    }

    #[inline]
    fn rem_self_s(&mut self, s: S) {
        (&mut self[0]).rem_self_s(s);
        (&mut self[1]).rem_self_s(s);
    }

    #[inline]
    fn add_self_m(&mut self, m: &Matrix2<S>) {
        (&mut self[0]).add_self_v(&m[0]);
        (&mut self[1]).add_self_v(&m[1]);
    }

    #[inline]
    fn sub_self_m(&mut self, m: &Matrix2<S>) {
        (&mut self[0]).sub_self_v(&m[0]);
        (&mut self[1]).sub_self_v(&m[1]);
    }

    fn transpose(&self) -> Matrix2<S> {
        Matrix2::new(self[0][0], self[1][0],
                     self[0][1], self[1][1])
    }

    #[inline]
    fn transpose_self(&mut self) {
        self.swap_elems((0, 1), (1, 0));
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
        if det.approx_eq(&zero()) {
            None
        } else {
            Some(Matrix2::new( self[1][1] / det, -self[0][1] / det,
                              -self[1][0] / det,  self[0][0] / det))
        }
    }

    #[inline]
    fn is_diagonal(&self) -> bool {
        (&self[0][1]).approx_eq(&zero()) &&
        (&self[1][0]).approx_eq(&zero())
    }


    #[inline]
    fn is_symmetric(&self) -> bool {
        (&self[0][1]).approx_eq(&self[1][0]) &&
        (&self[1][0]).approx_eq(&self[0][1])
    }
}

impl<S: BaseFloat + 'static> Matrix<S, Vector3<S>> for Matrix3<S> {
    #[inline]
    fn mul_s(&self, s: S) -> Matrix3<S> {
        Matrix3::from_cols(self[0].mul_s(s),
                           self[1].mul_s(s),
                           self[2].mul_s(s))
    }

    #[inline]
    fn div_s(&self, s: S) -> Matrix3<S> {
        Matrix3::from_cols(self[0].div_s(s),
                           self[1].div_s(s),
                           self[2].div_s(s))
    }

    #[inline]
    fn rem_s(&self, s: S) -> Matrix3<S> {
        Matrix3::from_cols(self[0].rem_s(s),
                           self[1].rem_s(s),
                           self[2].rem_s(s))
    }

    #[inline]
    fn add_m(&self, m: &Matrix3<S>) -> Matrix3<S> {
        Matrix3::from_cols(self[0].add_v(&m[0]),
                           self[1].add_v(&m[1]),
                           self[2].add_v(&m[2]))
    }

    #[inline]
    fn sub_m(&self, m: &Matrix3<S>) -> Matrix3<S> {
        Matrix3::from_cols(self[0].sub_v(&m[0]),
                           self[1].sub_v(&m[1]),
                           self[2].sub_v(&m[2]))
    }

    #[inline]
    fn mul_v(&self, v: &Vector3<S>) -> Vector3<S> {
        Vector3::new(self.row(0).dot(v),
                     self.row(1).dot(v),
                     self.row(2).dot(v))
    }

    fn mul_m(&self, other: &Matrix3<S>) -> Matrix3<S> {
        Matrix3::new(self.row(0).dot(&other[0]),self.row(1).dot(&other[0]),self.row(2).dot(&other[0]),
                     self.row(0).dot(&other[1]),self.row(1).dot(&other[1]),self.row(2).dot(&other[1]),
                     self.row(0).dot(&other[2]),self.row(1).dot(&other[2]),self.row(2).dot(&other[2]))
    }

    #[inline]
    fn neg_self(&mut self) {
        (&mut self[0]).neg_self();
        (&mut self[1]).neg_self();
        (&mut self[2]).neg_self();
    }

    #[inline]
    fn mul_self_s(&mut self, s: S) {
        (&mut self[0]).mul_self_s(s);
        (&mut self[1]).mul_self_s(s);
        (&mut self[2]).mul_self_s(s);
    }

    #[inline]
    fn div_self_s(&mut self, s: S) {
        (&mut self[0]).div_self_s(s);
        (&mut self[1]).div_self_s(s);
        (&mut self[2]).div_self_s(s);
    }

    #[inline]
    fn rem_self_s(&mut self, s: S) {
        (&mut self[0]).rem_self_s(s);
        (&mut self[1]).rem_self_s(s);
        (&mut self[2]).rem_self_s(s);
    }

    #[inline]
    fn add_self_m(&mut self, m: &Matrix3<S>) {
        (&mut self[0]).add_self_v(&m[0]);
        (&mut self[1]).add_self_v(&m[1]);
        (&mut self[2]).add_self_v(&m[2]);
    }

    #[inline]
    fn sub_self_m(&mut self, m: &Matrix3<S>) {
        (&mut self[0]).sub_self_v(&m[0]);
        (&mut self[1]).sub_self_v(&m[1]);
        (&mut self[2]).sub_self_v(&m[2]);
    }

    fn transpose(&self) -> Matrix3<S> {
        Matrix3::new(self[0][0], self[1][0], self[2][0],
                     self[0][1], self[1][1], self[2][1],
                     self[0][2], self[1][2], self[2][2])
    }

    #[inline]
    fn transpose_self(&mut self) {
        self.swap_elems((0, 1), (1, 0));
        self.swap_elems((0, 2), (2, 0));
        self.swap_elems((1, 2), (2, 1));
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
        if det.approx_eq(&zero()) { None } else {
            Some(Matrix3::from_cols(self[1].cross(&self[2]).div_s(det),
                                    self[2].cross(&self[0]).div_s(det),
                                    self[0].cross(&self[1]).div_s(det)).transpose())
        }
    }

    fn is_diagonal(&self) -> bool {
        (&self[0][1]).approx_eq(&zero()) &&
        (&self[0][2]).approx_eq(&zero()) &&

        (&self[1][0]).approx_eq(&zero()) &&
        (&self[1][2]).approx_eq(&zero()) &&

        (&self[2][0]).approx_eq(&zero()) &&
        (&self[2][1]).approx_eq(&zero())
    }

    fn is_symmetric(&self) -> bool {
        (&self[0][1]).approx_eq(&self[1][0]) &&
        (&self[0][2]).approx_eq(&self[2][0]) &&

        (&self[1][0]).approx_eq(&self[0][1]) &&
        (&self[1][2]).approx_eq(&self[2][1]) &&

        (&self[2][0]).approx_eq(&self[0][2]) &&
        (&self[2][1]).approx_eq(&self[1][2])
    }
}

// Using self.row(0).dot(other[0]) like the other matrix multiplies
// causes the LLVM to miss identical loads and multiplies. This optimization
// causes the code to be auto vectorized properly increasing the performance
// around ~4 times.
macro_rules! dot_matrix4(
    ($A:expr, $B:expr, $I:expr, $J:expr) => (
        ($A[0][$I]) * ($B[$J][0]) +
        ($A[1][$I]) * ($B[$J][1]) +
        ($A[2][$I]) * ($B[$J][2]) +
        ($A[3][$I]) * ($B[$J][3])
));

impl<S: BaseFloat + 'static> Matrix<S, Vector4<S>> for Matrix4<S> {
    #[inline]
    fn mul_s(&self, s: S) -> Matrix4<S> {
        Matrix4::from_cols(self[0].mul_s(s),
                           self[1].mul_s(s),
                           self[2].mul_s(s),
                           self[3].mul_s(s))
    }

    #[inline]
    fn div_s(&self, s: S) -> Matrix4<S> {
        Matrix4::from_cols(self[0].div_s(s),
                           self[1].div_s(s),
                           self[2].div_s(s),
                           self[3].div_s(s))
    }

    #[inline]
    fn rem_s(&self, s: S) -> Matrix4<S> {
        Matrix4::from_cols(self[0].rem_s(s),
                           self[1].rem_s(s),
                           self[2].rem_s(s),
                           self[3].rem_s(s))
    }

    #[inline]
    fn add_m(&self, m: &Matrix4<S>) -> Matrix4<S> {
        Matrix4::from_cols(self[0].add_v(&m[0]),
                           self[1].add_v(&m[1]),
                           self[2].add_v(&m[2]),
                           self[3].add_v(&m[3]))
    }

    #[inline]
    fn sub_m(&self, m: &Matrix4<S>) -> Matrix4<S> {
        Matrix4::from_cols(self[0].sub_v(&m[0]),
                           self[1].sub_v(&m[1]),
                           self[2].sub_v(&m[2]),
                           self[3].sub_v(&m[3]))
    }

    #[inline]
    fn mul_v(&self, v: &Vector4<S>) -> Vector4<S> {
        Vector4::new(self.row(0).dot(v),
                     self.row(1).dot(v),
                     self.row(2).dot(v),
                     self.row(3).dot(v))
    }

    fn mul_m(&self, other: &Matrix4<S>) -> Matrix4<S> {
        Matrix4::new(dot_matrix4!(self, other, 0, 0), dot_matrix4!(self, other, 1, 0), dot_matrix4!(self, other, 2, 0), dot_matrix4!(self, other, 3, 0),
                     dot_matrix4!(self, other, 0, 1), dot_matrix4!(self, other, 1, 1), dot_matrix4!(self, other, 2, 1), dot_matrix4!(self, other, 3, 1),
                     dot_matrix4!(self, other, 0, 2), dot_matrix4!(self, other, 1, 2), dot_matrix4!(self, other, 2, 2), dot_matrix4!(self, other, 3, 2),
                     dot_matrix4!(self, other, 0, 3), dot_matrix4!(self, other, 1, 3), dot_matrix4!(self, other, 2, 3), dot_matrix4!(self, other, 3, 3))
    }

    #[inline]
    fn neg_self(&mut self) {
        (&mut self[0]).neg_self();
        (&mut self[1]).neg_self();
        (&mut self[2]).neg_self();
        (&mut self[3]).neg_self();
    }

    #[inline]
    fn mul_self_s(&mut self, s: S) {
        (&mut self[0]).mul_self_s(s);
        (&mut self[1]).mul_self_s(s);
        (&mut self[2]).mul_self_s(s);
        (&mut self[3]).mul_self_s(s);
    }

    #[inline]
    fn div_self_s(&mut self, s: S) {
        (&mut self[0]).div_self_s(s);
        (&mut self[1]).div_self_s(s);
        (&mut self[2]).div_self_s(s);
        (&mut self[3]).div_self_s(s);
    }

    #[inline]
    fn rem_self_s(&mut self, s: S) {
        (&mut self[0]).rem_self_s(s);
        (&mut self[1]).rem_self_s(s);
        (&mut self[2]).rem_self_s(s);
        (&mut self[3]).rem_self_s(s);
    }

    #[inline]
    fn add_self_m(&mut self, m: &Matrix4<S>) {
        (&mut self[0]).add_self_v(&m[0]);
        (&mut self[1]).add_self_v(&m[1]);
        (&mut self[2]).add_self_v(&m[2]);
        (&mut self[3]).add_self_v(&m[3]);
    }

    #[inline]
    fn sub_self_m(&mut self, m: &Matrix4<S>) {
        (&mut self[0]).sub_self_v(&m[0]);
        (&mut self[1]).sub_self_v(&m[1]);
        (&mut self[2]).sub_self_v(&m[2]);
        (&mut self[3]).sub_self_v(&m[3]);
    }

    fn transpose(&self) -> Matrix4<S> {
        Matrix4::new(self[0][0], self[1][0], self[2][0], self[3][0],
                     self[0][1], self[1][1], self[2][1], self[3][1],
                     self[0][2], self[1][2], self[2][2], self[3][2],
                     self[0][3], self[1][3], self[2][3], self[3][3])
    }

    fn transpose_self(&mut self) {
        self.swap_elems((0, 1), (1, 0));
        self.swap_elems((0, 2), (2, 0));
        self.swap_elems((0, 3), (3, 0));
        self.swap_elems((1, 2), (2, 1));
        self.swap_elems((1, 3), (3, 1));
        self.swap_elems((2, 3), (3, 2));
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
        if !det.approx_eq(&zero()) {
            let one: S = one();
            let inv_det = one / det;
            let t = self.transpose();
            let cf = |&: i, j| {
                let mat = match i {
                    0 => Matrix3::from_cols(t.y.truncate_n(j),
                                            t.z.truncate_n(j),
                                            t.w.truncate_n(j)),
                    1 => Matrix3::from_cols(t.x.truncate_n(j),
                                            t.z.truncate_n(j),
                                            t.w.truncate_n(j)),
                    2 => Matrix3::from_cols(t.x.truncate_n(j),
                                            t.y.truncate_n(j),
                                            t.w.truncate_n(j)),
                    3 => Matrix3::from_cols(t.x.truncate_n(j),
                                            t.y.truncate_n(j),
                                            t.z.truncate_n(j)),
                    _ => panic!("out of range")
                };
                let sign = if (i+j) & 1 == 1 {-one} else {one};
                mat.determinant() * sign * inv_det
            };

            Some(Matrix4::new(cf(0, 0), cf(0, 1), cf(0, 2), cf(0, 3),
                              cf(1, 0), cf(1, 1), cf(1, 2), cf(1, 3),
                              cf(2, 0), cf(2, 1), cf(2, 2), cf(2, 3),
                              cf(3, 0), cf(3, 1), cf(3, 2), cf(3, 3)))

        } else {
            None
        }
    }

    fn is_diagonal(&self) -> bool {
        (&self[0][1]).approx_eq(&zero()) &&
        (&self[0][2]).approx_eq(&zero()) &&
        (&self[0][3]).approx_eq(&zero()) &&

        (&self[1][0]).approx_eq(&zero()) &&
        (&self[1][2]).approx_eq(&zero()) &&
        (&self[1][3]).approx_eq(&zero()) &&

        (&self[2][0]).approx_eq(&zero()) &&
        (&self[2][1]).approx_eq(&zero()) &&
        (&self[2][3]).approx_eq(&zero()) &&

        (&self[3][0]).approx_eq(&zero()) &&
        (&self[3][1]).approx_eq(&zero()) &&
        (&self[3][2]).approx_eq(&zero())
    }

    fn is_symmetric(&self) -> bool {
        (&self[0][1]).approx_eq(&self[1][0]) &&
        (&self[0][2]).approx_eq(&self[2][0]) &&
        (&self[0][3]).approx_eq(&self[3][0]) &&

        (&self[1][0]).approx_eq(&self[0][1]) &&
        (&self[1][2]).approx_eq(&self[2][1]) &&
        (&self[1][3]).approx_eq(&self[3][1]) &&

        (&self[2][0]).approx_eq(&self[0][2]) &&
        (&self[2][1]).approx_eq(&self[1][2]) &&
        (&self[2][3]).approx_eq(&self[3][2]) &&

        (&self[3][0]).approx_eq(&self[0][3]) &&
        (&self[3][1]).approx_eq(&self[1][3]) &&
        (&self[3][2]).approx_eq(&self[2][3])
    }
}

impl<S: BaseFloat> ApproxEq<S> for Matrix2<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Matrix2<S>, epsilon: &S) -> bool {
        self[0].approx_eq_eps(&other[0], epsilon) &&
        self[1].approx_eq_eps(&other[1], epsilon)
    }
}

impl<S: BaseFloat> ApproxEq<S> for Matrix3<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Matrix3<S>, epsilon: &S) -> bool {
        self[0].approx_eq_eps(&other[0], epsilon) &&
        self[1].approx_eq_eps(&other[1], epsilon) &&
        self[2].approx_eq_eps(&other[2], epsilon)
    }
}

impl<S: BaseFloat> ApproxEq<S> for Matrix4<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Matrix4<S>, epsilon: &S) -> bool {
        self[0].approx_eq_eps(&other[0], epsilon) &&
        self[1].approx_eq_eps(&other[1], epsilon) &&
        self[2].approx_eq_eps(&other[2], epsilon) &&
        self[3].approx_eq_eps(&other[3], epsilon)
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

impl<S: BaseFloat> ToMatrix3<S> for Matrix2<S> {
    /// Clone the elements of a 2-dimensional matrix into the top-left corner
    /// of a 3-dimensional identity matrix.
    fn to_matrix3(&self) -> Matrix3<S> {
        Matrix3::new(self[0][0], self[0][1], zero(),
                     self[1][0], self[1][1], zero(),
                         zero(),     zero(),  one())
    }
}

impl<S: BaseFloat> ToMatrix4<S> for Matrix2<S> {
    /// Clone the elements of a 2-dimensional matrix into the top-left corner
    /// of a 4-dimensional identity matrix.
    fn to_matrix4(&self) -> Matrix4<S> {
        Matrix4::new(self[0][0], self[0][1], zero(), zero(),
                     self[1][0], self[1][1], zero(), zero(),
                         zero(),     zero(),  one(), zero(),
                         zero(),     zero(), zero(),  one())
    }
}

impl<S: BaseFloat> ToMatrix4<S> for Matrix3<S> {
    /// Clone the elements of a 3-dimensional matrix into the top-left corner
    /// of a 4-dimensional identity matrix.
    fn to_matrix4(&self) -> Matrix4<S> {
        Matrix4::new(self[0][0], self[0][1], self[0][2], zero(),
                     self[1][0], self[1][1], self[1][2], zero(),
                     self[2][0], self[2][1], self[2][2], zero(),
                         zero(),     zero(),     zero(),  one())
    }
}

impl<S: BaseFloat + 'static> ToQuaternion<S> for Matrix3<S> {
    /// Convert the matrix to a quaternion
    fn to_quaternion(&self) -> Quaternion<S> {
        // http://www.cs.ucr.edu/~vbz/resources/quatut.pdf
        let trace = self.trace();
        let half: S = cast(0.5f64).unwrap();
        match () {
            () if trace >= zero::<S>() => {
                let s = (one::<S>() + trace).sqrt();
                let w = half * s;
                let s = half / s;
                let x = (self[1][2] - self[2][1]) * s;
                let y = (self[2][0] - self[0][2]) * s;
                let z = (self[0][1] - self[1][0]) * s;
                Quaternion::new(w, x, y, z)
            }
            () if (self[0][0] > self[1][1]) && (self[0][0] > self[2][2]) => {
                let s = (half + (self[0][0] - self[1][1] - self[2][2])).sqrt();
                let w = half * s;
                let s = half / s;
                let x = (self[0][1] - self[1][0]) * s;
                let y = (self[2][0] - self[0][2]) * s;
                let z = (self[1][2] - self[2][1]) * s;
                Quaternion::new(w, x, y, z)
            }
            () if self[1][1] > self[2][2] => {
                let s = (half + (self[1][1] - self[0][0] - self[2][2])).sqrt();
                let w = half * s;
                let s = half / s;
                let x = (self[0][1] - self[1][0]) * s;
                let y = (self[1][2] - self[2][1]) * s;
                let z = (self[2][0] - self[0][2]) * s;
                Quaternion::new(w, x, y, z)
            }
            () => {
                let s = (half + (self[2][2] - self[0][0] - self[1][1])).sqrt();
                let w = half * s;
                let s = half / s;
                let x = (self[2][0] - self[0][2]) * s;
                let y = (self[1][2] - self[2][1]) * s;
                let z = (self[0][1] - self[1][0]) * s;
                Quaternion::new(w, x, y, z)
            }
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
