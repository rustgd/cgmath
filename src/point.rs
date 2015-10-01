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
use array::Array1;
use bound::*;
use matrix::{Matrix, Matrix4};
use num::{BaseNum, BaseFloat};
use plane::Plane;
use vector::*;

/// A point in 2-dimensional space.
#[derive(PartialEq, Eq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
pub struct Point2<S> { pub x: S, pub y: S }

/// A point in 3-dimensional space.
#[derive(PartialEq, Eq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
pub struct Point3<S> { pub x: S, pub y: S, pub z: S }


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
    pub fn from_homogeneous(v: &Vector4<S>) -> Point3<S> {
        let e = v.truncate().mul_s(S::one() / v.w);
        Point3::new(e.x, e.y, e.z)  //FIXME
    }

    #[inline]
    pub fn to_homogeneous(&self) -> Vector4<S> {
        Vector4::new(self.x, self.y, self.z, S::one())
    }
}

/// Specifies the numeric operations for point types.
pub trait Point<S: BaseNum, V: Vector<S>>: Array1<S> + Clone // where
    // FIXME: blocked by rust-lang/rust#20671
    //
    // for<'a, 'b> &'a Self: Add<&'b V, Output = Self>,
    // for<'a, 'b> &'a Self: Sub<&'b Self, Output = V>,
    //
    // for<'a> &'a Self: Mul<S, Output = Self>,
    // for<'a> &'a Self: Div<S, Output = Self>,
    // for<'a> &'a Self: Rem<S, Output = Self>,
{
    /// Create a point at the origin.
    fn origin() -> Self;

    /// Create a point from a vector.
    fn from_vec(v: &V) -> Self;
    /// Convert a point to a vector.
    fn to_vec(&self) -> V;

    /// Multiply each component by a scalar, returning the new point.
    #[must_use]
    fn mul_s(&self, s: S) -> Self;
    /// Divide each component by a scalar, returning the new point.
    #[must_use]
    fn div_s(&self, s: S) -> Self;
    /// Subtract a scalar from each component, returning the new point.
    #[must_use]
    fn rem_s(&self, s: S) -> Self;

    /// Add a vector to this point, returning the new point.
    #[must_use]
    fn add_v(&self, v: &V) -> Self;
    /// Subtract another point from this one, returning a new vector.
    fn sub_p(&self, p: &Self) -> V;

    /// Multiply each component by a scalar, in-place.
    fn mul_self_s(&mut self, s: S);
    /// Divide each component by a scalar, in-place.
    fn div_self_s(&mut self, s: S);
    /// Take the remainder of each component by a scalar, in-place.
    fn rem_self_s(&mut self, s: S);

    /// Add a vector to this point, in-place.
    fn add_self_v(&mut self, v: &V);

    /// This is a weird one, but its useful for plane calculations.
    fn dot(&self, v: &V) -> S;

    #[must_use]
    fn min(&self, p: &Self) -> Self;

    #[must_use]
    fn max(&self, p: &Self) -> Self;
}

impl<S: BaseNum> Array1<S> for Point2<S> {}

impl<S: BaseNum> Point<S, Vector2<S>> for Point2<S> {
    #[inline]
    fn origin() -> Point2<S> {
        Point2::new(S::zero(), S::zero())
    }

    #[inline]
    fn from_vec(v: &Vector2<S>) -> Point2<S> {
        Point2::new(v.x, v.y)
    }

    #[inline]
    fn to_vec(&self) -> Vector2<S> {
        Vector2::new(self.x, self.y)
    }

    #[inline] fn mul_s(&self, s: S) -> Point2<S> { self * s }
    #[inline] fn div_s(&self, s: S) -> Point2<S> { self / s }
    #[inline] fn rem_s(&self, s: S) -> Point2<S> { self % s }
    #[inline] fn add_v(&self, v: &Vector2<S>) -> Point2<S> { self + v }
    #[inline] fn sub_p(&self, p: &Point2<S>) -> Vector2<S> { self - p }

    #[inline]
    fn mul_self_s(&mut self, s: S) {
        self.x = self.x * s;
        self.y = self.y * s;
    }

    #[inline]
    fn div_self_s(&mut self, s: S) {
        self.x = self.x / s;
        self.y = self.y / s;
    }

    #[inline]
    fn rem_self_s(&mut self, s: S) {
        self.x = self.x % s;
        self.y = self.y % s;
    }

    #[inline]
    fn add_self_v(&mut self, v: &Vector2<S>) {
        self.x = self.x + v.x;
        self.y = self.y + v.y;
    }

    #[inline]
    fn dot(&self, v: &Vector2<S>) -> S {
        self.x * v.x +
        self.y * v.y
    }

    #[inline]
    fn min(&self, p: &Point2<S>) -> Point2<S> {
        Point2::new(self.x.partial_min(p.x), self.y.partial_min(p.y))
    }

