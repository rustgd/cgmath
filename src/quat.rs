/**
 * > Every morning in the early part of October 1843, on my coming down to
 *   breakfast, your brother William Edward and yourself used to ask me: "Well,
 *   Papa, can you multiply triples?" Whereto I was always obliged to reply,
 *   with a sad shake of the head, "No, I can only add and subtract them."
 *
 *   Sir William Hamilton
 */

use std::cast::transmute;
use std::cmp::ApproxEq;
use std::num::{Zero, One};

use mat::{Mat3, BaseMat3};
use vec::{Vec3, BaseVec3, AffineVec, NumVec, NumVec3};

use num::NumAssign;

/**
 * A quaternion in scalar/vector form
 *
 * # Type parameters
 *
 * * `T` - The type of the components. Should be a floating point type.
 *
 * # Fields
 *
 * * `s` - the scalar component
 * * `v` - a vector containing the three imaginary components
 */
#[deriving(Eq)]
pub struct Quat<T> { s: T, v: Vec3<T> }

pub impl<T:Copy + Float + NumAssign> Quat<T> {
    /**
     * Construct the quaternion from one scalar component and three
     * imaginary components
     *
     * # Arguments
     *
     * * `w`  - the scalar component
     * * `xi` - the fist imaginary component
     * * `yj` - the second imaginary component
     * * `zk` - the third imaginary component
     */
    #[inline(always)]
    fn new(w: T, xi: T, yj: T, zk: T) -> Quat<T> {
        Quat::from_sv(w, BaseVec3::new(xi, yj, zk))
    }

    /**
     * Construct the quaternion from a scalar and a vector
     *
     * # Arguments
     *
     * * `s` - the scalar component
     * * `v` - a vector containing the three imaginary components
     */
    #[inline(always)]
    fn from_sv(s: T, v: Vec3<T>) -> Quat<T> {
        Quat { s: s, v: v }
    }

    /**
     * # Return value
     *
     * The multiplicative identity, ie: `q = 1 + 0i + 0j + 0i`
     */
    #[inline(always)]
    fn identity() -> Quat<T> {
        Quat::new(One::one(),
                  Zero::zero(),
                  Zero::zero(),
                  Zero::zero())
    }

    /**
     * # Return value
     *
     * The additive identity, ie: `q = 0 + 0i + 0j + 0i`
     */
    #[inline(always)]
    fn zero() -> Quat<T> {
        Quat::new(Zero::zero(),
                  Zero::zero(),
                  Zero::zero(),
                  Zero::zero())
    }

    #[inline(always)]
    fn from_angle_x(radians: T) -> Quat<T> {
        let _2 = num::cast(2);
        Quat::new((radians / _2).cos(),
                  radians.sin(),
                  Zero::zero(),
                  Zero::zero())
    }

    #[inline(always)]
    fn from_angle_y(radians: T) -> Quat<T> {
        let _2 = num::cast(2);
        Quat::new((radians / _2).cos(),
                  Zero::zero(),
                  radians.sin(),
                  Zero::zero())
    }

    #[inline(always)]
    fn from_angle_z(radians: T) -> Quat<T> {
        let _2 = num::cast(2);
        Quat::new((radians / _2).cos(),
                  Zero::zero(),
                  Zero::zero(),
                  radians.sin())
    }

    #[inline(always)]
    fn from_angle_xyz(radians_x: T, radians_y: T, radians_z: T) -> Quat<T> {
        // http://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles#Conversion
        let _2 = num::cast(2);
        let xdiv2 = radians_x / _2;
        let ydiv2 = radians_y / _2;
        let zdiv2 = radians_z / _2;
        Quat::new(zdiv2.cos() * xdiv2.cos() * ydiv2.cos() + zdiv2.sin() * xdiv2.sin() * ydiv2.sin(),
                  zdiv2.sin() * xdiv2.cos() * ydiv2.cos() - zdiv2.cos() * xdiv2.sin() * ydiv2.sin(),
                  zdiv2.cos() * xdiv2.sin() * ydiv2.cos() + zdiv2.sin() * xdiv2.cos() * ydiv2.sin(),
                  zdiv2.cos() * xdiv2.cos() * ydiv2.sin() - zdiv2.sin() * xdiv2.sin() * ydiv2.cos())
    }

