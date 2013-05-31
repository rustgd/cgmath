use std::cast::transmute;
use std::cmp::ApproxEq;
use std::num::{Zero, One};

use vec::*;
use quat::Quat;

use num::NumAssign;

/**
 * The base square matrix trait
 *
 * # Type parameters
 *
 * * `T` - The type of the elements of the matrix. Should be a floating point type.
 * * `V` - The type of the row and column vectors. Should have components of a
 *         floating point type and have the same number of dimensions as the
 *         number of rows and columns in the matrix.
 */
pub trait BaseMat<T,V>: Eq + Neg<Self> {
    /**
     * # Return value
     *
     * The column vector at `i`
     */
    fn col<'a>(&'a self, i: uint) -> &'a V;

    /**
     * # Return value
     *
     * The row vector at `i`
     */
    fn row(&self, i: uint) -> V;

    /**
     * Construct a diagonal matrix with the major diagonal set to `value`
     */
    fn from_value(value: T) -> Self;

    /**
     * # Return value
     *
     * The identity matrix
     */
    fn identity() -> Self;

    /**
     * # Return value
     *
     * A matrix with all elements set to zero
     */
    fn zero() -> Self;

    /**
     * # Return value
     *
     * The scalar multiplication of this matrix and `value`
     */
    fn mul_t(&self, value: T) -> Self;

    /**
     * # Return value
     *
     * The matrix vector product of the matrix and `vec`
     */
    fn mul_v(&self, vec: &V) -> V;

    /**
     * # Return value
     *
     * The matrix addition of the matrix and `other`
     */
    fn add_m(&self, other: &Self) -> Self;

    /**
     * # Return value
     *
     * The difference between the matrix and `other`
     */
    fn sub_m(&self, other: &Self) -> Self;

    /**
     * # Return value
     *
     * The matrix product of the matrix and `other`
     */
    fn mul_m(&self, other: &Self) -> Self;

    /**
     * # Return value
     *
     * The matrix dot product of the matrix and `other`
     */
    fn dot(&self, other: &Self) -> T;

    /**
     * # Return value
     *
     * The determinant of the matrix
     */
    fn determinant(&self) -> T;

    /**
     * # Return value
     *
     * The sum of the main diagonal of the matrix
     */
    fn trace(&self) -> T;

    /**
     * Returns the inverse of the matrix
     *
     * # Return value
     *
     * * `Some(m)` - if the inversion was successful, where `m` is the inverted matrix
     * * `None` - if the inversion was unsuccessful (because the matrix was not invertable)
     */
    fn inverse(&self) -> Option<Self>;

    /**
     * # Return value
     *
     * The transposed matrix
     */
    fn transpose(&self) -> Self;

    /**
     * # Return value
     *
     * A mutable reference to the column at `i`
     */
    fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut V;

    /**
     * Swap two columns of the matrix in place
     */
    fn swap_cols(&mut self, a: uint, b: uint);

    /**
     * Swap two rows of the matrix in place
     */
    fn swap_rows(&mut self, a: uint, b: uint);

    /**
     * Sets the matrix to `other`
     */
    fn set(&mut self, other: &Self);

    /**
     * Sets the matrix to the identity matrix
     */
    fn to_identity(&mut self);

    /**
     * Sets each element of the matrix to zero
     */
    fn to_zero(&mut self);

    /**
     * Multiplies the matrix by a scalar
     */
    fn mul_self_t(&mut self, value: T);

    /**
     * Add the matrix `other` to `self`
     */
    fn add_self_m(&mut self, other: &Self);

    /**
     * Subtract the matrix `other` from `self`
     */
    fn sub_self_m(&mut self, other: &Self);

    /**
     * Sets the matrix to its inverse
     *
     * # Failure
     *
     * Fails if the matrix is not invertable. Make sure you check with the
     * `is_invertible` method before you attempt this!
     */
    fn invert_self(&mut self);

    /**
     * Sets the matrix to its transpose
     */
    fn transpose_self(&mut self);

    /**
     * Check to see if the matrix is an identity matrix
     *
     * # Return value
     *
     * `true` if the matrix is approximately equal to the identity matrix
     */
    fn is_identity(&self) -> bool;

    /**
     * Check to see if the matrix is diagonal
     *
     * # Return value
     *
     * `true` all the elements outside the main diagonal are approximately
     * equal to zero.
     */
    fn is_diagonal(&self) -> bool;

    /**
     * Check to see if the matrix is rotated
     *
     * # Return value
     *
     * `true` if the matrix is not approximately equal to the identity matrix.
     */
    fn is_rotated(&self) -> bool;

    /**
     * Check to see if the matrix is symmetric
     *
     * # Return value
     *
     * `true` if the matrix is approximately equal to its transpose).
     */
    fn is_symmetric(&self) -> bool;

    /**
     * Check to see if the matrix is invertable
     *
     * # Return value
     *
     * `true` if  the matrix is invertable
     */
    fn is_invertible(&self) -> bool;

    /**
     * # Return value
     *
     * A pointer to the first element of the matrix
     */
    fn to_ptr(&self) -> *T;
}

/**
 * A 2 x 2 matrix
 */
pub trait BaseMat2<T,V>: BaseMat<T,V> {
    fn new(c0r0: T, c0r1: T,
           c1r0: T, c1r1: T) -> Self;

    fn from_cols(c0: V, c1: V) -> Self;

    fn from_angle(radians: T) -> Self;

    fn to_mat3(&self) -> Mat3<T>;

    fn to_mat4(&self) -> Mat4<T>;
}

/**
 * A 3 x 3 matrix
 */
pub trait BaseMat3<T,V>: BaseMat<T,V> {
    fn new(c0r0:T, c0r1:T, c0r2:T,
           c1r0:T, c1r1:T, c1r2:T,
           c2r0:T, c2r1:T, c2r2:T) -> Self;

    fn from_cols(c0: V, c1: V, c2: V) -> Self;

    fn from_angle_x(radians: T) -> Self;

    fn from_angle_y(radians: T) -> Self;

    fn from_angle_z(radians: T) -> Self;

    fn from_angle_xyz(radians_x: T, radians_y: T, radians_z: T) -> Self;

    fn from_angle_axis(radians: T, axis: &Vec3<T>) -> Self;

    fn from_axes(x: V, y: V, z: V) -> Self;

    fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> Self;

