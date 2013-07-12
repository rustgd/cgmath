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

use core::Vec4;
use geom::{Point2, Point3};

pub struct Simplex<T>;

impl<T> Simplex<T> {
    pub fn new() -> Simplex<T> {
        fail!("Not yet implemented!")
    }

    pub fn noise2(&self, _pos: Point2<T>) -> T {
        fail!("Not yet implemented!")
    }

    pub fn noise3(&self, _pos: Point3<T>) -> T {
        fail!("Not yet implemented!")
    }

    pub fn noise4(&self, _vec: Vec4<T>) -> T {
        fail!("Not yet implemented!")
    }
}
