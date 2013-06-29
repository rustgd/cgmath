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

pub use dim::Dimensional;

mod num_macros;
mod dim_macros;
mod vec_macros;

#[deriving(Clone, Eq)]
pub struct Vec2<T> { x: T, y: T }

// GLSL-style type aliases
pub type vec2  = Vec2<f32>;
pub type dvec2 = Vec2<f64>;
pub type bvec2 = Vec2<bool>;
pub type ivec2 = Vec2<i32>;
pub type uvec2 = Vec2<u32>;

// Rust-style type aliases
pub type Vec2f   = Vec2<float>;
pub type Vec2f32 = Vec2<f32>;
pub type Vec2f64 = Vec2<f64>;
pub type Vec2i   = Vec2<int>;
pub type Vec2i8  = Vec2<i8>;
pub type Vec2i16 = Vec2<i16>;
pub type Vec2i32 = Vec2<i32>;
pub type Vec2i64 = Vec2<i64>;
pub type Vec2u   = Vec2<uint>;
pub type Vec2u8  = Vec2<u8>;
pub type Vec2u16 = Vec2<u16>;
pub type Vec2u32 = Vec2<u32>;
pub type Vec2u64 = Vec2<u64>;
pub type Vec2b   = Vec2<bool>;

impl_dimensional!(Vec2, T, 2)
impl_dimensional_fns!(Vec2, T, 2)
impl_swap!(Vec2)
impl_approx!(Vec2)

impl_vec!(Vec2 { x, y })
impl_vec_clonable!(Vec2)
impl_vec_numeric!(Vec2)
impl_vec_neg!(Vec2)
impl_vec_euclidean!(Vec2)
impl_vec_ord!(Vec2)
impl_vec_eq!(Vec2)
impl_vec_bool!(Vec2)
impl_vec_not!(Vec2)

impl<T:Clone + Num> Vec2<T> {
    #[inline] pub fn unit_x() -> Vec2<T> { Vec2::new(one!(T), zero!(T)) }
    #[inline] pub fn unit_y() -> Vec2<T> { Vec2::new(zero!(T), one!(T)) }

    #[inline]
    pub fn perp_dot(&self, other: &Vec2<T>) -> T {
        (*self.index(0) * *other.index(1)) -
        (*self.index(1) * *other.index(0))
    }
}

#[cfg(test)]
mod vec2_tests {
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

#[deriving(Clone, Eq)]
pub struct Vec3<T> { x: T, y: T, z: T }

// GLSL-style type aliases
pub type vec3  = Vec3<f32>;
pub type dvec3 = Vec3<f64>;
pub type bvec3 = Vec3<bool>;
pub type ivec3 = Vec3<i32>;
pub type uvec3 = Vec3<u32>;

// Rust-style type aliases
pub type Vec3f   = Vec3<float>;
pub type Vec3f32 = Vec3<f32>;
pub type Vec3f64 = Vec3<f64>;
pub type Vec3i   = Vec3<int>;
pub type Vec3i8  = Vec3<i8>;
pub type Vec3i16 = Vec3<i16>;
pub type Vec3i32 = Vec3<i32>;
pub type Vec3i64 = Vec3<i64>;
pub type Vec3u   = Vec3<uint>;
pub type Vec3u8  = Vec3<u8>;
pub type Vec3u16 = Vec3<u16>;
pub type Vec3u32 = Vec3<u32>;
pub type Vec3u64 = Vec3<u64>;
pub type Vec3b   = Vec3<bool>;

impl_dimensional!(Vec3, T, 3)
impl_dimensional_fns!(Vec3, T, 3)
impl_swap!(Vec3)
impl_approx!(Vec3)

impl_vec!(Vec3 { x, y, z })
impl_vec_clonable!(Vec3)
impl_vec_numeric!(Vec3)
impl_vec_neg!(Vec3)
impl_vec_euclidean!(Vec3)
impl_vec_ord!(Vec3)
impl_vec_eq!(Vec3)
impl_vec_bool!(Vec3)
impl_vec_not!(Vec3)

impl<T:Clone + Num> Vec3<T> {
    #[inline] pub fn unit_x() -> Vec3<T> { Vec3::new(one!(T), zero!(T), zero!(T)) }
    #[inline] pub fn unit_y() -> Vec3<T> { Vec3::new(zero!(T), one!(T), zero!(T)) }
    #[inline] pub fn unit_z() -> Vec3<T> { Vec3::new(zero!(T), zero!(T), one!(T)) }

