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


extern crate cgmath;

use cgmath::*;

#[test]
fn test_line_intersection() {
    // collinear, intersection is line dest
    let r1 = Ray::new(Point2::new(0.0f32, 0.0), Vector2::new(0.25, 0.0));
    let l1 = Line::new(Point2::new(1.5f32, 0.0), Point2::new(0.5, 0.0));
    assert_eq!((r1, l1).intersection(), Some(Point2::new(0.5, 0.0)));

    // collinear, intersection is at ray origin
    let r2 = Ray::new(Point2::new(0.0f32, 0.0), Vector2::new(5.0, 0.0));
    let l2 = Line::new(Point2::new(-11.0f32, 0.0), Point2::new(1.0, 0.0));
    assert_eq!((r2, l2).intersection(), Some(Point2::new(0.0, 0.0)));

    // collinear, intersection is line origin
    let r3 = Ray::new(Point2::new(0.0f32, 1.0), Vector2::new(0.0, -0.25));
    let l3 = Line::new(Point2::new(0.0f32, 0.5), Point2::new(0.0, -0.5));
    assert_eq!((r3, l3).intersection(), Some(Point2::new(0.0, 0.5)));

    // collinear, no overlap
    let r4 = Ray::new(Point2::new(0.0f32, 0.0), Vector2::new(3.0, 0.0));
    let l4 = Line::new(Point2::new(-10.0f32, 0.0), Point2::new(-5.0, 0.0));
    assert_eq!((r4, l4).intersection(), None);

    // no intersection
    let r5 = Ray::new(Point2::new(5.0f32, 5.0), Vector2::new(40.0, 8.0));
    let l5 = Line::new(Point2::new(5.0f32, 4.8), Point2::new(10.0, 4.1));
    assert_eq!((r5, l5).intersection(), None); // no intersection

    // non-collinear intersection
    let r6 = Ray::new(Point2::new(0.0f32, 0.0), Vector2::new(10.0, 10.0));
    let l6 = Line::new(Point2::new(0.0f32, 10.0), Point2::new(10.0, 0.0));
    assert_eq!((r6, l6).intersection(), Some(Point2::new(5.0, 5.0)));

    // line is a point that does not intersect
    let r7 = Ray::new(Point2::new(0.0f32, 0.0), Vector2::new(1.0, 1.0));
    let l7 = Line::new(Point2::new(1.0f32, 0.0), Point2::new(1.0, 0.0));
    assert_eq!((r7, l7).intersection(), None);

    // line is a point that does intersect
    let r8 = Ray::new(Point2::new(0.0f32, 0.0), Vector2::new(1.0, 0.0));
    let l8 = Line::new(Point2::new(3.0f32, 0.0), Point2::new(3.0, 0.0));
    assert_eq!((r8, l8).intersection(), Some(Point2::new(3.0, 0.0)));

    // line is a collinear point but no intersection
    let r9 = Ray::new(Point2::new(0.0f32, 0.0), Vector2::new(1.0, 0.0));
    let l9 = Line::new(Point2::new(-1.0f32, 0.0), Point2::new(-1.0, 0.0));
    assert_eq!((r9, l9).intersection(), None);
}
