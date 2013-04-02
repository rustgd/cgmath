use core::num::{Zero, One};
use core::num::Zero::zero;
use core::num::One::one;
use std::cmp::{FuzzyEq, FUZZY_EPSILON};
use numeric::*;

/**
 * The base generic vector trait.
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 */
pub trait Vector<T>: Index<uint,T> + Eq {
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
    
    /**
     * Set to the cross product of the vector and `other`
     */
    fn cross_self(&mut self, other: &Self);
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

// Utility macros

macro_rules! zip_vec2(
    ($a:ident[] $method:ident $b:ident[]) => (
        Vector2::new($a[0].$method(&($b[0])),
                     $a[1].$method(&($b[1])))
    );
    ($a:ident[] $method:ident $b:ident) => (
        Vector2::new($a[0].$method(&($b)),
                     $a[1].$method(&($b)))
    );
)

macro_rules! zip_vec3(
    ($a:ident[] $method:ident $b:ident[]) => (
        Vector3::new($a[0].$method(&($b[0])),
                     $a[1].$method(&($b[1])),
                     $a[2].$method(&($b[2])))
    );
    ($a:ident[] $method:ident $b:ident) => (
        Vector3::new($a[0].$method(&($b)),
                     $a[1].$method(&($b)),
                     $a[2].$method(&($b)))
    );
)

macro_rules! zip_vec4(
    ($a:ident[] $method:ident $b:ident[]) => (
        Vector4::new($a[0].$method(&($b[0])),
                     $a[1].$method(&($b[1])),
                     $a[2].$method(&($b[2])),
                     $a[3].$method(&($b[3])))
    );
    ($a:ident[] $method:ident $b:ident) => (
        Vector4::new($a[0].$method(&($b)),
                     $a[1].$method(&($b)),
                     $a[2].$method(&($b)),
                     $a[3].$method(&($b)))
    );
)

macro_rules! zip_assign(
    ($a:ident[] $method:ident $b:ident[] ..2) => ({ $a.index_mut(0).$method($b[0]);     $a.index_mut(1).$method($b[1]); });
    ($a:ident[] $method:ident $b:ident[] ..3) => ({ zip_assign!($a[] $method $b[] ..2); $a.index_mut(2).$method($b[2]); });
    ($a:ident[] $method:ident $b:ident[] ..4) => ({ zip_assign!($a[] $method $b[] ..3); $a.index_mut(3).$method($b[3]); });
    
    ($a:ident[] $method:ident $b:ident   ..2) => ({ $a.index_mut(0).$method($b);        $a.index_mut(1).$method($b);    });
    ($a:ident[] $method:ident $b:ident   ..3) => ({ zip_assign!($a[] $method $b ..2);   $a.index_mut(2).$method($b);    });
    ($a:ident[] $method:ident $b:ident   ..4) => ({ zip_assign!($a[] $method $b ..3);   $a.index_mut(3).$method($b);    });
)

/**
 * A 2-dimensional vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 *
 * # Fields
 *
 * * `x` - the first component of the vector
 * * `y` - the second component of the vector
 */
 #[deriving(Eq)]
pub struct Vec2<T> { x: T, y: T }

impl<T:Copy + Eq> Vector<T> for Vec2<T> {
    #[inline(always)]
    fn from_value(value: T) -> Vec2<T> {
        Vector2::new(value, value)
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { cast::transmute(self) }
    }
    
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &'self mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => fail!(fmt!("index out of bounds: expected an index from 0 to 1, but found %u", i))
        }
    }

    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        *self.index_mut(a) <-> *self.index_mut(b);
    }
}

impl<T> Vector2<T> for Vec2<T> {
    #[inline(always)]
    fn new(x: T, y: T ) -> Vec2<T> {
        Vec2 { x: x, y: y }
    }
}

impl<T:Copy + Eq> Index<uint, T> for Vec2<T> {
    #[inline(always)]
    fn index(&self, i: &uint) -> T {
        unsafe { do vec::raw::buf_as_slice(self.to_ptr(), 2) |slice| { slice[*i] } }
    }
}

impl<T:Copy + Number + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> NumericVector<T> for Vec2<T> {
    #[inline(always)]
    fn identity() -> Vec2<T> {
        Vector2::new(one::<T>(), one::<T>())
    }