    #[inline]
    fn max(&self, p: &Point2<S>) -> Point2<S> {
        Point2::new(self.x.partial_max(p.x), self.y.partial_max(p.y))
    }
}

impl<S: BaseFloat> ApproxEq<S> for Point2<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Point2<S>, epsilon: &S) -> bool {
        self.x.approx_eq_eps(&other.x, epsilon) &&
        self.y.approx_eq_eps(&other.y, epsilon)
    }
}

impl<S: BaseNum> Array1<S> for Point3<S> {}

impl<S: BaseNum> Point<S, Vector3<S>> for Point3<S> {
    #[inline]
    fn origin() -> Point3<S> {
        Point3::new(S::zero(), S::zero(), S::zero())
    }

    #[inline]
    fn from_vec(v: &Vector3<S>) -> Point3<S> {
        Point3::new(v.x, v.y, v.z)
    }

    #[inline]
    fn to_vec(&self) -> Vector3<S> {
        Vector3::new(self.x, self.y, self.z)
    }

    #[inline] fn mul_s(&self, s: S) -> Point3<S> { self * s }
    #[inline] fn div_s(&self, s: S) -> Point3<S> { self / s }
    #[inline] fn rem_s(&self, s: S) -> Point3<S> { self % s }
    #[inline] fn add_v(&self, v: &Vector3<S>) -> Point3<S> { self + v }
    #[inline] fn sub_p(&self, p: &Point3<S>) -> Vector3<S> { self - p }

    #[inline]
    fn mul_self_s(&mut self, s: S) {
        self.x = self.x * s;
        self.y = self.y * s;
        self.z = self.z * s;
    }

    #[inline]
    fn div_self_s(&mut self, s: S) {
        self.x = self.x / s;
        self.y = self.y / s;
        self.z = self.z / s;
    }

    #[inline]
    fn rem_self_s(&mut self, s: S) {
        self.x = self.x % s;
        self.y = self.y % s;
        self.z = self.z % s;
    }

    #[inline]
    fn add_self_v(&mut self, v: &Vector3<S>) {
        self.x = self.x + v.x;
        self.y = self.y + v.y;
        self.z = self.z + v.z;
    }

    #[inline]
    fn dot(&self, v: &Vector3<S>) -> S {
        self.x * v.x +
        self.y * v.y +
        self.z * v.z
    }

    #[inline]
    fn min(&self, p: &Point3<S>) -> Point3<S> {
        Point3::new(self.x.partial_min(p.x), self.y.partial_min(p.y), self.z.partial_min(p.z))
    }

    #[inline]
    fn max(&self, p: &Point3<S>) -> Point3<S> {
        Point3::new(self.x.partial_max(p.x), self.y.partial_max(p.y), self.z.partial_max(p.z))
    }
}

impl<S: BaseFloat> ApproxEq<S> for Point3<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Point3<S>, epsilon: &S) -> bool {
        self.x.approx_eq_eps(&other.x, epsilon) &&
        self.y.approx_eq_eps(&other.y, epsilon) &&
        self.z.approx_eq_eps(&other.z, epsilon)
    }
}


macro_rules! impl_operators {
    ($PointN:ident { $($field:ident),+ }, $VectorN:ident) => {
        impl<'a, S: BaseNum> Mul<S> for &'a $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn mul(self, s: S) -> $PointN<S> {
                $PointN::new($(self.$field * s),+)
            }
        }

        impl<'a, S: BaseNum> Div<S> for &'a $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn div(self, s: S) -> $PointN<S> {
                $PointN::new($(self.$field / s),+)
            }
        }

        impl<'a, S: BaseNum> Rem<S> for &'a $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn rem(self, s: S) -> $PointN<S> {
                $PointN::new($(self.$field % s),+)
            }
        }

        impl<'a, 'b, S: BaseNum> Add<&'a $VectorN<S>> for &'b $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn add(self, v: &'a $VectorN<S>) -> $PointN<S> {
                $PointN::new($(self.$field + v.$field),+)
            }
        }

        impl<'a, 'b, S: BaseNum> Sub<&'a $PointN<S>> for &'b $PointN<S> {
            type Output = $VectorN<S>;

            #[inline]
            fn sub(self, p: &'a $PointN<S>) -> $VectorN<S> {
                $VectorN::new($(self.$field - p.$field),+)
            }
        }
    }
}

impl_operators!(Point2 { x, y }, Vector2);
impl_operators!(Point3 { x, y, z }, Vector3);

