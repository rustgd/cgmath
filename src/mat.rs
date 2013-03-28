use core::cmp::Eq;
use std::cmp::FuzzyEq;

use vec::Vec3;
use quat::Quat;

pub use mat2::{Mat2, mat2, dmat2};
pub use mat3::{Mat3, mat3, dmat3};
pub use mat4::{Mat4, mat4, dmat4};

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
pub trait Matrix<T,V>: Index<uint, V> + Eq + Neg<Self> {
    /**
     * # Return value
     *
     * The column vector at `i`
     */
    fn col(&self, i: uint) -> V;

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
pub trait Matrix2<T,V>: Matrix<T,V> {
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
pub trait Matrix3<T,V>: Matrix<T,V> {
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
pub trait Matrix4<T,V>: Matrix<T,V> {
    fn new(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
                       c1r0: T, c1r1: T, c1r2: T, c1r3: T,
                       c2r0: T, c2r1: T, c2r2: T, c2r3: T,
                       c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> Self;

    fn from_cols(c0: V, c1: V, c2: V, c3: V) -> Self;
}

/**
 * A mutable matrix
 */
pub trait MutableMatrix<T,V>: Matrix<T,V> {
    /**
     * # Return value
     *
     * A mutable reference to the column at `i`
     */
    fn col_mut(&mut self, i: uint) -> &'self mut V;

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
}