    #[inline(always)]
    fn zero() -> Vec2<T> {
        Vector2::new(zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self[0] == zero() &&
        self[1] == zero()
    }

    #[inline(always)]
    fn mul_t(&self, value: T) -> Vec2<T> {
        zip_vec2!(self[] mul value)
    }

    #[inline(always)]
    fn div_t(&self, value: T) -> Vec2<T> {
        zip_vec2!(self[] div value)
    }

    #[inline(always)]
    fn add_v(&self, other: &Vec2<T>) -> Vec2<T> {
        zip_vec2!(self[] add other[])
    }

    #[inline(always)]
    fn sub_v(&self, other: &Vec2<T>) -> Vec2<T> {
        zip_vec2!(self[] sub other[])
    }

    #[inline(always)]
    fn mul_v(&self, other: &Vec2<T>) -> Vec2<T> {
        zip_vec2!(self[] mul other[])
    }

    #[inline(always)]
    fn div_v(&self, other: &Vec2<T>) -> Vec2<T> {
        zip_vec2!(self[] div other[])
    }

    #[inline(always)]
    fn dot(&self, other: &Vec2<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1]
    }
    
    #[inline(always)]
    fn neg_self(&mut self) {
        *self.index_mut(0) = -self[0];
        *self.index_mut(1) = -self[1];
    }

    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        zip_assign!(self[] mul_assign value ..2);
    }

    #[inline(always)]
    fn div_self_t(&mut self, value: T) {
        zip_assign!(self[] div_assign value ..2);
    }

    #[inline(always)]
    fn add_self_v(&mut self, other: &Vec2<T>) {
        zip_assign!(self[] add_assign other[] ..2);
    }

    #[inline(always)]
    fn sub_self_v(&mut self, other: &Vec2<T>) {
        zip_assign!(self[] sub_assign other[] ..2);
    }

    #[inline(always)]
    fn mul_self_v(&mut self, other: &Vec2<T>) {
        zip_assign!(self[] mul_assign other[] ..2);
    }

    #[inline(always)]
    fn div_self_v(&mut self, other: &Vec2<T>) {
        zip_assign!(self[] div_assign other[] ..2);
    }
}

impl<T:Copy + Number + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Neg<Vec2<T>> for Vec2<T> {
    #[inline(always)]
    fn neg(&self) -> Vec2<T> {
        Vector2::new(-self[0], -self[1])
    }
}

impl<T:Copy + Number + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> NumericVector2<T> for Vec2<T> {
    #[inline(always)]
    fn unit_x() -> Vec2<T> {
        Vector2::new(one::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_y() -> Vec2<T> {
        Vector2::new(zero::<T>(), one::<T>())
    }

    #[inline(always)]
    fn perp_dot(&self, other: &Vec2<T>) ->T {
        (self[0] * other[1]) - (self[1] * other[0])
    }
}

impl<T:Copy + Number + Zero> ToHomogeneous<Vec3<T>> for Vec2<T> {
    #[inline(always)]
    fn to_homogeneous(&self) -> Vec3<T> {
        Vector3::new(self.x, self.y, zero())
    }
}

impl<T:Copy + Float + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> EuclideanVector<T> for Vec2<T> {
    #[inline(always)]
    fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline(always)]
    fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline(always)]
    fn distance2(&self, other: &Vec2<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline(always)]
    fn distance(&self, other: &Vec2<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline(always)]
    fn angle(&self, other: &Vec2<T>) -> T {
        atan2(self.perp_dot(other), self.dot(other))
    }

    #[inline(always)]
    fn normalize(&self) -> Vec2<T> {
        self.mul_t(one::<T>()/self.length())
    }

    #[inline(always)]
    fn normalize_to(&self, length: T) -> Vec2<T> {
        self.mul_t(length / self.length())
    }

    #[inline(always)]
    fn lerp(&self, other: &Vec2<T>, amount: T) -> Vec2<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }
    
    #[inline(always)]
    fn normalize_self(&mut self) {
        let n = one::<T>() / self.length();
        self.mul_self_t(n);
    }

    #[inline(always)]
    fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    fn lerp_self(&mut self, other: &Vec2<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Copy + Float + Zero + One + FuzzyEq<T>> FuzzyEq<T> for Vec2<T> {
    #[inline(always)]
    fn fuzzy_eq(&self, other: &Vec2<T>) -> bool {
        self.fuzzy_eq_eps(other, &num::cast(FUZZY_EPSILON))
    }

    #[inline(always)]
    fn fuzzy_eq_eps(&self, other: &Vec2<T>, epsilon: &T) -> bool {
        self[0].fuzzy_eq_eps(&other[0], epsilon) &&
        self[1].fuzzy_eq_eps(&other[1], epsilon)
    }
}

impl<T:Copy + Ord + Eq> OrdinalVector<T, Vec2<bool>> for Vec2<T> {
    #[inline(always)]
    fn less_than(&self, other: &Vec2<T>) -> Vec2<bool> {
        zip_vec2!(self[] lt other[])
    }

    #[inline(always)]
    fn less_than_equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        zip_vec2!(self[] le other[])
    }

    #[inline(always)]
    fn greater_than(&self, other: &Vec2<T>) -> Vec2<bool> {
        zip_vec2!(self[] gt other[])
    }

    #[inline(always)]
    fn greater_than_equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        zip_vec2!(self[] ge other[])
    }
}

