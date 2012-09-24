use float::consts::pi;
use float::tan;
use mat::Mat4;

//
//  Create a perspective projection matrix
//
//  fov is in degrees
//  http://www.opengl.org/wiki/GluPerspective_code
//
pure fn perspective(fovy:float, aspectRatio:float, near:float, far:float) -> Mat4 {
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
pure fn frustum(left:float, right:float, bottom:float, top:float, near:float, far:float) -> Mat4 {
    let m00 = (2f * near) / (right - left);
    let m01 = 0f;
    let m02 = 0f;
    let m03 = 0f;
    let m10 = 0f;
    let m11 = (2f * near)/(top - bottom);
    let m12 = 0f;
    let m13 = 0f;
    let m20 = (right + left) / (right - left);
    let m21 = (top + bottom) / (top - bottom);
    let m22 = -(far + near) / (far - near);
    let m23 = -1f;
    let m30 = 0f;
    let m31 = 0f;
    let m32 = -(2f * far * near) / (far - near);
    let m33 = 0f;
    
    return Mat4(m00, m01, m02, m03,
                m10, m11, m12, m13,
                m20, m21, m22, m23,
                m30, m31, m32, m33);
}