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

use quat::{Quat, ToQuat};
use vec::{Vec2, Vec3, Vec4};

mod num_macros;
mod dim_macros;
mod mat_macros;

#[deriving(Eq)]
pub struct Mat2<T> {
    x: Vec2<T>,
    y: Vec2<T>,
}

// GLSL-style type aliases
pub type mat2  = Mat2<f32>;
pub type dmat2 = Mat2<f64>;

// Rust-style type aliases
pub type Mat2f   = Mat2<float>;
pub type Mat2f32 = Mat2<f32>;
pub type Mat2f64 = Mat2<f64>;

impl_dimensional!(Mat2, Vec2<T>, 2)
impl_dimensional_fns!(Mat2, Vec2<T>, 2)
impl_approx!(Mat2)

impl_mat!(Mat2, Vec2)
impl_mat_copyable!(Mat2, Vec2)
impl_mat_numeric!(Mat2, Vec2)
impl_mat_approx_numeric!(Mat2)
impl_mat_neg!(Mat2)

pub trait ToMat2<T> {
    pub fn to_mat2(&self) -> Mat2<T>;
}

impl<T> Mat2<T> {
    #[inline]
    pub fn new(c0r0: T, c0r1: T,
               c1r0: T, c1r1: T) -> Mat2<T> {
        Mat2::from_cols(Vec2::new(c0r0, c0r1),
                        Vec2::new(c1r0, c1r1))
    }

    #[inline]
    pub fn from_cols(c0: Vec2<T>,
                     c1: Vec2<T>) -> Mat2<T> {
        Mat2 { x: c0, y: c1 }
    }
}

impl<T:Copy + Num> ToMat3<T> for Mat2<T> {
    #[inline]
    pub fn to_mat3(&self) -> Mat3<T> {
        Mat3::new(copy *self.elem(0, 0), copy *self.elem(0, 1), zero!(T),
                  copy *self.elem(1, 0), copy *self.elem(1, 1), zero!(T),
                  zero!(T), zero!(T), one!(T))
    }
}

impl<T:Copy + Num> ToMat4<T> for Mat2<T> {
    #[inline]
    pub fn to_mat4(&self) -> Mat4<T> {
        Mat4::new(copy  *self.elem(0, 0), copy   *self.elem(0, 1), zero!(T), zero!(T),
                  copy  *self.elem(1, 0), copy   *self.elem(1, 1), zero!(T), zero!(T),
                  zero!(T), zero!(T), one!(T), zero!(T),
                  zero!(T), zero!(T), zero!(T), one!(T))
    }
}

impl<T:Copy + Real> Mat2<T> {
    #[inline]
    pub fn from_angle(radians: T) -> Mat2<T> {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat2::new(copy cos_theta,  copy -sin_theta,
                  copy sin_theta,  copy cos_theta)
    }
}

#[cfg(test)]
mod mat2_tests{
    use mat::*;
    use vec::*;