impl<T:Copy + Eq> EquableVector<T, Vec2<bool>> for Vec2<T> {
    #[inline(always)]
    fn equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        zip_vec2!(self[] eq other[])
    }

    #[inline(always)]
    fn not_equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        zip_vec2!(self[] ne other[])
    }
}

impl BooleanVector for Vec2<bool> {
    #[inline(always)]
    fn any(&self) -> bool {
        self[0] || self[1]
    }

    #[inline(always)]
    fn all(&self) -> bool {
        self[0] && self[1]
    }

    #[inline(always)]
    fn not(&self) -> Vec2<bool> {
        Vector2::new(!self[0], !self[1])
    }
}

macro_rules! vec2_type(
    ($name:ident <bool>) => (
        pub impl $name {
            #[inline(always)] fn new(x: bool, y: bool) -> $name { Vector2::new(x, y) }
            #[inline(always)] fn from_value(v: bool) -> $name { Vector::from_value(v) }

            #[inline(always)] fn dim() -> uint { 2 }
            #[inline(always)] fn size_of() -> uint { sys::size_of::<$name>() }
        }
    );
    ($name:ident <$T:ty>) => (
        pub impl $name {
            #[inline(always)] fn new(x: $T, y: $T) -> $name { Vector2::new(x, y) }
            #[inline(always)] fn from_value(v: $T) -> $name { Vector::from_value(v) }
            #[inline(always)] fn identity() -> $name { NumericVector::identity() }
            #[inline(always)] fn zero() -> $name { NumericVector::zero() }

            #[inline(always)] fn unit_x() -> $name { NumericVector2::unit_x() }
            #[inline(always)] fn unit_y() -> $name { NumericVector2::unit_y() }

            #[inline(always)] fn dim() -> uint { 2 }
            #[inline(always)] fn size_of() -> uint { sys::size_of::<$name>() }
        }
    );
)

// GLSL-style type aliases, corresponding to Section 4.1.5 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

// a two-component single-precision floating-point vector
pub type vec2  = Vec2<f32>;
// a two-component double-precision floating-point vector
pub type dvec2 = Vec2<f64>;
// a two-component Boolean vector
pub type bvec2 = Vec2<bool>;
// a two-component signed integer vector
pub type ivec2 = Vec2<i32>;
// a two-component unsigned integer vector
pub type uvec2 = Vec2<u32>;

vec2_type!(vec2<f32>)
vec2_type!(dvec2<f64>)
vec2_type!(bvec2<bool>)
vec2_type!(ivec2<i32>)
vec2_type!(uvec2<u32>)

// Rust-style type aliases
pub type Vec2f   = Vec2<float>;
pub type Vec2f32 = Vec2<f32>;
pub type Vec2f64 = Vec2<f64>;
pub type Vec2i   = Vec2<int>;
pub type Vec2i8  = Vec2<i8>;
pub type Vec2i16 = Vec2<i16>;
pub type Vec2i32 = Vec2<i32>;
pub type Vec2i64 = Vec2<i64>;
pub type Vec2u   = Vec2<uint>;
pub type Vec2u8  = Vec2<u8>;
pub type Vec2u16 = Vec2<u16>;
pub type Vec2u32 = Vec2<u32>;
pub type Vec2u64 = Vec2<u64>;
pub type Vec2b   = Vec2<bool>;

vec2_type!(Vec2f<float>)
vec2_type!(Vec2f32<f32>)
vec2_type!(Vec2f64<f64>)
vec2_type!(Vec2i<int>)
vec2_type!(Vec2i8<i8>)
vec2_type!(Vec2i16<i16>)
vec2_type!(Vec2i32<i32>)
vec2_type!(Vec2i64<i64>)
vec2_type!(Vec2u<uint>)
vec2_type!(Vec2u8<u8>)
vec2_type!(Vec2u16<u16>)
vec2_type!(Vec2u32<u32>)
vec2_type!(Vec2u64<u64>)
vec2_type!(Vec2b<bool>)

