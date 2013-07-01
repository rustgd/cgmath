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

impl<T> Vec2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x: x, y: y }
    }
}

impl<T:Clone> Vec2<T> {
    #[inline]
    pub fn from_value(value: T) -> Vec2<T> {
        Vec2::new(value.clone(), value.clone())
    }
}

impl<T:Clone + Num> Vec2<T> {
    #[inline] pub fn identity() -> Vec2<T> { Vec2::from_value(one!(T)) }
    #[inline] pub fn zero() -> Vec2<T> { Vec2::from_value(zero!(T)) }

    #[inline] pub fn unit_x() -> Vec2<T> { Vec2::new(one!(T), zero!(T)) }
    #[inline] pub fn unit_y() -> Vec2<T> { Vec2::new(zero!(T), one!(T)) }

    #[inline]
    pub fn perp_dot(&self, other: &Vec2<T>) -> T {
        (*self.index(0) * *other.index(1)) -
        (*self.index(1) * *other.index(0))
    }

    #[inline] pub fn add_t(&self, value: T) -> Vec2<T> { Vec2::from_slice(self.map(|&x| x + value)) }
    #[inline] pub fn sub_t(&self, value: T) -> Vec2<T> { Vec2::from_slice(self.map(|&x| x - value)) }
    #[inline] pub fn mul_t(&self, value: T) -> Vec2<T> { Vec2::from_slice(self.map(|&x| x * value)) }
    #[inline] pub fn div_t(&self, value: T) -> Vec2<T> { Vec2::from_slice(self.map(|&x| x / value)) }
    #[inline] pub fn rem_t(&self, value: T) -> Vec2<T> { Vec2::from_slice(self.map(|&x| x % value)) }

    #[inline] pub fn add_v(&self, other: &Vec2<T>) -> Vec2<T> { Vec2::from_slice(self.zip(other, |&a, &b| a + b)) }
    #[inline] pub fn sub_v(&self, other: &Vec2<T>) -> Vec2<T> { Vec2::from_slice(self.zip(other, |&a, &b| a - b)) }
    #[inline] pub fn mul_v(&self, other: &Vec2<T>) -> Vec2<T> { Vec2::from_slice(self.zip(other, |&a, &b| a * b)) }
    #[inline] pub fn div_v(&self, other: &Vec2<T>) -> Vec2<T> { Vec2::from_slice(self.zip(other, |&a, &b| a / b)) }
    #[inline] pub fn rem_v(&self, other: &Vec2<T>) -> Vec2<T> { Vec2::from_slice(self.zip(other, |&a, &b| a % b)) }

    #[inline] pub fn neg_self(&mut self) { self.map_mut(|x| *x = -*x) }
    #[inline] pub fn add_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x + value.clone()) }
    #[inline] pub fn sub_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x - value.clone()) }
    #[inline] pub fn mul_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x * value.clone()) }
    #[inline] pub fn div_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x / value.clone()) }
    #[inline] pub fn rem_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x % value.clone()) }

    #[inline] pub fn add_self_v(&mut self, other: &Vec2<T>) { self.zip_mut(other, |a, &b| *a =*a + b) }
    #[inline] pub fn sub_self_v(&mut self, other: &Vec2<T>) { self.zip_mut(other, |a, &b| *a =*a - b) }
    #[inline] pub fn mul_self_v(&mut self, other: &Vec2<T>) { self.zip_mut(other, |a, &b| *a =*a * b) }
    #[inline] pub fn div_self_v(&mut self, other: &Vec2<T>) { self.zip_mut(other, |a, &b| *a =*a / b) }
    #[inline] pub fn rem_self_v(&mut self, other: &Vec2<T>) { self.zip_mut(other, |a, &b| *a =*a % b) }

    #[inline] pub fn dot(&self, other: &Vec2<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1)
    }
}

impl<T:Clone + Num> Neg<Vec2<T>> for Vec2<T> {
    #[inline]
    pub fn neg(&self) -> Vec2<T> {
        Vec2::from_slice(self.map(|&x| -x))
    }
}

impl<T:Clone + Not<T>> Not<Vec2<T>> for Vec2<T> {
    pub fn not(&self) -> Vec2<T> {
        Vec2::from_slice(self.map(|&x| !x))
    }
}

