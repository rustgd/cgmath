// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use frustum::Frustum;
use mat::Mat4;
use plane::Plane;

mod num_macros;

///
/// Create a perspective projection matrix
///
/// Note: the fovy parameter should be specified in degrees.
///
/// This is the equivalent of the gluPerspective function, the algorithm of which
/// can be found [here](http://www.opengl.org/wiki/GluPerspective_code).
///
pub fn perspective<T:Copy + Real>(fovy: T, aspectRatio: T, near: T, far: T) -> Mat4<T> {
    let ymax = near * (fovy / two!(T)).to_radians().tan();
    let xmax = ymax * aspectRatio;

    frustum(-xmax, xmax, -ymax, ymax, near, far)
}

///
/// Define a view frustrum
///
/// This is the equivalent of the now deprecated [glFrustrum]
/// (http://www.opengl.org/sdk/docs/man2/xhtml/glFrustum.xml) function.
///
pub fn frustum<T:Copy + Real>(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Mat4<T> {
    let c0r0 = (two!(T) * near) / (right - left);
    let c0r1 = zero!(T);
    let c0r2 = zero!(T);
    let c0r3 = zero!(T);

    let c1r0 = zero!(T);
    let c1r1 = (two!(T) * near) / (top - bottom);
    let c1r2 = zero!(T);
    let c1r3 = zero!(T);

    let c2r0 = (right + left) / (right - left);
    let c2r1 = (top + bottom) / (top - bottom);
    let c2r2 = -(far + near) / (far - near);
    let c2r3 = -one!(T);

    let c3r0 = zero!(T);
    let c3r1 = zero!(T);
    let c3r2 = -(two!(T) * far * near) / (far - near);
    let c3r3 = zero!(T);

    Mat4::new(c0r0, c0r1, c0r2, c0r3,
              c1r0, c1r1, c1r2, c1r3,
              c2r0, c2r1, c2r2, c2r3,
              c3r0, c3r1, c3r2, c3r3)
}

///
/// Create an orthographic projection matrix
///
/// This is the equivalent of the now deprecated [glOrtho]
/// (http://www.opengl.org/sdk/docs/man2/xhtml/glOrtho.xml) function.
///
pub fn ortho<T:Copy + Real>(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Mat4<T> {
    let c0r0 = two!(T) / (right - left);
    let c0r1 = zero!(T);
    let c0r2 = zero!(T);
    let c0r3 = zero!(T);

    let c1r0 = zero!(T);
    let c1r1 = two!(T) / (top - bottom);
    let c1r2 = zero!(T);
    let c1r3 = zero!(T);

    let c2r0 = zero!(T);
    let c2r1 = zero!(T);
    let c2r2 = -two!(T) / (far - near);
    let c2r3 = zero!(T);

    let c3r0 = -(right + left) / (right - left);
    let c3r1 = -(top + bottom) / (top - bottom);
    let c3r2 = -(far + near) / (far - near);
    let c3r3 = one!(T);

    Mat4::new(c0r0, c0r1, c0r2, c0r3,
              c1r0, c1r1, c1r2, c1r3,
              c2r0, c2r1, c2r2, c2r3,
              c3r0, c3r1, c3r2, c3r3)
}

pub trait Projection<T> {
    pub fn if_valid<U:Copy>(&self, f: &fn() -> U) -> Result<U, ~str>;
    pub fn to_mat4(&self) -> Result<Mat4<T>, ~str>;
    pub fn to_frustum(&self) -> Result<Frustum<T>, ~str>;
}

/// A symmetrical perspective projection based on a field-of-view angle
#[deriving(Eq)]
pub struct PerspectiveFOV<T> {
    fovy:   T,  //radians
    aspect: T,
    near:   T,
    far:    T,
}

impl<T:Copy + Real> PerspectiveFOV<T> {
    pub fn to_perspective(&self) -> Result<Perspective<T>, ~str> {
        do self.if_valid {
            let angle = self.fovy / two!(T);
            let ymax = self.near * angle.tan();
            let xmax = ymax * self.aspect;

            Perspective {
                left:   -xmax,
                right:   xmax,
                bottom: -ymax,
                top:     ymax,
                near:    self.near,
                far:     self.far,
            }
        }
    }
}

impl<T:Copy + Real> Projection<T> for PerspectiveFOV<T> {
    pub fn if_valid<U:Copy>(&self, f: &fn() -> U) -> Result<U, ~str> {
        let frac_pi_2: T = Real::frac_pi_2();
        cond! (
            (self.fovy   < zero!(T))  { Err(fmt!("The vertical field of view cannot be below zero, found: %?", self.fovy)) }
            (self.fovy   > frac_pi_2) { Err(fmt!("The vertical field of view cannot be greater than a half turn, found: %?", self.fovy)) }
            (self.aspect < zero!(T))  { Err(fmt!("The aspect ratio cannot be below zero, found: %?", self.aspect)) }
            (self.near   < zero!(T))  { Err(fmt!("The near plane distance cannot be below zero, found: %?", self.near)) }
            (self.far    < zero!(T))  { Err(fmt!("The far plane distance cannot be below zero, found: %?", self.far)) }
            (self.far    < self.near) { Err(fmt!("The far plane cannot be closer than the near plane, found: far: %?, near: %?", self.far, self.near)) }
            _                         { Ok(f()) }
        )
    }

    pub fn to_mat4(&self) -> Result<Mat4<T>, ~str> {
        do self.to_perspective().chain |proj| { proj.to_mat4() }
    }

    pub fn to_frustum(&self) -> Result<Frustum<T>, ~str> {
        do self.to_perspective().chain |proj| { proj.to_frustum() }
    }
}

/// A perspective projection with arbitrary left/right/bottom/top distances
#[deriving(Eq)]
pub struct Perspective<T> {
    left:   T,
    right:  T,
    bottom: T,
    top:    T,
    near:   T,
    far:    T,
}

impl<T:Copy + Real> Projection<T> for Perspective<T> {
    pub fn if_valid<U:Copy>(&self, f: &fn() -> U) -> Result<U, ~str> {
        cond! (
            (self.left   > self.right) { Err(fmt!("`left` cannot be greater than `right`, found: left: %? right: %?", self.left, self.right)) }
            (self.bottom > self.top)   { Err(fmt!("`bottom` cannot be greater than `top`, found: bottom: %? top: %?", self.bottom, self.top)) }
            (self.near   > self.far)   { Err(fmt!("`near` cannot be greater than `far`, found: near: %? far: %?", self.near, self.far)) }
            _                          { Ok(f()) }
        )
    }

    pub fn to_mat4(&self) -> Result<Mat4<T>, ~str> {
        do self.if_valid {
            let c0r0 = (two!(T) * self.near) / (self.right - self.left);
            let c0r1 = zero!(T);
            let c0r2 = zero!(T);
            let c0r3 = zero!(T);

            let c1r0 = zero!(T);
            let c1r1 = (two!(T) * self.near) / (self.top - self.bottom);
            let c1r2 = zero!(T);
            let c1r3 = zero!(T);

            let c2r0 = (self.right + self.left) / (self.right - self.left);
            let c2r1 = (self.top + self.bottom) / (self.top - self.bottom);
            let c2r2 = -(self.far + self.near) / (self.far - self.near);
            let c2r3 = -one!(T);

            let c3r0 = zero!(T);
            let c3r1 = zero!(T);
            let c3r2 = -(two!(T) * self.far * self.near) / (self.far - self.near);
            let c3r3 = zero!(T);

            Mat4::new(c0r0, c0r1, c0r2, c0r3,
                      c1r0, c1r1, c1r2, c1r3,
                      c2r0, c2r1, c2r2, c2r3,
                      c3r0, c3r1, c3r2, c3r3)
        }
    }

    pub fn to_frustum(&self) -> Result<Frustum<T>, ~str> {
        do self.if_valid {
            /*

            <---- l --->|<--- r ----->
            +-----------+-----------+ ^
             \          |          /  |
              \         |         /   |
               \    +   |   +    /    |
                \  / Nl | Nr \  /     |
                 \/     |     \/      |
            left  \     |     / right f
            plane  \    |    /  plane |
            ⟨Nl,Dl⟩ \   |   / ⟨Nr,Dr⟩ |
                     \θl|θr/          |
                      \ | /           |
                       \|/            |
                        +             v

            θl = tan⁻¹(l/f)

                         +
                        /  Nl
                       /
                      /
             \   |   /
              \θl|  /
               \ | /
                \|/ θl
                 +- - - -
                  \

            Nl = ⟨cos(θl), 0, sin(θl)⟩

            left plane = ⟨Nl, 0⟩
                       = ⟨cos(θl), 0, sin(θl), 0⟩

            */

            let theta_l = (self.left / self.far).atan();
            let theta_r = (self.right / self.far).atan();
            let theta_b = (self.bottom / self.far).atan();
            let theta_t = (self.top / self.far).atan();

            Frustum {
                left:   Plane::from_abcd(theta_l.cos(), zero!(T), theta_l.sin(), zero!(T)),
                right:  Plane::from_abcd(theta_r.cos(), zero!(T), theta_r.sin(), zero!(T)),
                bottom: Plane::from_abcd(zero!(T), theta_b.cos(), theta_b.sin(), zero!(T)),
                top:    Plane::from_abcd(zero!(T), theta_t.cos(), theta_t.sin(), zero!(T)),
                near:   Plane::from_abcd(zero!(T), zero!(T), -one!(T), -self.near),
                far:    Plane::from_abcd(zero!(T), zero!(T), one!(T), self.far),
            }
        }
    }
}

/// An orthographic projection with arbitrary left/right/bottom/top distances
#[deriving(Eq)]
pub struct Ortho<T> {
    left:   T,
    right:  T,
    bottom: T,
    top:    T,
    near:   T,
    far:    T,
}

impl<T:Copy + Real> Projection<T> for Ortho<T> {
    pub fn if_valid<U:Copy>(&self, f: &fn() -> U) -> Result<U, ~str> {
        cond! (
            (self.left   > self.right) { Err(fmt!("`left` cannot be greater than `right`, found: left: %? right: %?", self.left, self.right)) }
            (self.bottom > self.top)   { Err(fmt!("`bottom` cannot be greater than `top`, found: bottom: %? top: %?", self.bottom, self.top)) }
            (self.near   > self.far)   { Err(fmt!("`near` cannot be greater than `far`, found: near: %? far: %?", self.near, self.far)) }
            _                          { Ok(f()) }
        )
    }

    pub fn to_mat4(&self) -> Result<Mat4<T>, ~str> {
        do self.if_valid {
            let c0r0 = two!(T) / (self.right - self.left);
            let c0r1 = zero!(T);
            let c0r2 = zero!(T);
            let c0r3 = zero!(T);

            let c1r0 = zero!(T);
            let c1r1 = two!(T) / (self.top - self.bottom);
            let c1r2 = zero!(T);
            let c1r3 = zero!(T);

            let c2r0 = zero!(T);
            let c2r1 = zero!(T);
            let c2r2 = -two!(T) / (self.far - self.near);
            let c2r3 = -one!(T);

            let c3r0 = -(self.right + self.left) / (self.right - self.left);
            let c3r1 = -(self.top + self.bottom) / (self.top - self.bottom);
            let c3r2 = -(self.far + self.near) / (self.far - self.near);
            let c3r3 = one!(T);

            Mat4::new(c0r0, c0r1, c0r2, c0r3,
                      c1r0, c1r1, c1r2, c1r3,
                      c2r0, c2r1, c2r2, c2r3,
                      c3r0, c3r1, c3r2, c3r3)
        }
    }

    pub fn to_frustum(&self) -> Result<Frustum<T>, ~str> {
        do self.if_valid {
            Frustum {
                left:   Plane::from_abcd(one!(T), zero!(T), zero!(T), self.left),
                right:  Plane::from_abcd(-one!(T), zero!(T), zero!(T), self.right),
                bottom: Plane::from_abcd(zero!(T), one!(T), zero!(T), self.bottom),
                top:    Plane::from_abcd(zero!(T), -one!(T), zero!(T), self.top),
                near:   Plane::from_abcd(zero!(T), zero!(T), -one!(T), self.near),
                far:    Plane::from_abcd(zero!(T), zero!(T), one!(T), self.far),
            }
        }
    }
}