/**
 * A 3-dimensional vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 *
 * # Fields
 *
 * * `x` - the first component of the vector
 * * `y` - the second component of the vector
 * * `z` - the third component of the vector
 */
#[deriving(Eq)]
pub struct Vec3<T> { x: T, y: T, z: T }

impl<T:Copy + Eq> Vector<T> for Vec3<T> {
    #[inline(always)]
    fn from_value(value: T) -> Vec3<T> {
        Vector3::new(value, value, value)
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { cast::transmute(self) }
    }
    
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &'self mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => fail!(fmt!("index out of bounds: expected an index from 0 to 2, but found %u", i))
        }
    }

    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        *self.index_mut(a) <-> *self.index_mut(b);
    }
}

impl<T> Vector3<T> for Vec3<T> {
    #[inline(always)]
    fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x: x, y: y, z: z }
    }
}

impl<T:Copy + Eq> Index<uint, T> for Vec3<T> {
    #[inline(always)]
    fn index(&self, i: &uint) -> T {
        unsafe { do vec::raw::buf_as_slice(self.to_ptr(), 3) |slice| { slice[*i] } }
    }
}

impl<T:Copy + Number + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> NumericVector<T> for Vec3<T> {
    #[inline(always)]
    fn identity() -> Vec3<T> {
        Vector3::new(one::<T>(), one::<T>(), one::<T>())
    }

    #[inline(always)]
    fn zero() -> Vec3<T> {
        Vector3::new(zero::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self[0] == zero() &&
        self[1] == zero() &&
        self[2] == zero()
    }

    #[inline(always)]
    fn mul_t(&self, value: T) -> Vec3<T> {
        zip_vec3!(self[] mul value)
    }

    #[inline(always)]
    fn div_t(&self, value: T) -> Vec3<T> {
        zip_vec3!(self[] div value)
    }

    #[inline(always)]
    fn add_v(&self, other: &Vec3<T>) -> Vec3<T> {
        zip_vec3!(self[] add other[])
    }

    #[inline(always)]
    fn sub_v(&self, other: &Vec3<T>) -> Vec3<T> {
        zip_vec3!(self[] sub other[])
    }

    #[inline(always)]
    fn mul_v(&self, other: &Vec3<T>) -> Vec3<T> {
        zip_vec3!(self[] mul other[])
    }

    #[inline(always)]
    fn div_v(&self, other: &Vec3<T>) -> Vec3<T> {
        zip_vec3!(self[] div other[])
    }

    #[inline(always)]
    fn dot(&self, other: &Vec3<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2]
    }
    
    #[inline(always)]
    fn neg_self(&mut self) {
        *self.index_mut(0) = -self[0];
        *self.index_mut(1) = -self[1];
        *self.index_mut(2) = -self[2];
    }

    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        zip_assign!(self[] mul_assign value ..3);
    }

    #[inline(always)]
    fn div_self_t(&mut self, value: T) {
        zip_assign!(self[] div_assign value ..3);
    }

    #[inline(always)]
    fn add_self_v(&mut self, other: &Vec3<T>) {
        zip_assign!(self[] add_assign other[] ..3);
    }

    #[inline(always)]
    fn sub_self_v(&mut self, other: &Vec3<T>) {
        zip_assign!(self[] sub_assign other[] ..3);
    }

    #[inline(always)]
    fn mul_self_v(&mut self, other: &Vec3<T>) {
        zip_assign!(self[] mul_assign other[] ..3);
    }

    #[inline(always)]
    fn div_self_v(&mut self, other: &Vec3<T>) {
        zip_assign!(self[] div_assign other[] ..3);
    }
}

impl<T:Copy + Number + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Neg<Vec3<T>> for Vec3<T> {
    #[inline(always)]
    fn neg(&self) -> Vec3<T> {
        Vector3::new(-self[0], -self[1], -self[2])
    }
}

