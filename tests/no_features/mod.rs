use crate::utils::ExpansionTester;

#[test]
fn test_expansions()
{
	let mut test = ExpansionTester::new("tests/no_features", "testing");
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