macro_rules! fixed_array_conversions {
    ($PointN:ident <$S:ident> { $($field:ident : $index:expr),+ }, $n:expr) => {
        impl<$S> Into<[$S; $n]> for $PointN<$S> {
            #[inline]
            fn into(self) -> [$S; $n] {
                match self { $PointN { $($field),+ } => [$($field),+] }
            }
        }

        impl<$S> AsRef<[$S; $n]> for $PointN<$S> {
            #[inline]
            fn as_ref(&self) -> &[$S; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[$S; $n]> for $PointN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$S; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S: Clone> From<[$S; $n]> for $PointN<$S> {
            #[inline]
            fn from(v: [$S; $n]) -> $PointN<$S> {
                // We need to use a clone here because we can't pattern match on arrays yet
                $PointN { $($field: v[$index].clone()),+ }
            }
        }

        impl<'a, $S> From<&'a [$S; $n]> for &'a $PointN<$S> {
            #[inline]
            fn from(v: &'a [$S; $n]) -> &'a $PointN<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut [$S; $n]> for &'a mut $PointN<$S> {
            #[inline]
            fn from(v: &'a mut [$S; $n]) -> &'a mut $PointN<$S> {
                unsafe { mem::transmute(v) }
            }
        }
    }
}

fixed_array_conversions!(Point2<S> { x:0, y:1 }, 2);
fixed_array_conversions!(Point3<S> { x:0, y:1, z:2 }, 3);

macro_rules! tuple_conversions {
    ($PointN:ident <$S:ident> { $($field:ident),+ }, $Tuple:ty) => {
        impl<$S> Into<$Tuple> for $PointN<$S> {
            #[inline]
            fn into(self) -> $Tuple {
                match self { $PointN { $($field),+ } => ($($field),+) }
            }
        }

        impl<$S> AsRef<$Tuple> for $PointN<$S> {
            #[inline]
            fn as_ref(&self) -> &$Tuple {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<$Tuple> for $PointN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut $Tuple {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> From<$Tuple> for $PointN<$S> {
            #[inline]
            fn from(v: $Tuple) -> $PointN<$S> {
                // We need to use a clone here because we can't pattern match on arrays yet
                match v { ($($field),+) => $PointN { $($field: $field),+ } }
            }
        }

        impl<'a, $S> From<&'a $Tuple> for &'a $PointN<$S> {
            #[inline]
            fn from(v: &'a $Tuple) -> &'a $PointN<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut $Tuple> for &'a mut $PointN<$S> {
            #[inline]
            fn from(v: &'a mut $Tuple) -> &'a mut $PointN<$S> {
                unsafe { mem::transmute(v) }
            }
        }
    }
}

tuple_conversions!(Point2<S> { x, y }, (S, S));
tuple_conversions!(Point3<S> { x, y, z }, (S, S, S));

macro_rules! index_operators {
    ($PointN:ident<$S:ident>, $n:expr, $Output:ty, $I:ty) => {
        impl<$S> Index<$I> for $PointN<$S> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&'a self, i: $I) -> &'a $Output {
                let v: &[$S; $n] = self.as_ref(); &v[i]
            }
        }

        impl<$S> IndexMut<$I> for $PointN<$S> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: $I) -> &'a mut $Output {
                let v: &mut [$S; $n] = self.as_mut(); &mut v[i]
            }
        }
    }
}

index_operators!(Point2<S>, 2, S, usize);
index_operators!(Point3<S>, 3, S, usize);
index_operators!(Point2<S>, 2, [S], Range<usize>);
index_operators!(Point3<S>, 3, [S], Range<usize>);
index_operators!(Point2<S>, 2, [S], RangeTo<usize>);
index_operators!(Point3<S>, 3, [S], RangeTo<usize>);
index_operators!(Point2<S>, 2, [S], RangeFrom<usize>);
index_operators!(Point3<S>, 3, [S], RangeFrom<usize>);
index_operators!(Point2<S>, 2, [S], RangeFull);
index_operators!(Point3<S>, 3, [S], RangeFull);

impl<S: BaseNum> fmt::Debug for Point2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?}]", self.x, self.y)
    }
}

impl<S: BaseNum> fmt::Debug for Point3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?}, {:?}]", self.x, self.y, self.z)
    }
}

impl<S: BaseFloat + 'static> Bound<S> for Point3<S> {
    fn relate_plane(&self, plane: &Plane<S>) -> Relation {
        let dist = self.dot(&plane.n);
        if dist > plane.d {
            Relation::In
        }else if dist < plane.d {
            Relation::Out
        }else {
            Relation::Cross
        }
    }

    fn relate_clip_space(&self, projection: &Matrix4<S>) -> Relation {
        use std::cmp::Ordering::*;
        let p = projection.mul_v(&self.to_homogeneous());
        match (p.x.abs().partial_cmp(&p.w), p.y.abs().partial_cmp(&p.w), p.z.abs().partial_cmp(&p.w)) {
            (Some(Less), Some(Less), Some(Less)) => Relation::In,
            (Some(Greater), _, _) | (_, Some(Greater), _) | (_, _, Some(Greater)) => Relation::Out,
            _ => Relation::Cross,
        }
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