    #[inline(always)]
    fn from_angle_axis(radians: T, axis: &Vec3<T>) -> Quat<T> {
        let half = radians / num::cast(2);
        Quat::from_sv(half.cos(), axis.mul_t(half.sin()))
    }

    #[inline(always)]
    fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Quat<T> {
        let m: Mat3<T> = BaseMat3::from_axes(x, y, z); m.to_quat()
    }

    fn get_angle_axis(&self) -> (T, Vec3<T>) {
        fail!(~"Not yet implemented.")
    }

    #[inline(always)]
    fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> Quat<T> {
        let m: Mat3<T> = BaseMat3::look_at(dir, up); m.to_quat()
    }

    /**
     * # Return value
     *
     * The result of multiplying the quaternion a scalar
     */
    #[inline(always)]
    fn mul_t(&self, value: T) -> Quat<T> {
        Quat::new(self[0] * value,
                  self[1] * value,
                  self[2] * value,
                  self[3] * value)
    }

    /**
     * # Return value
     *
     * The result of dividing the quaternion a scalar
     */
    #[inline(always)]
    fn div_t(&self, value: T) -> Quat<T> {
        Quat::new(self[0] / value,
                  self[1] / value,
                  self[2] / value,
                  self[3] / value)
    }

    /**
     * # Return value
     *
     * The result of multiplying the quaternion by a vector
     */
    #[inline(always)]
    fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T>  {
        let tmp = self.v.cross(vec).add_v(&vec.mul_t(self.s));
        self.v.cross(&tmp).mul_t(num::cast(2)).add_v(vec)
    }