    #[inline]
    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new((*self.index(1) * *other.index(2)) - (*self.index(2) * *other.index(1)),
                  (*self.index(2) * *other.index(0)) - (*self.index(0) * *other.index(2)),
                  (*self.index(0) * *other.index(1)) - (*self.index(1) * *other.index(0)))
    }

    #[inline]
    pub fn cross_self(&mut self, other: &Vec3<T>) {
        *self = self.cross(other)
    }
}

#[cfg(test)]
mod vec3_tests{
    use vec::*;

    #[test]
    fn test_vec3() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let f1 = 1.5;
        let f2 = 0.5;

        let mut mut_a = a;

        assert_eq!(Vec3::new::<float>(1.0, 2.0, 3.0), a);
        assert_eq!(Vec3::from_value(1.0), Vec3::new::<float>(1.0, 1.0, 1.0));

        assert_eq!(Vec3::zero(), Vec3::new::<float>(0.0, 0.0, 0.0));
        assert_eq!(Vec3::unit_x(), Vec3::new::<float>(1.0, 0.0, 0.0));
        assert_eq!(Vec3::unit_y(), Vec3::new::<float>(0.0, 1.0, 0.0));
        assert_eq!(Vec3::unit_z(), Vec3::new::<float>(0.0, 0.0, 1.0));
        assert_eq!(Vec3::identity(), Vec3::new::<float>(1.0, 1.0, 1.0));

        *mut_a.index_mut(0) = 42.0;
        *mut_a.index_mut(1) = 43.0;
        *mut_a.index_mut(2) = 44.0;
        assert_eq!(mut_a, Vec3::new::<float>(42.0, 43.0, 44.0));
        mut_a = a;

        mut_a.swap(0, 2);
        assert_eq!(*mut_a.index(0), *a.index(2));
        assert_eq!(*mut_a.index(2), *a.index(0));
        mut_a = a;