impl<T:Copy + Number + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> NumericVector3<T> for Vec3<T> {
    #[inline(always)]
    fn unit_x() -> Vec3<T> {
        Vector3::new(one::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_y() -> Vec3<T> {
        Vector3::new(zero::<T>(), one::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_z() -> Vec3<T> {
        Vector3::new(zero::<T>(), zero::<T>(), one::<T>())
    }

    #[inline(always)]
    fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vector3::new((self[1] * other[2]) - (self[2] * other[1]),
                     (self[2] * other[0]) - (self[0] * other[2]),
                     (self[0] * other[1]) - (self[1] * other[0]))
    }
    
    #[inline(always)]
    fn cross_self(&mut self, other: &Vec3<T>) {
        *self = self.cross(other);
    }
}

impl<T:Copy + Number + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> ToHomogeneous<Vec4<T>> for Vec3<T> {
    #[inline(always)]
    fn to_homogeneous(&self) -> Vec4<T> {
        Vector4::new(self.x, self.y, self.z, zero())
    }
}

impl<T:Copy + Float + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> EuclideanVector<T> for Vec3<T> {
    #[inline(always)]
    fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline(always)]
    fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline(always)]
    fn distance2(&self, other: &Vec3<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline(always)]
    fn distance(&self, other: &Vec3<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline(always)]
    fn angle(&self, other: &Vec3<T>) -> T {
        atan2(self.cross(other).length(), self.dot(other))
    }

    #[inline(always)]
    fn normalize(&self) -> Vec3<T> {
        self.mul_t(one::<T>()/self.length())
    }

    #[inline(always)]
    fn normalize_to(&self, length: T) -> Vec3<T> {
        self.mul_t(length / self.length())
    }

    #[inline(always)]
    fn lerp(&self, other: &Vec3<T>, amount: T) -> Vec3<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }
    
    #[inline(always)]
    fn normalize_self(&mut self) {
        let n = one::<T>() / self.length();
        self.mul_self_t(n);
    }

    #[inline(always)]
    fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    fn lerp_self(&mut self, other: &Vec3<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Copy + Float + Zero + One + FuzzyEq<T>> FuzzyEq<T> for Vec3<T> {
    #[inline(always)]
    fn fuzzy_eq(&self, other: &Vec3<T>) -> bool {
        self.fuzzy_eq_eps(other, &num::cast(FUZZY_EPSILON))
    }

    #[inline(always)]
    fn fuzzy_eq_eps(&self, other: &Vec3<T>, epsilon: &T) -> bool {
        self[0].fuzzy_eq_eps(&other[0], epsilon) &&
        self[1].fuzzy_eq_eps(&other[1], epsilon) &&
        self[2].fuzzy_eq_eps(&other[2], epsilon)
    }
}

impl<T:Copy + Ord + Eq> OrdinalVector<T, Vec3<bool>> for Vec3<T> {
    #[inline(always)]
    fn less_than(&self, other: &Vec3<T>) -> Vec3<bool> {
        zip_vec3!(self[] lt other[])
    }

    #[inline(always)]
    fn less_than_equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        zip_vec3!(self[] le other[])
    }

    #[inline(always)]
    fn greater_than(&self, other: &Vec3<T>) -> Vec3<bool> {
        zip_vec3!(self[] gt other[])
    }

    #[inline(always)]
    fn greater_than_equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        zip_vec3!(self[] ge other[])
    }
}

impl<T:Copy + Eq> EquableVector<T, Vec3<bool>> for Vec3<T> {
    #[inline(always)]
    fn equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        zip_vec3!(self[] eq other[])
    }

    #[inline(always)]
    fn not_equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        zip_vec3!(self[] ne other[])
    }
}

impl BooleanVector for Vec3<bool> {
    #[inline(always)]
    fn any(&self) -> bool {
        self[0] || self[1] || self[2]
    }

    #[inline(always)]
    fn all(&self) -> bool {
        self[0] && self[1] && self[2]
    }

    #[inline(always)]
    fn not(&self) -> Vec3<bool> {
        Vector3::new(!self[0], !self[1], !self[2])
    }
}

macro_rules! vec3_type(
    ($name:ident <bool>) => (
        pub impl $name {
            #[inline(always)] fn new(x: bool, y: bool, z: bool) -> $name { Vector3::new(x, y, z) }
            #[inline(always)] fn from_value(v: bool) -> $name { Vector::from_value(v) }

            #[inline(always)] fn dim() -> uint { 3 }
            #[inline(always)] fn size_of() -> uint { sys::size_of::<$name>() }
        }
    );
    ($name:ident <$T:ty>) => (
        pub impl $name {
            #[inline(always)] fn new(x: $T, y: $T, z: $T) -> $name { Vector3::new(x, y, z) }
            #[inline(always)] fn from_value(v: $T) -> $name { Vector::from_value(v) }
            #[inline(always)] fn identity() -> $name { NumericVector::identity() }
            #[inline(always)] fn zero() -> $name { NumericVector::zero() }

            #[inline(always)] fn unit_x() -> $name { NumericVector3::unit_x() }
            #[inline(always)] fn unit_y() -> $name { NumericVector3::unit_y() }
            #[inline(always)] fn unit_z() -> $name { NumericVector3::unit_z() }

            #[inline(always)] fn dim() -> uint { 3 }
            #[inline(always)] fn size_of() -> uint { sys::size_of::<$name>() }
        }
    );
)

