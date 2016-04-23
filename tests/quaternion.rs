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

#[macro_use]
extern crate cgmath;

macro_rules! impl_test_mul {
    ($s:expr, $v:expr) => (
        // point * scalar ops
        assert_eq!($v * $s, Quaternion::from_sv($v.s * $s, $v.v * $s));
        assert_eq!($s * $v, Quaternion::from_sv($s * $v.s, $s * $v.v));
        assert_eq!(&$v * $s, $v * $s);
        assert_eq!($s * &$v, $s * $v);
        // commutativity
        assert_eq!($v * $s, $s * $v);
    )
}

macro_rules! impl_test_div {
    ($s:expr, $v:expr) => (
        // point / scalar ops
        assert_eq!($v / $s, Quaternion::from_sv($v.s / $s, $v.v / $s));
        assert_eq!($s / $v, Quaternion::from_sv($s / $v.s, $s / $v.v));
        assert_eq!(&$v / $s, $v / $s);
        assert_eq!($s / &$v, $s / $v);
    )
}

mod operators {
    use cgmath::*;

    #[test]
    fn test_mul() {
        impl_test_mul!(2.0f32, Quaternion::from(Euler { x: rad(1f32), y: rad(1f32), z: rad(1f32) }));
    }

    #[test]
    fn test_div() {
        impl_test_div!(2.0f32, Quaternion::from(Euler { x: rad(1f32), y: rad(1f32), z: rad(1f32) }));
    }
}

mod to_from_euler {
    use std::f32;

    use cgmath::*;

    fn check_euler(rotation: Euler<Rad<f32>>) {
        assert_approx_eq_eps!(Euler::from(Quaternion::from(rotation)), rotation, 0.001);
    }

    const HPI: f32 = f32::consts::FRAC_PI_2;

    #[test] fn test_zero()                  { check_euler(Euler { x: rad( 0f32), y: rad( 0f32), z: rad( 0f32) }); }
    #[test] fn test_yaw_pos_1()             { check_euler(Euler { x: rad( 0f32), y: rad( 1f32), z: rad( 0f32) }); }
    #[test] fn test_yaw_neg_1()             { check_euler(Euler { x: rad( 0f32), y: rad(-1f32), z: rad( 0f32) }); }
    #[test] fn test_pitch_pos_1()           { check_euler(Euler { x: rad( 1f32), y: rad( 0f32), z: rad( 0f32) }); }
    #[test] fn test_pitch_neg_1()           { check_euler(Euler { x: rad(-1f32), y: rad( 0f32), z: rad( 0f32) }); }
    #[test] fn test_roll_pos_1()            { check_euler(Euler { x: rad( 0f32), y: rad( 0f32), z: rad( 1f32) }); }
    #[test] fn test_roll_neg_1()            { check_euler(Euler { x: rad( 0f32), y: rad( 0f32), z: rad(-1f32) }); }
    #[test] fn test_pitch_yaw_roll_pos_1()  { check_euler(Euler { x: rad( 1f32), y: rad( 1f32), z: rad( 1f32) }); }
    #[test] fn test_pitch_yaw_roll_neg_1()  { check_euler(Euler { x: rad(-1f32), y: rad(-1f32), z: rad(-1f32) }); }
    #[test] fn test_pitch_yaw_roll_pos_hp() { check_euler(Euler { x: rad( 0f32), y: rad(  HPI), z: rad( 1f32) }); }
    #[test] fn test_pitch_yaw_roll_neg_hp() { check_euler(Euler { x: rad( 0f32), y: rad( -HPI), z: rad( 1f32) }); }
}

mod from {
    mod matrix3 {
        use cgmath::*;

        fn check_with_euler(x: Rad<f32>, y: Rad<f32>, z: Rad<f32>) {
            let matrix3 = Matrix3::from(Euler { x: x, y: y, z: z });
            let quaternion = Quaternion::from(matrix3);
            let quaternion_matrix3 = Matrix3::from(quaternion);
            assert_approx_eq!(matrix3, quaternion_matrix3);
        }

        // triggers: trace >= S::zero()
        #[test]
        fn test_positive_trace() {
            check_with_euler(rad(0.0f32), rad(0.0), rad(0.0f32));
        }

        // triggers: (mat[0][0] > mat[1][1]) && (mat[0][0] > mat[2][2])
        #[test]
        fn test_xx_maximum() {
            check_with_euler(rad(2.0f32), rad(1.0), rad(-1.2f32));
        }

        // triggers: mat[1][1] > mat[2][2]
        #[test]
        fn test_yy_maximum() {
            check_with_euler(rad(2.0f32), rad(1.0), rad(3.0f32));
        }

        // base case
        #[test]
        fn test_zz_maximum() {
            check_with_euler(rad(1.0f32), rad(1.0), rad(3.0f32));
        }
    }
}
