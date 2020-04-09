duplicate
=============================

Attribute macro for code duplication with substitution.

### Motivation

If you find yourself in need of copying a block of code and then making some small changes to fit the new use case, this crate is for you.

The `duplicate` attribute macro will duplicate an item any number of times while inserting custom code in the designated places in each duplicate.

For an in-depth explanation of the syntax and features, [see the documentation](https://docs.rs/duplicate).

### Example

```
use duplicate::duplicate;

/// Trait we want to implement for u8, u16, and u32
trait IsMax {
  /// Returns true if self is its maximum possible value.
  fn is_max(&self) -> bool;
}

#[duplicate(
  int_type  [ u8 ]  [ u16 ]    [ u32 ]
  max_value [ 255 ] [ 65_535 ] [ 4_294_967_295 ]
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

```
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

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
</sub>