    /**
     * # Return value
     *
     * The sum of this quaternion and `other`
     */
    #[inline(always)]
    fn add_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(self[0] + other[0],
                  self[1] + other[1],
                  self[2] + other[2],
                  self[3] + other[3])
    }

    /**
     * # Return value
     *
     * The sum of this quaternion and `other`
     */
    #[inline(always)]
    fn sub_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(self[0] - other[0],
                  self[1] - other[1],
                  self[2] - other[2],
                  self[3] - other[3])
    }

    /**
     * # Return value
     *
     * The the result of multipliplying the quaternion by `other`
     */
    #[inline(always)]
    fn mul_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(self.s * other.s   - self.v.x * other.v.x - self.v.y * other.v.y - self.v.z * other.v.z,
                  self.s * other.v.x + self.v.x * other.s   + self.v.y * other.v.z - self.v.z * other.v.y,
                  self.s * other.v.y + self.v.y * other.s   + self.v.z * other.v.x - self.v.x * other.v.z,
                  self.s * other.v.z + self.v.z * other.s   + self.v.x * other.v.y - self.v.y * other.v.x)
    }

    /**
     * # Return value
     *
     * The dot product of the quaternion and `other`
     */
    #[inline(always)]
    fn dot(&self, other: &Quat<T>) -> T {
        self.s * other.s + self.v.dot(&other.v)
    }

    /**
     * # Return value
     *
     * The conjugate of the quaternion
     */
    #[inline(always)]
    fn conjugate(&self) -> Quat<T> {
        Quat::from_sv(self.s, -self.v)
    }

    /**
     * # Return value
     *
     * The multiplicative inverse of the quaternion
     */
    #[inline(always)]
    fn inverse(&self) -> Quat<T> {
        self.conjugate().div_t(self.magnitude2())
    }

    /**
     * # Return value
     *
     * The squared magnitude of the quaternion. This is useful for
     * magnitude comparisons where the exact magnitude does not need to be
     * calculated.
     */
    #[inline(always)]
    fn magnitude2(&self) -> T {
        self.s * self.s + self.v.length2()
    }

    /**
     * # Return value
     *
     * The magnitude of the quaternion
     *
     * # Performance notes
     *
     * For instances where the exact magnitude of the quaternion does not need
     * to be known, for example for quaternion-quaternion magnitude comparisons,
     * it is advisable to use the `magnitude2` method instead.
     */
    #[inline(always)]
    fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }

    /**
     * # Return value
     *
     * The normalized quaternion
     */
    #[inline(always)]
    fn normalize(&self) -> Quat<T> {
        self.mul_t(One::one::<T>()/self.magnitude())
    }

    /**
     * Normalised linear interpolation
     *
     * # Return value
     *
     * The intoperlated quaternion
     */
    #[inline(always)]
    fn nlerp(&self, other: &Quat<T>, amount: T) -> Quat<T> {
        self.mul_t(One::one::<T>() - amount).add_q(&other.mul_t(amount)).normalize()
    }

    /**
     * Spherical Linear Intoperlation
     *
     * Perform a spherical linear interpolation between the quaternion and
     * `other`. Both quaternions should be normalized first.
     *
     * # Return value
     *
     * The intoperlated quaternion
     *
     * # Performance notes
     *
     * The `acos` operation used in `slerp` is an expensive operation, so unless
     * your quarternions a far away from each other it's generally more advisable
     * to use `nlerp` when you know your rotations are going to be small.
     *
     * - [Understanding Slerp, Then Not Using It]
     *   (http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/)
     * - [Arcsynthesis OpenGL tutorial]
     *   (http://www.arcsynthesis.org/gltut/Positioning/Tut08%20Interpolation.html)
     */
    #[inline(always)]
    fn slerp(&self, other: &Quat<T>, amount: T) -> Quat<T> {
        let dot = self.dot(other);

        let dot_threshold = num::cast(0.9995);

        if dot > dot_threshold {
            return self.nlerp(other, amount);               // if quaternions are close together use `nlerp`
        } else {
            let robust_dot = dot.clamp(&-One::one::<T>(),
                                       &One::one());        // stay within the domain of acos()

            let theta_0 = robust_dot.acos();                // the angle between the quaternions
            let theta = theta_0 * amount;                   // the fraction of theta specified by `amount`

            let q = other.sub_q(&self.mul_t(robust_dot))
                         .normalize();

            return self.mul_t(theta.cos())
                       .add_q(&q.mul_t(theta.sin()));
        }
    }

    /**
     * # Return value
     *
     * A pointer to the first component of the quaternion
     */
    #[inline(always)]
    fn to_ptr(&self) -> *T {
        unsafe { cast::transmute(self) }
    }

    /**
     * Convert the quaternion to a 3 x 3 rotation matrix
     */
    #[inline(always)]
    fn to_mat3(&self) -> Mat3<T> {
        let x2 = self.v.x + self.v.x;
        let y2 = self.v.y + self.v.y;
        let z2 = self.v.z + self.v.z;

        let xx2 = x2 * self.v.x;
        let xy2 = x2 * self.v.y;
        let xz2 = x2 * self.v.z;

        let yy2 = y2 * self.v.y;
        let yz2 = y2 * self.v.z;
        let zz2 = z2 * self.v.z;

        let sy2 = y2 * self.s;
        let sz2 = z2 * self.s;
        let sx2 = x2 * self.s;

        let _1: T = One::one();

        BaseMat3::new(_1 - yy2 - zz2,      xy2 + sz2,      xz2 - sy2,
                           xy2 - sz2, _1 - xx2 - zz2,      yz2 + sx2,
                           xz2 + sy2,      yz2 - sx2, _1 - xx2 - yy2)
    }
}

impl<T:Copy> Index<uint, T> for Quat<T> {
    #[inline(always)]
    fn index(&self, i: &uint) -> T {
        unsafe { transmute::<Quat<T>,[T,..4]>(*self)[*i] }
    }
}

impl<T:Copy + Float + NumAssign> Neg<Quat<T>> for Quat<T> {
    #[inline(always)]
    fn neg(&self) -> Quat<T> {
        Quat::new(-self[0], -self[1], -self[2], -self[3])
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Quat<T> {
    #[inline(always)]
    fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline(always)]
    fn approx_eq(&self, other: &Quat<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline(always)]
    fn approx_eq_eps(&self, other: &Quat<T>, epsilon: &T) -> bool {
        self[0].approx_eq_eps(&other[0], epsilon) &&
        self[1].approx_eq_eps(&other[1], epsilon) &&
        self[2].approx_eq_eps(&other[2], epsilon) &&
        self[3].approx_eq_eps(&other[3], epsilon)
    }
}