    fn to_mat4(&self) -> Mat4<T>;

    fn to_quat(&self) -> Quat<T>;
}

/**
 * A 4 x 4 matrix
 */
pub trait BaseMat4<T,V>: BaseMat<T,V> {
    fn new(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
           c1r0: T, c1r1: T, c1r2: T, c1r3: T,
           c2r0: T, c2r1: T, c2r2: T, c2r3: T,
           c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> Self;

    fn from_cols(c0: V, c1: V, c2: V, c3: V) -> Self;
}

/**
 *  A 2 x 2 column major matrix
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
 */
#[deriving(Eq)]
pub struct Mat2<T> { x: Vec2<T>, y: Vec2<T> }

impl<T:Copy + Float + NumAssign> BaseMat<T, Vec2<T>> for Mat2<T> {
    #[inline(always)]
    fn col<'a>(&'a self, i: uint) -> &'a Vec2<T> {
        unsafe { &'a transmute::<&'a Mat2<T>, &'a [Vec2<T>,..2]>(self)[i] }
    }

    #[inline(always)]
    fn row(&self, i: uint) -> Vec2<T> {
        BaseVec2::new(*self.col(0).index(i),
                      *self.col(1).index(i))
    }

    /**
     * Construct a 2 x 2 diagonal matrix with the major diagonal set to `value`
     *
     * # Arguments
     *
     * * `value` - the value to set the major diagonal to
     *
     * ~~~
     *        c0    c1
     *     +-----+-----+
     *  r0 | val |   0 |
     *     +-----+-----+
     *  r1 |   0 | val |
     *     +-----+-----+
     * ~~~
     */
    #[inline(always)]
    fn from_value(value: T) -> Mat2<T> {
        BaseMat2::new(value, Zero::zero(),
                      Zero::zero(), value)
    }

    /**
     * Returns the multiplicative identity matrix
     * ~~~
     *       c0   c1
     *     +----+----+
     *  r0 |  1 |  0 |
     *     +----+----+
     *  r1 |  0 |  1 |
     *     +----+----+
     * ~~~
     */
    #[inline(always)]
    fn identity() -> Mat2<T> {
        BaseMat2::new(  One::one::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(),   One::one::<T>())
    }

    /**
     * Returns the additive identity matrix
     * ~~~
     *       c0   c1
     *     +----+----+
     *  r0 |  0 |  0 |
     *     +----+----+
     *  r1 |  0 |  0 |
     *     +----+----+
     * ~~~
     */
    #[inline(always)]
    fn zero() -> Mat2<T> {
        BaseMat2::new(Zero::zero::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    fn mul_t(&self, value: T) -> Mat2<T> {
        BaseMat2::from_cols(self.col(0).mul_t(value),
                            self.col(1).mul_t(value))
    }

    #[inline(always)]
    fn mul_v(&self, vec: &Vec2<T>) -> Vec2<T> {
        BaseVec2::new(self.row(0).dot(vec),
                      self.row(1).dot(vec))
    }

    #[inline(always)]
    fn add_m(&self, other: &Mat2<T>) -> Mat2<T> {
        BaseMat2::from_cols(self.col(0).add_v(other.col(0)),
                            self.col(1).add_v(other.col(1)))
    }

    #[inline(always)]
    fn sub_m(&self, other: &Mat2<T>) -> Mat2<T> {
        BaseMat2::from_cols(self.col(0).sub_v(other.col(0)),
                            self.col(1).sub_v(other.col(1)))
    }

    #[inline(always)]
    fn mul_m(&self, other: &Mat2<T>) -> Mat2<T> {
        BaseMat2::new(self.row(0).dot(other.col(0)), self.row(1).dot(other.col(0)),
                      self.row(0).dot(other.col(1)), self.row(1).dot(other.col(1)))
    }

    fn dot(&self, other: &Mat2<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    fn determinant(&self) -> T {
       (*self.col(0).index(0)) *
       (*self.col(1).index(1)) -
       (*self.col(1).index(0)) *
       (*self.col(0).index(1))
    }

    fn trace(&self) -> T {
        (*self.col(0).index(0)) +
        (*self.col(1).index(1))
    }

    #[inline(always)]
    fn inverse(&self) -> Option<Mat2<T>> {
        let d = self.determinant();
        if d.approx_eq(&Zero::zero()) {
            None
        } else {
            Some(BaseMat2::new( self.col(1).index(1)/d, -self.col(0).index(1)/d,
                               -self.col(1).index(0)/d,  self.col(0).index(0)/d))
        }
    }

    #[inline(always)]
    fn transpose(&self) -> Mat2<T> {
        BaseMat2::new(*self.col(0).index(0), *self.col(1).index(0),
                      *self.col(0).index(1), *self.col(1).index(1))
    }

    #[inline(always)]
    fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut Vec2<T> {
        unsafe { &'a mut transmute::<&'a mut Mat2<T>, &'a mut [Vec2<T>,..2]>(self)[i] }
    }

    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        let tmp = *self.col(a);
        *self.col_mut(a) = *self.col(b);
        *self.col_mut(b) = tmp;
    }

    #[inline(always)]
    fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
    }

    #[inline(always)]
    fn set(&mut self, other: &Mat2<T>) {
        (*self) = (*other);
    }

    #[inline(always)]
    fn to_identity(&mut self) {
        (*self) = BaseMat::identity();
    }

    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = BaseMat::zero();
    }

    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        self.x.mul_self_t(value);
        self.y.mul_self_t(value);
    }

    #[inline(always)]
    fn add_self_m(&mut self, other: &Mat2<T>) {
        self.x.add_self_v(other.col(0));
        self.y.add_self_v(other.col(1));
    }

    #[inline(always)]
    fn sub_self_m(&mut self, other: &Mat2<T>) {
        self.x.sub_self_v(other.col(0));
        self.y.sub_self_v(other.col(1));
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
        let tmp01 = *self.col(0).index(1);
        let tmp10 = *self.col(1).index(0);

        *self.col_mut(0).index_mut(1) = *self.col(1).index(0);
        *self.col_mut(1).index_mut(0) = *self.col(0).index(1);

        *self.col_mut(1).index_mut(0) = tmp01;
        *self.col_mut(0).index_mut(1) = tmp10;
    }

    #[inline(always)]
    fn is_identity(&self) -> bool {
        self.approx_eq(&BaseMat::identity())
    }

