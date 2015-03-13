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

//! Generic spatial bounds.

use plane::Plane;

/// Spatial relation between two objects.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum Relation {
    /// Completely inside
    In,
    /// Crosses the boundary
    Cross,
    /// Completely outside
    Out,
}

/// Generic bound.
pub trait Bound<S> {
    /// Classify the spatial relation with a plane
    fn relate(&self, &Plane<S>) -> Relation;
}
