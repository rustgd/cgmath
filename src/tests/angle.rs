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

#[test]
fn angle_conv() {
    assert_approx_eq!(deg(-5.0).to_rad().to_deg(), deg(-5.0));
    assert_approx_eq!(deg(30.0).to_rad().to_deg(), deg(30.0));

    assert_approx_eq!(rad(-5.0).to_deg().to_rad(), rad(-5.0));
    assert_approx_eq!(rad(30.0).to_deg().to_rad(), rad(30.0));
}
