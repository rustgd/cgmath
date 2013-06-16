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

use dim::Dimensional;
mod macros;
mod dim_macros;
mod vec_macros;

#[deriving(Eq)]
pub struct Vec4<T> { x: T, y: T, z: T, w: T }

impl_dimensional!(Vec4, T, 4)
impl_dimensional_fns!(Vec4, T, 4)
impl_approx!(Vec4)
impl_swap!(Vec4)

impl_vec!(Vec4 { x, y, z, w })
impl_vec_copyable!(Vec4)
impl_vec_numeric!(Vec4)
impl_vec_neg!(Vec4)
impl_vec_euclidean!(Vec4)
impl_vec_ord!(Vec4)
impl_vec_eq!(Vec4)
impl_vec_bool!(Vec4)
impl_vec_not!(Vec4)

impl<T:Copy + Num> Vec4<T> {
    #[inline] pub fn unit_x() -> Vec4<T> { Vec4::new(one!(T), zero!(T), zero!(T), zero!(T)) }
    #[inline] pub fn unit_y() -> Vec4<T> { Vec4::new(zero!(T), one!(T), zero!(T), zero!(T)) }
    #[inline] pub fn unit_z() -> Vec4<T> { Vec4::new(zero!(T), zero!(T), one!(T), zero!(T)) }
    #[inline] pub fn unit_w() -> Vec4<T> { Vec4::new(zero!(T), zero!(T), zero!(T), one!(T)) }
}

#[cfg(test)]
mod tests {
    use vec::*;

    #[test]
    fn test_vec4() {
        let a = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
        let b = Vec4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
        let f1 = 1.5;
        let f2 = 0.5;

        let mut mut_a = a;

        assert_eq!(Vec4::new::<float>(1.0, 2.0, 3.0, 4.0), a);
        assert_eq!(Vec4::from_value(1.0), Vec4::new::<float>(1.0, 1.0, 1.0, 1.0));

        *mut_a.index_mut(0) = 42.0;
        *mut_a.index_mut(1) = 43.0;
        *mut_a.index_mut(2) = 44.0;
        *mut_a.index_mut(3) = 45.0;
        assert_eq!(mut_a, Vec4::new::<float>(42.0, 43.0, 44.0, 45.0));
        mut_a = a;

        mut_a.swap(0, 3);
        assert_eq!(*mut_a.index(0), *a.index(3));
        assert_eq!(*mut_a.index(3), *a.index(0));
        mut_a = a;

        mut_a.swap(1, 2);
        assert_eq!(*mut_a.index(1), *a.index(2));
        assert_eq!(*mut_a.index(2), *a.index(1));
        mut_a = a;

        assert_eq!(Vec4::zero(), Vec4::new::<float>(0.0, 0.0, 0.0, 0.0));
        assert_eq!(Vec4::unit_x(), Vec4::new::<float>(1.0, 0.0, 0.0, 0.0));
        assert_eq!(Vec4::unit_y(), Vec4::new::<float>(0.0, 1.0, 0.0, 0.0));
        assert_eq!(Vec4::unit_z(), Vec4::new::<float>(0.0, 0.0, 1.0, 0.0));
        assert_eq!(Vec4::unit_w(), Vec4::new::<float>(0.0, 0.0, 0.0, 1.0));
        assert_eq!(Vec4::identity(), Vec4::new::<float>(1.0, 1.0, 1.0, 1.0));

        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 2.0);
        assert_eq!(a.z, 3.0);
        assert_eq!(a.w, 4.0);
        assert_eq!(*a.index(0), 1.0);
        assert_eq!(*a.index(1), 2.0);
        assert_eq!(*a.index(2), 3.0);
        assert_eq!(*a.index(3), 4.0);

        assert_eq!(-a, Vec4::new::<float>(-1.0, -2.0, -3.0, -4.0));
        assert_eq!(a.neg(), Vec4::new::<float>(-1.0, -2.0, -3.0, -4.0));

        assert_eq!(a.mul_t(f1), Vec4::new::<float>( 1.5, 3.0, 4.5, 6.0));
        assert_eq!(a.div_t(f2), Vec4::new::<float>( 2.0, 4.0, 6.0, 8.0));

