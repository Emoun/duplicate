//! This file tests which features are on by default
//!
//! To tests this correctly, tests should be run without using the `--features`
//! and `--no-default-features` flags to ensure that only the default features
//! are enable.

/// Tests that a feature is enabled by default.
///
/// Must first be given a unique identifier for the feature (which doesn't
/// necessarily need to be the same as the feature name, but it might be a good
/// idea), and then a string containing the feature name.
///
/// ### Example
///
/// The following code will test whether a feature named "feature_name" is
/// enabled by default.
///
/// ```
/// default_feature!{feature_id "feature_name"}
/// ```
macro_rules! default_features {
	{
		$feature:ident $feature_string:literal
	} => {
		mod $feature
		{
			#[cfg(feature = $feature_string)]
			const IS_DEFAULT: bool = true;
			#[cfg(not(feature = $feature_string))]
			const IS_DEFAULT: bool = false;

			#[test]
			pub fn is_default()
			{
				assert!(IS_DEFAULT, "Feature '{}' is not enabled by default.", $feature_string);
			}
		}
	};
}

default_features!(pretty_errors "pretty_errors");
default_features!(module_disambiguation "module_disambiguation");
