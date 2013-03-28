use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::to_unsafe_ptr;
use core::util::swap;
use core::sys::size_of;
use core::vec::raw::buf_as_slice;

use std::cmp::{FuzzyEq, FUZZY_EPSILON};
use numeric::*;
use numeric::number::Number;
use numeric::number::Number::{zero,one};

use vec::{
    Vec4,
    Vector4,
    MutableVector,
    NumericVector,
    MutableNumericVector,
    vec4,
    dvec4,
};

use mat::{
    Mat3,
    Matrix,
    Matrix3,
    Matrix4,
    MutableMatrix,
};

/**
 *  A 4 x 4 column major matrix
 *
 * # Type parameters
 *
 * * `T` - The type of the elements of the matrix. Should be a floating point type.
 *
 * # Fields
 *
 * * `x` - the first column vector of the matrix
 * * `y` - the second column vector of the matrix
 * * `z` - the third column vector of the matrix
 * * `w` - the fourth column vector of the matrix
 */
#[deriving(Eq)]
pub struct Mat4<T> { x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T> }

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Matrix<T, Vec4<T>> for Mat4<T> {
    #[inline(always)]
    fn col(&self, i: uint) -> Vec4<T> { self[i] }

    #[inline(always)]
    fn row(&self, i: uint) -> Vec4<T> {
        Vector4::new(self[0][i],
                     self[1][i],
                     self[2][i],
                     self[3][i])
    }

    /**
     * Construct a 4 x 4 diagonal matrix with the major diagonal set to `value`
     *
     * # Arguments
     *
     * * `value` - the value to set the major diagonal to
     *
     * ~~~
     *        c0    c1    c2    c3
     *     +-----+-----+-----+-----+
     *  r0 | val |   0 |   0 |   0 |
     *     +-----+-----+-----+-----+
     *  r1 |   0 | val |   0 |   0 |
     *     +-----+-----+-----+-----+
     *  r2 |   0 |   0 | val |   0 |
     *     +-----+-----+-----+-----+
     *  r3 |   0 |   0 |   0 | val |
     *     +-----+-----+-----+-----+
     * ~~~
     */
    #[inline(always)]
    fn from_value(value: T) -> Mat4<T> {
        Matrix4::new(value, zero(), zero(), zero(),
                     zero(), value, zero(), zero(),
                     zero(), zero(), value, zero(),
                     zero(), zero(), zero(), value)
    }

    /**
     * Returns the multiplicative identity matrix
     * ~~~
     *       c0   c1   c2   c3
     *     +----+----+----+----+
     *  r0 |  1 |  0 |  0 |  0 |
     *     +----+----+----+----+
     *  r1 |  0 |  1 |  0 |  0 |
     *     +----+----+----+----+
     *  r2 |  0 |  0 |  1 |  0 |
     *     +----+----+----+----+
     *  r3 |  0 |  0 |  0 |  1 |
     *     +----+----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn identity() -> Mat4<T> {
        Matrix4::new( one::<T>(), zero::<T>(), zero::<T>(), zero::<T>(),
                     zero::<T>(),  one::<T>(), zero::<T>(), zero::<T>(),
                     zero::<T>(), zero::<T>(),  one::<T>(), zero::<T>(),
                     zero::<T>(), zero::<T>(), zero::<T>(),  one::<T>())
    }

    /**
     * Returns the additive identity matrix
     * ~~~
     *       c0   c1   c2   c3
     *     +----+----+----+----+
     *  r0 |  0 |  0 |  0 |  0 |
     *     +----+----+----+----+
     *  r1 |  0 |  0 |  0 |  0 |
     *     +----+----+----+----+
     *  r2 |  0 |  0 |  0 |  0 |
     *     +----+----+----+----+
     *  r3 |  0 |  0 |  0 |  0 |
     *     +----+----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn zero() -> Mat4<T> {
        Matrix4::new(zero::<T>(), zero::<T>(), zero::<T>(), zero::<T>(),
                     zero::<T>(), zero::<T>(), zero::<T>(), zero::<T>(),
                     zero::<T>(), zero::<T>(), zero::<T>(), zero::<T>(),
                     zero::<T>(), zero::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn mul_t(&self, value: T) -> Mat4<T> {
        Matrix4::from_cols(self[0].mul_t(value),
                           self[1].mul_t(value),
                           self[2].mul_t(value),
                           self[3].mul_t(value))
    }

    #[inline(always)]
    fn mul_v(&self, vec: &Vec4<T>) -> Vec4<T> {
        Vector4::new(self.row(0).dot(vec),
                     self.row(1).dot(vec),
                     self.row(2).dot(vec),
                     self.row(3).dot(vec))
    }

    #[inline(always)]
    fn add_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Matrix4::from_cols(self[0].add_v(&other[0]),
                           self[1].add_v(&other[1]),
                           self[2].add_v(&other[2]),
                           self[3].add_v(&other[3]))
    }

    #[inline(always)]
    fn sub_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Matrix4::from_cols(self[0].sub_v(&other[0]),
                           self[1].sub_v(&other[1]),
                           self[2].sub_v(&other[2]),
                           self[3].sub_v(&other[3]))
    }

    #[inline(always)]
    fn mul_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Matrix4::new(self.row(0).dot(&other.col(0)),
                     self.row(1).dot(&other.col(0)),
                     self.row(2).dot(&other.col(0)),
                     self.row(3).dot(&other.col(0)),

                     self.row(0).dot(&other.col(1)),
                     self.row(1).dot(&other.col(1)),
                     self.row(2).dot(&other.col(1)),
                     self.row(3).dot(&other.col(1)),

                     self.row(0).dot(&other.col(2)),
                     self.row(1).dot(&other.col(2)),
                     self.row(2).dot(&other.col(2)),
                     self.row(3).dot(&other.col(2)),

                     self.row(0).dot(&other.col(3)),
                     self.row(1).dot(&other.col(3)),
                     self.row(2).dot(&other.col(3)),
                     self.row(3).dot(&other.col(3)))

    }

    fn dot(&self, other: &Mat4<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    fn determinant(&self) -> T {
        let m0: Mat3<T> = Matrix3::new(self[1][1], self[2][1], self[3][1],
                                       self[1][2], self[2][2], self[3][2],
                                       self[1][3], self[2][3], self[3][3]);
        let m1: Mat3<T> = Matrix3::new(self[0][1], self[2][1], self[3][1],
                                       self[0][2], self[2][2], self[3][2],
                                       self[0][3], self[2][3], self[3][3]);
        let m2: Mat3<T> = Matrix3::new(self[0][1], self[1][1], self[3][1],
                                       self[0][2], self[1][2], self[3][2],
                                       self[0][3], self[1][3], self[3][3]);
        let m3: Mat3<T> = Matrix3::new(self[0][1], self[1][1], self[2][1],
                                       self[0][2], self[1][2], self[2][2],
                                       self[0][3], self[1][3], self[2][3]);

        self[0][0] * m0.determinant() -
        self[1][0] * m1.determinant() +
        self[2][0] * m2.determinant() -
        self[3][0] * m3.determinant()
    }

    fn trace(&self) -> T {
        self[0][0] + self[1][1] + self[2][2] + self[3][3]
    }

    fn inverse(&self) -> Option<Mat4<T>> {
        let d = self.determinant();
        if d.fuzzy_eq(&zero()) {
            None
        } else {

            // Gauss Jordan Elimination with partial pivoting
            // So take this matrix, A, augmented with the identity
            // and essentially reduce [A|I]

            let mut A = *self;
            let mut I: Mat4<T> = Matrix::identity();

            for uint::range(0, 4) |j| {
                // Find largest element in col j
                let mut i1 = j;
                for uint::range(j + 1, 4) |i| {
                    if abs(A[j][i]) > abs(A[j][i1]) {
                        i1 = i;
                    }
                }

                unsafe {
                    // Swap columns i1 and j in A and I to
                    // put pivot on diagonal
                    A.swap_cols(i1, j);
                    I.swap_cols(i1, j);

                    // Scale col j to have a unit diagonal
                    I.col_mut(j).div_self_t(&A[j][j]);
                    A.col_mut(j).div_self_t(&A[j][j]);

                    // Eliminate off-diagonal elems in col j of A,
                    // doing identical ops to I
                    for uint::range(0, 4) |i| {
                        if i != j {
                            I.col_mut(i).sub_self_v(&I[j].mul_t(A[i][j]));
                            A.col_mut(i).sub_self_v(&A[j].mul_t(A[i][j]));
                        }
                    }
                }
            }
            Some(I)
        }
    }

    #[inline(always)]
    fn transpose(&self) -> Mat4<T> {
        Matrix4::new(self[0][0], self[1][0], self[2][0], self[3][0],
                     self[0][1], self[1][1], self[2][1], self[3][1],
                     self[0][2], self[1][2], self[2][2], self[3][2],
                     self[0][3], self[1][3], self[2][3], self[3][3])
    }

    #[inline(always)]
    fn is_identity(&self) -> bool {
        self.fuzzy_eq(&Matrix::identity())
    }

    #[inline(always)]
    fn is_diagonal(&self) -> bool {
        self[0][1].fuzzy_eq(&zero()) &&
        self[0][2].fuzzy_eq(&zero()) &&
        self[0][3].fuzzy_eq(&zero()) &&

        self[1][0].fuzzy_eq(&zero()) &&
        self[1][2].fuzzy_eq(&zero()) &&
        self[1][3].fuzzy_eq(&zero()) &&

        self[2][0].fuzzy_eq(&zero()) &&
        self[2][1].fuzzy_eq(&zero()) &&
        self[2][3].fuzzy_eq(&zero()) &&

        self[3][0].fuzzy_eq(&zero()) &&
        self[3][1].fuzzy_eq(&zero()) &&
        self[3][2].fuzzy_eq(&zero())
    }

    #[inline(always)]
    fn is_rotated(&self) -> bool {
        !self.fuzzy_eq(&Matrix::identity())
    }

    #[inline(always)]
    fn is_symmetric(&self) -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[0][2].fuzzy_eq(&self[2][0]) &&
        self[0][3].fuzzy_eq(&self[3][0]) &&

        self[1][0].fuzzy_eq(&self[0][1]) &&
        self[1][2].fuzzy_eq(&self[2][1]) &&
        self[1][3].fuzzy_eq(&self[3][1]) &&

        self[2][0].fuzzy_eq(&self[0][2]) &&
        self[2][1].fuzzy_eq(&self[1][2]) &&
        self[2][3].fuzzy_eq(&self[3][2]) &&

        self[3][0].fuzzy_eq(&self[0][3]) &&
        self[3][1].fuzzy_eq(&self[1][3]) &&
        self[3][2].fuzzy_eq(&self[2][3])
    }

    #[inline(always)]
    fn is_invertible(&self) -> bool {
        !self.determinant().fuzzy_eq(&zero())
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Mat4<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Matrix4<T, Vec4<T>> for Mat4<T> {
    /**
     * Construct a 4 x 4 matrix
     *
     * # Arguments
     *
     * * `c0r0`, `c0r1`, `c0r2`, `c0r3` - the first column of the matrix
     * * `c1r0`, `c1r1`, `c1r2`, `c1r3` - the second column of the matrix
     * * `c2r0`, `c2r1`, `c2r2`, `c2r3` - the third column of the matrix
     * * `c3r0`, `c3r1`, `c3r2`, `c3r3` - the fourth column of the matrix
     *
     * ~~~
     *        c0     c1     c2     c3
     *     +------+------+------+------+
     *  r0 | c0r0 | c1r0 | c2r0 | c3r0 |
     *     +------+------+------+------+
     *  r1 | c0r1 | c1r1 | c2r1 | c3r1 |
     *     +------+------+------+------+
     *  r2 | c0r2 | c1r2 | c2r2 | c3r2 |
     *     +------+------+------+------+
     *  r3 | c0r3 | c1r3 | c2r3 | c3r3 |
     *     +------+------+------+------+
     * ~~~
     */
    #[inline(always)]
    fn new(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
                       c1r0: T, c1r1: T, c1r2: T, c1r3: T,
                       c2r0: T, c2r1: T, c2r2: T, c2r3: T,
                       c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> Mat4<T>  {
        Matrix4::from_cols(Vector4::new::<T,Vec4<T>>(c0r0, c0r1, c0r2, c0r3),
                           Vector4::new::<T,Vec4<T>>(c1r0, c1r1, c1r2, c1r3),
                           Vector4::new::<T,Vec4<T>>(c2r0, c2r1, c2r2, c2r3),
                           Vector4::new::<T,Vec4<T>>(c3r0, c3r1, c3r2, c3r3))
    }

    /**
     * Construct a 4 x 4 matrix from column vectors
     *
     * # Arguments
     *
     * * `c0` - the first column vector of the matrix
     * * `c1` - the second column vector of the matrix
     * * `c2` - the third column vector of the matrix
     * * `c3` - the fourth column vector of the matrix
     *
     * ~~~
     *        c0     c1     c2     c3
     *     +------+------+------+------+
     *  r0 | c0.x | c1.x | c2.x | c3.x |
     *     +------+------+------+------+
     *  r1 | c0.y | c1.y | c2.y | c3.y |
     *     +------+------+------+------+
     *  r2 | c0.z | c1.z | c2.z | c3.z |
     *     +------+------+------+------+
     *  r3 | c0.w | c1.w | c2.w | c3.w |
     *     +------+------+------+------+
     * ~~~
     */
    #[inline(always)]
    fn from_cols(c0: Vec4<T>,
                             c1: Vec4<T>,
                             c2: Vec4<T>,
                             c3: Vec4<T>) -> Mat4<T> {
        Mat4 { x: c0, y: c1, z: c2, w: c3 }
    }
}

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> MutableMatrix<T, Vec4<T>> for Mat4<T> {
    #[inline(always)]
    fn col_mut(&mut self, i: uint) -> &'self mut Vec4<T> {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => fail!(fmt!("index out of bounds: expected an index from 0 to 3, but found %u", i))
        }
    }

    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        swap(self.col_mut(a),
             self.col_mut(b));
    }

    #[inline(always)]
    fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
        self.z.swap(a, b);
        self.w.swap(a, b);
    }

    #[inline(always)]
    fn set(&mut self, other: &Mat4<T>) {
        (*self) = (*other);
    }

    #[inline(always)]
    fn to_identity(&mut self) {
        (*self) = Matrix::identity();
    }

    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = Matrix::zero();
    }

    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(&value);
        self.col_mut(1).mul_self_t(&value);
        self.col_mut(2).mul_self_t(&value);
        self.col_mut(3).mul_self_t(&value);
    }

    #[inline(always)]
    fn add_self_m(&mut self, other: &Mat4<T>) {
        self.col_mut(0).add_self_v(&other[0]);
        self.col_mut(1).add_self_v(&other[1]);
        self.col_mut(2).add_self_v(&other[2]);
        self.col_mut(3).add_self_v(&other[3]);
    }

    #[inline(always)]
    fn sub_self_m(&mut self, other: &Mat4<T>) {
        self.col_mut(0).sub_self_v(&other[0]);
        self.col_mut(1).sub_self_v(&other[1]);
        self.col_mut(2).sub_self_v(&other[2]);
        self.col_mut(3).sub_self_v(&other[3]);
    }

    #[inline(always)]
    fn invert_self(&mut self) {
        match self.inverse() {
            Some(m) => (*self) = m,
            None => fail!(~"Couldn't invert the matrix!")
        }
    }

    #[inline(always)]
    fn transpose_self(&mut self) {
        swap(self.col_mut(0).index_mut(1), self.col_mut(1).index_mut(0));
        swap(self.col_mut(0).index_mut(2), self.col_mut(2).index_mut(0));
        swap(self.col_mut(0).index_mut(3), self.col_mut(3).index_mut(0));

        swap(self.col_mut(1).index_mut(0), self.col_mut(0).index_mut(1));
        swap(self.col_mut(1).index_mut(2), self.col_mut(2).index_mut(1));
        swap(self.col_mut(1).index_mut(3), self.col_mut(3).index_mut(1));

        swap(self.col_mut(2).index_mut(0), self.col_mut(0).index_mut(2));
        swap(self.col_mut(2).index_mut(1), self.col_mut(1).index_mut(2));
        swap(self.col_mut(2).index_mut(3), self.col_mut(3).index_mut(2));

        swap(self.col_mut(3).index_mut(0), self.col_mut(0).index_mut(3));
        swap(self.col_mut(3).index_mut(1), self.col_mut(1).index_mut(3));
        swap(self.col_mut(3).index_mut(2), self.col_mut(2).index_mut(3));
    }
}

