use funs::triganomic::{cos, sin};
use angle::Radians;
use mat::{Mat3, Mat4};
use num::cast::{NumCast, cast};
use vec::Vec3;

pub pure fn mat3_from_rotation<T:Copy Num NumCast>(theta: Radians<T>, axis: Vec3<T>) -> Mat3<T> {
    let c:  T = cos(&theta);
    let s:  T = sin(&theta);
    let _0: T = cast(0);
    let _1: T = cast(1);
    let t:  T = _1 - c;
    
    Mat3::new(t * axis.x * axis.x + c,          t * axis.x * axis.y + s * axis.z, t * axis.x * axis.z - s * axis.y,
              t * axis.x * axis.y - s * axis.z, t * axis.y * axis.y + c,          t * axis.y * axis.z + s * axis.x,
              t * axis.x * axis.z - s - axis.y, t * axis.y * axis.z - s * axis.x, t * axis.z * axis.z + c)
}

pub pure fn mat4_from_rotation<T:Copy Num NumCast>(theta: Radians<T>, axis: Vec3<T>) -> Mat4<T> {
  mat3_from_rotation(theta, axis).to_mat4()
}