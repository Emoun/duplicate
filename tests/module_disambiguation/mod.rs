#[test]
pub fn test_expansions()
{
	use crate::utils::ExpansionTester;
	let mut test = ExpansionTester::new("tests/module_disambiguation", "testing");
	test.add_source_dir("from", vec![ExpansionTester::duplicate_for_inline()]);
	test.add_source_dir(
		"expected",
		vec![
			ExpansionTester::copy(),
			ExpansionTester::copy_with_prefix("inline_"),
		],
	);
	test.add_source_dir(
		"expected_both",
		vec![
			ExpansionTester::copy_with_prefix("inline_short_"),
			ExpansionTester::copy_with_prefix("inline_verbose_"),
			ExpansionTester::copy_with_prefix("short_"),
			ExpansionTester::copy_with_prefix("verbose_"),
		],
	);
	test.execute_tests();
}
