use num::Num;
use ncast::*;
use ntrait::Float;
use funs::exp::Exp;
use funs::trig::*;
use matrix::Mat4;

pub fn mat4_from_rotation
<
    ThetaT:NumCast Float,
    T:Copy Num NumCast
>
(theta: Radians<ThetaT>, axis: Vec3<T>) -> Mat4<T> {
    let c:  T = cos(&theta);
    let s:  T = sin(&theta);
    let _0: T = cast(0);
    let _1: T = cast(1);
    let t:  T = _1 - c;
    
    Mat4::new(t * axis.x * axis.x + c,          t * axis.x * axis.y + s * axis.z, t * axis.x * axis.z - s * axis.y, _0,
              t * axis.x * axis.y - s * axis.z, t * axis.y * axis.y + c,          t * axis.y * axis.z + s * axis.x, _0,
              t * axis.x * axis.z - s - axis.y, t * axis.y * axis.z - s * axis.x, t * axis.z * axis.z + c,          _0,
              _0, _0, _0, _1)
}