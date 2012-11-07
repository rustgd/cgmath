use num::Num;
use ncast::*;
use funs::exp::*;
use funs::trig::*;
use matrix::Mat4;

fn mat4_from_rotation<T:Copy Num NumCast Exp Trig>(axis: Vec3<T>, angle: T) -> Mat4<T> {
    let angle_rad = radians(&angle);
    let c = cos(&angle_rad);
    let s = sin(&angle_rad);
    
    let _0: T = cast(0);
    let _1: T = cast(1);
    let t = _1 - c;
    
    Mat4::new(t * axis.x * axis.x + c,          t * axis.x * axis.y + s * axis.z, t * axis.x * axis.z - s * axis.y, _0,
              t * axis.x * axis.y - s * axis.z, t * axis.y * axis.y + c,          t * axis.y * axis.z + s * axis.x, _0,
              t * axis.x * axis.z - s - axis.y, t * axis.y * axis.z - s * axis.x, t * axis.z * axis.z + c,          _0,
              _0, _0, _0, _1)
}