    #[inline(always)]
    fn is_diagonal(&self) -> bool {
        self.col(0).index(1).approx_eq(&Zero::zero()) &&
        self.col(1).index(0).approx_eq(&Zero::zero())
    }

    #[inline(always)]
    fn is_rotated(&self) -> bool {
        !self.approx_eq(&BaseMat::identity())
    }

    #[inline(always)]
    fn is_symmetric(&self) -> bool {
        self.col(0).index(1).approx_eq(self.col(1).index(0)) &&
        self.col(1).index(0).approx_eq(self.col(0).index(1))
    }

    #[inline(always)]
    fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&Zero::zero())
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { transmute(self) }
    }
}

impl<T:Copy + Float + NumAssign> BaseMat2<T, Vec2<T>> for Mat2<T> {
    /**
     * Construct a 2 x 2 matrix
     *
     * # Arguments
     *
     * * `c0r0`, `c0r1` - the first column of the matrix
     * * `c1r0`, `c1r1` - the second column of the matrix
     *
     * ~~~
     *        c0     c1
     *     +------+------+
     *  r0 | c0r0 | c1r0 |
     *     +------+------+
     *  r1 | c0r1 | c1r1 |
     *     +------+------+
     * ~~~
     */
    #[inline(always)]
    fn new(c0r0: T, c0r1: T,
           c1r0: T, c1r1: T) -> Mat2<T> {
        BaseMat2::from_cols(BaseVec2::new::<T,Vec2<T>>(c0r0, c0r1),
                            BaseVec2::new::<T,Vec2<T>>(c1r0, c1r1))
    }

    /**
     * Construct a 2 x 2 matrix from column vectors
     *
     * # Arguments
     *
     * * `c0` - the first column vector of the matrix
     * * `c1` - the second column vector of the matrix
     *
     * ~~~
     *        c0     c1
     *     +------+------+
     *  r0 | c0.x | c1.x |
     *     +------+------+
     *  r1 | c0.y | c1.y |
     *     +------+------+
     * ~~~
     */
    #[inline(always)]
    fn from_cols(c0: Vec2<T>, c1: Vec2<T>) -> Mat2<T> {
        Mat2 { x: c0, y: c1 }
    }

    #[inline(always)]
    fn from_angle(radians: T) -> Mat2<T> {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        BaseMat2::new(cos_theta, -sin_theta,
                      sin_theta,  cos_theta)
    }

    /**
     * Returns the the matrix with an extra row and column added
     * ~~~
     *       c0   c1                 c0   c1   c2
     *     +----+----+             +----+----+----+
     *  r0 |  a |  b |          r0 |  a |  b |  0 |
     *     +----+----+             +----+----+----+
     *  r1 |  c |  d |    =>    r1 |  c |  d |  0 |
     *     +----+----+             +----+----+----+
     *                          r2 |  0 |  0 |  1 |
     *                             +----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn to_mat3(&self) -> Mat3<T> {
        BaseMat3::new(*self.col(0).index(0), *self.col(0).index(1), Zero::zero(),
                      *self.col(1).index(0), *self.col(1).index(1), Zero::zero(),
                      Zero::zero(), Zero::zero(), One::one())
    }

    /**
     * Returns the the matrix with an extra two rows and columns added
     * ~~~
     *       c0   c1                 c0   c1   c2   c3
     *     +----+----+             +----+----+----+----+
     *  r0 |  a |  b |          r0 |  a |  b |  0 |  0 |
     *     +----+----+             +----+----+----+----+
     *  r1 |  c |  d |    =>    r1 |  c |  d |  0 |  0 |
     *     +----+----+             +----+----+----+----+
     *                          r2 |  0 |  0 |  1 |  0 |
     *                             +----+----+----+----+
     *                          r3 |  0 |  0 |  0 |  1 |
     *                             +----+----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn to_mat4(&self) -> Mat4<T> {
        BaseMat4::new(*self.col(0).index(0), *self.col(0).index(1), Zero::zero(), Zero::zero(),
                      *self.col(1).index(0), *self.col(1).index(1), Zero::zero(), Zero::zero(),
                      Zero::zero(), Zero::zero(), One::one(), Zero::zero(),
                      Zero::zero(), Zero::zero(), Zero::zero(), One::one())
    }
}

impl<T:Copy + Float + NumAssign> Neg<Mat2<T>> for Mat2<T> {
    #[inline(always)]
    fn neg(&self) -> Mat2<T> {
        BaseMat2::from_cols(-self.col(0), -self.col(1))
    }
}

impl<T:Copy + Float + NumAssign> ApproxEq<T> for Mat2<T> {
    #[inline(always)]
    fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    fn approx_eq(&self, other: &Mat2<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    fn approx_eq_eps(&self, other: &Mat2<T>, epsilon: &T) -> bool {
        self.col(0).approx_eq_eps(other.col(0), epsilon) &&
        self.col(1).approx_eq_eps(other.col(1), epsilon)
    }
}

/**
 *  A 3 x 3 column major matrix
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
 */
#[deriving(Eq)]
pub struct Mat3<T> { x: Vec3<T>, y: Vec3<T>, z: Vec3<T> }

impl<T:Copy + Float + NumAssign> BaseMat<T, Vec3<T>> for Mat3<T> {
    #[inline(always)]
    fn col<'a>(&'a self, i: uint) -> &'a Vec3<T> {
        unsafe { &'a transmute::<&'a Mat3<T>, &'a [Vec3<T>,..3]>(self)[i] }
    }

    #[inline(always)]
    fn row(&self, i: uint) -> Vec3<T> {
        BaseVec3::new(*self.col(0).index(i),
                      *self.col(1).index(i),
                      *self.col(2).index(i))
    }

    /**
     * Construct a 3 x 3 diagonal matrix with the major diagonal set to `value`
     *
     * # Arguments
     *
     * * `value` - the value to set the major diagonal to
     *
     * ~~~
     *        c0    c1    c2
     *     +-----+-----+-----+
     *  r0 | val |   0 |   0 |
     *     +-----+-----+-----+
     *  r1 |   0 | val |   0 |
     *     +-----+-----+-----+
     *  r2 |   0 |   0 | val |
     *     +-----+-----+-----+
     * ~~~
     */
    #[inline(always)]
    fn from_value(value: T) -> Mat3<T> {
        BaseMat3::new(value, Zero::zero(), Zero::zero(),
                      Zero::zero(), value, Zero::zero(),
                      Zero::zero(), Zero::zero(), value)
    }