        mut_a.swap(1, 2);
        assert_eq!(*mut_a.index(1), *a.index(2));
        assert_eq!(*mut_a.index(2), *a.index(1));
        mut_a = a;

        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 2.0);
        assert_eq!(a.z, 3.0);
        assert_eq!(*a.index(0), 1.0);
        assert_eq!(*a.index(1), 2.0);
        assert_eq!(*a.index(2), 3.0);

        assert_eq!(a.cross(&b), Vec3::new::<float>(-3.0, 6.0, -3.0));

        mut_a.cross_self(&b);
        assert_eq!(mut_a, a.cross(&b));
        mut_a = a;

        assert_eq!(-a, Vec3::new::<float>(-1.0, -2.0, -3.0));
        assert_eq!(a.neg(), Vec3::new::<float>(-1.0, -2.0, -3.0));

        assert_eq!(a.mul_t(f1), Vec3::new::<float>( 1.5, 3.0, 4.5));
        assert_eq!(a.div_t(f2), Vec3::new::<float>( 2.0, 4.0, 6.0));

        assert_eq!(a.add_v(&b), Vec3::new::<float>(    5.0,     7.0,     9.0));
        assert_eq!(a.sub_v(&b), Vec3::new::<float>(   -3.0,    -3.0,    -3.0));
        assert_eq!(a.mul_v(&b), Vec3::new::<float>(    4.0,    10.0,    18.0));
        assert_eq!(a.div_v(&b), Vec3::new::<float>(1.0/4.0, 2.0/5.0, 3.0/6.0));

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
    fn test_vec3_approx_eq() {
        assert!(!Vec3::new::<float>(0.000001, 0.000001, 0.000001).approx_eq(&Vec3::new::<float>(0.0, 0.0, 0.0)));
        assert!(Vec3::new::<float>(0.0000001, 0.0000001, 0.0000001).approx_eq(&Vec3::new::<float>(0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_vec3_euclidean() {
        let a = Vec3::new::<float>(2.0, 3.0, 6.0); // (2, 3, 6, 7) Pythagorean quadruple
        let b0 = Vec3::new::<float>(1.0, 4.0, 8.0); // (1, 4, 8, 9) Pythagorean quadruple
        let b = a.add_v(&b0);

        assert_eq!(a.length(), 7.0);
        assert_eq!(a.length2(), 7.0 * 7.0);

        assert_eq!(b0.length(), 9.0);
        assert_eq!(b0.length2(), 9.0 * 9.0);

        assert_eq!(a.distance(&b), 9.0);
        assert_eq!(a.distance2(&b), 9.0 * 9.0);

        assert!(Vec3::new::<float>(1.0, 0.0, 1.0).angle(&Vec3::new::<float>(1.0, 1.0, 0.0)).approx_eq(&Real::frac_pi_3()));
        assert!(Vec3::new::<float>(10.0, 0.0, 10.0).angle(&Vec3::new::<float>(5.0, 5.0, 0.0)).approx_eq(&Real::frac_pi_3()));
        assert!(Vec3::new::<float>(-1.0, 0.0, -1.0).angle(&Vec3::new::<float>(1.0, -1.0, 0.0)).approx_eq(&(2.0 * Real::frac_pi_3())));

        assert!(Vec3::new::<float>(2.0, 3.0, 6.0).normalize().approx_eq(&Vec3::new::<float>(2.0/7.0, 3.0/7.0, 6.0/7.0)));
        // TODO: test normalize_to, normalize_self, and normalize_self_to

        let c = Vec3::new::<float>(-2.0, -1.0, 1.0);
        let d = Vec3::new::<float>( 1.0,  0.0, 0.5);

        assert_eq!(c.lerp(&d, 0.75), Vec3::new::<float>(0.250, -0.250, 0.625));

        let mut mut_c = c;
        mut_c.lerp_self(&d, 0.75);
        assert_eq!(mut_c, c.lerp(&d, 0.75));
    }

    #[test]
    fn test_vec3_boolean() {
        let tft = Vec3::new(true, false, true);
        let fff = Vec3::new(false, false, false);
        let ttt = Vec3::new(true, true, true);

        assert_eq!(tft.any(), true);
        assert_eq!(tft.all(), false);
        assert_eq!(tft.not(), Vec3::new(false, true, false));

        assert_eq!(fff.any(), false);
        assert_eq!(fff.all(), false);
        assert_eq!(fff.not(), Vec3::new(true, true, true));

        assert_eq!(ttt.any(), true);
        assert_eq!(ttt.all(), true);
        assert_eq!(ttt.not(), Vec3::new(false, false, false));
    }
}

#[deriving(Clone, Eq)]
pub struct Vec4<T> { x: T, y: T, z: T, w: T }

// GLSL-style type aliases
pub type vec4  = Vec4<f32>;
pub type dvec4 = Vec4<f64>;
pub type bvec4 = Vec4<bool>;
pub type ivec4 = Vec4<i32>;
pub type uvec4 = Vec4<u32>;

// Rust-style type aliases
pub type Vec4f   = Vec4<float>;
pub type Vec4f32 = Vec4<f32>;
pub type Vec4f64 = Vec4<f64>;
pub type Vec4i   = Vec4<int>;
pub type Vec4i8  = Vec4<i8>;
pub type Vec4i16 = Vec4<i16>;
pub type Vec4i32 = Vec4<i32>;
pub type Vec4i64 = Vec4<i64>;
pub type Vec4u   = Vec4<uint>;
pub type Vec4u8  = Vec4<u8>;
pub type Vec4u16 = Vec4<u16>;
pub type Vec4u32 = Vec4<u32>;
pub type Vec4u64 = Vec4<u64>;
pub type Vec4b   = Vec4<bool>;

impl_dimensional!(Vec4, T, 4)
impl_dimensional_fns!(Vec4, T, 4)
impl_approx!(Vec4)
impl_swap!(Vec4)

impl_vec!(Vec4 { x, y, z, w })
impl_vec_clonable!(Vec4)
impl_vec_numeric!(Vec4)
impl_vec_neg!(Vec4)
impl_vec_euclidean!(Vec4)
impl_vec_ord!(Vec4)
impl_vec_eq!(Vec4)
impl_vec_bool!(Vec4)
impl_vec_not!(Vec4)

impl<T:Clone + Num> Vec4<T> {
    #[inline] pub fn unit_x() -> Vec4<T> { Vec4::new(one!(T), zero!(T), zero!(T), zero!(T)) }
    #[inline] pub fn unit_y() -> Vec4<T> { Vec4::new(zero!(T), one!(T), zero!(T), zero!(T)) }
    #[inline] pub fn unit_z() -> Vec4<T> { Vec4::new(zero!(T), zero!(T), one!(T), zero!(T)) }
    #[inline] pub fn unit_w() -> Vec4<T> { Vec4::new(zero!(T), zero!(T), zero!(T), one!(T)) }
}

#[cfg(test)]
mod vec4_tests {
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
