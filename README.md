# cgmath-rs

[![Build Status](https://travis-ci.org/brendanzab/cgmath.svg?branch=master)](https://travis-ci.org/brendanzab/cgmath)
[![Documentation](https://docs.rs/cgmath/badge.svg)](https://docs.rs/cgmath)
[![Version](https://img.shields.io/crates/v/cgmath.svg)](https://crates.io/crates/cgmath)
[![License](https://img.shields.io/crates/l/cgmath.svg)](https://github.com/brendanzab/cgmath/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/cgmath.svg)](https://crates.io/crates/cgmath)

A linear algebra and mathematics library for computer graphics.

The library provides:

- vectors: `Vector2`, `Vector3`, `Vector4`
- square matrices: `Matrix2`, `Matrix3`, `Matrix4`
- a quaternion type: `Quaternion`
- rotation matrices: `Basis2`, `Basis3`
- angle units: `Rad`, `Deg`
- points: `Point2`, `Point3`
- perspective projections: `Perspective`, `PerspectiveFov`, `Ortho`
- spatial transformations: `AffineMatrix3`, `Transform3`

Not all of the functionality has been implemented yet, and the existing code
is not fully covered by the testsuite. If you encounter any mistakes or
omissions please let me know by posting an issue, or even better: send me a
pull request with a fix.

## Conventions

cgmath interprets its vectors as column matrices (also known as "column
vectors"), meaning when transforming a vector with a matrix, the matrix goes
on the left. This is reflected in the fact that cgmath implements the
multiplication operator for Matrix * Vector, but not Vector * Matrix.

## Limitations

cgmath is _not_ an n-dimensional library and is aimed at computer graphics
applications rather than general linear algebra. It only offers the 2, 3, and
4 dimensional structures that are more than enough for most computer graphics
applications. This design decision was made in order to simplify the
implementation (Rust cannot parameterize over constants at compile time), and to
make dimension-specific optimisations easier in the future.

## Contributing

Pull requests are most welcome, especially in the realm of performance
enhancements and fixing any mistakes I may have made along the way. Unit tests
and benchmarks are also required, so help on that front would be most
appreciated.

## Support

Contact `bjz` on irc.mozilla.org [#rust](http://mibbit.com/?server=irc.mozilla.org&channel=%23rust)
and [#rust-gamedev](http://mibbit.com/?server=irc.mozilla.org&channel=%23rust-gamedev),
or [post an issue](https://github.com/bjz/cgmath/issues/new) on Github.
