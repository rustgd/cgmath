use core::cmp::ApproxEq;
use core::num::Zero::zero;
use core::num::One::one;

use num::NumAssign;

/**
 * The base generic vector trait.
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 */
pub trait BaseVec<T>: Index<uint,T> + Eq {
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
    fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T;

    /**
     * Swap two components of the vector in place
     */
    fn swap(&mut self, a: uint, b: uint);
}

/**
 * A generic 2-dimensional vector
 */
pub trait BaseVec2<T>: BaseVec<T> {
    fn new(x: T, y: T) -> Self;
}

/**
 * A generic 3-dimensional vector
 */
pub trait BaseVec3<T>: BaseVec<T> {
    fn new(x: T, y: T, z: T) -> Self;
}

/**
 * A generic 4-dimensional vector
 */
pub trait BaseVec4<T>: BaseVec<T> {
    fn new(x: T, y: T, z: T, w: T) -> Self;
}

/**
 * A vector with numeric components
 */
pub trait NumVec<T>: BaseVec<T> + Neg<Self> {
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
pub trait NumVec2<T>: NumVec<T> {
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
pub trait NumVec3<T>: NumVec<T> {
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
pub trait NumVec4<T>: NumVec<T> {
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
pub trait AffineVec<T>: NumVec<T> {
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
pub trait OrdVec<T, BoolVec>: BaseVec<T> {
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
pub trait EqVec<T, BoolVec>: BaseVec<T> {
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
pub trait BoolVec: BaseVec<bool> {
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

pub trait TrigVec<T>: BaseVec<T> {
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

pub trait ExpVec<T>: BaseVec<T> {
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

pub trait ApproxVec<T>: BaseVec<T> {
    // Whole-number approximation functions
    fn floor(&self)                    -> Self;
    fn trunc(&self)                    -> Self;
    fn round(&self)                    -> Self;
    // fn round_even(&self)            -> Self;
    fn ceil(&self)                     -> Self;
    fn fract(&self)                    -> Self;
}

pub trait SignedVec<T,BV>: BaseVec<T> {
    fn is_positive(&self)    -> BV;
    fn is_negative(&self)    -> BV;
    fn is_nonpositive(&self) -> BV;
    fn is_nonnegative(&self) -> BV;

    fn abs(&self) -> Self;
    fn sign(&self) -> Self;
    fn copysign(&self, other: Self) -> Self;
}

pub trait ExtentVec<T>: BaseVec<T> {
    fn min_v(&self, other: &Self) -> Self;
    fn max_v(&self, other: &Self) -> Self;
    fn clamp_v(&self, mn: &Self, mx: &Self) -> Self;

