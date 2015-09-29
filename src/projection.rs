// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
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

use rust_num::{Zero, One};
use rust_num::traits::cast;

use angle::{Angle, Rad, tan, cot};
use frustum::Frustum;
use matrix::Matrix4;
use num::BaseFloat;
use plane::Plane;

/// Create a perspective projection matrix.
///
/// This is the equivalent to the [gluPerspective]
/// (http://www.opengl.org/sdk/docs/man2/xhtml/gluPerspective.xml) function.
pub fn perspective<S: BaseFloat + 'static, A: Angle<S>>(fovy: A, aspect: S, near: S, far: S) -> Matrix4<S> {
    PerspectiveFov {
        fovy:   fovy,
        aspect: aspect,
        near:   near,
        far:    far,
    }.into()
}

/// Create a perspective matrix from a view frustrum.
///
/// This is the equivalent of the now deprecated [glFrustrum]
/// (http://www.opengl.org/sdk/docs/man2/xhtml/glFrustum.xml) function.
pub fn frustum<S: BaseFloat + 'static>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Matrix4<S> {
    Perspective {
        left:   left,
        right:  right,
        bottom: bottom,
        top:    top,
        near:   near,
        far:    far,
    }.into()
}

/// Create an orthographic projection matrix.
///
/// This is the equivalent of the now deprecated [glOrtho]
/// (http://www.opengl.org/sdk/docs/man2/xhtml/glOrtho.xml) function.
pub fn ortho<S: BaseFloat + 'static>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Matrix4<S> {
    Ortho {
        left:   left,
        right:  right,
        bottom: bottom,
        top:    top,
        near:   near,
        far:    far,
    }.into()
}

pub trait Projection<S: BaseFloat>: Into<Matrix4<S>> {
    fn to_frustum(&self) -> Frustum<S>;
}

/// A perspective projection based on a vertical field-of-view angle.
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct PerspectiveFov<S, A> {
    pub fovy:   A,
    pub aspect: S,
    pub near:   S,
    pub far:    S,
}

impl<S: BaseFloat, A: Angle<S>> PerspectiveFov<S, A> {
    pub fn to_perspective(&self) -> Perspective<S> {
        let angle = self.fovy.div_s(cast(2i8).unwrap());
        let angle: Rad<_> = angle.into();
        let ymax = self.near * tan(angle);
        let xmax = ymax * self.aspect;

        Perspective {
            left:   -xmax,
            right:   xmax,
            bottom: -ymax,
            top:     ymax,
            near:    self.near.clone(),
            far:     self.far.clone(),
        }
    }
}

impl<S: BaseFloat + 'static, A: Angle<S>> Projection<S> for PerspectiveFov<S, A> {
    fn to_frustum(&self) -> Frustum<S> {
        // TODO: Could this be faster?
        Frustum::from_matrix4(self.clone().into()).unwrap()
    }
}

impl<S: BaseFloat, A: Angle<S>> From<PerspectiveFov<S, A>> for Matrix4<S> {
    fn from(persp: PerspectiveFov<S, A>) -> Matrix4<S> {
        let half_turn: A = Angle::turn_div_2();

        assert!(persp.fovy   > A::zero(), "The vertical field of view cannot be below zero, found: {:?}", persp.fovy);
        assert!(persp.fovy   < half_turn, "The vertical field of view cannot be greater than a half turn, found: {:?}", persp.fovy);
        assert!(persp.aspect > S::zero(), "The aspect ratio cannot be below zero, found: {:?}", persp.aspect);
        assert!(persp.near   > S::zero(), "The near plane distance cannot be below zero, found: {:?}", persp.near);
        assert!(persp.far    > S::zero(), "The far plane distance cannot be below zero, found: {:?}", persp.far);
        assert!(persp.far    > persp.near, "The far plane cannot be closer than the near plane, found: far: {:?}, near: {:?}", persp.far, persp.near);

        let f: Rad<_> = persp.fovy.div_s(cast(2i8).unwrap()).into();
        let f = cot(f);
        let two: S = cast(2i8).unwrap();

        let c0r0 = f / persp.aspect;
        let c0r1 = S::zero();
        let c0r2 = S::zero();
        let c0r3 = S::zero();

        let c1r0 = S::zero();
        let c1r1 = f;
        let c1r2 = S::zero();
        let c1r3 = S::zero();

        let c2r0 = S::zero();
        let c2r1 = S::zero();
        let c2r2 = (persp.far + persp.near) / (persp.near - persp.far);
        let c2r3 = -S::one();

        let c3r0 = S::zero();
        let c3r1 = S::zero();
        let c3r2 = (two * persp.far * persp.near) / (persp.near - persp.far);
        let c3r3 = S::zero();

        Matrix4::new(c0r0, c0r1, c0r2, c0r3,
                     c1r0, c1r1, c1r2, c1r3,
                     c2r0, c2r1, c2r2, c2r3,
                     c3r0, c3r1, c3r2, c3r3)
    }
}

