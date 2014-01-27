# cgmath-rs

[![Build Status](https://travis-ci.org/bjz/cgmath-rs.png?branch=master)](https://travis-ci.org/bjz/cgmath-rs)

A linear algebra and mathematics library for computer graphics.

The library provides:

- vectors: `Vec2`, `Vec3`, `Vec4`
- square matrices: `Mat2`, `Mat3`, `Mat4`
- a quaternion type: `Quat`
- rotation matrices: `Rot2`, `Rot3`
- angle units: `Rad`, `Deg`
- points: `Point2`, `Point3`
- rays: `Ray2`, `Ray3`
- a plane type: `Plane`
- perspective projections: `Perspective`, `PerspectiveFov`, `Ortho`
- a view frustum: `Frustrum`
- spatial transformations: `AffineMatrix3`, `Transform3D`
- axis-aligned bounding boxes: `Aabb2`, `Aabb3`
- oriented bounding boxes: `Obb2`, `Obb3`
- collision primitives: `Sphere`, `Cylinder`

Not all of the functionality has been implemented yet, and the existing code
is not fully covered by the testsuite. If you encounter any mistakes or
omissions please let me know by posting an issue, or even better: send me a
pull request with a fix.

## Compilation

### Building the library

~~~
mkdir -p lib
rustc --out-dir lib ./src/cgmath/lib.rs
~~~

### Running the tests

~~~
mkdir -p lib bin
rustc --out-dir lib ./src/cgmath/lib.rs
rustc --out-dir bin --test -L ./lib ./src/test/test.rs
./bin/test
~~~

### Running the benchmarks

~~~
mkdir -p lib bin
rustc --out-dir lib ./src/cgmath/lib.rs
rustc --out-dir bin --test -L ./lib ./src/test/test.rs
./bin/bench --bench
~~~

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