impl<T:Copy + Float + FuzzyEq<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Neg<Mat4<T>> for Mat4<T> {
    #[inline(always)]
    fn neg(&self) -> Mat4<T> {
        Matrix4::from_cols(-self[0], -self[1], -self[2], -self[3])
    }
}

impl<T:Copy> Index<uint, Vec4<T>> for Mat4<T> {
    #[inline(always)]
    fn index(&self, i: &uint) -> Vec4<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat4<T>, *Vec4<T>>(
                to_unsafe_ptr(self)), 4) |slice| { slice[*i] }
        }
    }
}

impl<T:Copy + Float + FuzzyEq<T>> FuzzyEq<T> for Mat4<T> {
    #[inline(always)]
    fn fuzzy_eq(&self, other: &Mat4<T>) -> bool {
        self.fuzzy_eq_eps(other, &Number::from(FUZZY_EPSILON))
    }

    #[inline(always)]
    fn fuzzy_eq_eps(&self, other: &Mat4<T>, epsilon: &T) -> bool {
        self[0].fuzzy_eq_eps(&other[0], epsilon) &&
        self[1].fuzzy_eq_eps(&other[1], epsilon) &&
        self[2].fuzzy_eq_eps(&other[2], epsilon) &&
        self[3].fuzzy_eq_eps(&other[3], epsilon)
    }
}

