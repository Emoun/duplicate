mod errors;

#[test]
pub fn test_expansions()
{
	crate::utils::ExpansionTester::run_default_test_setup("tests/module_disambiguation", "testing");
}