    #[test]
    fn test_mat2() {
        let a = Mat2 { x: Vec2 { x: 1.0, y: 3.0 },
                       y: Vec2 { x: 2.0, y: 4.0 } };
        let b = Mat2 { x: Vec2 { x: 2.0, y: 4.0 },
                       y: Vec2 { x: 3.0, y: 5.0 } };

        let v1 = Vec2::new::<float>(1.0, 2.0);
        let f1 = 0.5;

        assert_eq!(a, Mat2::new::<float>(1.0, 3.0,
                                         2.0, 4.0));

        assert_eq!(a, Mat2::from_cols::<float>(Vec2::new::<float>(1.0, 3.0),
                                               Vec2::new::<float>(2.0, 4.0)));

        assert_eq!(Mat2::from_value::<float>(4.0),
                   Mat2::new::<float>(4.0, 0.0,
                                      0.0, 4.0));

        assert_eq!(*a.col(0), Vec2::new::<float>(1.0, 3.0));
        assert_eq!(*a.col(1), Vec2::new::<float>(2.0, 4.0));

        assert_eq!(a.row(0), Vec2::new::<float>(1.0, 2.0));
        assert_eq!(a.row(1), Vec2::new::<float>(3.0, 4.0));

        assert_eq!(*a.col(0), Vec2::new::<float>(1.0, 3.0));
        assert_eq!(*a.col(1), Vec2::new::<float>(2.0, 4.0));

        assert_eq!(Mat2::identity::<float>(),
                   Mat2::new::<float>(1.0, 0.0,
                                      0.0, 1.0));

        assert_eq!(Mat2::zero::<float>(),
                   Mat2::new::<float>(0.0, 0.0,
                                      0.0, 0.0));

        assert_eq!(a.determinant(), -2.0);
        assert_eq!(a.trace(), 5.0);

        assert_eq!(a.neg(),
                   Mat2::new::<float>(-1.0, -3.0,
                                      -2.0, -4.0));
        assert_eq!(-a, a.neg());
        assert_eq!(a.mul_t(f1),
                   Mat2::new::<float>(0.5, 1.5,
                                      1.0, 2.0));
        assert_eq!(a.mul_v(&v1), Vec2::new::<float>(5.0, 11.0));
        assert_eq!(a.add_m(&b),
                   Mat2::new::<float>(3.0, 7.0,
                                      5.0, 9.0));
        assert_eq!(a.sub_m(&b),
                   Mat2::new::<float>(-1.0, -1.0,
                                      -1.0, -1.0));
        assert_eq!(a.mul_m(&b),
                   Mat2::new::<float>(10.0, 22.0,
                                      13.0, 29.0));
        assert_eq!(a.dot(&b), 40.0);

        assert_eq!(a.transpose(),
                   Mat2::new::<float>(1.0, 2.0,
                                      3.0, 4.0));

        assert_eq!(a.inverse().unwrap(),
                   Mat2::new::<float>(-2.0,  1.5,
                                      1.0, -0.5));

        assert!(Mat2::new::<float>(0.0, 2.0,
                                   0.0, 5.0).inverse().is_none());

        let ident = Mat2::identity::<float>();

        assert!(ident.is_identity());
        assert!(ident.is_symmetric());
        assert!(ident.is_diagonal());
        assert!(!ident.is_rotated());
        assert!(ident.is_invertible());

        assert!(!a.is_identity());
        assert!(!a.is_symmetric());
        assert!(!a.is_diagonal());
        assert!(a.is_rotated());
        assert!(a.is_invertible());

        let c = Mat2::new::<float>(2.0, 1.0,
                                   1.0, 2.0);
        assert!(!c.is_identity());
        assert!(c.is_symmetric());
        assert!(!c.is_diagonal());
        assert!(c.is_rotated());
        assert!(c.is_invertible());

        assert!(Mat2::from_value::<float>(6.0).is_diagonal());

        assert_eq!(a.to_mat3(),
                   Mat3::new::<float>(1.0, 3.0, 0.0,
                                      2.0, 4.0, 0.0,
                                      0.0, 0.0, 1.0));

        assert_eq!(a.to_mat4(),
                   Mat4::new::<float>(1.0, 3.0, 0.0, 0.0,
                                      2.0, 4.0, 0.0, 0.0,
                                      0.0, 0.0, 1.0, 0.0,
                                      0.0, 0.0, 0.0, 1.0));
    }

    fn test_mat2_mut() {
        let a = Mat2 { x: Vec2 { x: 1.0, y: 3.0 },
                       y: Vec2 { x: 2.0, y: 4.0 } };
        let b = Mat2 { x: Vec2 { x: 2.0, y: 4.0 },
                       y: Vec2 { x: 3.0, y: 5.0 } };

        let f1 = 0.5;

        let mut mut_a = a;

        mut_a.swap_cols(0, 1);
        assert_eq!(mut_a.col(0), a.col(1));
        assert_eq!(mut_a.col(1), a.col(0));
        mut_a = a;

        mut_a.swap_rows(0, 1);
        assert_eq!(mut_a.row(0), a.row(1));
        assert_eq!(mut_a.row(1), a.row(0));
        mut_a = a;

        mut_a.to_identity();
        assert!(mut_a.is_identity());
        mut_a = a;

        mut_a.to_zero();
        assert_eq!(mut_a, Mat2::zero::<float>());
        mut_a = a;

        mut_a.mul_self_t(f1);
        assert_eq!(mut_a, a.mul_t(f1));
        mut_a = a;

        mut_a.add_self_m(&b);
        assert_eq!(mut_a, a.add_m(&b));
        mut_a = a;

        mut_a.sub_self_m(&b);
        assert_eq!(mut_a, a.sub_m(&b));
        mut_a = a;

        mut_a.invert_self();
        assert_eq!(mut_a, a.inverse().unwrap());
        mut_a = a;

        mut_a.transpose_self();
        assert_eq!(mut_a, a.transpose());
        // mut_a = a;
    }

    #[test]
    fn test_mat2_approx_eq() {
        assert!(!Mat2::new::<float>(0.000001, 0.000001,
                                    0.000001, 0.000001).approx_eq(&Mat2::zero::<float>()));
        assert!(Mat2::new::<float>(0.0000001, 0.0000001,
                                   0.0000001, 0.0000001).approx_eq(&Mat2::zero::<float>()));
    }
}

