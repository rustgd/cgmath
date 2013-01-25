use core::cmp::Eq;

use std::cmp::FuzzyEq;

use numeric::types::{Number, Radians};

pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;


/**
 * The base generic vector trait.
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 */
pub trait Vector<T>: Index<uint, T> Eq {
    /**
     * Construct the vector from a single value, copying it to each component
     */
    static pure fn from_value(value: T) -> self;
    
    /**
     * # Return value
     *
     * A pointer to the first component of the vector
     */
    pure fn to_ptr(&self) -> *T;
}

pub trait MutableVector<T>: Vector<T> {
    /**
     * Get a mutable reference to the component at `i`
     */
    fn index_mut(&mut self, i: uint) -> &self/mut T;
    
    /**
     * Swap two components of the vector in place
     */
    fn swap(&mut self, a: uint, b: uint);
}

/**
 * A generic 2-dimensional vector
 */
pub trait Vector2<T>: Vector<T> {
    // static pure fn new(x: T, y: T) -> self;
}

/**
 * A generic 3-dimensional vector
 */
pub trait Vector3<T>: Vector<T> {
    // static pure fn new(x: T, y: T, z: T) -> self;
}

/**
 * A generic 4-dimensional vector
 */
pub trait Vector4<T>: Vector<T> {
    // static pure fn new(x: T, y: T, z: T, w: T) -> self;
}

/**
 * A vector with numeric components
 */
pub trait NumericVector<T>: Vector<T> Neg<self> {
    /**
     * The standard basis vector
     *
     * # Return value
     *
     * A vector with each component set to one
     */
    static pure fn identity() -> self;
    
    /**
     * The null vector
     *
     * # Return value
     *
     * A vector with each component set to zero
     */
    static pure fn zero() -> self;
    
    /**
     * # Return value
     *
     * True if the vector is equal to zero
     */
    pure fn is_zero(&self) -> bool;
    
    /**
     * # Return value
     *
     * The scalar multiplication of the vector and `value`
     */
    pure fn mul_t(&self, value: T) -> self;
    
    /**
     * # Return value
     *
     * The scalar division of the vector and `value`
     */
    pure fn div_t(&self, value: T) -> self;
    
    /**
     * Component-wise vector addition
     */
    pure fn add_v(&self, other: &self) -> self;
    
    /**
     * Component-wise vector subtraction
     */
    pure fn sub_v(&self, other: &self) -> self;
    
    /**
     * Component-wise vector multiplication
     */
    pure fn mul_v(&self, other: &self) -> self;
    
    /**
     * Component-wise vector division
     */
    pure fn div_v(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The dot product of the vector and `other`
     */
    pure fn dot(&self, other: &self) -> T;
}

/**
 * A mutable vector with numeric components
 */
pub trait MutableNumericVector<T>: MutableVector<&self/T>
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
    fn add_self_v(&mut self, other: &self);
    
    /**
     * Set the vector to the component-wise vector difference
     */
    fn sub_self_v(&mut self, other: &self);
    
    /**
     * Set the vector to the component-wise vector product
     */
    fn mul_self_v(&mut self, other: &self);
    
    /**
     * Set the vector to the component-wise vector quotient
     */
    fn div_self_v(&mut self, other: &self);
}

/**
 * A 2-dimensional vector with numeric components
 */
pub trait NumericVector2<T>: NumericVector<T> {
    // static pure fn unit_x() -> self;
    // static pure fn unit_y() -> self;
    
    /**
     * # Return value
     *
     * The perp dot product of the vector and `other`
     */
    pure fn perp_dot(&self, other: &self) -> T;
}

/**
 * A 3-dimensional vector with numeric components
 */
pub trait NumericVector3<T>: NumericVector<T> {
    // static pure fn unit_x() -> self;
    // static pure fn unit_y() -> self;
    // static pure fn unit_z() -> self;
    
    /**
     * # Return value
     *
     * The cross product of the vector and `other`
     */
    pure fn cross(&self, other: &self) -> self;
}

/**
 * A mutable 3-dimensional vector with numeric components
 */
pub trait MutableNumericVector3<T>: MutableNumericVector<&self/T> {
    /**
     * Set to the cross product of the vector and `other`
     */
    fn cross_self(&mut self, other: &self);
}

/**
 * A 4-dimensional vector with numeric components
 */
pub trait NumericVector4<T>: NumericVector<T> {
    // static pure fn unit_x() -> self;
    // static pure fn unit_y() -> self;
    // static pure fn unit_z() -> self;
    // static pure fn unit_w() -> self;
}

pub trait ToHomogeneous<H> {
    /**
     * Convert to a homogenous coordinate
     */
    pure fn to_homogeneous(&self) -> H;
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
    pure fn length2(&self) -> T;
    
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
    pure fn length(&self) -> T;
    
    /**
     * # Return value
     *
     * The squared distance between the vector and `other`.
     */
    pure fn distance2(&self, other: &self) -> T;
    
    /**
     * # Return value
     *
     * The distance between the vector and `other`
     */
    pure fn distance(&self, other: &self) -> T;
    
    /**
     * # Return value
     *
     * The angle between the vector and `other`
     */
    pure fn angle(&self, other: &self) -> Radians<T>;
    
    /**
     * # Return value
     *
     * The normalized vector
     */
    pure fn normalize(&self) -> self;
    
    /**
     * Set the length of the vector whilst preserving the direction
     */
    pure fn normalize_to(&self, length: T) -> self;
    
    /**
     * Linearly intoperlate between the vector and `other`
     *
     * # Return value
     *
     * The intoperlated vector
     */
    pure fn lerp(&self, other: &self, amount: T) -> self;
}

/**
 * A mutable Euclidean (or Affine) vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This should be a floating point type.
 */
pub trait MutableEuclideanVector<T>: MutableNumericVector<&self/T>
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
    fn lerp_self(&mut self, other: &self, amount: T);
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
    pure fn less_than(&self, other: &self) -> BoolVec;
    
    /**
     * Component-wise compare of `self <= other`
     */
    pure fn less_than_equal(&self, other: &self) -> BoolVec;
    
    /**
     * Component-wise compare of `self > other`
     */
    pure fn greater_than(&self, other: &self) -> BoolVec;
    
    /**
     * Component-wise compare of `self >= other`
     */
    pure fn greater_than_equal(&self, other: &self) -> BoolVec;
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
    pure fn equal(&self, other: &self) -> BoolVec;
    
    /**
     * Component-wise compare of `self != other`
     */
    pure fn not_equal(&self, other: &self) -> BoolVec;
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
    pure fn any(&self) -> bool;
    
    /**
     * # Return value
     *
     * `true` only if all components are `true`
     */
    pure fn all(&self) -> bool;
    
    /**
     * # Return value
     *
     * the component-wise logical complement
     */
    pure fn not(&self) -> self;
}