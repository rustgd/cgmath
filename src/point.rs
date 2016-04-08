// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
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

//! Points are fixed positions in affine space with no length or direction. This
//! disinguishes them from vectors, which have a length and direction, but do
//! not have a fixed position.

use std::fmt;
use std::mem;
use std::ops::*;

use rust_num::{One, Zero};

use approx::ApproxEq;
use array::Array;
use matrix::Matrix;
use num::{BaseNum, BaseFloat};
use vector::*;

/// A point in 2-dimensional space.
///
/// This type is marked as `#[repr(C, packed)]`.
#[repr(C, packed)]
#[derive(PartialEq, Eq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
pub struct Point2<S> {
    pub x: S,
    pub y: S,
}

/// A point in 3-dimensional space.
///
/// This type is marked as `#[repr(C, packed)]`.
#[repr(C, packed)]
#[derive(PartialEq, Eq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
pub struct Point3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}


impl<S: BaseNum> Point2<S> {
    #[inline]
    pub fn new(x: S, y: S) -> Point2<S> {
        Point2 { x: x, y: y }
    }
}

impl<S: BaseNum> Point3<S> {
    #[inline]
    pub fn new(x: S, y: S, z: S) -> Point3<S> {
        Point3 { x: x, y: y, z: z }
    }
}

impl<S: BaseNum> Point3<S> {
    #[inline]
    pub fn from_homogeneous(v: Vector4<S>) -> Point3<S> {
        let e = v.truncate() * (S::one() / v.w);
        Point3::new(e.x, e.y, e.z)  //FIXME
    }

    #[inline]
    pub fn to_homogeneous(self) -> Vector4<S> {
        Vector4::new(self.x, self.y, self.z, S::one())
    }
}

/// Points in a [Euclidean space](https://en.wikipedia.org/wiki/Euclidean_space)
/// with an associated space of displacement vectors.
///
/// # Point-Vector distinction
///
/// `cgmath` distinguishes between points and vectors in the following way:
///
/// - Points are _locations_ relative to an origin
/// - Vectors are _displacements_ between those points
///
/// For example, to find the midpoint between two points, you can write the
/// following:
///
/// ```rust
/// use cgmath::Point3;
///
/// let p0 = Point3::new(1.0, 2.0, 3.0);
/// let p1 = Point3::new(-3.0, 1.0, 2.0);
/// let midpoint: Point3<f32> = p0 + (p1 - p0) * 0.5;
/// ```
///
/// Breaking the expression up, and adding explicit types makes it clearer
/// to see what is going on:
///
/// ```rust
/// # use cgmath::{Point3, Vector3};
/// #
/// # let p0 = Point3::new(1.0, 2.0, 3.0);
/// # let p1 = Point3::new(-3.0, 1.0, 2.0);
/// #
/// let dv: Vector3<f32> = p1 - p0;
/// let half_dv: Vector3<f32> = dv * 0.5;
/// let midpoint: Point3<f32> = p0 + half_dv;
/// ```
///
/// ## Converting between points and vectors
///
/// Points can be converted to and from displacement vectors using the
/// `EuclideanSpace::{from_vec, to_vec}` methods. Note that under the hood these
/// are implemented as inlined a type conversion, so should not have any
/// performance implications.
///
/// ## References
///
/// - [CGAL 4.7 - 2D and 3D Linear Geometry Kernel: 3.1 Points and Vectors](http://doc.cgal.org/latest/Kernel_23/index.html#Kernel_23PointsandVectors)
/// - [What is the difference between a point and a vector](http://math.stackexchange.com/q/645827)
///
pub trait EuclideanSpace: Copy + Clone where
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: Array<Element = <Self as EuclideanSpace>::Scalar>,

    Self: Add<<Self as EuclideanSpace>::Diff, Output = Self>,
    Self: Sub<Self, Output = <Self as EuclideanSpace>::Diff>,

    Self: Mul<<Self as EuclideanSpace>::Scalar, Output = Self>,
    Self: Div<<Self as EuclideanSpace>::Scalar, Output = Self>,
    Self: Rem<<Self as EuclideanSpace>::Scalar, Output = Self>,
{
    /// The associated scalar over which the space is defined.
    ///
    /// Due to the equality constraints demanded by `Self::Diff`, this is effectively just an
    /// alias to `Self::Diff::Scalar`.
    type Scalar: BaseNum;

    /// The associated space of displacement vectors.
    type Diff: VectorSpace<Scalar = Self::Scalar>;

    /// The point at the origin of the Euclidean space.
    fn origin() -> Self;

    /// Convert a displacement vector to a point.
    ///
    /// This can be considered equivalent to the addition of the displacement
    /// vector `v` to to `Self::origin()`.
    fn from_vec(v: Self::Diff) -> Self;

    /// Convert a point to a displacement vector.
    ///
    /// This can be seen as equivalent to the displacement vector from
    /// `Self::origin()` to `self`.
    fn to_vec(self) -> Self::Diff;

    /// This is a weird one, but its useful for plane calculations.
    fn dot(self, v: Self::Diff) -> Self::Scalar;
}