// GLSL-style type aliases, corresponding to Section 4.1.5 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

// a three-component single-precision floating-point vector
pub type vec3  = Vec3<f32>;
// a three-component double-precision floating-point vector
pub type dvec3 = Vec3<f64>;
// a three-component Boolean vector
pub type bvec3 = Vec3<bool>;
// a three-component signed integer vector
pub type ivec3 = Vec3<i32>;
// a three-component unsigned integer vector
pub type uvec3 = Vec3<u32>;

vec3_type!(vec3<f32>)
vec3_type!(dvec3<f64>)
vec3_type!(bvec3<bool>)
vec3_type!(ivec3<i32>)
vec3_type!(uvec3<u32>)

// Rust-style type aliases
pub type Vec3f   = Vec3<float>;
pub type Vec3f32 = Vec3<f32>;
pub type Vec3f64 = Vec3<f64>;
pub type Vec3i   = Vec3<int>;
pub type Vec3i8  = Vec3<i8>;
pub type Vec3i16 = Vec3<i16>;
pub type Vec3i32 = Vec3<i32>;
pub type Vec3i64 = Vec3<i64>;
pub type Vec3u   = Vec3<uint>;
pub type Vec3u8  = Vec3<u8>;
pub type Vec3u16 = Vec3<u16>;
pub type Vec3u32 = Vec3<u32>;
pub type Vec3u64 = Vec3<u64>;
pub type Vec3b   = Vec3<bool>;

vec3_type!(Vec3f<float>)
vec3_type!(Vec3f32<f32>)
vec3_type!(Vec3f64<f64>)
vec3_type!(Vec3i<int>)
vec3_type!(Vec3i8<i8>)
vec3_type!(Vec3i16<i16>)
vec3_type!(Vec3i32<i32>)
vec3_type!(Vec3i64<i64>)
vec3_type!(Vec3u<uint>)
vec3_type!(Vec3u8<u8>)
vec3_type!(Vec3u16<u16>)
vec3_type!(Vec3u32<u32>)
vec3_type!(Vec3u64<u64>)
vec3_type!(Vec3b<bool>)

/**
 * A 4-dimensional vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 *
 * # Fields
 *
 * * `x` - the first component of the vector
 * * `y` - the second component of the vector
 * * `z` - the third component of the vector
 * * `w` - the fourth component of the vector
 */
#[deriving(Eq)]
pub struct Vec4<T> { x: T, y: T, z: T, w: T }

impl<T:Copy + Eq> Vector<T> for Vec4<T> {
    #[inline(always)]
    fn from_value(value: T) -> Vec4<T> {
        Vector4::new(value, value, value, value)
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { cast::transmute(self) }
    }
    
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &'self mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => fail!(fmt!("index out of bounds: expected an index from 0 to 3, but found %u", i))
        }
    }

    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        *self.index_mut(a) <-> *self.index_mut(b);
    }
}

impl<T> Vector4<T> for Vec4<T> {
    #[inline(always)]
    fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 { x: x, y: y, z: z, w: w }
    }
}

impl<T:Copy + Eq> Index<uint, T> for Vec4<T> {
    #[inline(always)]
    fn index(&self, i: &uint) -> T {
        unsafe { do vec::raw::buf_as_slice(self.to_ptr(), 4) |slice| { slice[*i] } }
    }
}

impl<T:Copy + Number + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> NumericVector<T> for Vec4<T> {
    #[inline(always)]
    fn identity() -> Vec4<T> {
        Vector4::new(one::<T>(), one::<T>(), one::<T>(), one::<T>())
    }

    #[inline(always)]
    fn zero() -> Vec4<T> {
        Vector4::new(zero::<T>(), zero::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self[0] == zero() &&
        self[1] == zero() &&
        self[2] == zero() &&
        self[3] == zero()
    }

    #[inline(always)]
    fn mul_t(&self, value: T) -> Vec4<T> {
        zip_vec4!(self[] mul value)
    }

    #[inline(always)]
    fn div_t(&self, value: T) -> Vec4<T> {
        zip_vec4!(self[] div value)
    }

    #[inline(always)]
    fn add_v(&self, other: &Vec4<T>) -> Vec4<T> {
        zip_vec4!(self[] add other[])
    }

    #[inline(always)]
    fn sub_v(&self, other: &Vec4<T>) -> Vec4<T> {
        zip_vec4!(self[] sub other[])
    }

    #[inline(always)]
    fn mul_v(&self, other: &Vec4<T>) -> Vec4<T> {
        zip_vec4!(self[] mul other[])
    }

    #[inline(always)]
    fn div_v(&self, other: &Vec4<T>) -> Vec4<T> {
        zip_vec4!(self[] div other[])
    }

    #[inline(always)]
    fn dot(&self, other: &Vec4<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2] +
        self[3] * other[3]
    }
    
