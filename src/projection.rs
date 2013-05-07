use mat::{Mat4, BaseMat4};

use num::NumAssign;

/**
 * Create a perspective projection matrix
 *
 * Note: the fovy parameter should be specified in degrees.
 *
 * This is the equivalent of the gluPerspective function, the algorithm of which
 * can be found [here](http://www.opengl.org/wiki/GluPerspective_code).
 */
#[inline(always)]
pub fn perspective<T:Copy + Float + NumAssign>(fovy: T, aspectRatio: T, near: T, far: T) -> Mat4<T> {
    let _2: T = num::cast(2);

    let ymax = near * (fovy / _2).to_radians().tan();
    let xmax = ymax * aspectRatio;

    frustum(-xmax, xmax, -ymax, ymax, near, far)
}

/**
 * Define a view frustrum
 *
 * This is the equivalent of the now deprecated [glFrustrum]
 * (http://www.opengl.org/sdk/docs/man2/xhtml/glFrustum.xml) function.
 */
#[inline(always)]
pub fn frustum<T:Copy + Float + NumAssign>(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Mat4<T> {
    let _0: T = num::cast(0);
    let _1: T = num::cast(1);
    let _2: T = num::cast(2);

    let c0r0 = (_2 * near) / (right - left);
    let c0r1 = _0;
    let c0r2 = _0;
    let c0r3 = _0;

    let c1r0 = _0;
    let c1r1 = (_2 * near) / (top - bottom);
    let c1r2 = _0;
    let c1r3 = _0;

    let c2r0 = (right + left) / (right - left);
    let c2r1 = (top + bottom) / (top - bottom);
    let c2r2 = -(far + near) / (far - near);
    let c2r3 = -_1;

    let c3r0 = _0;
    let c3r1 = _0;
    let c3r2 = -(_2 * far * near) / (far - near);
    let c3r3 = _0;

    BaseMat4::new(c0r0, c0r1, c0r2, c0r3,
                  c1r0, c1r1, c1r2, c1r3,
                  c2r0, c2r1, c2r2, c2r3,
                  c3r0, c3r1, c3r2, c3r3)
}

/**
 * Create an orthographic projection matrix
 *
 * This is the equivalent of the now deprecated [glOrtho]
 * (http://www.opengl.org/sdk/docs/man2/xhtml/glOrtho.xml) function.
 */
#[inline(always)]
pub fn ortho<T:Copy + Float + NumAssign>(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Mat4<T> {
    let _0: T = num::cast(0);
    let _1: T = num::cast(1);
    let _2: T = num::cast(2);

    let c0r0 = _2 / (right - left);
    let c0r1 = _0;
    let c0r2 = _0;
    let c0r3 = _0;

    let c1r0 = _0;
    let c1r1 = _2 / (top - bottom);
    let c1r2 = _0;
    let c1r3 = _0;

    let c2r0 = _0;
    let c2r1 = _0;
    let c2r2 = -_2 / (far - near);
    let c2r3 = _0;

    let c3r0 = -(right + left) / (right - left);
    let c3r1 = -(top + bottom) / (top - bottom);
    let c3r2 = -(far + near) / (far - near);
    let c3r3 = _1;

    BaseMat4::new(c0r0, c0r1, c0r2, c0r3,
                  c1r0, c1r1, c1r2, c1r3,
                  c2r0, c2r1, c2r2, c2r3,
                  c3r0, c3r1, c3r2, c3r3)
}
