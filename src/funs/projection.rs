use float::consts::pi;
use float::tan;

use matrix::Mat4;
use ncast::*;

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
    let _0 = cast(0);
    
    let c0r0 = cast((2f * near) / (right - left));
    let c0r1 = _0;
    let c0r2 = _0;
    let c0r3 = _0;
    let c1r0 = _0;
    let c1r1 = cast((2f * near) / (top - bottom));
    let c1r2 = _0;
    let c1r3 = _0;
    let c2r0 = cast((right + left) / (right - left));
    let c2r1 = cast((top + bottom) / (top - bottom));
    let c2r2 = cast(-(far + near) / (far - near));
    let c2r3 = cast(-1);
    let c3r0 = _0;
    let c3r1 = _0;
    let c3r2 = cast(-(2f * far * near) / (far - near));
    let c3r3 = _0;
    
    return Mat4::new(c0r0, c0r1, c0r2, c0r3,
                     c1r0, c1r1, c1r2, c1r3,
                     c2r0, c2r1, c2r2, c2r3,
                     c3r0, c3r1, c3r2, c3r3);
}