    #[inline(always)]
    fn neg_self(&mut self) {
        *self.index_mut(0) = -self[0];
        *self.index_mut(1) = -self[1];
        *self.index_mut(2) = -self[2];
        *self.index_mut(3) = -self[3];
    }

    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        zip_assign!(self[] mul_assign value ..4);
    }

    #[inline(always)]
    fn div_self_t(&mut self, value: T) {
        zip_assign!(self[] div_assign value ..4);
    }

    #[inline(always)]
    fn add_self_v(&mut self, other: &Vec4<T>) {
        zip_assign!(self[] add_assign other[] ..4);
    }

    #[inline(always)]
    fn sub_self_v(&mut self, other: &Vec4<T>) {
        zip_assign!(self[] sub_assign other[] ..4);
    }

    #[inline(always)]
    fn mul_self_v(&mut self, other: &Vec4<T>) {
        zip_assign!(self[] mul_assign other[] ..4);
    }

    #[inline(always)]
    fn div_self_v(&mut self, other: &Vec4<T>) {
        zip_assign!(self[] div_assign other[] ..4);
    }
}

impl<T:Copy + Number + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Neg<Vec4<T>> for Vec4<T> {
    #[inline(always)]
    fn neg(&self) -> Vec4<T> {
        Vector4::new(-self[0], -self[1], -self[2], -self[3])
    }
}

impl<T:Copy + Number + Zero + One> NumericVector4<T> for Vec4<T> {
    #[inline(always)]
    fn unit_x() -> Vec4<T> {
        Vector4::new(one::<T>(), zero::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_y() -> Vec4<T> {
        Vector4::new(zero::<T>(), one::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_z() -> Vec4<T> {
        Vector4::new(zero::<T>(), zero::<T>(), one::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_w() -> Vec4<T> {
        Vector4::new(zero::<T>(), zero::<T>(), zero::<T>(), one::<T>())
    }
}

impl<T:Copy + Float + Zero + One + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> EuclideanVector<T> for Vec4<T> {
    #[inline(always)]
    fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline(always)]
    fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline(always)]
    fn distance2(&self, other: &Vec4<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline(always)]
    fn distance(&self, other: &Vec4<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline(always)]
    fn angle(&self, other: &Vec4<T>) -> T {
        acos(self.dot(other) / (self.length() * other.length()))
    }

    #[inline(always)]
    fn normalize(&self) -> Vec4<T> {
        self.mul_t(one::<T>()/self.length())
    }

    #[inline(always)]
    fn normalize_to(&self, length: T) -> Vec4<T> {
        self.mul_t(length / self.length())
    }

    #[inline(always)]
    fn lerp(&self, other: &Vec4<T>, amount: T) -> Vec4<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }
    
    #[inline(always)]
    fn normalize_self(&mut self) {
        let n = one::<T>() / self.length();
        self.mul_self_t(n);
    }

    #[inline(always)]
    fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    fn lerp_self(&mut self, other: &Vec4<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Copy + Float + Zero + One + FuzzyEq<T>> FuzzyEq<T> for Vec4<T> {
    #[inline(always)]
    fn fuzzy_eq(&self, other: &Vec4<T>) -> bool {
        self.fuzzy_eq_eps(other, &num::cast(FUZZY_EPSILON))
    }

    #[inline(always)]
    fn fuzzy_eq_eps(&self, other: &Vec4<T>, epsilon: &T) -> bool {
        self[0].fuzzy_eq_eps(&other[0], epsilon) &&
        self[1].fuzzy_eq_eps(&other[1], epsilon) &&
        self[2].fuzzy_eq_eps(&other[2], epsilon) &&
        self[3].fuzzy_eq_eps(&other[3], epsilon)
    }
}

impl<T:Copy + Ord + Eq> OrdinalVector<T, Vec4<bool>> for Vec4<T> {
    #[inline(always)]
    fn less_than(&self, other: &Vec4<T>) -> Vec4<bool> {
        zip_vec4!(self[] lt other[])
    }

    #[inline(always)]
    fn less_than_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        zip_vec4!(self[] le other[])
    }

    #[inline(always)]
    fn greater_than(&self, other: &Vec4<T>) -> Vec4<bool> {
        zip_vec4!(self[] gt other[])
    }

    #[inline(always)]
    fn greater_than_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        zip_vec4!(self[] ge other[])
    }
}

impl<T:Copy + Eq> EquableVector<T, Vec4<bool>> for Vec4<T> {
    #[inline(always)]
    fn equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        zip_vec4!(self[] eq other[])
    }

    #[inline(always)]
    fn not_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        zip_vec4!(self[] ne other[])
    }
}

impl BooleanVector for Vec4<bool> {
    #[inline(always)]
    fn any(&self) -> bool {
        self[0] || self[1] || self[2] || self[3]
    }