#[deriving(Eq)]
pub struct Mat3<T> {
    x: Vec3<T>,
    y: Vec3<T>,
    z: Vec3<T>,
}

// GLSL-style type aliases
pub type mat3  = Mat3<f32>;
pub type dmat3 = Mat3<f64>;

// Rust-style type aliases
pub type Mat3f   = Mat3<float>;
pub type Mat3f32 = Mat3<f32>;
pub type Mat3f64 = Mat3<f64>;

impl_dimensional!(Mat3, Vec3<T>, 3)
impl_dimensional_fns!(Mat3, Vec3<T>, 3)
impl_approx!(Mat3)

impl_mat!(Mat3, Vec3)
impl_mat_copyable!(Mat3, Vec3)
impl_mat_numeric!(Mat3, Vec3)
impl_mat_approx_numeric!(Mat3)
impl_mat_neg!(Mat3)

pub trait ToMat3<T> {
    pub fn to_mat3(&self) -> Mat3<T>;
}

impl<T> Mat3<T> {
    #[inline]
    pub fn new(c0r0:T, c0r1:T, c0r2:T,
               c1r0:T, c1r1:T, c1r2:T,
               c2r0:T, c2r1:T, c2r2:T) -> Mat3<T> {
        Mat3::from_cols(Vec3::new(c0r0, c0r1, c0r2),
                        Vec3::new(c1r0, c1r1, c1r2),
                        Vec3::new(c2r0, c2r1, c2r2))
    }

    #[inline]
    pub fn from_cols(c0: Vec3<T>,
                     c1: Vec3<T>,
                     c2: Vec3<T>) -> Mat3<T> {
        Mat3 { x: c0, y: c1, z: c2 }
    }
}

impl<T:Copy + Num> ToMat4<T> for Mat3<T> {
    #[inline]
    pub fn to_mat4(&self) -> Mat4<T> {
        Mat4::new(copy *self.elem(0, 0), copy *self.elem(0, 1), copy *self.elem(0, 2), zero!(T),
                  copy *self.elem(1, 0), copy *self.elem(1, 1), copy *self.elem(1, 2), zero!(T),
                  copy *self.elem(2, 0), copy *self.elem(2, 1), copy *self.elem(2, 2), zero!(T),
                  zero!(T), zero!(T), zero!(T), one!(T))
    }
}

impl<T:Copy + Real> Mat3<T> {
    /// Construct a matrix from an angular rotation around the `x` axis
    pub fn from_angle_x(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat3::new(one!(T), zero!(T), zero!(T),
                  zero!(T), copy cos_theta, copy sin_theta,
                  zero!(T), copy -sin_theta, copy cos_theta)
    }

    /// Construct a matrix from an angular rotation around the `y` axis
    pub fn from_angle_y(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat3::new(copy cos_theta, zero!(T), copy -sin_theta,
                  zero!(T), one!(T), zero!(T),
                  copy sin_theta, zero!(T), copy cos_theta)
    }

    /// Construct a matrix from an angular rotation around the `z` axis
    pub fn from_angle_z(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        Mat3::new(copy cos_theta, copy sin_theta, zero!(T),
                  copy -sin_theta, copy cos_theta, zero!(T),
                  zero!(T), zero!(T), one!(T))
    }

    /// Construct a matrix from Euler angles
    ///
    /// # Arguments
    ///
    /// - `theta_x`: the angular rotation around the `x` axis (pitch)
    /// - `theta_y`: the angular rotation around the `y` axis (yaw)
    /// - `theta_z`: the angular rotation around the `z` axis (roll)
    pub fn from_angle_xyz(radians_x: T, radians_y: T, radians_z: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
        let cx = radians_x.cos();
        let sx = radians_x.sin();
        let cy = radians_y.cos();
        let sy = radians_y.sin();
        let cz = radians_z.cos();
        let sz = radians_z.sin();

        Mat3::new(cy*cz, cy*sz, -sy,
                  -cx*sz + sx*sy*cz, cx*cz + sx*sy*sz, sx*cy,
                  sx*sz + cx*sy*cz, -sx*cz + cx*sy*sz, cx*cy)
    }

    /// Construct a matrix from an axis and an angular rotation
    pub fn from_angle_axis(radians: T, axis: &Vec3<T>) -> Mat3<T> {
        let c = radians.cos();
        let s = radians.sin();
        let _1_c = one!(T) - c;

        let x = copy axis.x;
        let y = copy axis.y;
        let z = copy axis.z;

        Mat3::new(_1_c*x*x + c, _1_c*x*y + s*z, _1_c*x*z - s*y,
                  _1_c*x*y - s*z, _1_c*y*y + c, _1_c*y*z + s*x,
                  _1_c*x*z + s*y, _1_c*y*z - s*x, _1_c*z*z + c)
    }

