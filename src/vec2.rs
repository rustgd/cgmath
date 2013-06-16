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
pub struct Vec2<T> { x: T, y: T }

impl_dimensional!(Vec2, T, 2)
impl_dimensional_fns!(Vec2, T, 2)
impl_swap!(Vec2)
impl_approx!(Vec2)

impl_vec!(Vec2 { x, y })
impl_vec_copyable!(Vec2)
impl_vec_numeric!(Vec2)
impl_vec_neg!(Vec2)
impl_vec_euclidean!(Vec2)
impl_vec_ord!(Vec2)
impl_vec_eq!(Vec2)
impl_vec_bool!(Vec2)
impl_vec_not!(Vec2)

impl<T:Copy + Num> Vec2<T> {
    #[inline] pub fn unit_x() -> Vec2<T> { Vec2::new(one!(T), zero!(T)) }
    #[inline] pub fn unit_y() -> Vec2<T> { Vec2::new(zero!(T), one!(T)) }

    #[inline]
    pub fn perp_dot(&self, other: &Vec2<T>) -> T {
        (*self.index(0) * *other.index(1)) -
        (*self.index(1) * *other.index(0))
    }
}

#[cfg(test)]
mod tests {
    use vec::*;

    #[test]
    fn test_vec2() {
        let a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: 3.0, y: 4.0 };
        let f1 = 1.5;
        let f2 = 0.5;

        let mut mut_a = a;

        assert_eq!(Vec2::new::<float>(1.0, 2.0), a);
        assert_eq!(Vec2::from_value(1.0), Vec2::new::<float>(1.0, 1.0));

        assert_eq!(Vec2::zero(), Vec2::new::<float>(0.0, 0.0));
        assert_eq!(Vec2::unit_x(), Vec2::new::<float>(1.0, 0.0));
        assert_eq!(Vec2::unit_y(), Vec2::new::<float>(0.0, 1.0));
        assert_eq!(Vec2::identity(), Vec2::new::<float>(1.0, 1.0));

        *mut_a.index_mut(0) = 42.0;
        *mut_a.index_mut(1) = 43.0;
        assert_eq!(mut_a, Vec2::new::<float>(42.0, 43.0));
        mut_a = a;

        mut_a.swap(0, 1);
        assert_eq!(*mut_a.index(0), *a.index(1));
        assert_eq!(*mut_a.index(1), *a.index(0));
        mut_a = a;

        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 2.0);
        assert_eq!(*a.index(0), 1.0);
        assert_eq!(*a.index(1), 2.0);

        assert_eq!(-a, Vec2::new::<float>(-1.0, -2.0));
        assert_eq!(a.neg(), Vec2::new::<float>(-1.0, -2.0));

        assert_eq!(a.mul_t(f1), Vec2::new::<float>( 1.5, 3.0));
        assert_eq!(a.div_t(f2), Vec2::new::<float>( 2.0, 4.0));

        assert_eq!(a.add_v(&b), Vec2::new::<float>(    4.0,     6.0));
        assert_eq!(a.sub_v(&b), Vec2::new::<float>(   -2.0,    -2.0));
        assert_eq!(a.mul_v(&b), Vec2::new::<float>(    3.0,     8.0));
        assert_eq!(a.div_v(&b), Vec2::new::<float>(1.0/3.0, 2.0/4.0));

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
    fn test_vec2_approx_eq() {
        assert!(!Vec2::new::<float>(0.000001, 0.000001).approx_eq(&Vec2::new::<float>(0.0, 0.0)));
        assert!(Vec2::new::<float>(0.0000001, 0.0000001).approx_eq(&Vec2::new::<float>(0.0, 0.0)));
    }

    #[test]
    fn test_vec2_euclidean() {
        let a = Vec2::new::<float>(5.0, 12.0); // (5, 12, 13) Pythagorean triple
        let b0 = Vec2::new::<float>(3.0, 4.0); // (3, 4, 5) Pythagorean triple
        let b = a.add_v(&b0);

        assert_eq!(a.length(), 13.0);
        assert_eq!(a.length2(), 13.0 * 13.0);

        assert_eq!(b0.length(), 5.0);
        assert_eq!(b0.length2(), 5.0 * 5.0);

        assert_eq!(a.distance(&b), 5.0);
        assert_eq!(a.distance2(&b), 5.0 * 5.0);

        assert!(Vec2::new::<float>(1.0, 0.0).angle(&Vec2::new::<float>(0.0, 1.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec2::new::<float>(10.0, 0.0).angle(&Vec2::new::<float>(0.0, 5.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec2::new::<float>(-1.0, 0.0).angle(&Vec2::new::<float>(0.0, 1.0)).approx_eq(&-Real::frac_pi_2::<float>()));

        assert!(Vec2::new::<float>(3.0, 4.0).normalize().approx_eq(&Vec2::new::<float>(3.0/5.0, 4.0/5.0)));
        // TODO: test normalize_to, normalize_self, and normalize_self_to

        let c = Vec2::new::<float>(-2.0, -1.0);
        let d = Vec2::new::<float>( 1.0,  0.0);

        assert_eq!(c.lerp(&d, 0.75), Vec2::new::<float>(0.250, -0.250));

        let mut mut_c = c;
        mut_c.lerp_self(&d, 0.75);
        assert_eq!(mut_c, c.lerp(&d, 0.75));
    }

    #[test]
    fn test_vec2_boolean() {
        let tf = Vec2::new(true, false);
        let ff = Vec2::new(false, false);
        let tt = Vec2::new(true, true);

        assert_eq!(tf.any(), true);
        assert_eq!(tf.all(), false);
        assert_eq!(tf.not(), Vec2::new(false, true));

        assert_eq!(ff.any(), false);
        assert_eq!(ff.all(), false);
        assert_eq!(ff.not(), Vec2::new(true, true));

        assert_eq!(tt.any(), true);
        assert_eq!(tt.all(), true);
        assert_eq!(tt.not(), Vec2::new(false, false));
    }
}
