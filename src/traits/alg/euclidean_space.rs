// Copyright 2013 The OMath Developers. For a full listing of the authors,
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

use std::num;

use traits::alg::Ring;
use traits::alg::InnerProductSpace;

/// The Euclidean space is a vector space over the Real numbers.
pub trait EuclideanSpace<S:Real + Ring>: InnerProductSpace<S> {
    fn dot(&self, other: &Self) -> S {
        self.inner(other)
    }

    /// Returns `true` if the vector is perpendicular (at right angles to)
    /// the other vector.
    fn is_perpendicular(&self, other: &Self) -> bool {
        self.is_orthogonal(other)
    }

    /// Returns the squared length of the vector. This does not perform an
    /// expensive square root operation like in the `length` method and can
    /// therefore be more efficient for comparing the lengths of two vectors.
    fn length2(&self) -> S {
        self.dot(self)
    }

    /// The norm of the vector.
    fn length(&self) -> S {
        num::sqrt(self.dot(self))
    }

    /// The angle between the vector and `other`.
    fn angle(&self, other: &Self) -> S;

    /// Returns a vector with the same direction, but with a `length` (or
    /// `norm`) of `1`.
    fn normalize(&self) -> Self {
        self.normalize_to(num::one::<S>())
    }

    /// Returns a vector with the same direction and a given `length`.
    fn normalize_to(&self, length: S) -> Self {
        *self * (length / self.length())
    }

    /// Returns the result of linarly interpolating the length of the vector
    /// to the length of `other` by the specified amount.
    fn lerp(&self, other: &Self, amount: S) -> Self {
        *self + (*other - *self) * amount
    }
}