    #[inline]
    pub fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat3<T> {
        Mat3::from_cols(x, y, z)
    }

    pub fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> Mat3<T> {
        let dir_ = dir.normalize();
        let side = dir_.cross(&up.normalize());
        let up_  = side.cross(&dir_).normalize();

        Mat3::from_axes(up_, side, dir_)
    }
}

impl<T:Copy + Real> ToQuat<T> for Mat3<T> {
    /// Convert the matrix to a quaternion
    pub fn to_quat(&self) -> Quat<T> {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf

        let mut s;
        let w; let x; let y; let z;
        let trace = self.trace();

        // FIXME: We don't have any numeric conversions in std yet :P
        let half = one!(T) / two!(T);

        cond! (
            (trace >= zero!(T)) {
                s = (one!(T) + trace).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.elem(1, 2) - *self.elem(2, 1)) * s;
                y = (*self.elem(2, 0) - *self.elem(0, 2)) * s;
                z = (*self.elem(0, 1) - *self.elem(1, 0)) * s;
            }
            ((*self.elem(0, 0) > *self.elem(1, 1))
            && (*self.elem(0, 0) > *self.elem(2, 2))) {
                s = (half + (*self.elem(0, 0) - *self.elem(1, 1) - *self.elem(2, 2))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.elem(0, 1) - *self.elem(1, 0)) * s;
                y = (*self.elem(2, 0) - *self.elem(0, 2)) * s;
                z = (*self.elem(1, 2) - *self.elem(2, 1)) * s;
            }
            (*self.elem(1, 1) > *self.elem(2, 2)) {
                s = (half + (*self.elem(1, 1) - *self.elem(0, 0) - *self.elem(2, 2))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.elem(0, 1) - *self.elem(1, 0)) * s;
                y = (*self.elem(1, 2) - *self.elem(2, 1)) * s;
                z = (*self.elem(2, 0) - *self.elem(0, 2)) * s;
            }
            _ {
                s = (half + (*self.elem(2, 2) - *self.elem(0, 0) - *self.elem(1, 1))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.elem(2, 0) - *self.elem(0, 2)) * s;
                y = (*self.elem(1, 2) - *self.elem(2, 1)) * s;
                z = (*self.elem(0, 1) - *self.elem(1, 0)) * s;
            }
        )
        Quat::new(w, x, y, z)
    }
}

#[cfg(test)]
mod mat3_tests{
    use mat::*;
    use vec::*;