    /**
     * Returns the multiplicative identity matrix
     * ~~~
     *       c0   c1   c2
     *     +----+----+----+
     *  r0 |  1 |  0 |  0 |
     *     +----+----+----+
     *  r1 |  0 |  1 |  0 |
     *     +----+----+----+
     *  r2 |  0 |  0 |  1 |
     *     +----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn identity() -> Mat3<T> {
        BaseMat3::new(One::one::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(), One::one::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(), Zero::zero::<T>(), One::one::<T>())
    }

    /**
     * Returns the additive identity matrix
     * ~~~
     *       c0   c1   c2
     *     +----+----+----+
     *  r0 |  0 |  0 |  0 |
     *     +----+----+----+
     *  r1 |  0 |  0 |  0 |
     *     +----+----+----+
     *  r2 |  0 |  0 |  0 |
     *     +----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn zero() -> Mat3<T> {
        BaseMat3::new(Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    fn mul_t(&self, value: T) -> Mat3<T> {
        BaseMat3::from_cols(self.col(0).mul_t(value),
                            self.col(1).mul_t(value),
                            self.col(2).mul_t(value))
    }

    #[inline(always)]
    fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T> {
        BaseVec3::new(self.row(0).dot(vec),
                      self.row(1).dot(vec),
                      self.row(2).dot(vec))
    }

    #[inline(always)]
    fn add_m(&self, other: &Mat3<T>) -> Mat3<T> {
        BaseMat3::from_cols(self.col(0).add_v(other.col(0)),
                            self.col(1).add_v(other.col(1)),
                            self.col(2).add_v(other.col(2)))
    }

    #[inline(always)]
    fn sub_m(&self, other: &Mat3<T>) -> Mat3<T> {
        BaseMat3::from_cols(self.col(0).sub_v(other.col(0)),
                            self.col(1).sub_v(other.col(1)),
                            self.col(2).sub_v(other.col(2)))
    }

    #[inline(always)]
    fn mul_m(&self, other: &Mat3<T>) -> Mat3<T> {
        BaseMat3::new(self.row(0).dot(other.col(0)),
                      self.row(1).dot(other.col(0)),
                      self.row(2).dot(other.col(0)),

                      self.row(0).dot(other.col(1)),
                      self.row(1).dot(other.col(1)),
                      self.row(2).dot(other.col(1)),

                      self.row(0).dot(other.col(2)),
                      self.row(1).dot(other.col(2)),
                      self.row(2).dot(other.col(2)))
    }

    fn dot(&self, other: &Mat3<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    fn determinant(&self) -> T {
        self.col(0).dot(&self.col(1).cross(self.col(2)))
    }

    fn trace(&self) -> T {
        *self.col(0).index(0) +
        *self.col(1).index(1) +
        *self.col(2).index(2)
    }

    // #[inline(always)]
    fn inverse(&self) -> Option<Mat3<T>> {
        let d = self.determinant();
        if d.approx_eq(&Zero::zero()) {
            None
        } else {
            let m: Mat3<T> = BaseMat3::from_cols(self.col(1).cross(self.col(2)).div_t(d),
                                                 self.col(2).cross(self.col(0)).div_t(d),
                                                 self.col(0).cross(self.col(1)).div_t(d));
            Some(m.transpose())
        }
    }

    #[inline(always)]
    fn transpose(&self) -> Mat3<T> {
        BaseMat3::new(*self.col(0).index(0), *self.col(1).index(0), *self.col(2).index(0),
                      *self.col(0).index(1), *self.col(1).index(1), *self.col(2).index(1),
                      *self.col(0).index(2), *self.col(1).index(2), *self.col(2).index(2))
    }

    #[inline(always)]
    fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut Vec3<T> {
        unsafe { &'a mut transmute::<&'a mut Mat3<T>, &'a mut [Vec3<T>,..3]>(self)[i] }
    }

    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        let tmp = *self.col(a);
        *self.col_mut(a) = *self.col(b);
        *self.col_mut(b) = tmp;
    }

    #[inline(always)]
    fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
        self.z.swap(a, b);
    }

    #[inline(always)]
    fn set(&mut self, other: &Mat3<T>) {
        (*self) = (*other);
    }

    #[inline(always)]
    fn to_identity(&mut self) {
        (*self) = BaseMat::identity();
    }

    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = BaseMat::zero();
    }

    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(value);
        self.col_mut(1).mul_self_t(value);
        self.col_mut(2).mul_self_t(value);
    }

    #[inline(always)]
    fn add_self_m(&mut self, other: &Mat3<T>) {
        self.col_mut(0).add_self_v(other.col(0));
        self.col_mut(1).add_self_v(other.col(1));
        self.col_mut(2).add_self_v(other.col(2));
    }

    #[inline(always)]
    fn sub_self_m(&mut self, other: &Mat3<T>) {
        self.col_mut(0).sub_self_v(other.col(0));
        self.col_mut(1).sub_self_v(other.col(1));
        self.col_mut(2).sub_self_v(other.col(2));
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
        let tmp01 = *self.col(0).index(1);
        let tmp02 = *self.col(0).index(2);
        let tmp10 = *self.col(1).index(0);
        let tmp12 = *self.col(1).index(2);
        let tmp20 = *self.col(2).index(0);
        let tmp21 = *self.col(2).index(1);

        *self.col_mut(0).index_mut(1) = *self.col(1).index(0);
        *self.col_mut(0).index_mut(2) = *self.col(2).index(0);
        *self.col_mut(1).index_mut(0) = *self.col(0).index(1);
        *self.col_mut(1).index_mut(2) = *self.col(2).index(1);
        *self.col_mut(2).index_mut(0) = *self.col(0).index(2);
        *self.col_mut(2).index_mut(1) = *self.col(1).index(2);

        *self.col_mut(1).index_mut(0) = tmp01;
        *self.col_mut(2).index_mut(0) = tmp02;
        *self.col_mut(0).index_mut(1) = tmp10;
        *self.col_mut(2).index_mut(1) = tmp12;
        *self.col_mut(0).index_mut(2) = tmp20;
        *self.col_mut(1).index_mut(2) = tmp21;
    }

    #[inline(always)]
    fn is_identity(&self) -> bool {
        self.approx_eq(&BaseMat::identity())
    }

    #[inline(always)]
    fn is_diagonal(&self) -> bool {
        self.col(0).index(1).approx_eq(&Zero::zero()) &&
        self.col(0).index(2).approx_eq(&Zero::zero()) &&

        self.col(1).index(0).approx_eq(&Zero::zero()) &&
        self.col(1).index(2).approx_eq(&Zero::zero()) &&

        self.col(2).index(0).approx_eq(&Zero::zero()) &&
        self.col(2).index(1).approx_eq(&Zero::zero())
    }

    #[inline(always)]
    fn is_rotated(&self) -> bool {
        !self.approx_eq(&BaseMat::identity())
    }

    #[inline(always)]
    fn is_symmetric(&self) -> bool {
        self.col(0).index(1).approx_eq(self.col(1).index(0)) &&
        self.col(0).index(2).approx_eq(self.col(2).index(0)) &&

        self.col(1).index(0).approx_eq(self.col(0).index(1)) &&
        self.col(1).index(2).approx_eq(self.col(2).index(1)) &&

        self.col(2).index(0).approx_eq(self.col(0).index(2)) &&
        self.col(2).index(1).approx_eq(self.col(1).index(2))
    }

    #[inline(always)]
    fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&Zero::zero())
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { transmute(self) }
    }
}

impl<T:Copy + Float + NumAssign> BaseMat3<T, Vec3<T>> for Mat3<T> {
    /**
     * Construct a 3 x 3 matrix
     *
     * # Arguments
     *
     * * `c0r0`, `c0r1`, `c0r2` - the first column of the matrix
     * * `c1r0`, `c1r1`, `c1r2` - the second column of the matrix
     * * `c2r0`, `c2r1`, `c2r2` - the third column of the matrix
     *
     * ~~~
     *         c0     c1     c2
     *      +------+------+------+
     *   r0 | c0r0 | c1r0 | c2r0 |
     *      +------+------+------+
     *   r1 | c0r1 | c1r1 | c2r1 |
     *      +------+------+------+
     *   r2 | c0r2 | c1r2 | c2r2 |
     *      +------+------+------+
     * ~~~
     */
    #[inline(always)]
    fn new(c0r0:T, c0r1:T, c0r2:T,
           c1r0:T, c1r1:T, c1r2:T,
           c2r0:T, c2r1:T, c2r2:T) -> Mat3<T> {
        BaseMat3::from_cols(BaseVec3::new::<T,Vec3<T>>(c0r0, c0r1, c0r2),
                            BaseVec3::new::<T,Vec3<T>>(c1r0, c1r1, c1r2),
                            BaseVec3::new::<T,Vec3<T>>(c2r0, c2r1, c2r2))
    }

