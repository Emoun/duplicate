#[test]
fn test_expansions()
{
	crate::utils::ExpansionTester::run_default_test_setup("tests/no_features", "testing");
}

/// Test that using the crate from an edition 2021 crate works, even if
/// the use expands to code that only works with edition 2021.
#[rustversion::since(1.56)]
#[test]
fn test_edition_2021()
{
	let output = std::process::Command::new("cargo")
		.arg("build")
		.current_dir("tests/no_features/edition_2021")
		.output()
		.unwrap();
	assert!(
		output.status.success(),
		"Failed to build edition 2021: {:?}",
		output
	);
}

/// Tests that nowhere in the source code do we call `Group::new` as that has
/// the huge trap of setting the span to `Span::call_site`, which could be
/// extremely problematic (e.g. it means the crate's edition could leak to the
/// user's code)
///
/// Note: use `new_group` instead of `Group::new` as the sanctioned way to
/// create new groups
#[test]
fn ensure_no_group_new()
{
	let re = regex::Regex::new(r"[[:^alpha:]]Group(\s)*::(\s)*new").unwrap();
	for path in std::fs::read_dir("src").unwrap()
	{
		let path = path.unwrap().path();
		let file_content = std::fs::read_to_string(&path).unwrap();
		assert!(
			!re.is_match(file_content.as_str()),
			"Found 'Group::new' in {:?}",
			path
		);
	}
}
