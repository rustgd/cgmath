# cgmath-rs

A linear algebra and mathematics library for computer graphics.

The library provides:

- vectors: `Vec2`, `Vec3`, `Vec4`
- square matrices: `Mat2`, `Mat3`, `Mat4`
- a quaternion type: `Quat`
- rotation matrices: `Rot2`, `Rot3`
- rotations: `Euler`, `AxisAngle`
- angle units: `Rad`, `Deg`
- points: `Point2`, `Point3`, `Point4`
- rays: `Ray2`, `Ray3`
- plane: `Plane`
- perspective projections: `Perspective`, `PerspectiveFov`, `Ortho`
- a view frustum: `Frustrum`,
- axis-aligned bounding boxes: `Aabb2`, `Aabb3`
- oriented bounding boxes: `Obb2`, `Obb3`
- collision primitives: `Sphere`, `Cylinder`
- and more to come...?

Not all of the functionality has been implemented yet, and the existing code
is not fully covered by the testsuite. If you encounter any mistakes or
omissions please let me know by posting an issue, or even better: send me a
pull request with a fix.

## Compilation

~~~
rustpkg build cgmath
~~~

## Limitations

cgmath is _not_ an n-dimensional library and is aimed at computer graphics
applications rather than general linear algebra. It only offers the 2, 3, and
4 dimensional structures that are more than useful for most computer graphics
applications. This design decision was made in order to simplify the
implementation (Rust cannot paramerise over constants at compile time), and to
make dimension-specific optimisations easier in the future. Those looking for
n-dimensional mathematics can look to [nalgebra](https://github.com/sebcrozet/nalgebra).

Currently, operators are not overloaded. This is due to limitations in Rust's
generics that will be fixed sometime in the future.

## Contributing

Pull requests are most welcome, especially in the realm of performance
enhancements and fixing any mistakes I may have made along the way. Unit tests
and benchmarks are also required, so help on that front would be most
appreciated.
