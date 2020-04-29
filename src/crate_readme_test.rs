#![cfg(doctest)]
use doc_comment::doctest;

// Tests the crate readme file's Rust examples.
doctest!("../cargo-readme.md");
