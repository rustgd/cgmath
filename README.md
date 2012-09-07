# om3D: A Linear Algebra Library for Rust

Here's some linear algebra I've been working on. I've translated it over from my unpublished D library that I was using to teach myself 3D mathematics.

For some reason my makefile isn't working - it give me the following error:

    rustc src/om3d.rc --lib --out-dir=lib
    error: failed to resolve imports
    src/quat.rs:5:7: 5:14 error: unresolved import
    src/quat.rs:5 import mat::*;
                         ^~~~~~~
    src/mat.rs:5:7: 5:15 error: unresolved import
    src/mat.rs:5 import quat::*;
                        ^~~~~~~~
    src/projection.rs:2:7: 2:14 error: unresolved import
    src/projection.rs:2 import mat::*;
                               ^~~~~~~
    error: aborting due to 4 previous errors
    make: *** [all] Error 101
    
Any assistance would be most appreciated!

## Todo:

- Unittests: I have full unittest coverage on my D project so the algorithms should be correct I just haven't implemented the yet because of the compilation issue above
- Vector functions: abs, lerp, min, max
- Matrix Inversion: ugh
- Matrix rotation
- Euler and Axial rotations

## Disclaimer:

I'm new to Rust and a novice at linear algebra. I also haven't written any unittests yet (although I plan to soon). I've almost certainly made mistakes, so use this at your own risk!

At the time of writing (September 2012) the Rust language is still in a state of flux. There is a good chance that the code will be soon out of date.

~B☼