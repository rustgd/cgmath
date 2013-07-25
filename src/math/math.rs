// Copyright 2013 The Lmath Developers. For a full listing of the authors,
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

//! Core datatypes and conversion traits for 3D mathematics

pub use self::mat::{Mat, NumMat, FloatMat};
pub use self::mat::{Mat2, ToMat2};
pub use self::mat::{Mat3, ToMat3};
pub use self::mat::{Mat4, ToMat4};
pub use self::quat::{Quat, ToQuat};
pub use self::vec::{NumVec, FloatVec};
pub use self::vec::{OrdVec, EqVec, BoolVec};
pub use self::vec::{Vec2, ToVec2, AsVec2};
pub use self::vec::{Vec3, ToVec3, AsVec3};
pub use self::vec::{Vec4, ToVec4, AsVec4};

pub use self::plane::Plane3;
pub use self::point::Point;
pub use self::point::{Point2, AsPoint2};
pub use self::point::{Point3, AsPoint3};
pub use self::ray::{Ray2, Ray3};

pub mod mat;
pub mod quat;
pub mod vec;

pub mod plane;
pub mod point;
pub mod ray;

pub trait Dimensioned<T,Slice> {
    pub fn i<'a>(&'a self, i: uint) -> &'a T;
    pub fn mut_i<'a>(&'a mut self, i: uint) -> &'a mut T;
    pub fn as_slice<'a>(&'a self) -> &'a Slice;
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut Slice;
}

pub trait SwapComponents {
    pub fn swap(&mut self, a: uint, b: uint);
}

// `Dimensioned` impls for primitive numeric types

impl_dimensioned!(u8)
impl_dimensioned!(u16)
impl_dimensioned!(u32)
impl_dimensioned!(u64)
impl_dimensioned!(uint)
impl_dimensioned!(i8)
impl_dimensioned!(i16)
impl_dimensioned!(i32)
impl_dimensioned!(i64)
impl_dimensioned!(int)
impl_dimensioned!(f32)
impl_dimensioned!(f64)
impl_dimensioned!(float)

// Helper type aliases for implementing `Dimendioned` and `SwapComponents`
// for tuples.

pub type Tuple1<T> = (T,);
pub type Tuple2<T> = (T,T);
pub type Tuple3<T> = (T,T,T);
pub type Tuple4<T> = (T,T,T,T);
pub type Tuple5<T> = (T,T,T,T,T);
pub type Tuple6<T> = (T,T,T,T,T,T);

// `Dimensioned` impls for tuples

impl_dimensioned!(Tuple1, T, 1)
impl_dimensioned!(Tuple2, T, 2)
impl_dimensioned!(Tuple3, T, 3)
impl_dimensioned!(Tuple4, T, 4)
impl_dimensioned!(Tuple5, T, 5)
impl_dimensioned!(Tuple6, T, 6)

// `SwapComponents` impls for tuples

impl_swap_components!(Tuple1)
impl_swap_components!(Tuple2)
impl_swap_components!(Tuple3)
impl_swap_components!(Tuple4)
impl_swap_components!(Tuple5)
impl_swap_components!(Tuple6)

// Helper type aliases for implementing `Dimendioned` and `SwapComponents`
// for fixed length vectors.

pub type Fixed1<T> = [T, ..1];
pub type Fixed2<T> = [T, ..2];
pub type Fixed3<T> = [T, ..3];
pub type Fixed4<T> = [T, ..4];
pub type Fixed5<T> = [T, ..5];
pub type Fixed6<T> = [T, ..6];

// `Dimensioned` impls for fixed length vectors

impl_dimensioned!(Fixed1, T, 1)
impl_dimensioned!(Fixed2, T, 2)
impl_dimensioned!(Fixed3, T, 3)
impl_dimensioned!(Fixed4, T, 4)
impl_dimensioned!(Fixed5, T, 5)
impl_dimensioned!(Fixed6, T, 6)

// `SwapComponents` impls for fixed length vectors

impl_swap_components!(Fixed1)
impl_swap_components!(Fixed2)
impl_swap_components!(Fixed3)
impl_swap_components!(Fixed4)
impl_swap_components!(Fixed5)
impl_swap_components!(Fixed6)