macro_rules! impl_point {
    ($PointN:ident { $($field:ident),+ }, $VectorN:ident, $n:expr) => {
        impl<S: BaseNum> Array for $PointN<S> {
            type Element = S;

            #[inline]
            fn from_value(scalar: S) -> $PointN<S> {
                $PointN { $($field: scalar),+ }
            }

            #[inline]
            fn sum(self) -> S where S: Add<Output = S> {
                fold_array!(add, { $(self.$field),+ })
            }

            #[inline]
            fn product(self) -> S where S: Mul<Output = S> {
                fold_array!(mul, { $(self.$field),+ })
            }

            #[inline]
            fn min(self) -> S where S: PartialOrd {
                fold_array!(partial_min, { $(self.$field),+ })
            }

            #[inline]
            fn max(self) -> S where S: PartialOrd {
                fold_array!(partial_max, { $(self.$field),+ })
            }
        }

        impl<S: BaseNum> EuclideanSpace for $PointN<S> {
            type Scalar = S;
            type Diff = $VectorN<S>;

            #[inline]
            fn origin() -> $PointN<S> {
                $PointN { $($field: S::zero()),+ }
            }

            #[inline]
            fn from_vec(v: $VectorN<S>) -> $PointN<S> {
                $PointN::new($(v.$field),+)
            }

            #[inline]
            fn to_vec(self) -> $VectorN<S> {
                $VectorN::new($(self.$field),+)
            }

            #[inline]
            fn dot(self, v: $VectorN<S>) -> S {
                $VectorN::new($(self.$field * v.$field),+).sum()
            }
        }

        impl<S: BaseFloat> ApproxEq for $PointN<S> {
            type Epsilon = S;

            #[inline]
            fn approx_eq_eps(&self, other: &$PointN<S>, epsilon: &S) -> bool {
                $(self.$field.approx_eq_eps(&other.$field, epsilon))&&+
            }
        }

        impl_operator!(<S: BaseNum> Add<$VectorN<S> > for $PointN<S> {
            fn add(lhs, rhs) -> $PointN<S> { $PointN::new($(lhs.$field + rhs.$field),+) }
        });
        impl_assignment_operator!(<S: BaseNum> AddAssign<$VectorN<S> > for $PointN<S> {
            fn add_assign(&mut self, vector) { $(self.$field += vector.$field);+ }
        });

        impl_operator!(<S: BaseNum> Sub<$PointN<S> > for $PointN<S> {
            fn sub(lhs, rhs) -> $VectorN<S> { $VectorN::new($(lhs.$field - rhs.$field),+) }
        });

        impl_operator!(<S: BaseNum> Mul<S> for $PointN<S> {
            fn mul(point, scalar) -> $PointN<S> { $PointN::new($(point.$field * scalar),+) }
        });
        impl_operator!(<S: BaseNum> Div<S> for $PointN<S> {
            fn div(point, scalar) -> $PointN<S> { $PointN::new($(point.$field / scalar),+) }
        });
        impl_operator!(<S: BaseNum> Rem<S> for $PointN<S> {
            fn rem(point, scalar) -> $PointN<S> { $PointN::new($(point.$field % scalar),+) }
        });
        impl_assignment_operator!(<S: BaseNum> MulAssign<S> for $PointN<S> {
            fn mul_assign(&mut self, scalar) { $(self.$field *= scalar);+ }
        });
        impl_assignment_operator!(<S: BaseNum> DivAssign<S> for $PointN<S> {
            fn div_assign(&mut self, scalar) { $(self.$field /= scalar);+ }
        });
        impl_assignment_operator!(<S: BaseNum> RemAssign<S> for $PointN<S> {
            fn rem_assign(&mut self, scalar) { $(self.$field %= scalar);+ }
        });

        impl_scalar_ops!($PointN<usize> { $($field),+ });
        impl_scalar_ops!($PointN<u8> { $($field),+ });
        impl_scalar_ops!($PointN<u16> { $($field),+ });
        impl_scalar_ops!($PointN<u32> { $($field),+ });
        impl_scalar_ops!($PointN<u64> { $($field),+ });
        impl_scalar_ops!($PointN<isize> { $($field),+ });
        impl_scalar_ops!($PointN<i8> { $($field),+ });
        impl_scalar_ops!($PointN<i16> { $($field),+ });
        impl_scalar_ops!($PointN<i32> { $($field),+ });
        impl_scalar_ops!($PointN<i64> { $($field),+ });
        impl_scalar_ops!($PointN<f32> { $($field),+ });
        impl_scalar_ops!($PointN<f64> { $($field),+ });

        impl_index_operators!($PointN<S>, $n, S, usize);
        impl_index_operators!($PointN<S>, $n, [S], Range<usize>);
        impl_index_operators!($PointN<S>, $n, [S], RangeTo<usize>);
        impl_index_operators!($PointN<S>, $n, [S], RangeFrom<usize>);
        impl_index_operators!($PointN<S>, $n, [S], RangeFull);
    }
}