    #[test]
    fn test_mat3() {
        let a = Mat3 { x: Vec3 { x: 1.0, y: 4.0, z:  7.0 },
                       y: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                       z: Vec3 { x: 3.0, y: 6.0, z:  9.0 } };
        let b = Mat3 { x: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                       y: Vec3 { x: 3.0, y: 6.0, z:  9.0 },
                       z: Vec3 { x: 4.0, y: 7.0, z: 10.0 } };

        let v1 = Vec3::new::<float>(1.0, 2.0, 3.0);
        let f1 = 0.5;

        assert_eq!(a, Mat3::new::<float>(1.0, 4.0, 7.0,
                                         2.0, 5.0, 8.0,
                                         3.0, 6.0, 9.0));

        assert_eq!(a, Mat3::from_cols::<float>(Vec3::new::<float>(1.0, 4.0, 7.0),
                                               Vec3::new::<float>(2.0, 5.0, 8.0),
                                               Vec3::new::<float>(3.0, 6.0, 9.0)));

        assert_eq!(*a.col(0), Vec3::new::<float>(1.0, 4.0, 7.0));
        assert_eq!(*a.col(1), Vec3::new::<float>(2.0, 5.0, 8.0));
        assert_eq!(*a.col(2), Vec3::new::<float>(3.0, 6.0, 9.0));

        assert_eq!(a.row(0), Vec3::new::<float>(1.0, 2.0, 3.0));
        assert_eq!(a.row(1), Vec3::new::<float>(4.0, 5.0, 6.0));
        assert_eq!(a.row(2), Vec3::new::<float>(7.0, 8.0, 9.0));

        assert_eq!(*a.col(0), Vec3::new::<float>(1.0, 4.0, 7.0));
        assert_eq!(*a.col(1), Vec3::new::<float>(2.0, 5.0, 8.0));
        assert_eq!(*a.col(2), Vec3::new::<float>(3.0, 6.0, 9.0));

        assert_eq!(Mat3::identity::<float>(),
                   Mat3::new::<float>(1.0, 0.0, 0.0,
                                      0.0, 1.0, 0.0,
                                      0.0, 0.0, 1.0));

        assert_eq!(Mat3::zero::<float>(),
                   Mat3::new::<float>(0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0));

        assert_eq!(a.determinant(), 0.0);
        assert_eq!(a.trace(), 15.0);

        assert_eq!(a.neg(),
                   Mat3::new::<float>(-1.0, -4.0, -7.0,
                                      -2.0, -5.0, -8.0,
                                      -3.0, -6.0, -9.0));
        assert_eq!(-a, a.neg());

        assert_eq!(a.mul_t(f1),
                   Mat3::new::<float>(0.5, 2.0, 3.5,
                                      1.0, 2.5, 4.0,
                                      1.5, 3.0, 4.5));
        assert_eq!(a.mul_v(&v1), Vec3::new::<float>(14.0, 32.0, 50.0));

        assert_eq!(a.add_m(&b),
                   Mat3::new::<float>(3.0,  9.0, 15.0,
                                      5.0, 11.0, 17.0,
                                      7.0, 13.0, 19.0));
        assert_eq!(a.sub_m(&b),
                   Mat3::new::<float>(-1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0));
        assert_eq!(a.mul_m(&b),
                   Mat3::new::<float>(36.0,  81.0, 126.0,
                                      42.0,  96.0, 150.0,
                                      48.0, 111.0, 174.0));
        assert_eq!(a.dot(&b), 330.0);

        assert_eq!(a.transpose(),
                   Mat3::new::<float>(1.0, 2.0, 3.0,
                                      4.0, 5.0, 6.0,
                                      7.0, 8.0, 9.0));

        assert!(a.inverse().is_none());

        assert_eq!(Mat3::new::<float>(2.0, 4.0, 6.0,
                                      0.0, 2.0, 4.0,
                                      0.0, 0.0, 1.0).inverse().unwrap(),
                   Mat3::new::<float>(0.5, -1.0,  1.0,
                                      0.0,  0.5, -2.0,
                                      0.0,  0.0,  1.0));

        let ident = Mat3::identity::<float>();

        assert_eq!(ident.inverse().unwrap(), ident);

        assert!(ident.is_identity());
        assert!(ident.is_symmetric());
        assert!(ident.is_diagonal());
        assert!(!ident.is_rotated());
        assert!(ident.is_invertible());

        assert!(!a.is_identity());
        assert!(!a.is_symmetric());
        assert!(!a.is_diagonal());
        assert!(a.is_rotated());
        assert!(!a.is_invertible());

        let c = Mat3::new::<float>(3.0, 2.0, 1.0,
                                   2.0, 3.0, 2.0,
                                   1.0, 2.0, 3.0);
        assert!(!c.is_identity());
        assert!(c.is_symmetric());
        assert!(!c.is_diagonal());
        assert!(c.is_rotated());
        assert!(c.is_invertible());

        assert!(Mat3::from_value::<float>(6.0).is_diagonal());

        assert_eq!(a.to_mat4(),
                   Mat4::new::<float>(1.0, 4.0, 7.0, 0.0,
                                      2.0, 5.0, 8.0, 0.0,
                                      3.0, 6.0, 9.0, 0.0,
                                      0.0, 0.0, 0.0, 1.0));

        // to_Quaternion
    }

    fn test_mat3_mut() {
        let a = Mat3 { x: Vec3 { x: 1.0, y: 4.0, z:  7.0 },
                       y: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                       z: Vec3 { x: 3.0, y: 6.0, z:  9.0 } };
        let b = Mat3 { x: Vec3 { x: 2.0, y: 5.0, z:  8.0 },
                       y: Vec3 { x: 3.0, y: 6.0, z:  9.0 },
                       z: Vec3 { x: 4.0, y: 7.0, z: 10.0 } };
        let c = Mat3 { x: Vec3 { x: 2.0, y: 4.0, z:  6.0 },
                       y: Vec3 { x: 0.0, y: 2.0, z:  4.0 },
                       z: Vec3 { x: 0.0, y: 0.0, z:  1.0 } };

        let f1 = 0.5;

        let mut mut_a = a;
        let mut mut_c = c;

        mut_a.swap_cols(0, 2);
        assert_eq!(mut_a.col(0), a.col(2));
        assert_eq!(mut_a.col(2), a.col(0));
        mut_a = a;

        mut_a.swap_cols(1, 2);
        assert_eq!(mut_a.col(1), a.col(2));
        assert_eq!(mut_a.col(2), a.col(1));
        mut_a = a;

        mut_a.swap_rows(0, 2);
        assert_eq!(mut_a.row(0), a.row(2));
        assert_eq!(mut_a.row(2), a.row(0));
        mut_a = a;

        mut_a.swap_rows(1, 2);
        assert_eq!(mut_a.row(1), a.row(2));
        assert_eq!(mut_a.row(2), a.row(1));
        mut_a = a;

        mut_a.to_identity();
        assert!(mut_a.is_identity());
        mut_a = a;

        mut_a.to_zero();
        assert_eq!(mut_a, Mat3::zero::<float>());
        mut_a = a;

        mut_a.mul_self_t(f1);
        assert_eq!(mut_a, a.mul_t(f1));
        mut_a = a;

        mut_a.add_self_m(&b);
        assert_eq!(mut_a, a.add_m(&b));
        mut_a = a;

        mut_a.sub_self_m(&b);
        assert_eq!(mut_a, a.sub_m(&b));
        mut_a = a;

        mut_c.invert_self();
        assert_eq!(mut_c, c.inverse().unwrap());
        // mut_c = c;

        mut_a.transpose_self();
        assert_eq!(mut_a, a.transpose());
        // mut_a = a;
    }

