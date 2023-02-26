#[test]
fn test_expansion_errors()
{
	crate::utils::ExpansionTester::run_default_test_setup_errors(
		"tests/pretty_errors",
		"testing",
		true,
	);
}
