// Copyright 2016 The CGMath Developers. For a full listing of the authors,
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

use rand::{Rand, Rng};
use num_traits::{cast, Zero};

use structure::*;

use angle::Rad;
use approx::ApproxEq;
use quaternion::Quaternion;
use num::BaseFloat;

/// A set of [Euler angles] representing a rotation in three-dimensional space.
///
/// This type is marked as `#[repr(C, packed)]`.
///
/// # Defining rotations using Euler angles
///
/// Note that while [Euler angles] are intuitive to define, they are prone to
/// [gimbal lock] and are challenging to interpolate between. Instead we
/// recommend that you convert them to a more robust representation, such as a
/// quaternion or or rotation matrix. To this end, `From<Euler<A>>` conversions
/// are provided for the following types:
///
/// - [`Basis3`](struct.Basis3.html)
/// - [`Matrix3`](struct.Matrix3.html)
/// - [`Matrix4`](struct.Matrix4.html)
/// - [`Quaternion`](struct.Quaternion.html)
///
/// For example, to define a quaternion that applies the following:
///
/// 1. a 45° rotation around the _x_ axis
/// 2. a 180° rotation around the _y_ axis
/// 3. a -30° rotation around the _z_ axis
///
/// you can use the following code:
///
/// ```
/// use cgmath::{Deg, Euler, Quaternion};
///
/// let rotation = Quaternion::from(Euler {
///     x: Deg::new(45.0),
///     y: Deg::new(180.0),
///     z: Deg::new(15.0),
/// });
/// ```
///
/// [Euler angles]: https://en.wikipedia.org/wiki/Euler_angles
/// [gimbal lock]: https://en.wikipedia.org/wiki/Gimbal_lock#Gimbal_lock_in_applied_mathematics
/// [convert]: #defining-rotations-using-euler-angles
#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
#[derive(PartialEq, Eq)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Euler<A: Angle> {
    /// The angle to apply around the _x_ axis.
    pub x: A,
    /// The angle to apply around the _y_ axis.
    pub y: A,
    /// The angle to apply around the _z_ axis.
    pub z: A,
}

impl<S: BaseFloat> From<Quaternion<S>> for Euler<Rad<S>> {
    fn from(src: Quaternion<S>) -> Euler<Rad<S>> {
        let sig: S = cast(0.499).unwrap();
        let two: S = cast(2).unwrap();
        let one: S = cast(1).unwrap();

        let (qw, qx, qy, qz) = (src.s, src.v.x, src.v.y, src.v.z);
        let (sqw, sqx, sqy, sqz) = (qw * qw, qx * qx, qy * qy, qz * qz);

        let unit = sqx + sqy + sqz + sqw;
        let test = qx * qy + qz * qw;

        if test > sig * unit {
            Euler {
                x: Rad::turn_div_4(),
                y: Rad::zero(),
                z: Rad::atan2(qx, qw) * two,
            }
        } else if test < -sig * unit {
            Euler {
                x: -Rad::turn_div_4(),
                y: Rad::zero(),
                z: Rad::atan2(qx, qw) * two,
            }
        } else {
            Euler {
                x: Rad::asin(two * (qx * qy + qz * qw)),
                y: Rad::atan2(two * (qy * qw - qx * qz), one - two * (sqy + sqz)),
                z: Rad::atan2(two * (qx * qw - qy * qz), one - two * (sqx + sqz)),
            }
        }
    }
}

impl<A: Angle> ApproxEq for Euler<A> {
    type Epsilon = A::Unitless;

    #[inline]
    fn approx_eq_eps(&self, other: &Euler<A>, epsilon: &A::Unitless) -> bool {
        self.x.approx_eq_eps(&other.x, epsilon) &&
        self.y.approx_eq_eps(&other.y, epsilon) &&
        self.z.approx_eq_eps(&other.z, epsilon)
    }
}

impl<A: Angle + Rand> Rand for Euler<A> {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Euler<A> {
        Euler { x: rng.gen(), y: rng.gen(), z: rng.gen() }
    }
}