    #[test]
    fn test_mat3_approx_eq() {
        assert!(!Mat3::new::<float>(0.000001, 0.000001, 0.000001,
                                    0.000001, 0.000001, 0.000001,
                                    0.000001, 0.000001, 0.000001)
                .approx_eq(&Mat3::zero::<float>()));
        assert!(Mat3::new::<float>(0.0000001, 0.0000001, 0.0000001,
                                    0.0000001, 0.0000001, 0.0000001,
                                    0.0000001, 0.0000001, 0.0000001)
                .approx_eq(&Mat3::zero::<float>()));
    }
}

#[deriving(Eq)]
pub struct Mat4<T> {
    x: Vec4<T>,
    y: Vec4<T>,
    z: Vec4<T>,
    w: Vec4<T>,
}

// GLSL-style type aliases
pub type mat4  = Mat4<f32>;
pub type dmat4 = Mat4<f64>;

// Rust-style type aliases
pub type Mat4f   = Mat4<float>;
pub type Mat4f32 = Mat4<f32>;
pub type Mat4f64 = Mat4<f64>;

impl_dimensional!(Mat4, Vec4<T>, 4)
impl_dimensional_fns!(Mat4, Vec4<T>, 4)
impl_approx!(Mat4)

impl_mat!(Mat4, Vec4)
impl_mat_copyable!(Mat4, Vec4)
impl_mat_numeric!(Mat4, Vec4)
impl_mat_approx_numeric!(Mat4)
impl_mat_neg!(Mat4)

pub trait ToMat4<T> {
    pub fn to_mat4(&self) -> Mat4<T>;
}

impl<T> Mat4<T> {
    #[inline]
    pub fn new(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
               c1r0: T, c1r1: T, c1r2: T, c1r3: T,
               c2r0: T, c2r1: T, c2r2: T, c2r3: T,
               c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> Mat4<T>  {
        Mat4::from_cols(Vec4::new(c0r0, c0r1, c0r2, c0r3),
                        Vec4::new(c1r0, c1r1, c1r2, c1r3),
                        Vec4::new(c2r0, c2r1, c2r2, c2r3),
                        Vec4::new(c3r0, c3r1, c3r2, c3r3))
    }

    #[inline]
    pub fn from_cols(c0: Vec4<T>,
                     c1: Vec4<T>,
                     c2: Vec4<T>,
                     c3: Vec4<T>) -> Mat4<T> {
        Mat4 { x: c0, y: c1, z: c2, w: c3 }
    }
}

#[cfg(test)]
mod mat4_tests {
    use mat::*;
    use vec::*;

