# cgmath-rs

[![Build Status](https://travis-ci.org/bjz/cgmath-rs.svg?branch=master)](https://travis-ci.org/bjz/cgmath-rs)
[![Version](https://img.shields.io/crates/v/cgmath.svg)](https://crates.io/crates/cgmath)
[![License](https://img.shields.io/crates/l/cgmath.svg)](https://github.com/bjz/cgmath-rs/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/cgmath.svg)](https://crates.io/crates/cgmath)

[Documentation](http://bjz.github.io/cgmath)

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

## Limitations

cgmath is _not_ an n-dimensional library and is aimed at computer graphics
applications rather than general linear algebra. It only offers the 2, 3, and
4 dimensional structures that are more than useful for most computer graphics
applications. This design decision was made in order to simplify the
implementation (Rust cannot paramerise over constants at compile time), and to
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
