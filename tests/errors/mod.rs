use crate::utils::{
	run_basic_expansion_error_tests, run_error_highlight_tests, run_error_hint_tests,
};

/// Tests all expected basic error messages in 'basic' on their respective
/// source files in 'source'
///
/// Expects every source file to have an expected basic.
#[test]
fn basic_expansion_errors()
{
	run_basic_expansion_error_tests(file_dir!());
}

/// Tests the expected hints in 'hint' against their respective source files in
/// 'source'.
///
/// Only tests source files that have a hint file.
#[test]
fn hint_expansion_errors()
{
	run_error_hint_tests(file_dir!());
}

/// Tests the expected code highlights in 'highlight' against their respective
/// source files in 'source'.
///
/// Only tests source files that have a highlight file.
#[test]
fn highlight_expansion_errors()
{
	run_error_highlight_tests(file_dir!());
}
