use core::cmp::Eq;

pub use vec2::{Vec2, vec2, dvec2, bvec2, ivec2, uvec2};
pub use vec3::{Vec3, vec3, dvec3, bvec3, ivec3, uvec3};
pub use vec4::{Vec4, vec4, dvec4, bvec4, ivec4, uvec4};


/**
 * The base generic vector trait.
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 */
pub trait Vector<T>: Index<uint, T> + Eq {
    /**
     * Construct the vector from a single value, copying it to each component
     */
    fn from_value(value: T) -> Self;

    /**
     * # Return value
     *
     * A pointer to the first component of the vector
     */
    fn to_ptr(&self) -> *T;
}

pub trait MutableVector<'self, T>: Vector<T> {
    /**
     * Get a mutable reference to the component at `i`
     */
    fn index_mut(&mut self, i: uint) -> &'self mut T;

    /**
     * Swap two components of the vector in place
     */
    fn swap(&mut self, a: uint, b: uint);
}

/**
 * A generic 2-dimensional vector
 */
pub trait Vector2<T>: Vector<T> {
    fn new(x: T, y: T) -> Self;
}

/**
 * A generic 3-dimensional vector
 */
pub trait Vector3<T>: Vector<T> {
    fn new(x: T, y: T, z: T) -> Self;
}

/**
 * A generic 4-dimensional vector
 */
pub trait Vector4<T>: Vector<T> {
    fn new(x: T, y: T, z: T, w: T) -> Self;
}

/**
 * A vector with numeric components
 */
pub trait NumericVector<T>: Vector<T> + Neg<Self> {
    /**
     * The standard basis vector
     *
     * # Return value
     *
     * A vector with each component set to one
     */
    fn identity() -> Self;

    /**
     * The null vector
     *
     * # Return value
     *
     * A vector with each component set to zero
     */
    fn zero() -> Self;

    /**
     * # Return value
     *
     * True if the vector is equal to zero
     */
    fn is_zero(&self) -> bool;

    /**
     * # Return value
     *
     * The scalar multiplication of the vector and `value`
     */
    fn mul_t(&self, value: T) -> Self;

    /**
     * # Return value
     *
     * The scalar division of the vector and `value`
     */
    fn div_t(&self, value: T) -> Self;

    /**
     * Component-wise vector addition
     */
    fn add_v(&self, other: &Self) -> Self;

    /**
     * Component-wise vector subtraction
     */
    fn sub_v(&self, other: &Self) -> Self;

    /**
     * Component-wise vector multiplication
     */
    fn mul_v(&self, other: &Self) -> Self;

    /**
     * Component-wise vector division
     */
    fn div_v(&self, other: &Self) -> Self;

    /**
     * # Return value
     *
     * The dot product of the vector and `other`
     */
    fn dot(&self, other: &Self) -> T;
}

/**
 * A 2-dimensional vector with numeric components
 */
pub trait NumericVector2<T>: NumericVector<T> {
    fn unit_x() -> Self;
    fn unit_y() -> Self;

    /**
     * # Return value
     *
     * The perp dot product of the vector and `other`
     */
    fn perp_dot(&self, other: &Self) -> T;
}

/**
 * A 3-dimensional vector with numeric components
 */
pub trait NumericVector3<T>: NumericVector<T> {
    fn unit_x() -> Self;
    fn unit_y() -> Self;
    fn unit_z() -> Self;

    /**
     * # Return value
     *
     * The cross product of the vector and `other`
     */
    fn cross(&self, other: &Self) -> Self;
}

/**
 * A 4-dimensional vector with numeric components
 */
pub trait NumericVector4<T>: NumericVector<T> {
    fn unit_x() -> Self;
    fn unit_y() -> Self;
    fn unit_z() -> Self;
    fn unit_w() -> Self;
}

/**
 * A mutable vector with numeric components
 */
pub trait MutableNumericVector<'self, T>: MutableVector<&'self T> +
                                          NumericVector<T> {
    /**
     * Negate the vector
     */
    fn neg_self(&mut self);

    /**
     * Multiply the vector by a scalar
     */
    fn mul_self_t(&mut self, value: T);

    /**
     * Divide the vector by a scalar
     */
    fn div_self_t(&mut self, value: T);

    /**
     * Set the vector to the component-wise vector sum
     */
    fn add_self_v(&mut self, other: &Self);

    /**
     * Set the vector to the component-wise vector difference
     */
    fn sub_self_v(&mut self, other: &Self);

    /**
     * Set the vector to the component-wise vector product
     */
    fn mul_self_v(&mut self, other: &Self);

    /**
     * Set the vector to the component-wise vector quotient
     */
    fn div_self_v(&mut self, other: &Self);
}

/**
 * A mutable 3-dimensional vector with numeric components
 */
pub trait MutableNumericVector3<'self, T>: MutableNumericVector<&'self T> {
    /**
     * Set to the cross product of the vector and `other`
     */
    fn cross_self(&mut self, other: &Self);
}

pub trait ToHomogeneous<H> {
    /**
     * Convert to a homogenous coordinate
     */
    fn to_homogeneous(&self) -> H;
}

/**
 * A Euclidean (or Affine) vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This should be a floating point type.
 */
pub trait EuclideanVector<T>: NumericVector<T> {
    /**
     * # Return value
     *
     * The squared length of the vector. This is useful for comparisons where
     * the exact length does not need to be calculated.
     */
    fn length2(&self) -> T;

    /**
     * # Return value
     *
     * The length of the vector
     *
     * # Performance notes
     *
     * For instances where the exact length of the vector does not need to be
     * known, for example for quaternion-quaternion length comparisons,
     * it is advisable to use the `length2` method instead.
     */
    fn length(&self) -> T;