    /**
     * Construct a 3 x 3 matrix from column vectors
     *
     * # Arguments
     *
     * * `c0` - the first column vector of the matrix
     * * `c1` - the second column vector of the matrix
     * * `c2` - the third column vector of the matrix
     *
     * ~~~
     *        c0     c1     c2
     *     +------+------+------+
     *  r0 | c0.x | c1.x | c2.x |
     *     +------+------+------+
     *  r1 | c0.y | c1.y | c2.y |
     *     +------+------+------+
     *  r2 | c0.z | c1.z | c2.z |
     *     +------+------+------+
     * ~~~
     */
    #[inline(always)]
    fn from_cols(c0: Vec3<T>, c1: Vec3<T>, c2: Vec3<T>) -> Mat3<T> {
        Mat3 { x: c0, y: c1, z: c2 }
    }

    /**
     * Construct a matrix from an angular rotation around the `x` axis
     */
    #[inline(always)]
    fn from_angle_x(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        BaseMat3::new(  One::one(), Zero::zero(), Zero::zero(),
                      Zero::zero(),    cos_theta,    sin_theta,
                      Zero::zero(),   -sin_theta,    cos_theta)
    }

    /**
     * Construct a matrix from an angular rotation around the `y` axis
     */
    #[inline(always)]
    fn from_angle_y(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        BaseMat3::new(   cos_theta, Zero::zero(),   -sin_theta,
                      Zero::zero(),   One::one(), Zero::zero(),
                         sin_theta, Zero::zero(),    cos_theta)
    }

    /**
     * Construct a matrix from an angular rotation around the `z` axis
     */
    #[inline(always)]
    fn from_angle_z(radians: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        BaseMat3::new(   cos_theta,    sin_theta, Zero::zero(),
                        -sin_theta,    cos_theta, Zero::zero(),
                      Zero::zero(), Zero::zero(),   One::one())
    }

    /**
     * Construct a matrix from Euler angles
     *
     * # Arguments
     *
     * * `theta_x` - the angular rotation around the `x` axis (pitch)
     * * `theta_y` - the angular rotation around the `y` axis (yaw)
     * * `theta_z` - the angular rotation around the `z` axis (roll)
     */
    #[inline(always)]
    fn from_angle_xyz(radians_x: T, radians_y: T, radians_z: T) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
        let cx = radians_x.cos();
        let sx = radians_x.sin();
        let cy = radians_y.cos();
        let sy = radians_y.sin();
        let cz = radians_z.cos();
        let sz = radians_z.sin();