impl<T:Clone + Real> Vec2<T> {
    #[inline]
    pub fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline]
    pub fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline]
    pub fn distance2(&self, other: &Vec2<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline]
    pub fn distance(&self, other: &Vec2<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline]
    pub fn angle(&self, other: &Vec2<T>) -> T {
        self.perp_dot(other).atan2(&self.dot(other))
    }

    #[inline]
    pub fn normalize(&self) -> Vec2<T> {
        self.mul_t(one!(T)/self.length())
    }

    #[inline]
    pub fn normalize_to(&self, length: T) -> Vec2<T> {
        self.mul_t(length / self.length())
    }

    #[inline]
    pub fn lerp(&self, other: &Vec2<T>, amount: T) -> Vec2<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    #[inline]
    pub fn normalize_self(&mut self) {
        let rlen = self.length().recip();
        self.mul_self_t(rlen);
    }

    #[inline]
    pub fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    pub fn lerp_self(&mut self, other: &Vec2<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Clone + Ord> Vec2<T> {
    #[inline] pub fn lt_t(&self, value: T) -> Vec2<bool> { Vec2::from_slice(self.map(|&x| x < value)) }
    #[inline] pub fn le_t(&self, value: T) -> Vec2<bool> { Vec2::from_slice(self.map(|&x| x <= value)) }
    #[inline] pub fn ge_t(&self, value: T) -> Vec2<bool> { Vec2::from_slice(self.map(|&x| x >= value)) }
    #[inline] pub fn gt_t(&self, value: T) -> Vec2<bool> { Vec2::from_slice(self.map(|&x| x > value)) }

    #[inline] pub fn lt_v(&self, other: &Vec2<T>) -> Vec2<bool> { Vec2::from_slice(self.zip(other, |&a, &b| a < b)) }
    #[inline] pub fn le_v(&self, other: &Vec2<T>) -> Vec2<bool> { Vec2::from_slice(self.zip(other, |&a, &b| a <= b)) }
    #[inline] pub fn ge_v(&self, other: &Vec2<T>) -> Vec2<bool> { Vec2::from_slice(self.zip(other, |&a, &b| a >= b)) }
    #[inline] pub fn gt_v(&self, other: &Vec2<T>) -> Vec2<bool> { Vec2::from_slice(self.zip(other, |&a, &b| a > b)) }
}

impl<T:Clone + Eq> Vec2<T> {
    #[inline] pub fn eq_t(&self, value: T) -> Vec2<bool> { Vec2::from_slice(self.map(|&x| x == value)) }
    #[inline] pub fn ne_t(&self, value: T) -> Vec2<bool> { Vec2::from_slice(self.map(|&x| x != value)) }

    #[inline] pub fn eq_v(&self, other: &Vec2<T>) -> Vec2<bool> { Vec2::from_slice(self.zip(other, |&a, &b| a == b)) }
    #[inline] pub fn ne_v(&self, other: &Vec2<T>) -> Vec2<bool> { Vec2::from_slice(self.zip(other, |&a, &b| a != b)) }
}

impl Vec2<bool> {
    #[inline]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1)
    }

    #[inline]
    pub fn all(&self) -> bool {
        *self.index(0) && *self.index(1)
    }

    #[inline]
    pub fn not(&self) -> Vec2<bool> {
        Vec2::from_slice(self.map(|&x| !x))
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

impl<T> Vec3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x: x, y: y, z: z }
    }
}

impl<T:Clone> Vec3<T> {
    #[inline]
    pub fn from_value(value: T) -> Vec3<T> {
        Vec3::new(value.clone(), value.clone(), value.clone())
    }
}

impl<T:Clone + Num> Vec3<T> {
    #[inline] pub fn identity() -> Vec3<T> { Vec3::from_value(one!(T)) }
    #[inline] pub fn zero() -> Vec3<T> { Vec3::from_value(zero!(T)) }

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

    #[inline] pub fn add_t(&self, value: T) -> Vec3<T> { Vec3::from_slice(self.map(|&x| x + value)) }
    #[inline] pub fn sub_t(&self, value: T) -> Vec3<T> { Vec3::from_slice(self.map(|&x| x - value)) }
    #[inline] pub fn mul_t(&self, value: T) -> Vec3<T> { Vec3::from_slice(self.map(|&x| x * value)) }
    #[inline] pub fn div_t(&self, value: T) -> Vec3<T> { Vec3::from_slice(self.map(|&x| x / value)) }
    #[inline] pub fn rem_t(&self, value: T) -> Vec3<T> { Vec3::from_slice(self.map(|&x| x % value)) }

