// Compile-fail tests for test_suite_macro error diagnostics.
//
// Each file in tests/ui/ must have a corresponding .stderr snapshot.
// Run `TRYBUILD=overwrite cargo test` to (re)generate them.

#[test]
fn trybuild() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/trybuild/*.rs");
}
