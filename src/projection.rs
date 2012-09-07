import float::consts::pi;
import float::tan;
import mat::mat4;

//
//  Create a perspective projection matrix
//
//  fov is in degrees
//  http://www.opengl.org/wiki/GluPerspective_code
//
pure fn perspective(fovy:float, aspectRatio:float, near:float, far:float) -> mat4 {
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
pure fn frustum(left:float, right:float, bottom:float, top:float, near:float, far:float) -> mat4 {
    let m00:float = (2f * near) / (right - left);
    let m01:float = 0f;
    let m02:float = 0f;
    let m03:float = 0f;
    let m10:float = 0f;
    let m11:float = (2f * near)/(top - bottom);
    let m12:float = 0f;
    let m13:float = 0f;
    let m20:float = (right + left) / (right - left);
    let m21:float = (top + bottom) / (top - bottom);
    let m22:float = -(far + near) / (far - near);
    let m23:float = -1f;
    let m30:float = 0f;
    let m31:float = 0f;
    let m32:float = -(2f * far * near) / (far - near);
    let m33:float = 0f;
    
    return mat4(m00, m01, m02, m03,
                m10, m11, m12, m13,
                m20, m21, m22, m23,
                m30, m31, m32, m33);
}