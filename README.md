# Lmath-rs

Lmath is generic linear algebra library for Rust. There is still much to do, unit tests to write, bugs to fix, and performance enhancements to make. Help is much appreciated, so please don't hesitate to send me a pull request.

## Current issues

Some of the trait bounds are currently very brittle and verbose. For example you may see implementations like the following:

    pub impl<T:Copy Num NumCast Trig Exp Extent Ord FuzzyEq> Quat<T>: Quaternion<T> { ... }

Ick! Luckily this will be largely eliminated with trait inheritance which will be coming to Rust in the future. That will mean that the above Quaternion implementation would become something like:

    pub impl<T:FloatExt> Quat<T>: Quaternion<T> { ... }

## Todo:

- ~~Matrix inversion~~
- ~~Matrix rotation~~
- ~~Angle types (degrees/radians)~~
- N x M matrices
- Operator Overloading
- Implement component-wise functions for vectors
- Euler and axial rotations
- Performance improvements
- Increase unit test coverage
- Improve documentation
- ~~Publish on Cargo Central~~
- Swizzle functions

Dependant on rust/master:

- Implement trait inheritance
- Make use of static functions for constants and constructors


~B☼
