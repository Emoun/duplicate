duplicate
=============================

[![Rust](https://github.com/Emoun/duplicate/workflows/Rust/badge.svg)](https://github.com/Emoun/duplicate/actions)
[![Latest Version](https://img.shields.io/crates/v/duplicate.svg)](https://crates.io/crates/duplicate)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/duplicate)

attribute macro for code duplication with substitution.

This document is meant for contributors to the crate. The documentation is hosted at [docs.rs](https://docs.rs/duplicate).

# Testing

#### Setup

This crate uses [macrotest](https://crates.io/crates/macrotest) for testing expansions. 
Therefore, before running these tests the nightly compiler must be installed together with `rustfmt` and `cargo-expand`:

```
cargo install cargo-expand
rustup toolchain install nightly
rustup component add rustfmt
```

The tests can then be run normally using `cargo test` as seen below.

#### Test Groups

Tests are divided into the following groups:

- `no_features`:
Tests the minimal API of the crate with no features enabled. 

```
cargo test --no-default-features -- --skip default_features::
```

- `default_features`: 
Test that the correct features are enabled by default.
This is to ensure a change doesn't change which features are on by default.
However, this does not test the features themselves.

```
cargo test default_features::
```

- `features`:
Tests any combination of features. After `--features` add a comma separated list of features to test:

```
cargo test --features module_disambiguation,pretty_errors -- --skip default_features::
```

- `documentation`:
Tests code in the documentation. Even though some of the other test groups might test some of the documentaion code, they are not guaranteed to run all tests. E.g. the test of the cargo readme file (`cargo-readme.md` are only run when this command is used.
```
cargo test --doc --all-features
```

#### Warnings

Compilation warnings are prohibited in this crate and cause CI failure.
However, this prohibition of off by default in the code to allow for warnings while work is still in progress.

To make compilation fail on warnings, simply add `--features=fail-on-warnings` to your build/test command. E.g.:

```
cargo test default_features:: --features=fail-on-warnings
```

# Formatting

We use `rustfmt` to manage the formatting of this crate's code.
To have cargo format the code for you, you must have the nightly compiler installed (but not necessarily the default) and then run the command:

```
cargo +nightly fmt
```

The CI/CD will check the formatting and fail if it isn't formatted correctly.

# Release Deployment

Deployment of new versions of this crate is managed by CI/CD using git tags. 
To trigger a new release, simply push a tag to the repository containing only the version number:

```
git tag 1.0.0
git push --tags
```

We do not use the `Cargo.toml` to manage the versioning of this crate.
The version given in it should not be changed! 
It must remain as `0.0.0` so CI/CD can correctly modify it for every release.

CI/CD will also reset the change log as part of the release, so do not change the `## [Unreleased]` line nor add an entry for new releases yourself.

CI/CD will also add the new release's changes to `cargo-readme.md` under the `Changelog` section. So do not touch that either.

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

