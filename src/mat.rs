use core::cmp::Eq;

use std::cmp::FuzzyEq;

use angle::Angle;
use quat::ToQuat;

pub mod mat2;
pub mod mat3;
pub mod mat4;

pub use mat2::Mat2;
pub use mat3::Mat3;
pub use mat4::Mat4;

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
pub trait Matrix<T,V>: Index<uint, V> Eq Neg<self> {
    /**
     * # Return value
     *
     * The column vector at `i`
     */
    pure fn col(&self, i: uint) -> V;
    
    /**
     * # Return value
     *
     * The row vector at `i`
     */
    pure fn row(&self, i: uint) -> V;
    
    /**
     * # Return value
     *
     * The identity matrix
     */
    static pure fn identity() -> self;
    
    /**
     * # Return value
     *
     * A matrix with all elements set to zero
     */
    static pure fn zero() -> self;
    
    /**
     * # Return value
     *
     * The scalar multiplication of this matrix and `value`
     */
    pure fn mul_t(&self, value: T) -> self;
    
    /**
     * # Return value
     *
     * The matrix vector product of the matrix and `vec`
     */
    pure fn mul_v(&self, vec: &V) -> V;
    
    /**
     * # Return value
     *
     * The matrix addition of the matrix and `other`
     */
    pure fn add_m(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The difference between the matrix and `other`
     */
    pure fn sub_m(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The matrix product of the matrix and `other`
     */
    pure fn mul_m(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The matrix dot product of the matrix and `other`
     */
    pure fn dot(&self, other: &self) -> T;
    
    /**
     * # Return value
     *
     * The determinant of the matrix
     */
    pure fn determinant(&self) -> T;
    
    /**
     * # Return value
     *
     * The sum of the main diagonal of the matrix
     */
    pure fn trace(&self) -> T;
    
    /**
     * Returns the inverse of the matrix
     * 
     * # Return value
     *
     * * `Some(m)` - if the inversion was successful, where `m` is the inverted matrix
     * * `None` - if the inversion was unsuccessful (because the matrix was not invertable)
     */
    pure fn inverse(&self) -> Option<self>;
    
    /**
     * # Return value
     *
     * The transposed matrix
     */
    pure fn transpose(&self) -> self;
    
    /**
     * Check to see if the matrix is an identity matrix
     *
     * # Return value
     * 
     * `true` if the matrix is approximately equal to the identity matrix
     */
    pure fn is_identity(&self) -> bool;
    
    /**
     * Check to see if the matrix is diagonal
     *
     * # Return value
     *
     * `true` all the elements outside the main diagonal are approximately
     * equal to zero.
     */
    pure fn is_diagonal(&self) -> bool;
    
    /**
     * Check to see if the matrix is rotated
     *
     * # Return value
     *
     * `true` if the matrix is not approximately equal to the identity matrix.
     */
    pure fn is_rotated(&self) -> bool;
    
    /**
     * Check to see if the matrix is symmetric
     *
     * # Return value
     *
     * `true` if the matrix is approximately equal to its transpose).
     */
    pure fn is_symmetric(&self) -> bool;
    
    /**
     * Check to see if the matrix is invertable
     *
     * # Return value
     *
     * `true` if  the matrix is invertable
     */
    pure fn is_invertible(&self) -> bool;
    
    /**
     * # Return value
     *
     * A pointer to the first element of the matrix
     */
    pure fn to_ptr(&self) -> *T;
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
    fn col_mut(&mut self, i: uint) -> &self/mut V;
    
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
    fn set(&mut self, other: &self);
    
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
    fn add_self_m(&mut self, other: &self);
    
    /**
     * Subtract the matrix `other` from `self`
     */
    fn sub_self_m(&mut self, other: &self);
    
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

/**
 * A 2 x 2 matrix
 */
pub trait Matrix2<T,V>: Matrix<T,V> {
    pure fn to_mat3(&self) -> Mat3<T>;
    pure fn to_mat4(&self) -> Mat4<T>;
}

/**
 * A 3 x 3 matrix
 */
pub trait Matrix3<T,V>: Matrix<T,V> ToQuat<T> {
    static pure fn from_axis_angle<A:Angle<T>>(axis: &V, theta: A) -> Mat3<T>;
    pure fn to_mat4(&self) -> Mat4<T>;
}

/**
 * A 4 x 4 matrix
 */
pub trait Matrix4<T,V>: Matrix<T,V> {
}