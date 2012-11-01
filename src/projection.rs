use float::consts::pi;
use float::tan;

use matrix::Mat4;
use num_util::*;

//
//  Create a perspective projection matrix
//
//  fov is in degrees
//  http://www.opengl.org/wiki/GluPerspective_code
//
#[inline(always)]
pure fn perspective<T:Copy NumCast Neg<T>>(fovy: float, aspectRatio: float, near: float, far: float) -> Mat4<T> {
    let ymax = near * tan(fovy * pi / 360f);
    let xmax = ymax * aspectRatio;
    return frustum(-xmax, xmax, -ymax, ymax, near, far);
}

//
//  Define a view frustrum
//  http://www.felixgers.de/teaching/jogl/perspectiveProjection.html
//  http://www.opengl.org/wiki/GluPerspective_code
//
//  TODO: double check algorithm
//
#[inline(always)]
pure fn frustum<T:Copy NumCast Neg<T>>(left: float, right: float, bottom: float, top: float, near: float, far: float) -> Mat4<T> {
    let m00 = cast((2f * near) / (right - left));
    let m01 = cast(0f);
    let m02 = cast(0f);
    let m03 = cast(0f);
    let m10 = cast(0f);
    let m11 = cast((2f * near) / (top - bottom));
    let m12 = cast(0f);
    let m13 = cast(0f);
    let m20 = cast((right + left) / (right - left));
    let m21 = cast((top + bottom) / (top - bottom));
    let m22 = cast(-(far + near) / (far - near));
    let m23 = cast(-1f);
    let m30 = cast(0f);
    let m31 = cast(0f);
    let m32 = cast(-(2f * far * near) / (far - near));
    let m33 = cast(0f);
    
    return Mat4(m00, m01, m02, m03,
                m10, m11, m12, m13,
                m20, m21, m22, m23,
                m30, m31, m32, m33);
}