        assert_eq!(a.add_v(&b), Vec4::new::<float>(    6.0,     8.0,    10.0,    12.0));
        assert_eq!(a.sub_v(&b), Vec4::new::<float>(   -4.0,    -4.0,    -4.0,    -4.0));
        assert_eq!(a.mul_v(&b), Vec4::new::<float>(    5.0,    12.0,    21.0,    32.0));
        assert_eq!(a.div_v(&b), Vec4::new::<float>(1.0/5.0, 2.0/6.0, 3.0/7.0, 4.0/8.0));

        assert_eq!(a.dot(&b), 70.0);

        mut_a.neg_self();
        assert_eq!(mut_a, -a);
        mut_a = a;

        mut_a.mul_self_t(f1);
        assert_eq!(mut_a, a.mul_t(f1));
        mut_a = a;

        mut_a.div_self_t(f2);
        assert_eq!(mut_a, a.div_t(f2));
        mut_a = a;

        mut_a.add_self_v(&b);
        assert_eq!(mut_a, a.add_v(&b));
        mut_a = a;

        mut_a.sub_self_v(&b);
        assert_eq!(mut_a, a.sub_v(&b));
        mut_a = a;

        mut_a.mul_self_v(&b);
        assert_eq!(mut_a, a.mul_v(&b));
        mut_a = a;

        mut_a.div_self_v(&b);
        assert_eq!(mut_a, a.div_v(&b));
    }

    #[test]
    fn test_vec4_approx_eq() {
        assert!(!Vec4::new::<float>(0.000001, 0.000001, 0.000001, 0.000001).approx_eq(&Vec4::new::<float>(0.0, 0.0, 0.0, 0.0)));
        assert!(Vec4::new::<float>(0.0000001, 0.0000001, 0.0000001, 0.0000001).approx_eq(&Vec4::new::<float>(0.0, 0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_vec4_euclidean() {
        let a = Vec4::new::<float>(1.0, 2.0, 4.0, 10.0); // (1, 2, 4, 10, 11) Pythagorean quintuple
        let b0 = Vec4::new::<float>(1.0, 2.0, 8.0, 10.0); // (1, 2, 8, 10, 13) Pythagorean quintuple
        let b = a.add_v(&b0);

        assert_eq!(a.length(), 11.0);
        assert_eq!(a.length2(), 11.0 * 11.0);

        assert_eq!(b0.length(), 13.0);
        assert_eq!(b0.length2(), 13.0 * 13.0);

        assert_eq!(a.distance(&b), 13.0);
        assert_eq!(a.distance2(&b), 13.0 * 13.0);

        assert!(Vec4::new::<float>(1.0, 0.0, 1.0, 0.0).angle(&Vec4::new::<float>(0.0, 1.0, 0.0, 1.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec4::new::<float>(10.0, 0.0, 10.0, 0.0).angle(&Vec4::new::<float>(0.0, 5.0, 0.0, 5.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec4::new::<float>(-1.0, 0.0, -1.0, 0.0).angle(&Vec4::new::<float>(0.0, 1.0, 0.0, 1.0)).approx_eq(&Real::frac_pi_2()));

        assert!(Vec4::new::<float>(1.0, 2.0, 4.0, 10.0).normalize().approx_eq(&Vec4::new::<float>(1.0/11.0, 2.0/11.0, 4.0/11.0, 10.0/11.0)));
        // TODO: test normalize_to, normalize_self, and normalize_self_to

        let c = Vec4::new::<float>(-2.0, -1.0, 1.0, 2.0);
        let d = Vec4::new::<float>( 1.0,  0.0, 0.5, 1.0);

        assert_eq!(c.lerp(&d, 0.75), Vec4::new::<float>(0.250, -0.250, 0.625, 1.250));

        let mut mut_c = c;
        mut_c.lerp_self(&d, 0.75);
        assert_eq!(mut_c, c.lerp(&d, 0.75));
    }

    #[test]
    fn test_vec4_boolean() {
        let tftf = Vec4::new(true, false, true, false);
        let ffff = Vec4::new(false, false, false, false);
        let tttt = Vec4::new(true, true, true, true);

        assert_eq!(tftf.any(), true);
        assert_eq!(tftf.all(), false);
        assert_eq!(tftf.not(), Vec4::new(false, true, false, true));

        assert_eq!(ffff.any(), false);
        assert_eq!(ffff.all(), false);
        assert_eq!(ffff.not(), Vec4::new(true, true, true, true));

        assert_eq!(tttt.any(), true);
        assert_eq!(tttt.all(), true);
        assert_eq!(tttt.not(), Vec4::new(false, false, false, false));
    }
}