    #[inline] pub fn add_v(&self, other: &Vec3<T>) -> Vec3<T> { Vec3::from_slice(self.zip(other, |&a, &b| a + b)) }
    #[inline] pub fn sub_v(&self, other: &Vec3<T>) -> Vec3<T> { Vec3::from_slice(self.zip(other, |&a, &b| a - b)) }
    #[inline] pub fn mul_v(&self, other: &Vec3<T>) -> Vec3<T> { Vec3::from_slice(self.zip(other, |&a, &b| a * b)) }
    #[inline] pub fn div_v(&self, other: &Vec3<T>) -> Vec3<T> { Vec3::from_slice(self.zip(other, |&a, &b| a / b)) }
    #[inline] pub fn rem_v(&self, other: &Vec3<T>) -> Vec3<T> { Vec3::from_slice(self.zip(other, |&a, &b| a % b)) }

    #[inline] pub fn neg_self(&mut self) { self.map_mut(|x| *x = -*x) }
    #[inline] pub fn add_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x + value.clone()) }
    #[inline] pub fn sub_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x - value.clone()) }
    #[inline] pub fn mul_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x * value.clone()) }
    #[inline] pub fn div_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x / value.clone()) }
    #[inline] pub fn rem_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x % value.clone()) }

    #[inline] pub fn add_self_v(&mut self, other: &Vec3<T>) { self.zip_mut(other, |a, &b| *a =*a + b) }
    #[inline] pub fn sub_self_v(&mut self, other: &Vec3<T>) { self.zip_mut(other, |a, &b| *a =*a - b) }
    #[inline] pub fn mul_self_v(&mut self, other: &Vec3<T>) { self.zip_mut(other, |a, &b| *a =*a * b) }
    #[inline] pub fn div_self_v(&mut self, other: &Vec3<T>) { self.zip_mut(other, |a, &b| *a =*a / b) }
    #[inline] pub fn rem_self_v(&mut self, other: &Vec3<T>) { self.zip_mut(other, |a, &b| *a =*a % b) }

    #[inline] pub fn dot(&self, other: &Vec3<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1) +
        *self.index(2) * *other.index(2)
    }
}

impl<T:Clone + Num> Neg<Vec3<T>> for Vec3<T> {
    #[inline]
    pub fn neg(&self) -> Vec3<T> {
        Vec3::from_slice(self.map(|&x| -x))
    }
}

impl<T:Clone + Not<T>> Not<Vec3<T>> for Vec3<T> {
    pub fn not(&self) -> Vec3<T> {
        Vec3::from_slice(self.map(|&x| !x))
    }
}

impl<T:Clone + Real> Vec3<T> {
    #[inline]
    pub fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline]
    pub fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline]
    pub fn distance2(&self, other: &Vec3<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline]
    pub fn distance(&self, other: &Vec3<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline]
    pub fn angle(&self, other: &Vec3<T>) -> T {
        self.cross(other).length().atan2(&self.dot(other))
    }

    #[inline]
    pub fn normalize(&self) -> Vec3<T> {
        self.mul_t(one!(T)/self.length())
    }

    #[inline]
    pub fn normalize_to(&self, length: T) -> Vec3<T> {
        self.mul_t(length / self.length())
    }

    #[inline]
    pub fn lerp(&self, other: &Vec3<T>, amount: T) -> Vec3<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    #[inline]
    pub fn normalize_self(&mut self) {
        let rlen = self.length().recip();
        self.mul_self_t(rlen);
    }

    #[inline]
    pub fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    pub fn lerp_self(&mut self, other: &Vec3<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Clone + Ord> Vec3<T> {
    #[inline] pub fn lt_t(&self, value: T) -> Vec3<bool> { Vec3::from_slice(self.map(|&x| x < value)) }
    #[inline] pub fn le_t(&self, value: T) -> Vec3<bool> { Vec3::from_slice(self.map(|&x| x <= value)) }
    #[inline] pub fn ge_t(&self, value: T) -> Vec3<bool> { Vec3::from_slice(self.map(|&x| x >= value)) }
    #[inline] pub fn gt_t(&self, value: T) -> Vec3<bool> { Vec3::from_slice(self.map(|&x| x > value)) }

    #[inline] pub fn lt_v(&self, other: &Vec3<T>) -> Vec3<bool> { Vec3::from_slice(self.zip(other, |&a, &b| a < b)) }
    #[inline] pub fn le_v(&self, other: &Vec3<T>) -> Vec3<bool> { Vec3::from_slice(self.zip(other, |&a, &b| a <= b)) }
    #[inline] pub fn ge_v(&self, other: &Vec3<T>) -> Vec3<bool> { Vec3::from_slice(self.zip(other, |&a, &b| a >= b)) }
    #[inline] pub fn gt_v(&self, other: &Vec3<T>) -> Vec3<bool> { Vec3::from_slice(self.zip(other, |&a, &b| a > b)) }
}

impl<T:Clone + Eq> Vec3<T> {
    #[inline] pub fn eq_t(&self, value: T) -> Vec3<bool> { Vec3::from_slice(self.map(|&x| x == value)) }
    #[inline] pub fn ne_t(&self, value: T) -> Vec3<bool> { Vec3::from_slice(self.map(|&x| x != value)) }

    #[inline] pub fn eq_v(&self, other: &Vec3<T>) -> Vec3<bool> { Vec3::from_slice(self.zip(other, |&a, &b| a == b)) }
    #[inline] pub fn ne_v(&self, other: &Vec3<T>) -> Vec3<bool> { Vec3::from_slice(self.zip(other, |&a, &b| a != b)) }
}

impl Vec3<bool> {
    #[inline]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1) || *self.index(2)
    }

    #[inline]
    pub fn all(&self) -> bool {
        *self.index(0) && *self.index(1) && *self.index(2)
    }

    #[inline]
    pub fn not(&self) -> Vec3<bool> {
        Vec3::from_slice(self.map(|&x| !x))
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

impl<T> Vec4<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 { x: x, y: y, z: z, w: w }
    }
}

