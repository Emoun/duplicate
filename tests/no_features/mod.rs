use crate::utils::ExpansionTester;

#[test]
fn test_expansions()
{
	let mut test = ExpansionTester::new("tests/no_features", "testing");
	test.add_source_dir("from", ExpansionTester::copy());
	test.add_source_dir("expected", ExpansionTester::copy());
	test.add_source_dir("expected_both", ExpansionTester::duplicate_for_syntaxes());
	test.execute_tests();
}