    fn min_t(&self, other: T) -> Self;
    fn max_t(&self, other: T) -> Self;
    fn clamp_t(&self, mn: T, mx: T) -> Self;
}

pub trait MixVec<T>: BaseVec<T> {
    // Functions for blending numbers together
    fn mix(&self, other: Self, value: Self) -> Self;
    fn smooth_step(&self, edge0: Self, edge1: Self) -> Self;
    fn step(&self, edge: Self) -> Self;
}

// Utility macros

macro_rules! zip_vec2(
    ($a:ident[] $method:ident $b:ident[]) => (
        BaseVec2::new($a[0].$method(&($b[0])),
                      $a[1].$method(&($b[1])))
    );
    ($a:ident[] $method:ident $b:ident) => (
        BaseVec2::new($a[0].$method(&($b)),
                      $a[1].$method(&($b)))
    );
)

macro_rules! zip_vec3(
    ($a:ident[] $method:ident $b:ident[]) => (
        BaseVec3::new($a[0].$method(&($b[0])),
                      $a[1].$method(&($b[1])),
                      $a[2].$method(&($b[2])))
    );
    ($a:ident[] $method:ident $b:ident) => (
        BaseVec3::new($a[0].$method(&($b)),
                      $a[1].$method(&($b)),
                      $a[2].$method(&($b)))
    );
)

macro_rules! zip_vec4(
    ($a:ident[] $method:ident $b:ident[]) => (
        BaseVec4::new($a[0].$method(&($b[0])),
                      $a[1].$method(&($b[1])),
                      $a[2].$method(&($b[2])),
                      $a[3].$method(&($b[3])))
    );
    ($a:ident[] $method:ident $b:ident) => (
        BaseVec4::new($a[0].$method(&($b)),
                      $a[1].$method(&($b)),
                      $a[2].$method(&($b)),
                      $a[3].$method(&($b)))
    );
)

macro_rules! zip_assign(
    ($a:ident[] $method:ident $b:ident[] ..2) => ({ $a.index_mut(0).$method(&$b[0]);    $a.index_mut(1).$method(&$b[1]); });
    ($a:ident[] $method:ident $b:ident[] ..3) => ({ zip_assign!($a[] $method $b[] ..2); $a.index_mut(2).$method(&$b[2]); });
    ($a:ident[] $method:ident $b:ident[] ..4) => ({ zip_assign!($a[] $method $b[] ..3); $a.index_mut(3).$method(&$b[3]); });

    ($a:ident[] $method:ident $b:ident   ..2) => ({ $a.index_mut(0).$method(&$b);       $a.index_mut(1).$method(&$b);    });
    ($a:ident[] $method:ident $b:ident   ..3) => ({ zip_assign!($a[] $method $b ..2);   $a.index_mut(2).$method(&$b);    });
    ($a:ident[] $method:ident $b:ident   ..4) => ({ zip_assign!($a[] $method $b ..3);   $a.index_mut(3).$method(&$b);    });
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

impl<T:Copy + Eq> BaseVec<T> for Vec2<T> {
    #[inline(always)]
    fn from_value(value: T) -> Vec2<T> {
        BaseVec2::new(value, value)
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { cast::transmute(self) }
    }

    #[inline(always)]
    fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T {
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

impl<T> BaseVec2<T> for Vec2<T> {
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

impl<T:Copy + Num + NumAssign> NumVec<T> for Vec2<T> {
    #[inline(always)]
    fn identity() -> Vec2<T> {
        BaseVec2::new(one::<T>(), one::<T>())
    }

    #[inline(always)]
    fn zero() -> Vec2<T> {
        BaseVec2::new(zero::<T>(), zero::<T>())
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

impl<T:Copy + Num> Neg<Vec2<T>> for Vec2<T> {
    #[inline(always)]
    fn neg(&self) -> Vec2<T> {
        BaseVec2::new(-self[0], -self[1])
    }
}

impl<T:Copy + Num> NumVec2<T> for Vec2<T> {
    #[inline(always)]
    fn unit_x() -> Vec2<T> {
        BaseVec2::new(one::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_y() -> Vec2<T> {
        BaseVec2::new(zero::<T>(), one::<T>())
    }

    #[inline(always)]
    fn perp_dot(&self, other: &Vec2<T>) ->T {
        (self[0] * other[1]) - (self[1] * other[0])
    }
}

impl<T:Copy + Num> ToHomogeneous<Vec3<T>> for Vec2<T> {
    #[inline(always)]
    fn to_homogeneous(&self) -> Vec3<T> {
        BaseVec3::new(self.x, self.y, zero())
    }
}

impl<T:Copy + Real + NumAssign> AffineVec<T> for Vec2<T> {
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
        self.perp_dot(other).atan2(self.dot(other))
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

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Vec2<T> {
    #[inline(always)]
    fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    fn approx_eq(&self, other: &Vec2<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    fn approx_eq_eps(&self, other: &Vec2<T>, epsilon: &T) -> bool {
        self[0].approx_eq_eps(&other[0], epsilon) &&
        self[1].approx_eq_eps(&other[1], epsilon)
    }
}

impl<T:Copy + Ord + Eq> OrdVec<T, Vec2<bool>> for Vec2<T> {
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

impl<T:Copy + Eq> EqVec<T, Vec2<bool>> for Vec2<T> {
    #[inline(always)]
    fn equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        zip_vec2!(self[] eq other[])
    }

    #[inline(always)]
    fn not_equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        zip_vec2!(self[] ne other[])
    }
}

impl BoolVec for Vec2<bool> {
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
        BaseVec2::new(!self[0], !self[1])
    }
}

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

impl<T:Copy + Eq> BaseVec<T> for Vec3<T> {
    #[inline(always)]
    fn from_value(value: T) -> Vec3<T> {
        BaseVec3::new(value, value, value)
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { cast::transmute(self) }
    }

    #[inline(always)]
    fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T {
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

impl<T> BaseVec3<T> for Vec3<T> {
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

impl<T:Copy + Num + NumAssign> NumVec<T> for Vec3<T> {
    #[inline(always)]
    fn identity() -> Vec3<T> {
        BaseVec3::new(one::<T>(), one::<T>(), one::<T>())
    }

    #[inline(always)]
    fn zero() -> Vec3<T> {
        BaseVec3::new(zero::<T>(), zero::<T>(), zero::<T>())
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

impl<T:Copy + Num> Neg<Vec3<T>> for Vec3<T> {
    #[inline(always)]
    fn neg(&self) -> Vec3<T> {
        BaseVec3::new(-self[0], -self[1], -self[2])
    }
}

impl<T:Copy + Num> NumVec3<T> for Vec3<T> {
    #[inline(always)]
    fn unit_x() -> Vec3<T> {
        BaseVec3::new(one::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_y() -> Vec3<T> {
        BaseVec3::new(zero::<T>(), one::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_z() -> Vec3<T> {
        BaseVec3::new(zero::<T>(), zero::<T>(), one::<T>())
    }

    #[inline(always)]
    fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        BaseVec3::new((self[1] * other[2]) - (self[2] * other[1]),
                      (self[2] * other[0]) - (self[0] * other[2]),
                      (self[0] * other[1]) - (self[1] * other[0]))
    }

    #[inline(always)]
    fn cross_self(&mut self, other: &Vec3<T>) {
        *self = self.cross(other);
    }
}

impl<T:Copy + Num> ToHomogeneous<Vec4<T>> for Vec3<T> {
    #[inline(always)]
    fn to_homogeneous(&self) -> Vec4<T> {
        BaseVec4::new(self.x, self.y, self.z, zero())
    }
}

impl<T:Copy + Real + NumAssign> AffineVec<T> for Vec3<T> {
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
        self.cross(other).length().atan2(self.dot(other))
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

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Vec3<T> {
    #[inline(always)]
    fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    fn approx_eq(&self, other: &Vec3<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    fn approx_eq_eps(&self, other: &Vec3<T>, epsilon: &T) -> bool {
        self[0].approx_eq_eps(&other[0], epsilon) &&
        self[1].approx_eq_eps(&other[1], epsilon) &&
        self[2].approx_eq_eps(&other[2], epsilon)
    }
}

impl<T:Copy + Ord + Eq> OrdVec<T, Vec3<bool>> for Vec3<T> {
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

impl<T:Copy + Eq> EqVec<T, Vec3<bool>> for Vec3<T> {
    #[inline(always)]
    fn equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        zip_vec3!(self[] eq other[])
    }

    #[inline(always)]
    fn not_equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        zip_vec3!(self[] ne other[])
    }
}

impl BoolVec for Vec3<bool> {
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
        BaseVec3::new(!self[0], !self[1], !self[2])
    }
}

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

impl<T:Copy + Eq> BaseVec<T> for Vec4<T> {
    #[inline(always)]
    fn from_value(value: T) -> Vec4<T> {
        BaseVec4::new(value, value, value, value)
    }

    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { cast::transmute(self) }
    }

    #[inline(always)]
    fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T {
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

impl<T> BaseVec4<T> for Vec4<T> {
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

impl<T:Copy + Num + NumAssign> NumVec<T> for Vec4<T> {
    #[inline(always)]
    fn identity() -> Vec4<T> {
        BaseVec4::new(one::<T>(), one::<T>(), one::<T>(), one::<T>())
    }

    #[inline(always)]
    fn zero() -> Vec4<T> {
        BaseVec4::new(zero::<T>(), zero::<T>(), zero::<T>(), zero::<T>())
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

impl<T:Copy + Num> Neg<Vec4<T>> for Vec4<T> {
    #[inline(always)]
    fn neg(&self) -> Vec4<T> {
        BaseVec4::new(-self[0], -self[1], -self[2], -self[3])
    }
}

impl<T:Copy + Num> NumVec4<T> for Vec4<T> {
    #[inline(always)]
    fn unit_x() -> Vec4<T> {
        BaseVec4::new(one::<T>(), zero::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_y() -> Vec4<T> {
        BaseVec4::new(zero::<T>(), one::<T>(), zero::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_z() -> Vec4<T> {
        BaseVec4::new(zero::<T>(), zero::<T>(), one::<T>(), zero::<T>())
    }

    #[inline(always)]
    fn unit_w() -> Vec4<T> {
        BaseVec4::new(zero::<T>(), zero::<T>(), zero::<T>(), one::<T>())
    }
}

impl<T:Copy + Real + NumAssign> AffineVec<T> for Vec4<T> {
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
        (self.dot(other) / (self.length() * other.length())).acos()
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

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Vec4<T> {
    #[inline(always)]
    fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    fn approx_eq(&self, other: &Vec4<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    fn approx_eq_eps(&self, other: &Vec4<T>, epsilon: &T) -> bool {
        self[0].approx_eq_eps(&other[0], epsilon) &&
        self[1].approx_eq_eps(&other[1], epsilon) &&
        self[2].approx_eq_eps(&other[2], epsilon) &&
        self[3].approx_eq_eps(&other[3], epsilon)
    }
}

impl<T:Copy + Ord + Eq> OrdVec<T, Vec4<bool>> for Vec4<T> {
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

impl<T:Copy + Eq> EqVec<T, Vec4<bool>> for Vec4<T> {
    #[inline(always)]
    fn equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        zip_vec4!(self[] eq other[])
    }

    #[inline(always)]
    fn not_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        zip_vec4!(self[] ne other[])
    }
}

impl BoolVec for Vec4<bool> {
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
        BaseVec4::new(!self[0], !self[1], !self[2], !self[3])
    }
}
