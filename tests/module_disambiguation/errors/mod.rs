use crate::utils::{
	run_basic_expansion_error_tests, run_error_highlight_tests, run_error_hint_tests,
};

#[test]
fn basic_expansion_errors()
{
	run_basic_expansion_error_tests(file_dir!());
}

#[test]
fn highlight_expansion_errors()
{
	run_error_highlight_tests(file_dir!());
}

#[test]
fn hint_expansion_errors()
{
	run_error_hint_tests(file_dir!());
}
