# cgmath-rs

A linear algebra and mathematics library for computer graphics.

The library provides:

- vectors: `Vec2`, `Vec3`, `Vec4`
- points: `Point2`, `Point3`, `Point4`
- square matricies: `Mat2`, `Mat3`, `Mat4`
- a quaternion type: `Quat`
- perspective projections
- and more to come...?

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
make graphics-specific optimisations easier in the future. Those looking for
n-dimensional mathematics can look to [nalgebra](https://github.com/sebcrozet/nalgebra).

Currently, operators are not overloaded. This is due to limitations in Rust's
generics that will be fixed sometime in the future.

## Contributing

Pull requests are most welcome, especially in the realm of performance
enhancements and fixing any mistakes I may have made along the way. Unit tests
and benchmarks are also required, so help on that front would be most
appreaciated.