        BaseMat3::new(            cy*cz,             cy*sz,   -sy,
                      -cx*sz + sx*sy*cz,  cx*cz + sx*sy*sz, sx*cy,
                       sx*sz + cx*sy*cz, -sx*cz + cx*sy*sz, cx*cy)
    }

    /**
     * Construct a matrix from an axis and an angular rotation
     */
    #[inline(always)]
    fn from_angle_axis(radians: T, axis: &Vec3<T>) -> Mat3<T> {
        let c = radians.cos();
        let s = radians.sin();
        let _1_c =  One::one::<T>() - c;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        BaseMat3::new(_1_c*x*x + c,   _1_c*x*y + s*z, _1_c*x*z - s*y,
                      _1_c*x*y - s*z, _1_c*y*y + c,   _1_c*y*z + s*x,
                      _1_c*x*z + s*y, _1_c*y*z - s*x, _1_c*z*z + c)
    }

    #[inline(always)]
    fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat3<T> {
        BaseMat3::from_cols(x, y, z)
    }

    #[inline(always)]
    fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> Mat3<T> {
        let dir_ = dir.normalize();
        let side = dir_.cross(&up.normalize());
        let up_  = side.cross(&dir_).normalize();

        BaseMat3::from_axes(up_, side, dir_)
    }

    /**
     * Returns the the matrix with an extra row and column added
     * ~~~
     *       c0   c1   c2                 c0   c1   c2   c3
     *     +----+----+----+             +----+----+----+----+
     *  r0 |  a |  b |  c |          r0 |  a |  b |  c |  0 |
     *     +----+----+----+             +----+----+----+----+
     *  r1 |  d |  e |  f |    =>    r1 |  d |  e |  f |  0 |
     *     +----+----+----+             +----+----+----+----+
     *  r2 |  g |  h |  i |          r2 |  g |  h |  i |  0 |
     *     +----+----+----+             +----+----+----+----+
     *                               r3 |  0 |  0 |  0 |  1 |
     *                                  +----+----+----+----+
     * ~~~
     */
    #[inline(always)]
    fn to_mat4(&self) -> Mat4<T> {
        BaseMat4::new(*self.col(0).index(0), *self.col(0).index(1), *self.col(0).index(2), Zero::zero(),
                      *self.col(1).index(0), *self.col(1).index(1), *self.col(1).index(2), Zero::zero(),
                      *self.col(2).index(0), *self.col(2).index(1), *self.col(2).index(2), Zero::zero(),
                      Zero::zero(), Zero::zero(), Zero::zero(),   One::one())
    }

    /**
     * Convert the matrix to a quaternion
     */
    #[inline(always)]
    fn to_quat(&self) -> Quat<T> {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf

        let mut s;
        let w, x, y, z;
        let trace = self.trace();

        let _1:   T = num::cast(1.0);
        let half: T = num::cast(0.5);

        cond! (
            (trace >= Zero::zero()) {
                s = (_1 + trace).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.col(1).index(2) - *self.col(2).index(1)) * s;
                y = (*self.col(2).index(0) - *self.col(0).index(2)) * s;
                z = (*self.col(0).index(1) - *self.col(1).index(0)) * s;
            }
            ((*self.col(0).index(0) > *self.col(1).index(1))
            && (*self.col(0).index(0) > *self.col(2).index(2))) {
                s = (half + (*self.col(0).index(0) -
                             *self.col(1).index(1) -
                             *self.col(2).index(2))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.col(0).index(1) - *self.col(1).index(0)) * s;
                y = (*self.col(2).index(0) - *self.col(0).index(2)) * s;
                z = (*self.col(1).index(2) - *self.col(2).index(1)) * s;
            }
            (*self.col(1).index(1) > *self.col(2).index(2)) {
                s = (half + (*self.col(1).index(1) -
                             *self.col(0).index(0) -
                             *self.col(2).index(2))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.col(0).index(1) - *self.col(1).index(0)) * s;
                y = (*self.col(1).index(2) - *self.col(2).index(1)) * s;
                z = (*self.col(2).index(0) - *self.col(0).index(2)) * s;
            }
            _ {
                s = (half + (*self.col(2).index(2) -
                             *self.col(0).index(0) -
                             *self.col(1).index(1))).sqrt();
                w = half * s;
                s = half / s;
                x = (*self.col(2).index(0) - *self.col(0).index(2)) * s;
                y = (*self.col(1).index(2) - *self.col(2).index(1)) * s;
                z = (*self.col(0).index(1) - *self.col(1).index(0)) * s;
            }
        )

        Quat::new(w, x, y, z)
    }
}

impl<T:Copy + Float + NumAssign> Neg<Mat3<T>> for Mat3<T> {
    #[inline(always)]
    fn neg(&self) -> Mat3<T> {
        BaseMat3::from_cols(-self.col(0), -self.col(1), -self.col(2))
    }
}

impl<T:Copy + Float + NumAssign> ApproxEq<T> for Mat3<T> {
    #[inline(always)]
    fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    fn approx_eq(&self, other: &Mat3<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    fn approx_eq_eps(&self, other: &Mat3<T>, epsilon: &T) -> bool {
        self.col(0).approx_eq_eps(other.col(0), epsilon) &&
        self.col(1).approx_eq_eps(other.col(1), epsilon) &&
        self.col(2).approx_eq_eps(other.col(2), epsilon)
    }
}

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

impl<T:Copy + Float + NumAssign> BaseMat<T, Vec4<T>> for Mat4<T> {
    #[inline(always)]
    fn col<'a>(&'a self, i: uint) -> &'a Vec4<T> {
        unsafe { &'a transmute::<&'a Mat4<T>, &'a [Vec4<T>,..4]>(self)[i] }
    }

    #[inline(always)]
    fn row(&self, i: uint) -> Vec4<T> {
        BaseVec4::new(*self.col(0).index(i),
                      *self.col(1).index(i),
                      *self.col(2).index(i),
                      *self.col(3).index(i))
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
        BaseMat4::new(value, Zero::zero(), Zero::zero(), Zero::zero(),
                      Zero::zero(), value, Zero::zero(), Zero::zero(),
                      Zero::zero(), Zero::zero(), value, Zero::zero(),
                      Zero::zero(), Zero::zero(), Zero::zero(), value)
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
        BaseMat4::new(One::one::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(), One::one::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(), Zero::zero::<T>(), One::one::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), One::one::<T>())
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
        BaseMat4::new(Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(),
                      Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline(always)]
    fn mul_t(&self, value: T) -> Mat4<T> {
        BaseMat4::from_cols(self.col(0).mul_t(value),
                            self.col(1).mul_t(value),
                            self.col(2).mul_t(value),
                            self.col(3).mul_t(value))
    }

    #[inline(always)]
    fn mul_v(&self, vec: &Vec4<T>) -> Vec4<T> {
        BaseVec4::new(self.row(0).dot(vec),
                      self.row(1).dot(vec),
                      self.row(2).dot(vec),
                      self.row(3).dot(vec))
    }

    #[inline(always)]
    fn add_m(&self, other: &Mat4<T>) -> Mat4<T> {
        BaseMat4::from_cols(self.col(0).add_v(other.col(0)),
                            self.col(1).add_v(other.col(1)),
                            self.col(2).add_v(other.col(2)),
                            self.col(3).add_v(other.col(3)))
    }

