use crate::utils::ExpansionTester;
#[cfg(feature = "pretty_errors")]
use std::path::{Path, PathBuf};

/// Tests all expected basic error messages in 'basic' on their respective
/// source files in 'source'
///
/// Expects every source file to have an expected basic.
#[test]
fn basic_expansion_errors()
{
	let mut tester = ExpansionTester::new_errors("tests/errors", "testing_basic");

	// First setup all basic tests
	tester.add_source_dir(
		"basic",
		vec![
			ExpansionTester::copy_with_prefix_postfix("basic_", ".expanded.rs"),
			ExpansionTester::copy_with_prefix_postfix("inline_basic_", ".expanded.rs"),
		],
	);
	tester.add_source_dir(
		"source",
		vec![ExpansionTester::duplicate_for_inline_with_prefix("basic_")],
	);
	tester.execute_tests();
}

/// Copies the source file with the same name as the current file
/// into the testing directory in both attribute and inline version (see
/// duplicate_for_inline)
#[cfg(feature = "pretty_errors")]
pub fn get_source(prefix: &str) -> Box<dyn '_ + Fn(&Path, &dyn AsRef<Path>)>
{
	Box::new(move |file, destination| {
		let mut source_file_name = std::ffi::OsString::from(file.file_name().unwrap());
		source_file_name.push(".rs");

		let mut source_file_path = PathBuf::from(file.parent().unwrap().parent().unwrap());
		source_file_path.push("source");
		source_file_path.push(source_file_name);

		assert!(
			source_file_path.exists(),
			"Missing file: {:?}",
			source_file_path.as_os_str()
		);

		ExpansionTester::duplicate_for_inline_with_prefix(prefix)(&source_file_path, destination);
	})
}

/// Tests the expected hints in 'hint' against their respective source files in
/// 'source'.
///
/// Only tests source files that have a hint file.
#[cfg(feature = "pretty_errors")]
#[test]
fn hint_expansion_errors()
{
	let mut tester = ExpansionTester::new_errors("tests/errors", "testing_hint");

	tester.add_source_dir(
		"hint",
		vec![
			ExpansionTester::copy_with_prefix_postfix("hint_", ".expanded.rs"),
			ExpansionTester::copy_with_prefix_postfix("inline_hint_", ".expanded.rs"),
			get_source("hint_"),
		],
	);

	tester.execute_tests();
}

/// Tests the expected code highlights in 'highlight' against their respective
/// source files in 'source'.
///
/// Only tests source files that have a highlight file.
#[cfg(feature = "pretty_errors")]
#[test]
fn highlight_expansion_errors()
{
	let mut tester = ExpansionTester::new_errors("tests/errors", "testing_highlight");

	tester.add_source_dir(
		"highlight",
		vec![
			ExpansionTester::copy_with_prefix_postfix("highlight_", ".expanded.rs"),
			ExpansionTester::copy_with_prefix_postfix("inline_highlight_", ".expanded.rs"),
			get_source("highlight_"),
		],
	);

	tester.execute_tests();
}