    /**
     * # Return value
     *
     * The squared distance between the vector and `other`.
     */
    fn distance2(&self, other: &Self) -> T;

    /**
     * # Return value
     *
     * The distance between the vector and `other`
     */
    fn distance(&self, other: &Self) -> T;

    /**
     * # Return value
     *
     * The angle between the vector and `other` in radians
     */
    fn angle(&self, other: &Self) -> T;

    /**
     * # Return value
     *
     * The normalized vector
     */
    fn normalize(&self) -> Self;

    /**
     * Set the length of the vector whilst preserving the direction
     */
    fn normalize_to(&self, length: T) -> Self;

    /**
     * Linearly intoperlate between the vector and `other`
     *
     * # Return value
     *
     * The intoperlated vector
     */
    fn lerp(&self, other: &Self, amount: T) -> Self;
}

/**
 * A mutable Euclidean (or Affine) vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This should be a floating point type.
 */
pub trait MutableEuclideanVector<'self, T>: MutableNumericVector<&'self T> +
                                            EuclideanVector<T> {
    /**
     * Normalize the vector
     */
    fn normalize_self(&mut self);

    /**
     * Set the vector to a specified length whilst preserving the direction
     */
    fn normalize_self_to(&mut self, length: T);

    /**
     * Linearly intoperlate the vector towards `other`
     */
    fn lerp_self(&mut self, other: &Self, amount: T);
}

/**
 * Component-wise vector comparison methods
 *
 * The methods contained in this trait correspond to the relational functions
 * mentioned in Section 8.7 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */
pub trait OrdinalVector<T, BoolVec>: Vector<T> {
    /**
     * Component-wise compare of `self < other`
     */
    fn less_than(&self, other: &Self) -> BoolVec;

    /**
     * Component-wise compare of `self <= other`
     */
    fn less_than_equal(&self, other: &Self) -> BoolVec;

    /**
     * Component-wise compare of `self > other`
     */
    fn greater_than(&self, other: &Self) -> BoolVec;

    /**
     * Component-wise compare of `self >= other`
     */
    fn greater_than_equal(&self, other: &Self) -> BoolVec;
}

/**
 * Component-wise equality comparison methods
 *
 * The methods contained in this trait correspond to the relational functions
 * mentioned in Section 8.7 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */
pub trait EquableVector<T, BoolVec>: Vector<T> {
    /**
     * Component-wise compare of `self == other`
     */
    fn equal(&self, other: &Self) -> BoolVec;

    /**
     * Component-wise compare of `self != other`
     */
    fn not_equal(&self, other: &Self) -> BoolVec;
}

/**
 * A vector with boolean components
 *
 * The methods contained in this trait correspond to the relational functions
 * mentioned in Section 8.7 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */
pub trait BooleanVector: Vector<bool> {
    /**
     * # Return value
     *
     * `true` if of any component is `true`
     */
    fn any(&self) -> bool;

    /**
     * # Return value
     *
     * `true` only if all components are `true`
     */
    fn all(&self) -> bool;

    /**
     * # Return value
     *
     * the component-wise logical complement
     */
    fn not(&self) -> Self;
}

pub trait TrigVec<T>: Vector<T> {
    fn radians(&self) -> Self;
    fn degrees(&self) -> Self;

    // Triganometric functions
    fn sin(&self)                      -> Self;
    fn cos(&self)                      -> Self;
    fn tan(&self)                      -> Self;

    // Inverse triganometric functions
    fn asin(&self)                     -> Self;
    fn acos(&self)                     -> Self;
    fn atan(&self)                     -> Self;
    fn atan2(&self, other: Self)       -> Self;

    // Hyperbolic triganometric functions
    fn sinh(&self)                     -> Self;
    fn cosh(&self)                     -> Self;
    fn tanh(&self)                     -> Self;
    // fn asinh()                      -> Self;
    // fn acosh()                      -> Self;
    // fn atanh()                      -> Self;
}

pub trait ExpVec<T>: Vector<T> {
    // Exponential functions
    fn pow_t(&self, n: Self)           -> Self;
    fn pow_v(&self, n: T)              -> Self;
    fn exp(&self)                      -> Self;
    fn exp2(&self)                     -> Self;
    fn ln(&self)                       -> Self;
    fn ln2(&self)                      -> Self;
    fn sqrt(&self)                     -> Self;
    fn inv_sqrt(&self)                 -> Self;
}

pub trait ApproxVec<T>: Vector<T> {
    // Whole-number approximation functions
    fn floor(&self)                    -> Self;
    fn trunc(&self)                    -> Self;
    fn round(&self)                    -> Self;
    // fn round_even(&self)            -> Self;
    fn ceil(&self)                     -> Self;
    fn fract(&self)                    -> Self;
}

pub trait SignedVec<T,BV>: Vector<T> {
    fn is_positive(&self)    -> BV;
    fn is_negative(&self)    -> BV;
    fn is_nonpositive(&self) -> BV;
    fn is_nonnegative(&self) -> BV;

    fn abs(&self) -> Self;
    fn sign(&self) -> Self;
    fn copysign(&self, other: Self) -> Self;
}

pub trait ExtentVec<T>: Vector<T> {
    fn min_v(&self, other: &Self) -> Self;
    fn max_v(&self, other: &Self) -> Self;
    fn clamp_v(&self, mn: &Self, mx: &Self) -> Self;

    fn min_t(&self, other: T) -> Self;
    fn max_t(&self, other: T) -> Self;
    fn clamp_t(&self, mn: T, mx: T) -> Self;
}

pub trait MixVec<T>: Vector<T> {
    // Functions for blending numbers together
    fn mix(&self, other: Self, value: Self) -> Self;
    fn smooth_step(&self, edge0: Self, edge1: Self) -> Self;
    fn step(&self, edge: Self) -> Self;
}