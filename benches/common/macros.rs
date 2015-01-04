// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directionectory of this distribution.
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

#![macro_escape]

macro_rules! bench_binop(
    ($name: ident, $t1: ty, $t2: ty, $binop: ident) => {
        #[bench]
        fn $name(bh: &mut Bencher) {
            const LEN: uint = 1 << 13;

            let mut rng = IsaacRng::new_unseeded();

            let elems1 =  Vec::from_fn(LEN, |_| rng.gen::<$t1>());
            let elems2 =  Vec::from_fn(LEN, |_| rng.gen::<$t2>());
            let mut i = 0;

            bh.iter(|| {
                i = (i + 1) & (LEN - 1);

                unsafe {
                    test::black_box(elems1.unsafe_get(i).$binop(elems2.unsafe_get(i)))
                }
            })
        }
    }
);

macro_rules! bench_binop_deref(
    ($name: ident, $t1: ty, $t2: ty, $binop: ident) => {
        #[bench]
        fn $name(bh: &mut Bencher) {
            const LEN: uint = 1 << 13;

            let mut rng = IsaacRng::new_unseeded();

            let elems1 =  Vec::from_fn(LEN, |_| rng.gen::<$t1>());
            let elems2 =  Vec::from_fn(LEN, |_| rng.gen::<$t2>());
            let mut i = 0;

            bh.iter(|| {
                i = (i + 1) & (LEN - 1);

                unsafe {
                    test::black_box(elems1.unsafe_get(i).$binop(*elems2.unsafe_get(i)))
                }
            })
        }
    }
);

macro_rules! bench_unop(
    ($name: ident, $t: ty, $unop: ident) => {
        #[bench]
        fn $name(bh: &mut Bencher) {
            const LEN: uint = 1 << 13;

            let mut rng = IsaacRng::new_unseeded();

            let mut elems =  Vec::from_fn(LEN, |_| rng.gen::<$t>());
            let mut i = 0;

            bh.iter(|| {
                i = (i + 1) & (LEN - 1);

                unsafe {
                    test::black_box(elems.unchecked_mut(i).$unop())
                }
            })
        }
    }
);

macro_rules! bench_construction(
    ($name: ident, $t: ty, $constructor: path $(, $args: ident: $types: ty)*) => {
        #[bench]
        fn $name(bh: &mut Bencher) {
            const LEN: uint = 1 << 13;

            let mut rng = IsaacRng::new_unseeded();

            $(let $args = Vec::from_fn(LEN, |_| rng.gen::<$types>());)*
            let mut i = 0;

            bh.iter(|| {
                i = (i + 1) & (LEN - 1);

                unsafe {
                    let res: $t = $constructor($(*$args.unsafe_get(i),)*);
                    test::black_box(res)
                }
            })
        }
    }
);