impl<T:Clone> Vec4<T> {
    #[inline]
    pub fn from_value(value: T) -> Vec4<T> {
        Vec4::new(value.clone(), value.clone(), value.clone(), value.clone())
    }
}

impl<T:Clone + Num> Vec4<T> {
    #[inline] pub fn identity() -> Vec4<T> { Vec4::from_value(one!(T)) }
    #[inline] pub fn zero() -> Vec4<T> { Vec4::from_value(zero!(T)) }

    #[inline] pub fn unit_x() -> Vec4<T> { Vec4::new(one!(T), zero!(T), zero!(T), zero!(T)) }
    #[inline] pub fn unit_y() -> Vec4<T> { Vec4::new(zero!(T), one!(T), zero!(T), zero!(T)) }
    #[inline] pub fn unit_z() -> Vec4<T> { Vec4::new(zero!(T), zero!(T), one!(T), zero!(T)) }
    #[inline] pub fn unit_w() -> Vec4<T> { Vec4::new(zero!(T), zero!(T), zero!(T), one!(T)) }

    #[inline] pub fn add_t(&self, value: T) -> Vec4<T> { Vec4::from_slice(self.map(|&x| x + value)) }
    #[inline] pub fn sub_t(&self, value: T) -> Vec4<T> { Vec4::from_slice(self.map(|&x| x - value)) }
    #[inline] pub fn mul_t(&self, value: T) -> Vec4<T> { Vec4::from_slice(self.map(|&x| x * value)) }
    #[inline] pub fn div_t(&self, value: T) -> Vec4<T> { Vec4::from_slice(self.map(|&x| x / value)) }
    #[inline] pub fn rem_t(&self, value: T) -> Vec4<T> { Vec4::from_slice(self.map(|&x| x % value)) }

    #[inline] pub fn add_v(&self, other: &Vec4<T>) -> Vec4<T> { Vec4::from_slice(self.zip(other, |&a, &b| a + b)) }
    #[inline] pub fn sub_v(&self, other: &Vec4<T>) -> Vec4<T> { Vec4::from_slice(self.zip(other, |&a, &b| a - b)) }
    #[inline] pub fn mul_v(&self, other: &Vec4<T>) -> Vec4<T> { Vec4::from_slice(self.zip(other, |&a, &b| a * b)) }
    #[inline] pub fn div_v(&self, other: &Vec4<T>) -> Vec4<T> { Vec4::from_slice(self.zip(other, |&a, &b| a / b)) }
    #[inline] pub fn rem_v(&self, other: &Vec4<T>) -> Vec4<T> { Vec4::from_slice(self.zip(other, |&a, &b| a % b)) }

    #[inline] pub fn neg_self(&mut self) { self.map_mut(|x| *x = -*x) }
    #[inline] pub fn add_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x + value.clone()) }
    #[inline] pub fn sub_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x - value.clone()) }
    #[inline] pub fn mul_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x * value.clone()) }
    #[inline] pub fn div_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x / value.clone()) }
    #[inline] pub fn rem_self_t(&mut self, value: T) { self.map_mut(|x| *x = *x % value.clone()) }