/// A perspective projection with arbitrary left/right/bottom/top distances
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Perspective<S> {
    pub left:   S,
    pub right:  S,
    pub bottom: S,
    pub top:    S,
    pub near:   S,
    pub far:    S,
}

impl<S: BaseFloat + 'static> Projection<S> for Perspective<S> {
    fn to_frustum(&self) -> Frustum<S> {
        // TODO: Could this be faster?
        Frustum::from_matrix4(self.clone().into()).unwrap()
    }
}

impl<S: BaseFloat + 'static> From<Perspective<S>> for Matrix4<S> {
    fn from(persp: Perspective<S>) -> Matrix4<S> {
        assert!(persp.left   <= persp.right, "`left` cannot be greater than `right`, found: left: {:?} right: {:?}", persp.left, persp.right);
        assert!(persp.bottom <= persp.top,   "`bottom` cannot be greater than `top`, found: bottom: {:?} top: {:?}", persp.bottom, persp.top);
        assert!(persp.near   <= persp.far,   "`near` cannot be greater than `far`, found: near: {:?} far: {:?}", persp.near, persp.far);

        let two: S = cast(2i8).unwrap();

        let c0r0 = (two * persp.near) / (persp.right - persp.left);
        let c0r1 = S::zero();
        let c0r2 = S::zero();
        let c0r3 = S::zero();

        let c1r0 = S::zero();
        let c1r1 = (two * persp.near) / (persp.top - persp.bottom);
        let c1r2 = S::zero();
        let c1r3 = S::zero();

        let c2r0 = (persp.right + persp.left) / (persp.right - persp.left);
        let c2r1 = (persp.top + persp.bottom) / (persp.top - persp.bottom);
        let c2r2 = -(persp.far + persp.near) / (persp.far - persp.near);
        let c2r3 = -S::one();

        let c3r0 = S::zero();
        let c3r1 = S::zero();
        let c3r2 = -(two * persp.far * persp.near) / (persp.far - persp.near);
        let c3r3 = S::zero();

        Matrix4::new(c0r0, c0r1, c0r2, c0r3,
                     c1r0, c1r1, c1r2, c1r3,
                     c2r0, c2r1, c2r2, c2r3,
                     c3r0, c3r1, c3r2, c3r3)
    }
}

/// An orthographic projection with arbitrary left/right/bottom/top distances
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Ortho<S> {
    pub left:   S,
    pub right:  S,
    pub bottom: S,
    pub top:    S,
    pub near:   S,
    pub far:    S,
}

impl<S: BaseFloat> Projection<S> for Ortho<S> {
    fn to_frustum(&self) -> Frustum<S> {
        Frustum {
            left:   Plane::from_abcd( S::one(), S::zero(), S::zero(), self.left.clone()),
            right:  Plane::from_abcd(-S::one(), S::zero(), S::zero(), self.right.clone()),
            bottom: Plane::from_abcd(S::zero(), S::one(), S::zero(), self.bottom.clone()),
            top:    Plane::from_abcd(S::zero(), -S::one(), S::zero(), self.top.clone()),
            near:   Plane::from_abcd(S::zero(), S::zero(), -S::one(), self.near.clone()),
            far:    Plane::from_abcd(S::zero(), S::zero(), S::one(), self.far.clone()),
        }
    }
}

impl<S: BaseFloat> From<Ortho<S>> for Matrix4<S> {
    fn from(ortho: Ortho<S>) -> Matrix4<S> {
        let two: S = cast(2i8).unwrap();

        let c0r0 = two / (ortho.right - ortho.left);
        let c0r1 = S::zero();
        let c0r2 = S::zero();
        let c0r3 = S::zero();

        let c1r0 = S::zero();
        let c1r1 = two / (ortho.top - ortho.bottom);
        let c1r2 = S::zero();
        let c1r3 = S::zero();

        let c2r0 = S::zero();
        let c2r1 = S::zero();
        let c2r2 = -two / (ortho.far - ortho.near);
        let c2r3 = S::zero();

        let c3r0 = -(ortho.right + ortho.left) / (ortho.right - ortho.left);
        let c3r1 = -(ortho.top + ortho.bottom) / (ortho.top - ortho.bottom);
        let c3r2 = -(ortho.far + ortho.near) / (ortho.far - ortho.near);
        let c3r3 = S::one();

        Matrix4::new(c0r0, c0r1, c0r2, c0r3,
                     c1r0, c1r1, c1r2, c1r3,
                     c2r0, c2r1, c2r2, c2r3,
                     c3r0, c3r1, c3r2, c3r3)
    }
}
