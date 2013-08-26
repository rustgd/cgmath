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

use types::vector::{Vec2, Vec3, Vec4};

#[deriving(Clone, Eq)] pub struct Mat2<T> { x: Vec2<T>, y: Vec2<T> }
#[deriving(Clone, Eq)] pub struct Mat3<T> { x: Vec3<T>, y: Vec3<T>, z: Vec3<T> }
#[deriving(Clone, Eq)] pub struct Mat4<T> { x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T> }