    #[test]
    fn test_mat4() {
        let a = Mat4 { x: Vec4 { x: 1.0, y: 5.0, z:  9.0, w: 13.0 },
                       y: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                       z: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                       w: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 } };
        let b = Mat4 { x: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                       y: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                       z: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 },
                       w: Vec4 { x: 5.0, y: 9.0, z: 13.0, w: 17.0 } };
        let c = Mat4 { x: Vec4 { x: 3.0, y: 2.0, z:  1.0, w:  1.0 },
                       y: Vec4 { x: 2.0, y: 3.0, z:  2.0, w:  2.0 },
                       z: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  3.0 },
                       w: Vec4 { x: 0.0, y: 1.0, z:  1.0, w:  0.0 } };

        let v1 = Vec4::new::<float>(1.0, 2.0, 3.0, 4.0);
        let f1 = 0.5;

        assert_eq!(a, Mat4::new::<float>(1.0, 5.0,  9.0, 13.0,
                                         2.0, 6.0, 10.0, 14.0,
                                         3.0, 7.0, 11.0, 15.0,
                                         4.0, 8.0, 12.0, 16.0));

        assert_eq!(a, Mat4::from_cols::<float>(Vec4::new::<float>(1.0, 5.0,  9.0, 13.0),
                                               Vec4::new::<float>(2.0, 6.0, 10.0, 14.0),
                                               Vec4::new::<float>(3.0, 7.0, 11.0, 15.0),
                                               Vec4::new::<float>(4.0, 8.0, 12.0, 16.0)));

        assert_eq!(Mat4::from_value::<float>(4.0),
                   Mat4::new::<float>(4.0, 0.0, 0.0, 0.0,
                                      0.0, 4.0, 0.0, 0.0,
                                      0.0, 0.0, 4.0, 0.0,
                                      0.0, 0.0, 0.0, 4.0));

        assert_eq!(*a.col(0), Vec4::new::<float>(1.0, 5.0,  9.0, 13.0));
        assert_eq!(*a.col(1), Vec4::new::<float>(2.0, 6.0, 10.0, 14.0));
        assert_eq!(*a.col(2), Vec4::new::<float>(3.0, 7.0, 11.0, 15.0));
        assert_eq!(*a.col(3), Vec4::new::<float>(4.0, 8.0, 12.0, 16.0));

        assert_eq!(a.row(0), Vec4::new::<float>( 1.0,  2.0,  3.0,  4.0));
        assert_eq!(a.row(1), Vec4::new::<float>( 5.0,  6.0,  7.0,  8.0));
        assert_eq!(a.row(2), Vec4::new::<float>( 9.0, 10.0, 11.0, 12.0));
        assert_eq!(a.row(3), Vec4::new::<float>(13.0, 14.0, 15.0, 16.0));

        assert_eq!(*a.col(0), Vec4::new::<float>(1.0, 5.0,  9.0, 13.0));
        assert_eq!(*a.col(1), Vec4::new::<float>(2.0, 6.0, 10.0, 14.0));
        assert_eq!(*a.col(2), Vec4::new::<float>(3.0, 7.0, 11.0, 15.0));
        assert_eq!(*a.col(3), Vec4::new::<float>(4.0, 8.0, 12.0, 16.0));

        assert_eq!(Mat4::identity::<float>(),
                   Mat4::new::<float>(1.0, 0.0, 0.0, 0.0,
                                      0.0, 1.0, 0.0, 0.0,
                                      0.0, 0.0, 1.0, 0.0,
                                      0.0, 0.0, 0.0, 1.0));

        assert_eq!(Mat4::zero::<float>(),
                   Mat4::new::<float>(0.0, 0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0, 0.0));

        assert_eq!(a.determinant(), 0.0);
        assert_eq!(a.trace(), 34.0);

        assert_eq!(a.neg(),
                   Mat4::new::<float>(-1.0, -5.0,  -9.0, -13.0,
                                      -2.0, -6.0, -10.0, -14.0,
                                      -3.0, -7.0, -11.0, -15.0,
                                      -4.0, -8.0, -12.0, -16.0));
        assert_eq!(-a, a.neg());
        assert_eq!(a.mul_t(f1),
                   Mat4::new::<float>(0.5, 2.5, 4.5, 6.5,
                                      1.0, 3.0, 5.0, 7.0,
                                      1.5, 3.5, 5.5, 7.5,
                                      2.0, 4.0, 6.0, 8.0));
        assert_eq!(a.mul_v(&v1),
                   Vec4::new::<float>(30.0, 70.0, 110.0, 150.0));
        assert_eq!(a.add_m(&b),
                   Mat4::new::<float>(3.0, 11.0, 19.0, 27.0,
                                      5.0, 13.0, 21.0, 29.0,
                                      7.0, 15.0, 23.0, 31.0,
                                      9.0, 17.0, 25.0, 33.0));
        assert_eq!(a.sub_m(&b),
                   Mat4::new::<float>(-1.0, -1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0, -1.0,
                                      -1.0, -1.0, -1.0, -1.0));
        assert_eq!(a.mul_m(&b),
                   Mat4::new::<float>(100.0, 228.0, 356.0, 484.0,
                                      110.0, 254.0, 398.0, 542.0,
                                      120.0, 280.0, 440.0, 600.0,
                                      130.0, 306.0, 482.0, 658.0));
        assert_eq!(a.dot(&b), 1632.0);
        assert_eq!(a.transpose(),
                   Mat4::new::<float>( 1.0,  2.0,  3.0,  4.0,
                                       5.0,  6.0,  7.0,  8.0,
                                       9.0, 10.0, 11.0, 12.0,
                                      13.0, 14.0, 15.0, 16.0));

        assert_approx_eq!(c.inverse().unwrap(),
                          Mat4::new::<float>( 5.0, -4.0,  1.0,  0.0,
                                             -4.0,  8.0, -4.0,  0.0,
                                              4.0, -8.0,  4.0,  8.0,
                                             -3.0,  4.0,  1.0, -8.0).mul_t(0.125));

        let ident = Mat4::identity::<float>();

        assert_eq!(ident.inverse().unwrap(), ident);

        assert!(ident.is_identity());
        assert!(ident.is_symmetric());
        assert!(ident.is_diagonal());
        assert!(!ident.is_rotated());
        assert!(ident.is_invertible());

        assert!(!a.is_identity());
        assert!(!a.is_symmetric());
        assert!(!a.is_diagonal());
        assert!(a.is_rotated());
        assert!(!a.is_invertible());

        let c = Mat4::new::<float>(4.0, 3.0, 2.0, 1.0,
                                   3.0, 4.0, 3.0, 2.0,
                                   2.0, 3.0, 4.0, 3.0,
                                   1.0, 2.0, 3.0, 4.0);
        assert!(!c.is_identity());
        assert!(c.is_symmetric());
        assert!(!c.is_diagonal());
        assert!(c.is_rotated());
        assert!(c.is_invertible());

        assert!(Mat4::from_value::<float>(6.0).is_diagonal());
    }

    fn test_mat4_mut() {
        let a = Mat4 { x: Vec4 { x: 1.0, y: 5.0, z:  9.0, w: 13.0 },
                       y: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                       z: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                       w: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 } };
        let b = Mat4 { x: Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
                       y: Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
                       z: Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 },
                       w: Vec4 { x: 5.0, y: 9.0, z: 13.0, w: 17.0 } };
        let c = Mat4 { x: Vec4 { x: 3.0, y: 2.0, z:  1.0, w:  1.0 },
                       y: Vec4 { x: 2.0, y: 3.0, z:  2.0, w:  2.0 },
                       z: Vec4 { x: 1.0, y: 2.0, z:  3.0, w:  3.0 },
                       w: Vec4 { x: 0.0, y: 1.0, z:  1.0, w:  0.0 } };

        let f1 = 0.5;

        let mut mut_a = a;
        let mut mut_c = c;

        mut_a.swap_cols(0, 3);
        assert_eq!(mut_a.col(0), a.col(3));
        assert_eq!(mut_a.col(3), a.col(0));
        mut_a = a;

        mut_a.swap_cols(1, 2);
        assert_eq!(mut_a.col(1), a.col(2));
        assert_eq!(mut_a.col(2), a.col(1));
        mut_a = a;

        mut_a.swap_rows(0, 3);
        assert_eq!(mut_a.row(0), a.row(3));
        assert_eq!(mut_a.row(3), a.row(0));
        mut_a = a;

        mut_a.swap_rows(1, 2);
        assert_eq!(mut_a.row(1), a.row(2));
        assert_eq!(mut_a.row(2), a.row(1));
        mut_a = a;

        mut_a.to_identity();
        assert!(mut_a.is_identity());
        mut_a = a;

        mut_a.to_zero();
        assert_eq!(mut_a, Mat4::zero::<float>());
        mut_a = a;

        mut_a.mul_self_t(f1);
        assert_eq!(mut_a, a.mul_t(f1));
        mut_a = a;

        mut_a.add_self_m(&b);
        assert_eq!(mut_a, a.add_m(&b));
        mut_a = a;

        mut_a.sub_self_m(&b);
        assert_eq!(mut_a, a.sub_m(&b));
        mut_a = a;

        mut_c.invert_self();
        assert_eq!(mut_c, c.inverse().unwrap());
        // mut_c = c;

        mut_a.transpose_self();
        assert_eq!(mut_a, a.transpose());
        // mut_a = a;
    }

    #[test]
    fn test_mat4_approx_eq() {
        assert!(!Mat4::new::<float>(0.000001, 0.000001, 0.000001, 0.000001,
                                    0.000001, 0.000001, 0.000001, 0.000001,
                                    0.000001, 0.000001, 0.000001, 0.000001,
                                    0.000001, 0.000001, 0.000001, 0.000001)
                .approx_eq(&Mat4::zero::<float>()));
        assert!(Mat4::new::<float>(0.0000001, 0.0000001, 0.0000001, 0.0000001,
                                   0.0000001, 0.0000001, 0.0000001, 0.0000001,
                                   0.0000001, 0.0000001, 0.0000001, 0.0000001,
                                   0.0000001, 0.0000001, 0.0000001, 0.0000001)
                .approx_eq(&Mat4::zero::<float>()));
    }
}
