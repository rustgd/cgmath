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

pub use self::affine_space::AffineSpace;
pub use self::coordinate::Coordinate;
pub use self::division_ring::DivisionRing;
pub use self::euclidean_space::EuclideanSpace;
pub use self::inner_product_space::InnerProductSpace;
pub use self::module::Module;
pub use self::ordered_ring::OrderedRing;
pub use self::ring::Ring;
pub use self::scalar_mul::ScalarMul;
pub use self::vector_space::VectorSpace;

pub mod affine_space;
pub mod coordinate;
pub mod division_ring;
pub mod euclidean_space;
pub mod inner_product_space;
pub mod module;
pub mod ordered_ring;
pub mod ring;
pub mod scalar_mul;
pub mod vector_space;
