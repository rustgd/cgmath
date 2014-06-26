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

use cgmath::line::*;
use cgmath::point::*;
use cgmath::intersect::Intersect;

#[test]
fn test_line_intersection() {
    // collinear, origins pointing towards each other, first intersection
    // from l1.origin is in an endpoint in l2
    let l1 = Line::new(Point2::new(0.0f64, 0.0f64), Point2::new(10.0f64, 0.0f64));
    let l2 = Line::new(Point2::new(1.5f64, 0.0f64), Point2::new(0.5f64, 0.0f64));
    assert_eq!((l1, l2).intersection(), Some(Point2::new(0.5f64, 0.0f64)));

    // collinear, first intersection from p1.origin is at p1.origin itself
    let l3 = Line::new(Point2::new(0.0f64, 0.0f64), Point2::new(10.0f64, 0.0f64));
    let l4 = Line::new(Point2::new(-11.0f64, 0.0f64), Point2::new(1.0f64, 0.0f64));
    assert_eq!((l3, l4).intersection(), Some(Point2::new(0.0f64, 0.0f64)));

    // no intersection
    let l5 = Line::new(Point2::new(5.0f64, 5.0f64), Point2::new(10.0f64, 6.0f64));
    let l6 = Line::new(Point2::new(5.0f64, 4.8f64), Point2::new(10.0f64, 4.1f64));
    assert_eq!((l5, l6).intersection(), None); // no intersection

    // collinear, origins pointing same direction
    let l7 = Line::new(Point2::new(0.0f64, 1.0f64), Point2::new(0.0f64, 0.0f64));
    let l8 = Line::new(Point2::new(0.0f64, 0.5f64), Point2::new(0.0f64, -0.5f64));
    assert_eq!((l7, l8).intersection(), Some(Point2::new(0.0f64, 0.5f64)));

    // collinear, no overlap
    let l9 = Line::new(Point2::new(0.0f64, 0.0f64), Point2::new(3.0f64, 0.0f64));
    let l10 = Line::new(Point2::new(10.0f64, 0.0f64), Point2::new(5.0f64, 0.0f64));
    assert_eq!((l9, l10).intersection(), None);

    // intersection found
    let l11 = Line::new(Point2::new(0.0f64, 0.0f64), Point2::new(10.0f64, 10.0f64));
    let l12 = Line::new(Point2::new(0.0f64, 10.0f64), Point2::new(10.0f64, 0.0f64));
    assert_eq!((l11, l12).intersection(), Some(Point2::new(5.0f64, 5.0f64)));

    // special case of both lines being the same point
    let l13 = Line::new(Point2::new(0.0f64, 0.0f64), Point2::new(0.0f64, 0.0f64));
    let l14 = Line::new(Point2::new(0.0f64, 0.0f64), Point2::new(0.0f64, 0.0f64));
    assert_eq!((l13, l14).intersection(), Some(Point2::new(0.0f64, 0.0f64)));

    // both lines are points that are distinct
    let l15 = Line::new(Point2::new(0.0f64, 0.0f64), Point2::new(0.0f64, 0.0f64));
    let l16 = Line::new(Point2::new(1.0f64, 0.0f64), Point2::new(1.0f64, 0.0f64));
    assert_eq!((l15, l16).intersection(), None);

    // one line is a point that intersects the other segment
    let l15 = Line::new(Point2::new(0.0f64, 0.0f64), Point2::new(10.0f64, 0.0f64));
    let l16 = Line::new(Point2::new(3.0f64, 0.0f64), Point2::new(3.0f64, 0.0f64));
    assert_eq!((l15, l16).intersection(), Some(Point2::new(3.0f64, 0.0f64)));

    // one line is a point that is collinear but does not intersect with
    // the other line
    let l17 = Line::new(Point2::new(0.0f64, 0.0f64), Point2::new(0.0f64, 0.0f64));
    let l18 = Line::new(Point2::new(1.0f64, 0.0f64), Point2::new(3.0f64, 0.0f64));
    assert_eq!((l17, l18).intersection(), None);

    // one line is a point that is not collinear but does not intersect
    // with the other line
    let l19 = Line::new(Point2::new(0.0f64, 0.0f64), Point2::new(0.0f64, 0.0f64));
    let l20 = Line::new(Point2::new(1.0f64, 0.0f64), Point2::new(2.0f64, 10.0f64));
    assert_eq!((l19, l20).intersection(), None);
}
