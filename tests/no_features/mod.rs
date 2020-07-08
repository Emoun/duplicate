use crate::utils::ExpansionTester;
use std::ffi::OsString;

#[test]
fn test_expansions()
{
	let mut test = ExpansionTester::new("tests/no_features", "testing");
	test.add_source_dir("from", ExpansionTester::copy());
	test.add_source_dir("expected", ExpansionTester::copy());
	fn expect_for_short_and_verbose(file: OsString) -> Vec<OsString>
	{
		let mut short = OsString::from("short_");
		short.push(&file);
		let mut verbose = OsString::from("verbose_");
		verbose.push(file);
		vec![short, verbose]
	}
	test.add_source_dir(
		"expected_both",
		ExpansionTester::duplicate(expect_for_short_and_verbose),
	);
	test.execute_tests();
}
