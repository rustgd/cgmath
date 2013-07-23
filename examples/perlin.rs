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

//! An example of using perlin noise

// Noise rendering based off rust/src/test/bench/noise.rs

extern mod lmath;

use std::uint::range;
use lmath::noise::perlin::Perlin;
use lmath::math::Point2;

static WIDTH:  uint = 100;
static HEIGHT: uint = 100;

fn main() {
    let symbols = [" ", "░", "▒", "▓", "█", "█"];
    let mut pixels = [[0f32, ..WIDTH], ..HEIGHT];

    let perlin = Perlin::new::<f32>();

    for range(0, HEIGHT) |y| {
        for range(0, WIDTH) |x| {
            pixels[y][x] = perlin.noise2(
                Point2::new(x as f32 * 0.1f32,
                            y as f32 * 0.1f32)
            ) * 0.5f32 + 0.5f32;
        };
    };

    for range(0, HEIGHT) |y| {
        for range(0, WIDTH) |x| {
            print(symbols[pixels[y][x] / 0.2f32 as int]);
        }
        println("");
    }
}
