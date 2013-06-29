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

use vec::Vec3;
use point::Point3;

/// A three-dimensional ray
///
/// # Fields
///
/// - `pos`: the endpoint of the ray
/// - `dir`: the direction vector
#[deriving(Clone, Eq)]
pub struct Ray3<T> {
    pos: Point3<T>,
    dir: Vec3<T>,
}

impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for Ray3<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &Ray3<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &Ray3<T>, epsilon: &T) -> bool {
        self.pos.approx_eq_eps(&other.pos, epsilon) &&
        self.dir.approx_eq_eps(&other.dir, epsilon)
    }
}