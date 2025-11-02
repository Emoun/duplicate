# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Documentaion on nested invocations now clarifies that inner calls to `duplicate` are expanded before outer calls.

### Fixed

- Hints no longer print unusual Unicode characters. This was caused by [an issue with the `proc-macro2-diagnostics`](https://github.com/SergioBenitez/proc-macro2-diagnostics/issues/11) crate.
- Fixed an issue where nested calls would result in outer calls only using substitutes from the first substitution group in all duplicates.

## [2.0.0] - 2024-09-16

### Added

- `substitute!` and `substitute_item` allow the use of global substitutions without duplication. See [#49](https://github.com/Emoun/duplicate/issues/49).

### Changed

- [BREAKING] Increased base MSRV to 1.65.
- [BREAKING] `duplicate!` and `duplicate_item` no longer allow using exclusively global substitutions.
- Edition increased to 2021.
- Replaced `proc-macro-error` dependency with `proc-macro2-diagnostics` for printing nice error messages and hints. See [#61](https://github.com/Emoun/duplicate/issues/61).
- Updated `heck` dependency to version 0.5.

## [1.0.0] - 2023-03-10

### Changed

- Overhauled the `pretty_errors` feature to more consistently provide useful hints and code highlights. See [#19](https://github.com/Emoun/duplicate/issues/19).

### Fixed

- `duplicate` now also substitutes the invocations of nested `duplicate`'s. See [#48](https://github.com/Emoun/duplicate/issues/48).
- Forgetting to enclose subtitution parameters in brackets now reports sensible error. See [#30](https://github.com/Emoun/duplicate/issues/30).
- Short syntax no longer accepts providing no substitution groups after the identifier list. See [#29](https://github.com/Emoun/duplicate/issues/29).
- Fixed several issues where code was accepted which shouldn't have been.

## [0.4.1] - 2022-07-17

### Fixed

- Fixed an issue where `duplicate`'s edition would leak into the user's code. See [#47](https://github.com/Emoun/duplicate/issues/47).

## [0.4.0] - 2022-02-20

### Added

- Nested invocations can now be used everywhere.

### Changed

- [BREAKING] Renamed `duplicate::duplicate` to `duplicate::duplicate_item`. See [#40](https://github.com/Emoun/duplicate/issues/40).
- [BREAKING] Renamed `duplicate::duplicate_inline` to `duplicate::duplicate`. See [#40](https://github.com/Emoun/duplicate/issues/40).
- [BREAKING] Nested invocation now uses `duplicate!{[<invocation>] <body>}` syntax instead of `#[<invocation>][<body>]`. See [#28](https://github.com/Emoun/duplicate/issues/28).
- [BREAKING] Increased base MSRV to 1.42. See [#45](https://github.com/Emoun/duplicate/issues/45)
- [BREAKING] Relaxed MSRV policy. Only the minimal versions of direct and transitive dependencies are guaranteed to work for the MSRV. See [#44](https://github.com/Emoun/duplicate/issues/44)
- No longer pinning dependencies following new MSRV policy.
- Updated `heck` dependency to version 0.4.

## [0.3.0] - 2021-06-08

### Added

- Global Substitution: Allows substitutions that are applied to all duplicates equally. See [#23](https://github.com/Emoun/duplicate/issues/23).

### Changed

- [BREAKING] Limited which group delimiters are allowed in various syntactic positions.
The specific delimiters required now follow the use of delimiters in previous documentation examples. 
See  [#25](https://github.com/Emoun/duplicate/issues/25).
- Substituted the `convert_case` dependency for `heck`. See [#22](https://github.com/Emoun/duplicate/issues/22).


### Fixed

- Now also substitutes code inside substitution arguments. See [#24](https://github.com/Emoun/duplicate/issues/24).
- Module_disambiguation: Now only substitutes the module name and not any matching identifier in the body of the module. See [#27](https://github.com/Emoun/duplicate/issues/27).

## [0.2.9] - 2020-09-28

### Added

- New _Minimum Supported Rust Version_ (MSRV) Policy. Briefly, MSRV is 1.34 but may be increased by enabling features. See [#18](https://github.com/Emoun/duplicate/issues/18#issuecomment-697554595).
- The crate's readme now contains section on MSRV policy.
- Parameterized substitution is now also available for the verbose syntax. See [#8](https://github.com/Emoun/duplicate/issues/8).

## [0.2.8] - 2020-09-24

## [0.2.7] - 2020-08-10

### Added

- `duplicate_inline`: Function-like procedural macro version of `duplicate`. 
Has the same functionality, but can be used in other contexts and can duplicate any number of items. 
See also [#6](https://github.com/Emoun/duplicate/issues/6).

### Changed

- Updated `proc_macro_error` dependency to version 1.0.4.

## [0.2.6] - 2020-07-13

### Added

- New feature named `pretty_errors` (enabled by default). When enabled, errors are more detailed and helpful.
- New feature named `module_disambiguation` (enabled by default). When enabled, automatically uses a suitable substitution identifier to disambiguate the name of a module being duplicated. See the documentation for more details. See also [#7](https://github.com/Emoun/duplicate/issues/7).

### Changed

- Errors are now less detailed and helpful unless the `pretty_errors` feature is enabled.
- The dependence on the `proc_macro_error` crate is now optional and used by the `pretty_errors` feature.

## [0.2.5] - 2020-06-29

### Fixed

- Fixed a build issue when using version 1.0.3 of `proc_macro_error`. See [#11](https://github.com/Emoun/duplicate/issues/11).

### Changed

- The `proc_macro_error` dependency is fixed to version 1.0.3 to avoid potential breaking changes with future updates to that crate. 

## [0.2.4] - 2020-06-23

### Fixed

- Fixed issue with the short syntax where substitutions that included any bracket type would be expanded wrong. See [#9](https://github.com/Emoun/duplicate/issues/9).

## [0.2.3] - 2020-06-21 [YANKED]

### Added

- Short syntax now supports parameterized substitution. 
This allows identifiers to take arguments that can be used to customize the substitution for each use.
See [#5](https://github.com/Emoun/duplicate/issues/5).

## [0.2.2] - 2020-05-17

### Added

- Short syntax now supports nested macro invocations. Can only be used after the initial list of substitution identifiers. See [#2](https://github.com/Emoun/duplicate/issues/2).

### Changed

- Updated documentation. The short syntax is now used for the primary examples.
- Crate readme license section no longer includes paragraph on the licensing of contibutions.

### Fixed

- Fixed an issue where `#[duplicate(..)]`  would throw an error if it was generated by a macro expansion. 
It would fail if a substitution identifier originated from a macro variable.
This is caused by the variable's expansion sometimes being wrapped in a 
[group](https://doc.rust-lang.org/proc_macro/struct.Group.html) with 
[no delimiters](https://doc.rust-lang.org/proc_macro/enum.Delimiter.html#variant.None), 
which `duplicate` couldn't handle.

## [0.2.1] - 2020-04-26

### Changed

- Updated crate readme to new short syntax.

## [0.2.0] - 2020-04-26

### Added

- Crate readme changelog section now asserts the use of [Semantic Versioning](https://semver.org/spec/v2.0.0.html) and [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

### Changed

- New short syntax with row-based substitution groups instead of the old column-based syntax.
See [github issue](https://github.com/Emoun/duplicate/issues/1).

## [0.1.5] - 2020-04-12

### Changed

- Crate readme format titles are now bigger than changelog entries.

## [0.1.4] - 2020-04-12

### Added

- Crate readme now includes list of changes when published to `crates.io`.

## [0.1.3] - 2020-04-12

## [0.1.2] - 2020-04-9

### Added

- `duplicate` attribute macro for code duplication and substitution.
- Short invocation syntax for `duplicate`.
- Verbose invocation syntax for `duplicate`.