#[test]
fn test_expansion_errors()
{
	crate::utils::ExpansionTester::run_default_test_setup_errors(
		"tests/no_features_errors",
		"testing",
		true,
	);
}
