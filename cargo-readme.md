duplicate
=============================

Crate for easy code duplication with substitution.

## Motivation

If you find yourself in need of copying a block of code and then making some small changes to fit the new use case, this crate is for you.

The `duplicate` attribute macro will duplicate an item any number of times while inserting custom code in the designated places in each duplicate.
The `duplicate_inline` function-like procedural macro will do the same for any code you give it.

For an in-depth explanation of the syntax and features, [see the documentation](https://docs.rs/duplicate).

## Example

```rust
use duplicate::duplicate;

/// Trait we want to implement for u8, u16, and u32
trait IsMax {
  /// Returns true if self is its maximum possible value.
  fn is_max(&self) -> bool;
}

#[duplicate(
  int_type  max_value;
  [ u8 ]    [ 255 ];
  [ u16 ]   [ 65_535 ];
  [ u32 ]   [ 4_294_967_295 ];
)]
impl IsMax for int_type {
  fn is_max(&self) -> bool {
    *self == max_value
  }
}

assert!(!42u8.is_max());
assert!(!42u16.is_max());
assert!(!42u32.is_max());
```
Expands to:

```rust
use duplicate::duplicate;

/// Trait we want to implement for u8, u16, and u32
trait IsMax {
  /// Returns true if self is its maximum possible value.
  fn is_max(&self) -> bool;
}

impl IsMax for u8 {
  fn is_max(&self) -> bool {
    *self == 255
  }
}
impl IsMax for u16 {
  fn is_max(&self) -> bool {
    *self == 65_535
  }
}
impl IsMax for u32 {
  fn is_max(&self) -> bool {
    *self == 4_294_967_295
  }
}

assert!(!42u8.is_max());
assert!(!42u16.is_max());
assert!(!42u32.is_max());
```

## Changelog

This project adheres to [Semantic Versioning.](https://semver.org/spec/v2.0.0.html)
During initial development (with versions 0.y.z), bumps to the minor version (y) signify breaking changes.

[changelog_body]

This changelog format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and shows only the changes since the previous version.
[See the full changelog](https://github.com/Emoun/duplicate/blob/master/CHANGELOG.md) for changes to all released versions.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
