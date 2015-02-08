// Copyright 2014 The CGMath Developers. For a full listing of the authors,
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

#[macro_use]
extern crate cgmath;

use cgmath::*;

#[test]
fn macro_assert_approx_eq_eps() {
    assert_approx_eq_eps!(1.0f64, 1.001, 0.01);
}

#[test]
#[should_fail]
fn macro_assert_approx_eq_eps_fail() {
    assert_approx_eq_eps!(1.0f32, 1.02, 0.01);
}

#[test]
fn macro_assert_approx_eq() {
    assert_approx_eq!(7.0f32 / 5.0, 1.4);
}

#[test]
#[should_fail]
fn macro_assert_approx_eq_fail() {
    assert_approx_eq!(1.0f64 / 3.0, 0.333);
}
