duplicate
=============================

[![Build Status](https://api.travis-ci.org/Emoun/duplicate.svg?branch=master)](https://travis-ci.org/Emoun/duplicate)
[![Latest Version](https://img.shields.io/crates/v/duplicate.svg)](https://crates.io/crates/duplicate)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/duplicate)

attribute macro for code duplication with substitution.

This document is meant for contributors to the crate. The documentation is hosted at [docs.rs](https://docs.rs/duplicate).

# Testing

Tests are divided into the following groups:

- [`no_features`](#anch-tests-no-features):
Tests the minimal API of the crate with no features enabled. 
Most other tests also run these tests.

- [`default_features`](#anch-tests-default-features): 
Tests which features are defaults but does not test any functionality.

#### Setup

This crate uses [macrotest](https://crates.io/crates/macrotest) for testing expansions. 
Therefore, before running these tests the nightly compiler must be installed together with `rustfmt` and `cargo-expand`:

```
cargo install cargo-expand
rustup toolchain install nightly
rustup component add rustfmt
```

The tests can then be run normally using `cargo test`.


#### <a name="anch-tests-no-features"></a>`no-features`

Test the basic API of the crate without any features enabled:

```
cargo test no_features:: --no-default-features
```

#### <a name="anch-tests-default-features"></a>`default_features`

Test that the correct features are enabled by default.
This is to ensure a change doesn't change which features are on by default.
However, this does not test the features themselves:

```
cargo test default_features::
```

# Formatting

We use `rustfmt` to manage the formatting of this crate's code.
To have cargo format the code for you, you must have the nightly compiler installed (but not necessarily the default) and then run the command:

```
cargo +nightly fmt
```

Travis-CI will check the formatting and fail if it isn't formatted correctly.

# Release Deployment

Deployment of new versions of this crate is managed by Travis-CI using git tags. 
To trigger a new release, simply push a tag to the repository containing only the version number:

```
git tag 1.0.0
git push --tags
```

We do not use the `Cargo.toml` to manage the versioning of this crate.
The version given in it should not be changed! 
It must remain as `0.0.0` so Travis-CI can correctly change it for every release.

Travis will also reset the change log as part of the release, so do not change the `## [Unreleased]` line nor add an entry for new releases yourself.

Travis will also add the new release's changes to `cargo-readme.md` under the `Changelog` section. So do not touch that either.

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