    #[inline(always)]
    fn sub_m(&self, other: &Mat4<T>) -> Mat4<T> {
        BaseMat4::from_cols(self.col(0).sub_v(other.col(0)),
                            self.col(1).sub_v(other.col(1)),
                            self.col(2).sub_v(other.col(2)),
                            self.col(3).sub_v(other.col(3)))
    }

    #[inline(always)]
    fn mul_m(&self, other: &Mat4<T>) -> Mat4<T> {
        BaseMat4::new(self.row(0).dot(other.col(0)),
                      self.row(1).dot(other.col(0)),
                      self.row(2).dot(other.col(0)),
                      self.row(3).dot(other.col(0)),

                      self.row(0).dot(other.col(1)),
                      self.row(1).dot(other.col(1)),
                      self.row(2).dot(other.col(1)),
                      self.row(3).dot(other.col(1)),

                      self.row(0).dot(other.col(2)),
                      self.row(1).dot(other.col(2)),
                      self.row(2).dot(other.col(2)),
                      self.row(3).dot(other.col(2)),

                      self.row(0).dot(other.col(3)),
                      self.row(1).dot(other.col(3)),
                      self.row(2).dot(other.col(3)),
                      self.row(3).dot(other.col(3)))

    }

    fn dot(&self, other: &Mat4<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    fn determinant(&self) -> T {
        let m0: Mat3<T> = BaseMat3::new(*self.col(1).index(1), *self.col(2).index(1), *self.col(3).index(1),
                                        *self.col(1).index(2), *self.col(2).index(2), *self.col(3).index(2),
                                        *self.col(1).index(3), *self.col(2).index(3), *self.col(3).index(3));
        let m1: Mat3<T> = BaseMat3::new(*self.col(0).index(1), *self.col(2).index(1), *self.col(3).index(1),
                                        *self.col(0).index(2), *self.col(2).index(2), *self.col(3).index(2),
                                        *self.col(0).index(3), *self.col(2).index(3), *self.col(3).index(3));
        let m2: Mat3<T> = BaseMat3::new(*self.col(0).index(1), *self.col(1).index(1), *self.col(3).index(1),
                                        *self.col(0).index(2), *self.col(1).index(2), *self.col(3).index(2),
                                        *self.col(0).index(3), *self.col(1).index(3), *self.col(3).index(3));
        let m3: Mat3<T> = BaseMat3::new(*self.col(0).index(1), *self.col(1).index(1), *self.col(2).index(1),
                                        *self.col(0).index(2), *self.col(1).index(2), *self.col(2).index(2),
                                        *self.col(0).index(3), *self.col(1).index(3), *self.col(2).index(3));

        self.col(0).index(0) * m0.determinant() -
        self.col(1).index(0) * m1.determinant() +
        self.col(2).index(0) * m2.determinant() -
        self.col(3).index(0) * m3.determinant()
    }

    fn trace(&self) -> T {
        *self.col(0).index(0) +
        *self.col(1).index(1) +
        *self.col(2).index(2) +
        *self.col(3).index(3)
    }

    fn inverse(&self) -> Option<Mat4<T>> {
        let d = self.determinant();
        if d.approx_eq(&Zero::zero()) {
            None
        } else {

            // Gauss Jordan Elimination with partial pivoting
            // So take this matrix, A, augmented with the identity
            // and essentially reduce [A|I]

            let mut A = *self;
            let mut I: Mat4<T> = BaseMat::identity();

            for uint::range(0, 4) |j| {
                // Find largest element in col j
                let mut i1 = j;
                for uint::range(j + 1, 4) |i| {
                    if A.col(j).index(i).abs() > A.col(j).index(i1).abs() {
                        i1 = i;
                    }
                }

                // Swap columns i1 and j in A and I to
                // put pivot on diagonal
                A.swap_cols(i1, j);
                I.swap_cols(i1, j);

                // Scale col j to have a unit diagonal
                let ajj = *A.col(j).index(j);
                I.col_mut(j).div_self_t(ajj);
                A.col_mut(j).div_self_t(ajj);

                // Eliminate off-diagonal elems in col j of A,
                // doing identical ops to I
                for uint::range(0, 4) |i| {
                    if i != j {
                        let ij_mul_aij = I.col(j).mul_t(*A.col(i).index(j));
                        let aj_mul_aij = A.col(j).mul_t(*A.col(i).index(j));
                        I.col_mut(i).sub_self_v(&ij_mul_aij);
                        A.col_mut(i).sub_self_v(&aj_mul_aij);
                    }
                }
            }
            Some(I)
        }
    }

    #[inline(always)]
    fn transpose(&self) -> Mat4<T> {
        BaseMat4::new(*self.col(0).index(0), *self.col(1).index(0), *self.col(2).index(0), *self.col(3).index(0),
                      *self.col(0).index(1), *self.col(1).index(1), *self.col(2).index(1), *self.col(3).index(1),
                      *self.col(0).index(2), *self.col(1).index(2), *self.col(2).index(2), *self.col(3).index(2),
                      *self.col(0).index(3), *self.col(1).index(3), *self.col(2).index(3), *self.col(3).index(3))
    }

    #[inline(always)]
    fn col_mut<'a>(&'a mut self, i: uint) -> &'a mut Vec4<T> {
        unsafe { &'a mut transmute::<&'a mut Mat4<T>, &'a mut [Vec4<T>,..4]>(self)[i] }
    }

    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        let tmp = *self.col(a);
        *self.col_mut(a) = *self.col(b);
        *self.col_mut(b) = tmp;
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
        (*self) = BaseMat::identity();
    }

    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = BaseMat::zero();
    }

    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(value);
        self.col_mut(1).mul_self_t(value);
        self.col_mut(2).mul_self_t(value);
        self.col_mut(3).mul_self_t(value);
    }

    #[inline(always)]
    fn add_self_m(&mut self, other: &Mat4<T>) {
        self.col_mut(0).add_self_v(other.col(0));
        self.col_mut(1).add_self_v(other.col(1));
        self.col_mut(2).add_self_v(other.col(2));
        self.col_mut(3).add_self_v(other.col(3));
    }

    #[inline(always)]
    fn sub_self_m(&mut self, other: &Mat4<T>) {
        self.col_mut(0).sub_self_v(other.col(0));
        self.col_mut(1).sub_self_v(other.col(1));
        self.col_mut(2).sub_self_v(other.col(2));
        self.col_mut(3).sub_self_v(other.col(3));
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
        let tmp01 = *self.col(0).index(1);
        let tmp02 = *self.col(0).index(2);
        let tmp03 = *self.col(0).index(3);
        let tmp10 = *self.col(1).index(0);
        let tmp12 = *self.col(1).index(2);
        let tmp13 = *self.col(1).index(3);
        let tmp20 = *self.col(2).index(0);
        let tmp21 = *self.col(2).index(1);
        let tmp23 = *self.col(2).index(3);
        let tmp30 = *self.col(3).index(0);
        let tmp31 = *self.col(3).index(1);
        let tmp32 = *self.col(3).index(2);

        *self.col_mut(0).index_mut(1) = *self.col(1).index(0);
        *self.col_mut(0).index_mut(2) = *self.col(2).index(0);
        *self.col_mut(0).index_mut(3) = *self.col(3).index(0);
        *self.col_mut(1).index_mut(0) = *self.col(0).index(1);
        *self.col_mut(1).index_mut(2) = *self.col(2).index(1);
        *self.col_mut(1).index_mut(3) = *self.col(3).index(1);
        *self.col_mut(2).index_mut(0) = *self.col(0).index(2);
        *self.col_mut(2).index_mut(1) = *self.col(1).index(2);
        *self.col_mut(2).index_mut(3) = *self.col(3).index(2);
        *self.col_mut(3).index_mut(0) = *self.col(0).index(3);
        *self.col_mut(3).index_mut(1) = *self.col(1).index(3);
        *self.col_mut(3).index_mut(2) = *self.col(2).index(3);

        *self.col_mut(1).index_mut(0) = tmp01;
        *self.col_mut(2).index_mut(0) = tmp02;
        *self.col_mut(3).index_mut(0) = tmp03;
        *self.col_mut(0).index_mut(1) = tmp10;
        *self.col_mut(2).index_mut(1) = tmp12;
        *self.col_mut(3).index_mut(1) = tmp13;
        *self.col_mut(0).index_mut(2) = tmp20;
        *self.col_mut(1).index_mut(2) = tmp21;
        *self.col_mut(3).index_mut(2) = tmp23;
        *self.col_mut(0).index_mut(3) = tmp30;
        *self.col_mut(1).index_mut(3) = tmp31;
        *self.col_mut(2).index_mut(3) = tmp32;
    }

    #[inline(always)]
    fn is_identity(&self) -> bool {
        self.approx_eq(&BaseMat::identity())
    }

    #[inline(always)]
    fn is_diagonal(&self) -> bool {
        self.col(0).index(1).approx_eq(&Zero::zero()) &&
        self.col(0).index(2).approx_eq(&Zero::zero()) &&
        self.col(0).index(3).approx_eq(&Zero::zero()) &&

        self.col(1).index(0).approx_eq(&Zero::zero()) &&
        self.col(1).index(2).approx_eq(&Zero::zero()) &&
        self.col(1).index(3).approx_eq(&Zero::zero()) &&

        self.col(2).index(0).approx_eq(&Zero::zero()) &&
        self.col(2).index(1).approx_eq(&Zero::zero()) &&
        self.col(2).index(3).approx_eq(&Zero::zero()) &&

        self.col(3).index(0).approx_eq(&Zero::zero()) &&
        self.col(3).index(1).approx_eq(&Zero::zero()) &&
        self.col(3).index(2).approx_eq(&Zero::zero())
    }

    #[inline(always)]
    fn is_rotated(&self) -> bool {
        !self.approx_eq(&BaseMat::identity())
    }

    #[inline(always)]
    fn is_symmetric(&self) -> bool {
        self.col(0).index(1).approx_eq(self.col(1).index(0)) &&
        self.col(0).index(2).approx_eq(self.col(2).index(0)) &&
        self.col(0).index(3).approx_eq(self.col(3).index(0)) &&

        self.col(1).index(0).approx_eq(self.col(0).index(1)) &&
        self.col(1).index(2).approx_eq(self.col(2).index(1)) &&
        self.col(1).index(3).approx_eq(self.col(3).index(1)) &&

        self.col(2).index(0).approx_eq(self.col(0).index(2)) &&
        self.col(2).index(1).approx_eq(self.col(1).index(2)) &&
        self.col(2).index(3).approx_eq(self.col(3).index(2)) &&

        self.col(3).index(0).approx_eq(self.col(0).index(3)) &&
        self.col(3).index(1).approx_eq(self.col(1).index(3)) &&
        self.col(3).index(2).approx_eq(self.col(2).index(3))
    }

    #[inline(always)]
    fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&Zero::zero())
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { transmute(self) }
    }
}

