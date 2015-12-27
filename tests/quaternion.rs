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

mod to_from_euler {
    use std::f32;

    use cgmath::*;

    fn check_euler(pitch: Rad<f32>, yaw: Rad<f32>, roll: Rad<f32>) {
        let quat = Quaternion::from_euler(pitch, yaw, roll);
        let (found_pitch, found_yaw, found_roll) = quat.to_euler();

        assert_approx_eq_eps!(pitch, found_pitch, 0.001);
        assert_approx_eq_eps!(yaw, found_yaw, 0.001);
        assert_approx_eq_eps!(roll, found_roll, 0.001);
    }

    const HPI: f32 = f32::consts::FRAC_PI_2;

    #[test] fn test_zero()                  { check_euler(rad(0.0), rad(0.0), rad(0.0)); }
    #[test] fn test_yaw_pos_1()             { check_euler(rad(0.0), rad(1.0), rad(0.0)); }
    #[test] fn test_yaw_neg_1()             { check_euler(rad(0.0), rad(-1.0), rad(0.0)); }
    #[test] fn test_pitch_pos_1()           { check_euler(rad(1.0), rad(0.0), rad(0.0)); }
    #[test] fn test_pitch_neg_1()           { check_euler(rad(-1.0), rad(0.0), rad(0.0)); }
    #[test] fn test_roll_pos_1()            { check_euler(rad(0.0), rad(0.0), rad(1.0)); }
    #[test] fn test_roll_neg_1()            { check_euler(rad(0.0), rad(0.0), rad(-1.0)); }
    #[test] fn test_pitch_yaw_roll_pos_1()  { check_euler(rad(1.0), rad(1.0), rad(1.0)); }
    #[test] fn test_pitch_yaw_roll_neg_1()  { check_euler(rad(-1.0), rad(-1.0), rad(-1.0)); }
    #[test] fn test_pitch_yaw_roll_pos_hp() { check_euler(rad(0.0), rad(HPI), rad(1.0)); }
    #[test] fn test_pitch_yaw_roll_neg_hp() { check_euler(rad(0.0), rad(-HPI), rad(1.0)); }
}

mod from_axis_angle {
    mod axis_x {
        use cgmath::*;

        fn check_from_axis_angle(pitch: Rad<f32>) {
            let found = Quaternion::from_axis_angle(Vector3::unit_x(), pitch);
            let expected = Quaternion::from_euler(pitch, rad(0.0), rad(0.0));

            assert_approx_eq_eps!(found, expected, 0.001);
        }

        #[test] fn test_zero()      { check_from_axis_angle(rad(0.0)); }
        #[test] fn test_pos_1()     { check_from_axis_angle(rad(1.0)); }
        #[test] fn test_neg_1()     { check_from_axis_angle(rad(-1.0)); }
    }

    mod axis_y {
        use cgmath::*;

        fn check_from_axis_angle(yaw: Rad<f32>) {
            let found = Quaternion::from_axis_angle(Vector3::unit_y(), yaw);
            let expected = Quaternion::from_euler(rad(0.0), yaw, rad(0.0));

            assert_approx_eq_eps!(found, expected, 0.001);
        }

        #[test] fn test_zero()      { check_from_axis_angle(rad(0.0)); }
        #[test] fn test_pos_1()     { check_from_axis_angle(rad(1.0)); }
        #[test] fn test_neg_1()     { check_from_axis_angle(rad(-1.0)); }
    }

    mod axis_z {
        use cgmath::*;

        fn check_from_axis_angle(roll: Rad<f32>) {
            let found = Quaternion::from_axis_angle(Vector3::unit_z(), roll);
            let expected = Quaternion::from_euler(rad(0.0), rad(0.0), roll);

            assert_approx_eq_eps!(found, expected, 0.001);
        }

        #[test] fn test_zero()      { check_from_axis_angle(rad(0.0)); }
        #[test] fn test_pos_1()     { check_from_axis_angle(rad(1.0)); }
        #[test] fn test_neg_1()     { check_from_axis_angle(rad(-1.0)); }
    }
}

mod from_angle_x {
    use cgmath::*;

    fn check_from_angle_x(pitch: Rad<f32>) {
        let found = Quaternion::from_angle_x(pitch);
        let expected = Quaternion::from_euler(pitch, rad(0.0), rad(0.0));

        assert_approx_eq_eps!(found, expected, 0.001);
    }

    #[test] fn test_zero()      { check_from_angle_x(rad(0.0)); }
    #[test] fn test_pos_1()     { check_from_angle_x(rad(1.0)); }
    #[test] fn test_neg_1()     { check_from_angle_x(rad(-1.0)); }
}

mod from_angle_y {
    use cgmath::*;

    fn check_from_angle_y(yaw: Rad<f32>) {
        let found = Quaternion::from_angle_y(yaw);
        let expected = Quaternion::from_euler(rad(0.0), yaw, rad(0.0));

        assert_approx_eq_eps!(found, expected, 0.001);
    }

    #[test] fn test_zero()      { check_from_angle_y(rad(0.0)); }
    #[test] fn test_pos_1()     { check_from_angle_y(rad(1.0)); }
    #[test] fn test_neg_1()     { check_from_angle_y(rad(-1.0)); }
}

mod from_angle_z {
    use cgmath::*;

    fn check_from_angle_z(roll: Rad<f32>) {
        let found = Quaternion::from_angle_z(roll);
        let expected = Quaternion::from_euler(rad(0.0), rad(0.0), roll);

        assert_approx_eq_eps!(found, expected, 0.001);
    }

    #[test] fn test_zero()      { check_from_angle_z(rad(0.0)); }
    #[test] fn test_pos_1()     { check_from_angle_z(rad(1.0)); }
    #[test] fn test_neg_1()     { check_from_angle_z(rad(-1.0)); }
}

mod from {
    mod matrix3 {
        use cgmath::*;

        fn check_with_euler(x: Rad<f32>, y: Rad<f32>, z: Rad<f32>) {
            let matrix3 = Matrix3::from_euler(x, y, z);
            let quaternion = Quaternion::from(matrix3);
            let quaternion_matrix3 = Matrix3::from(quaternion);
            assert_approx_eq!(matrix3, quaternion_matrix3);
        }

        // triggers: trace >= S::zero()
        #[test]
        fn test_positive_trace() {
            check_with_euler(rad(0.0), rad(0.0), rad(0.0));
        }

        // triggers: (mat[0][0] > mat[1][1]) && (mat[0][0] > mat[2][2])
        #[test]
        fn test_xx_maximum() {
            check_with_euler(rad(2.0), rad(1.0), rad(-1.2));
        }

        // triggers: mat[1][1] > mat[2][2]
        #[test]
        fn test_yy_maximum() {
            check_with_euler(rad(2.0), rad(1.0), rad(3.0));
        }

        // base case
        #[test]
        fn test_zz_maximum() {
            check_with_euler(rad(1.0), rad(1.0), rad(3.0));
        }
    }
}
