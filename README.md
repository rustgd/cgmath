# cgmath-rs

[![Build Status](https://travis-ci.org/bjz/cgmath-rs.png?branch=master)](https://travis-ci.org/bjz/cgmath-rs)

A linear algebra and mathematics library for computer graphics.

The library provides:

- vectors: `Vector2`, `Vector3`, `Vector4`
- square matrices: `Matrix2`, `Matrix3`, `Matrix4`
- a quaternion type: `Quaternion`
- rotation matrices: `Basis2`, `Basis3`
- angle units: `Rad`, `Deg`
- points: `Point2`, `Point3`
- a generic ray: `Ray`
- a plane type: `Plane`
- perspective projections: `Perspective`, `PerspectiveFov`, `Ortho`
- a view frustum: `Frustum`
- spatial transformations: `AffineMatrix3`, `Transform3`
- axis-aligned bounding boxes: `Aabb2`, `Aabb3`
- oriented bounding boxes: `Obb2`, `Obb3`
- collision primitives: `Sphere`, `Cylinder`

Not all of the functionality has been implemented yet, and the existing code
is not fully covered by the testsuite. If you encounter any mistakes or
omissions please let me know by posting an issue, or even better: send me a
pull request with a fix.

## Documentation

[View](http://rust-ci.org/bjz/cgmath-rs/doc/cgmath/index.html)

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
or [post an issue](https://github.com/bjz/cgmath-rs/issues/new) on Github.