impl<T:Copy + Float + NumAssign> BaseMat4<T, Vec4<T>> for Mat4<T> {
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
        BaseMat4::from_cols(BaseVec4::new::<T,Vec4<T>>(c0r0, c0r1, c0r2, c0r3),
                            BaseVec4::new::<T,Vec4<T>>(c1r0, c1r1, c1r2, c1r3),
                            BaseVec4::new::<T,Vec4<T>>(c2r0, c2r1, c2r2, c2r3),
                            BaseVec4::new::<T,Vec4<T>>(c3r0, c3r1, c3r2, c3r3))
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
    fn from_cols(c0: Vec4<T>, c1: Vec4<T>, c2: Vec4<T>, c3: Vec4<T>) -> Mat4<T> {
        Mat4 { x: c0, y: c1, z: c2, w: c3 }
    }
}

impl<T:Copy + Float + NumAssign> Neg<Mat4<T>> for Mat4<T> {
    #[inline(always)]
    fn neg(&self) -> Mat4<T> {
        BaseMat4::from_cols(-self.col(0), -self.col(1), -self.col(2), -self.col(3))
    }
}

impl<T:Copy + Float + NumAssign> ApproxEq<T> for Mat4<T> {
    #[inline(always)]
    fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    fn approx_eq(&self, other: &Mat4<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    fn approx_eq_eps(&self, other: &Mat4<T>, epsilon: &T) -> bool {
        self.col(0).approx_eq_eps(other.col(0), epsilon) &&
        self.col(1).approx_eq_eps(other.col(1), epsilon) &&
        self.col(2).approx_eq_eps(other.col(2), epsilon) &&
        self.col(3).approx_eq_eps(other.col(3), epsilon)
    }
}
