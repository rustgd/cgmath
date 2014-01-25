// Copyright 2013 The CGMath Developers. For a full listing of the authors,
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

use cgmath::angle::*;
use cgmath::approx::ApproxEq;

#[test]
fn conv() {
    assert!(deg(-5.0).to_rad().to_deg().approx_eq( &deg(-5.0) ));
    assert!(deg(30.0).to_rad().to_deg().approx_eq( &deg(30.0) ));

    assert!(rad(-5.0).to_deg().to_rad().approx_eq( &rad(-5.0) ));
    assert!(rad(30.0).to_deg().to_rad().approx_eq( &rad(30.0) ));
}

#[test]
fn equiv() {
    assert!(Deg::<f32>::full_turn().equiv(&-Deg::<f32>::full_turn()))
    assert!(Deg::<f32>::turn_div_2().equiv(&-Deg::<f32>::turn_div_2()))
    assert!(Deg::<f32>::turn_div_3().sub_a(Deg::<f32>::full_turn()).equiv(&Deg::<f32>::turn_div_3()))

    assert!(Rad::<f32>::full_turn().equiv(&-Rad::<f32>::full_turn()))
    assert!(Rad::<f32>::turn_div_2().equiv(&-Rad::<f32>::turn_div_2()))
    assert!(Rad::<f32>::turn_div_3().sub_a(Rad::<f32>::full_turn()).equiv(&Rad::<f32>::turn_div_3()))
}
