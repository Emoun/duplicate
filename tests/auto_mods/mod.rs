#[test]
pub fn test_expansions()
{
	use crate::utils::ExpansionTester;
	let mut test = ExpansionTester::new("tests/auto_mods", "testing");
	test.add_source_dir("from", ExpansionTester::copy());
	test.add_source_dir("expected", ExpansionTester::copy());
	test.execute_tests();
}
