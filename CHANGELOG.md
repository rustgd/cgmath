# Change Log

All notable changes to this project will be documented in this file, following
the format defined at [keepachangelog.com](http://keepachangelog.com/).
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added
- Add missing by-ref and by-val permutations of `Quaternion` operators.

### Changed
- `Vector` and `Point` are now constrained to require specific operators to be
  overloaded. This means that generic code can now use operators, instead of
  the operator methods.

### Removed
- Remove redundant `Point::{min, max}` methods - these are now covered by the
  `Array::{min, max}` methods that were introduced in 0.5.0.
- Removed `ToComponents`, `ToComponents2`, and `ToComponents3`. If you were
  relying on `ToComponents::decompose`, you can produce the same effect by
  accessing the fields on `Decomposed` directly. To create the scale vector,
  use: `Vector::from_value(transform.scale)`.
- Removed `CompositeTransform`, `CompositeTransform2`, and `CompositeTransform3`.
- Remove `Vector::one`. Vectors don't really have a multiplicative identity.
  If you really want a `one` vector, you can do something like:
  `Vector::from_value(1.0)`.
- Remove operator methods from `Vector` and `Point` traits in favor of operator
  overloading.
- Remove `*_self` methods from `Vector` and `Point`. These were of little
  performance benefit, and assignment operator overloading will be coming soon!

## [v0.6.0] - 2015-12-12

### Added
- This CHANGELOG for keeping track of notable changes.
- `Matrix4::{from_scale, from_nonuniform_scale}` for easily constructing
  homogeneous scale matrices.

### Changed
- Renamed `SquareMatrix::one` to `SquareMatrix::identity`. `identity` is easier
  to search for,
  and the more common name for the multiplicative identity for matrices.
- Matrix impls have now been constrained to `S: BaseFloat`.

## [v0.5.0] - 2015-11-20

### Changed
- Take many point and vector parameters by value.
- Take point and vector operator overloads by value.
- Divide `Matrix` trait into `Matrix` and `SquareMatrix`, opening the door for
  non-square matrices in the future.
- Make many trait type parameters associated types.
- Move element-wise methods from `Vector` and `Point` onto the `Array1` trait,
  and rename it to `Array`.
- Make pointer access methods on `Array` match the naming scheme of those in the
  standard library.

### Removed
- Removed collision types: `Ray`, `Plane`, `Frustum`, `Aabb2`, `Aabb3` `Obb2`,
  `Obb3` `Sphere`, `Cylinder`. These can now be found at
  [csherratt/collision-rs](https://github.com/csherratt/collision-rs).
- Remove `Array2` trait, moving methods onto the `Matrix` trait.

## [v0.4.0] - 2015-10-25

## [v0.3.1] - 2015-09-20

## [v0.3.0] - 2015-09-20

## [v0.2.0] - 2015-05-11

## [v0.1.6] - 2015-05-10

## [v0.1.5] - 2015-04-25

## [v0.1.4] - 2015-04-24

## [v0.1.3] - 2015-04-06

## [v0.1.2] - 2015-04-01

## [v0.1.1] - 2015-03-25

## [v0.1.0] - 2015-03-15

## [v0.0.8] - 2015-03-09

## [v0.0.7] - 2015-03-01

## [v0.0.6] - 2015-02-21

## [v0.0.5] - 2015-02-16

## [v0.0.4] - 2015-02-11

## [v0.0.3] - 2015-02-08

## v0.0.1 - 2014-06-24

[Unreleased]: https://github.com/bjz/cgmath-rs/compare/v0.6.0...HEAD
[v0.6.0]: https://github.com/bjz/cgmath-rs/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/bjz/cgmath-rs/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/bjz/cgmath-rs/compare/v0.3.1...v0.4.0
[v0.3.1]: https://github.com/bjz/cgmath-rs/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/bjz/cgmath-rs/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/bjz/cgmath-rs/compare/v0.1.6...v0.2.0
[v0.1.6]: https://github.com/bjz/cgmath-rs/compare/v0.1.5...v0.1.6
[v0.1.5]: https://github.com/bjz/cgmath-rs/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/bjz/cgmath-rs/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/bjz/cgmath-rs/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/bjz/cgmath-rs/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/bjz/cgmath-rs/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/bjz/cgmath-rs/compare/v0.0.8...v0.1.0
[v0.0.8]: https://github.com/bjz/cgmath-rs/compare/v0.0.7...v0.0.8
[v0.0.7]: https://github.com/bjz/cgmath-rs/compare/v0.0.6...v0.0.7
[v0.0.6]: https://github.com/bjz/cgmath-rs/compare/v0.0.5...v0.0.6
[v0.0.5]: https://github.com/bjz/cgmath-rs/compare/v0.0.4...v0.0.5
[v0.0.4]: https://github.com/bjz/cgmath-rs/compare/v0.0.3...v0.0.4
[v0.0.3]: https://github.com/bjz/cgmath-rs/compare/v0.0.1...v0.0.3