    #[inline(always)]
    fn all(&self) -> bool {
        self[0] && self[1] && self[2] && self[3]
    }

    #[inline(always)]
    fn not(&self) -> Vec4<bool> {
        Vector4::new(!self[0], !self[1], !self[2], !self[3])
    }
}

macro_rules! vec4_type(
    ($name:ident <bool>) => (
        pub impl $name {
            #[inline(always)] fn new(x: bool, y: bool, z: bool, w: bool) -> $name { Vector4::new(x, y, z, w) }
            #[inline(always)] fn from_value(v: bool) -> $name { Vector::from_value(v) }

            #[inline(always)] fn dim() -> uint { 4 }
            #[inline(always)] fn size_of() -> uint { sys::size_of::<$name>() }
        }
    );
    ($name:ident <$T:ty>) => (
        pub impl $name {
            #[inline(always)] fn new(x: $T, y: $T, z: $T, w: $T) -> $name { Vector4::new(x, y, z, w) }
            #[inline(always)] fn from_value(v: $T) -> $name { Vector::from_value(v) }
            #[inline(always)] fn identity() -> $name { NumericVector::identity() }
            #[inline(always)] fn zero() -> $name { NumericVector::zero() }

            #[inline(always)] fn unit_x() -> $name { NumericVector4::unit_x() }
            #[inline(always)] fn unit_y() -> $name { NumericVector4::unit_y() }
            #[inline(always)] fn unit_z() -> $name { NumericVector4::unit_z() }
            #[inline(always)] fn unit_w() -> $name { NumericVector4::unit_w() }

            #[inline(always)] fn dim() -> uint { 4 }
            #[inline(always)] fn size_of() -> uint { sys::size_of::<$name>() }
        }
    );
)

// GLSL-style type aliases, corresponding to Section 4.1.5 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

// a four-component single-precision floating-point vector
pub type vec4  = Vec4<f32>;
// a four-component double-precision floating-point vector
pub type dvec4 = Vec4<f64>;
// a four-component Boolean vector
pub type bvec4 = Vec4<bool>;
// a four-component signed integer vector
pub type ivec4 = Vec4<i32>;
// a four-component unsigned integer vector
pub type uvec4 = Vec4<u32>;

vec4_type!(vec4<f32>)
vec4_type!(dvec4<f64>)
vec4_type!(bvec4<bool>)
vec4_type!(ivec4<i32>)
vec4_type!(uvec4<u32>)

// Rust-style type aliases
pub type Vec4f   = Vec4<float>;
pub type Vec4f32 = Vec4<f32>;
pub type Vec4f64 = Vec4<f64>;
pub type Vec4i   = Vec4<int>;
pub type Vec4i8  = Vec4<i8>;
pub type Vec4i16 = Vec4<i16>;
pub type Vec4i32 = Vec4<i32>;
pub type Vec4i64 = Vec4<i64>;
pub type Vec4u   = Vec4<uint>;
pub type Vec4u8  = Vec4<u8>;
pub type Vec4u16 = Vec4<u16>;
pub type Vec4u32 = Vec4<u32>;
pub type Vec4u64 = Vec4<u64>;
pub type Vec4b   = Vec4<bool>;

vec4_type!(Vec4f<float>)
vec4_type!(Vec4f32<f32>)
vec4_type!(Vec4f64<f64>)
vec4_type!(Vec4i<int>)
vec4_type!(Vec4i8<i8>)
vec4_type!(Vec4i16<i16>)
vec4_type!(Vec4i32<i32>)
vec4_type!(Vec4i64<i64>)
vec4_type!(Vec4u<uint>)
vec4_type!(Vec4u8<u8>)
vec4_type!(Vec4u16<u16>)
vec4_type!(Vec4u32<u32>)
vec4_type!(Vec4u64<u64>)
vec4_type!(Vec4b<bool>)