macro_rules! impl_scalar_ops {
    ($PointN:ident<$S:ident> { $($field:ident),+ }) => {
        impl_operator!(Mul<$PointN<$S>> for $S {
            fn mul(scalar, point) -> $PointN<$S> { $PointN::new($(scalar * point.$field),+) }
        });
        impl_operator!(Div<$PointN<$S>> for $S {
            fn div(scalar, point) -> $PointN<$S> { $PointN::new($(scalar / point.$field),+) }
        });
        impl_operator!(Rem<$PointN<$S>> for $S {
            fn rem(scalar, point) -> $PointN<$S> { $PointN::new($(scalar % point.$field),+) }
        });
    };
}

impl_point!(Point2 { x, y }, Vector2, 2);
impl_point!(Point3 { x, y, z }, Vector3, 3);

impl_fixed_array_conversions!(Point2<S> { x: 0, y: 1 }, 2);
impl_fixed_array_conversions!(Point3<S> { x: 0, y: 1, z: 2 }, 3);

impl_tuple_conversions!(Point2<S> { x, y }, (S, S));
impl_tuple_conversions!(Point3<S> { x, y, z }, (S, S, S));

impl<S: fmt::Debug> fmt::Debug for Point2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Point2 "));
        <[S; 2] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Point3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Point3 "));
        <[S; 3] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

#[cfg(test)]
mod tests {
    mod point2 {
        use point::*;

        const POINT2: Point2<i32> = Point2 { x: 1, y: 2 };

        #[test]
        fn test_index() {
            assert_eq!(POINT2[0], POINT2.x);
            assert_eq!(POINT2[1], POINT2.y);
        }

