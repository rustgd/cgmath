use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::to_unsafe_ptr;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;
use numeric::funs::*;
use numeric::types::angle::Angle;
use numeric::types::float::Float;
use numeric::types::number::Number;

use vec::Vec4;

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
#[deriving_eq]
pub struct Mat4<T> { x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T> }

pub impl<T:Copy Float> Mat4<T> {
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
    static pure fn new(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
                       c1r0: T, c1r1: T, c1r2: T, c1r3: T,
                       c2r0: T, c2r1: T, c2r2: T, c2r3: T,
                       c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> Mat4<T>  {
        Mat4::from_cols(Vec4::new(c0r0, c0r1, c0r2, c0r3),
                        Vec4::new(c1r0, c1r1, c1r2, c1r3),
                        Vec4::new(c2r0, c2r1, c2r2, c2r3),
                        Vec4::new(c3r0, c3r1, c3r2, c3r3))
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
    static pure fn from_cols(c0: Vec4<T>,
                             c1: Vec4<T>,
                             c2: Vec4<T>,
                             c3: Vec4<T>) -> Mat4<T> {
        Mat4 { x: c0, y: c1, z: c2, w: c3 }
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
    static pure fn from_value(value: T) -> Mat4<T> {
        let _0 = Number::from(0);
        Mat4::new(value,    _0,    _0,    _0,
                     _0, value,    _0,    _0,
                     _0,    _0, value,    _0,
                     _0,    _0,    _0, value)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn identity() -> Mat4<T> {
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat4::new(_1, _0, _0, _0,
                  _0, _1, _0, _0,
                  _0, _0, _1, _0,
                  _0, _0, _0, _1)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn zero() -> Mat4<T> {
        let _0 = Number::from(0);
        Mat4::new(_0, _0, _0, _0,
                  _0, _0, _0, _0,
                  _0, _0, _0, _0,
                  _0, _0, _0, _0)
    }
}

pub impl<T:Copy Float> Mat4<T>: Matrix<T, Vec4<T>> {
    #[inline(always)]
    pure fn col(&self, i: uint) -> Vec4<T> { self[i] }
    
    #[inline(always)]
    pure fn row(&self, i: uint) -> Vec4<T> {
        Vec4::new(self[0][i],
                  self[1][i],
                  self[2][i],
                  self[3][i])
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
    static pure fn identity() -> Mat4<T> {
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat4::new(_1, _0, _0, _0,
                  _0, _1, _0, _0,
                  _0, _0, _1, _0,
                  _0, _0, _0, _1)
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
    static pure fn zero() -> Mat4<T> {
        let _0 = Number::from(0);
        Mat4::new(_0, _0, _0, _0,
                  _0, _0, _0, _0,
                  _0, _0, _0, _0,
                  _0, _0, _0, _0)
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Mat4<T> {
        Mat4::from_cols(self[0].mul_t(value),
                        self[1].mul_t(value),
                        self[2].mul_t(value),
                        self[3].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(&self, vec: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self.row(0).dot(vec),
                  self.row(1).dot(vec),
                  self.row(2).dot(vec),
                  self.row(3).dot(vec))
    }
    
    #[inline(always)]
    pure fn add_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self[0].add_v(&other[0]),
                        self[1].add_v(&other[1]),
                        self[2].add_v(&other[2]),
                        self[3].add_v(&other[3]))
    }
    
    #[inline(always)]
    pure fn sub_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self[0].sub_v(&other[0]),
                        self[1].sub_v(&other[1]),
                        self[2].sub_v(&other[2]),
                        self[3].sub_v(&other[3]))
    }
    
    #[inline(always)]
    pure fn mul_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::new(self.row(0).dot(&other.col(0)),
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
    
    pure fn dot(&self, other: &Mat4<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    pure fn determinant(&self) -> T {
        self[0][0]*Mat3::new(self[1][1], self[2][1], self[3][1],
                             self[1][2], self[2][2], self[3][2],
                             self[1][3], self[2][3], self[3][3]).determinant() -
        self[1][0]*Mat3::new(self[0][1], self[2][1], self[3][1],
                             self[0][2], self[2][2], self[3][2],
                             self[0][3], self[2][3], self[3][3]).determinant() +
        self[2][0]*Mat3::new(self[0][1], self[1][1], self[3][1],
                             self[0][2], self[1][2], self[3][2],
                             self[0][3], self[1][3], self[3][3]).determinant() -
        self[3][0]*Mat3::new(self[0][1], self[1][1], self[2][1],
                             self[0][2], self[1][2], self[2][2],
                             self[0][3], self[1][3], self[2][3]).determinant()
    }

    pure fn trace(&self) -> T {
        self[0][0] + self[1][1] + self[2][2] + self[3][3]
    }

    pure fn inverse(&self) -> Option<Mat4<T>> {
        let d = self.determinant();
        if d.fuzzy_eq(&Number::from(0)) {
            None
        } else {
            
            // Gauss Jordan Elimination with partial pivoting
            // So take this matrix, A, augmented with the identity
            // and essentially reduce [A|I]
            
            let mut A = *self;
            // let mut I: Mat4<T> = Matrix::identity();     // FIXME: there's something wrong with static functions here!
            let mut I = Mat4::identity();
            
            for uint::range(0, 4) |j| {
                // Find largest element in col j
                let mut i1 = j;
                for uint::range(j + 1, 4) |i| {
                    if abs(&A[j][i]) > abs(&A[j][i1]) {
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
    pure fn transpose(&self) -> Mat4<T> {
        Mat4::new(self[0][0], self[1][0], self[2][0], self[3][0],
                  self[0][1], self[1][1], self[2][1], self[3][1],
                  self[0][2], self[1][2], self[2][2], self[3][2],
                  self[0][3], self[1][3], self[2][3], self[3][3])
    }
    
    #[inline(always)]
    pure fn is_identity(&self) -> bool {
        // self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        self.fuzzy_eq(&Mat4::identity())
    }
    
    #[inline(always)]
    pure fn is_diagonal(&self) -> bool {
        let _0 = Number::from(0);
        self[0][1].fuzzy_eq(&_0) &&
        self[0][2].fuzzy_eq(&_0) &&
        self[0][3].fuzzy_eq(&_0) &&
        
        self[1][0].fuzzy_eq(&_0) &&
        self[1][2].fuzzy_eq(&_0) &&
        self[1][3].fuzzy_eq(&_0) &&
        
        self[2][0].fuzzy_eq(&_0) &&
        self[2][1].fuzzy_eq(&_0) &&
        self[2][3].fuzzy_eq(&_0) &&
        
        self[3][0].fuzzy_eq(&_0) &&
        self[3][1].fuzzy_eq(&_0) &&
        self[3][2].fuzzy_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated(&self) -> bool {
        // !self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        !self.fuzzy_eq(&Mat4::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric(&self) -> bool {
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
    pure fn is_invertible(&self) -> bool {
        !self.determinant().fuzzy_eq(&Number::zero())
    }
    
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Mat4<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T:Copy Float> Mat4<T>: MutableMatrix<T, Vec4<T>> {
    #[inline(always)]
    fn col_mut(&mut self, i: uint) -> &self/mut Vec4<T> {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 3, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        util::swap(self.col_mut(a),
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
        (*self) = Mat4::identity();
    }
    
    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = Mat4::zero();
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
            None => fail(~"Couldn't invert the matrix!")
        }
    }
    
    #[inline(always)]
    fn transpose_self(&mut self) {
        util::swap(self.col_mut(0).index_mut(1), self.col_mut(1).index_mut(0));
        util::swap(self.col_mut(0).index_mut(2), self.col_mut(2).index_mut(0));
        util::swap(self.col_mut(0).index_mut(3), self.col_mut(3).index_mut(0));
        
        util::swap(self.col_mut(1).index_mut(0), self.col_mut(0).index_mut(1));
        util::swap(self.col_mut(1).index_mut(2), self.col_mut(2).index_mut(1));
        util::swap(self.col_mut(1).index_mut(3), self.col_mut(3).index_mut(1));
        
        util::swap(self.col_mut(2).index_mut(0), self.col_mut(0).index_mut(2));
        util::swap(self.col_mut(2).index_mut(1), self.col_mut(1).index_mut(2));
        util::swap(self.col_mut(2).index_mut(3), self.col_mut(3).index_mut(2));
        
        util::swap(self.col_mut(3).index_mut(0), self.col_mut(0).index_mut(3));
        util::swap(self.col_mut(3).index_mut(1), self.col_mut(1).index_mut(3));
        util::swap(self.col_mut(3).index_mut(2), self.col_mut(2).index_mut(3));
    }
}

pub impl<T> Mat4<T>: Matrix4<T, Vec4<T>> {}

pub impl<T:Copy Float> Mat4<T>: Neg<Mat4<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Mat4<T> {
        Mat4::from_cols(-self[0], -self[1], -self[2], -self[3])
    }
}

pub impl<T:Copy> Mat4<T>: Index<uint, Vec4<T>> {
    #[inline(always)]
    pure fn index(&self, i: uint) -> Vec4<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat4<T>, *Vec4<T>>(
                to_unsafe_ptr(self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy Float> Mat4<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat4<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
}