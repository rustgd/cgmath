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

pub use self::mat::{Mat2, ToMat2};
pub use self::mat::{Mat3, ToMat3};
pub use self::mat::{Mat4, ToMat4};
pub use self::quat::{Quat, ToQuat};
pub use self::vec::{NumVec, RealVec};
pub use self::vec::{OrdVec, EqVec, BoolVec};
pub use self::vec::{Vec2, ToVec2, AsVec2};
pub use self::vec::{Vec3, ToVec3, AsVec3};
pub use self::vec::{Vec4, ToVec4, AsVec4};

pub mod mat;
pub mod quat;
pub mod vec;

pub trait Dimensional<T,Slice> {
    pub fn index<'a>(&'a self, i: uint) -> &'a T;
    pub fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T;
    pub fn as_slice<'a>(&'a self) -> &'a Slice;
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut Slice;
}

pub trait Swap {
    pub fn swap(&mut self, a: uint, b: uint);
}

pub trait ComponentWise<T> {
    pub fn component_add(&self) -> T;
    pub fn component_mul(&self) -> T;
    pub fn component_min(&self) -> T;
    pub fn component_max(&self) -> T;
}