        #[test]
        fn test_index_mut() {
            let mut p = POINT2;
            *&mut p[0] = 0;
            assert_eq!(p, [0, 2].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            POINT2[2];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&POINT2[..0], &[]);
            assert_eq!(&POINT2[..1], &[1]);
            assert_eq!(POINT2[..0].len(), 0);
            assert_eq!(POINT2[..1].len(), 1);
            assert_eq!(&POINT2[2..], &[]);
            assert_eq!(&POINT2[1..], &[2]);
            assert_eq!(POINT2[2..].len(), 0);
            assert_eq!(POINT2[1..].len(), 1);
            assert_eq!(&POINT2[..], &[1, 2]);
            assert_eq!(POINT2[..].len(), 2);
        }

        #[test]
        fn test_into() {
            let p = POINT2;
            {
                let p: [i32; 2] = p.into();
                assert_eq!(p, [1, 2]);
            }
            {
                let p: (i32, i32) = p.into();
                assert_eq!(p, (1, 2));
            }
        }

        #[test]
        fn test_as_ref() {
            let p = POINT2;
            {
                let p: &[i32; 2] = p.as_ref();
                assert_eq!(p, &[1, 2]);
            }
            {
                let p: &(i32, i32) = p.as_ref();
                assert_eq!(p, &(1, 2));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut p = POINT2;
            {
                let p: &mut [i32; 2] = p.as_mut();
                assert_eq!(p, &mut [1, 2]);
            }
            {
                let p: &mut (i32, i32) = p.as_mut();
                assert_eq!(p, &mut (1, 2));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Point2::from([1, 2]), POINT2);
            {
                let p = &[1, 2];
                let p: &Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
            {
                let p = &mut [1, 2];
                let p: &mut Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
            assert_eq!(Point2::from((1, 2)), POINT2);
            {
                let p = &(1, 2);
                let p: &Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
            {
                let p = &mut (1, 2);
                let p: &mut Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
        }
    }

    mod point3 {
        use point::*;

        const POINT3: Point3<i32> = Point3 { x: 1, y: 2, z: 3 };

        #[test]
        fn test_index() {
            assert_eq!(POINT3[0], POINT3.x);
            assert_eq!(POINT3[1], POINT3.y);
            assert_eq!(POINT3[2], POINT3.z);
        }

        #[test]
        fn test_index_mut() {
            let mut p = POINT3;
            *&mut p[1] = 0;
            assert_eq!(p, [1, 0, 3].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            POINT3[3];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&POINT3[..1], &[1]);
            assert_eq!(&POINT3[..2], &[1, 2]);
            assert_eq!(POINT3[..1].len(), 1);
            assert_eq!(POINT3[..2].len(), 2);
            assert_eq!(&POINT3[2..], &[3]);
            assert_eq!(&POINT3[1..], &[2, 3]);
            assert_eq!(POINT3[2..].len(), 1);
            assert_eq!(POINT3[1..].len(), 2);
            assert_eq!(&POINT3[..], &[1, 2, 3]);
            assert_eq!(POINT3[..].len(), 3);
        }

        #[test]
        fn test_into() {
            let p = POINT3;
            {
                let p: [i32; 3] = p.into();
                assert_eq!(p, [1, 2, 3]);
            }
            {
                let p: (i32, i32, i32) = p.into();
                assert_eq!(p, (1, 2, 3));
            }
        }

        #[test]
        fn test_as_ref() {
            let p = POINT3;
            {
                let p: &[i32; 3] = p.as_ref();
                assert_eq!(p, &[1, 2, 3]);
            }
            {
                let p: &(i32, i32, i32) = p.as_ref();
                assert_eq!(p, &(1, 2, 3));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut p = POINT3;
            {
                let p: &mut [i32; 3] = p.as_mut();
                assert_eq!(p, &mut [1, 2, 3]);
            }
            {
                let p: &mut (i32, i32, i32) = p.as_mut();
                assert_eq!(p, &mut (1, 2, 3));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Point3::from([1, 2, 3]), POINT3);
            {
                let p = &[1, 2, 3];
                let p: &Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
            {
                let p = &mut [1, 2, 3];
                let p: &mut Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
            assert_eq!(Point3::from((1, 2, 3)), POINT3);
            {
                let p = &(1, 2, 3);
                let p: &Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
            {
                let p = &mut (1, 2, 3);
                let p: &mut Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
        }
    }
}