    #[inline] pub fn add_self_v(&mut self, other: &Vec4<T>) { self.zip_mut(other, |a, &b| *a =*a + b) }
    #[inline] pub fn sub_self_v(&mut self, other: &Vec4<T>) { self.zip_mut(other, |a, &b| *a =*a - b) }
    #[inline] pub fn mul_self_v(&mut self, other: &Vec4<T>) { self.zip_mut(other, |a, &b| *a =*a * b) }
    #[inline] pub fn div_self_v(&mut self, other: &Vec4<T>) { self.zip_mut(other, |a, &b| *a =*a / b) }
    #[inline] pub fn rem_self_v(&mut self, other: &Vec4<T>) { self.zip_mut(other, |a, &b| *a =*a % b) }

    #[inline] pub fn dot(&self, other: &Vec4<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1) +
        *self.index(2) * *other.index(2) +
        *self.index(3) * *other.index(3)
    }
}

impl<T:Clone + Num> Neg<Vec4<T>> for Vec4<T> {
    #[inline]
    pub fn neg(&self) -> Vec4<T> {
        Vec4::from_slice(self.map(|&x| -x))
    }
}

impl<T:Clone + Not<T>> Not<Vec4<T>> for Vec4<T> {
    pub fn not(&self) -> Vec4<T> {
        Vec4::from_slice(self.map(|&x| !x))
    }
}

impl<T:Clone + Real> Vec4<T> {
    #[inline]
    pub fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline]
    pub fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline]
    pub fn distance2(&self, other: &Vec4<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline]
    pub fn distance(&self, other: &Vec4<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline]
    pub fn angle(&self, other: &Vec4<T>) -> T {
        (self.dot(other) / (self.length() * other.length())).acos()
    }

    #[inline]
    pub fn normalize(&self) -> Vec4<T> {
        self.mul_t(one!(T)/self.length())
    }

    #[inline]
    pub fn normalize_to(&self, length: T) -> Vec4<T> {
        self.mul_t(length / self.length())
    }

    #[inline]
    pub fn lerp(&self, other: &Vec4<T>, amount: T) -> Vec4<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    #[inline]
    pub fn normalize_self(&mut self) {
        let rlen = self.length().recip();
        self.mul_self_t(rlen);
    }

    #[inline]
    pub fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    pub fn lerp_self(&mut self, other: &Vec4<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Clone + Ord> Vec4<T> {
    #[inline] pub fn lt_t(&self, value: T) -> Vec4<bool> { Vec4::from_slice(self.map(|&x| x < value)) }
    #[inline] pub fn le_t(&self, value: T) -> Vec4<bool> { Vec4::from_slice(self.map(|&x| x <= value)) }
    #[inline] pub fn ge_t(&self, value: T) -> Vec4<bool> { Vec4::from_slice(self.map(|&x| x >= value)) }
    #[inline] pub fn gt_t(&self, value: T) -> Vec4<bool> { Vec4::from_slice(self.map(|&x| x > value)) }

    #[inline] pub fn lt_v(&self, other: &Vec4<T>) -> Vec4<bool> { Vec4::from_slice(self.zip(other, |&a, &b| a < b)) }
    #[inline] pub fn le_v(&self, other: &Vec4<T>) -> Vec4<bool> { Vec4::from_slice(self.zip(other, |&a, &b| a <= b)) }
    #[inline] pub fn ge_v(&self, other: &Vec4<T>) -> Vec4<bool> { Vec4::from_slice(self.zip(other, |&a, &b| a >= b)) }
    #[inline] pub fn gt_v(&self, other: &Vec4<T>) -> Vec4<bool> { Vec4::from_slice(self.zip(other, |&a, &b| a > b)) }
}

impl<T:Clone + Eq> Vec4<T> {
    #[inline] pub fn eq_t(&self, value: T) -> Vec4<bool> { Vec4::from_slice(self.map(|&x| x == value)) }
    #[inline] pub fn ne_t(&self, value: T) -> Vec4<bool> { Vec4::from_slice(self.map(|&x| x != value)) }

    #[inline] pub fn eq_v(&self, other: &Vec4<T>) -> Vec4<bool> { Vec4::from_slice(self.zip(other, |&a, &b| a == b)) }
    #[inline] pub fn ne_v(&self, other: &Vec4<T>) -> Vec4<bool> { Vec4::from_slice(self.zip(other, |&a, &b| a != b)) }
}

impl Vec4<bool> {
    #[inline]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1) || *self.index(2) || *self.index(3)
    }

    #[inline]
    pub fn all(&self) -> bool {
        *self.index(0) && *self.index(1) && *self.index(2) && *self.index(3)
    }

    #[inline]
    pub fn not(&self) -> Vec4<bool> {
        Vec4::from_slice(self.map(|&x| !x))
    }
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