// GLSL-style type aliases, corresponding to Section 4.1.6 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

pub type mat4 = Mat4<f32>;      // a 4×4 single-precision floating-point matrix
pub type dmat4 = Mat4<f64>;     // a 4×4 double-precision floating-point matrix

// Static method wrappers for GLSL-style types

impl mat4 {
    #[inline(always)] fn new(c0r0: f32, c0r1: f32, c0r2: f32, c0r3: f32, c1r0: f32, c1r1: f32, c1r2: f32, c1r3: f32, c2r0: f32, c2r1: f32, c2r2: f32, c2r3: f32, c3r0: f32, c3r1: f32, c3r2: f32, c3r3: f32)
        -> mat4 { Matrix4::new(c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1, c3r2, c3r3) }
    #[inline(always)] fn from_cols(c0: vec4, c1: vec4, c2: vec4, c3: vec4)
        -> mat4 { Matrix4::from_cols(c0, c1, c2, c3) }
    #[inline(always)] fn from_value(v: f32) -> mat4 { Matrix::from_value(v) }

    #[inline(always)] fn identity() -> mat4 { Matrix::identity() }
    #[inline(always)] fn zero() -> mat4 { Matrix::zero() }

    #[inline(always)] fn dim() -> uint { 4 }
    #[inline(always)] fn rows() -> uint { 4 }
    #[inline(always)] fn cols() -> uint { 4 }
    #[inline(always)] fn size_of() -> uint { size_of::<mat4>() }
}

impl dmat4 {
    #[inline(always)] fn new(c0r0: f64, c0r1: f64, c0r2: f64, c0r3: f64, c1r0: f64, c1r1: f64, c1r2: f64, c1r3: f64, c2r0: f64, c2r1: f64, c2r2: f64, c2r3: f64, c3r0: f64, c3r1: f64, c3r2: f64, c3r3: f64)
        -> dmat4 { Matrix4::new(c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1, c3r2, c3r3) }
    #[inline(always)] fn from_cols(c0: dvec4, c1: dvec4, c2: dvec4, c3: dvec4)
        -> dmat4 { Matrix4::from_cols(c0, c1, c2, c3) }
    #[inline(always)] fn from_value(v: f64) -> dmat4 { Matrix::from_value(v) }

    #[inline(always)] fn identity() -> dmat4 { Matrix::identity() }
    #[inline(always)] fn zero() -> dmat4 { Matrix::zero() }

    #[inline(always)] fn dim() -> uint { 4 }
    #[inline(always)] fn rows() -> uint { 4 }
    #[inline(always)] fn cols() -> uint { 4 }
    #[inline(always)] fn size_of() -> uint { size_of::<dmat4>() }
}
