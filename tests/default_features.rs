//! This file tests which features are on by default
//!
//! To tests this correctly, tests should be run without using the '--features'
//! flag to ensure all default features are enabled and no other.

/// Tests that a feature is enabled by default.
///
/// Must first be given a unique identifier for the feature (which doesn't
/// necessarily need to be the same as the feature name), and then a string
/// containing the feature name.
///
/// ### Example
///
/// The following code will test whether a feature name "feature_name" is
/// enabled by default.
///
/// ```
/// default_feature!{feature_id "feature_name"}
/// ```
macro_rules! default_features {
	(  $feature:ident $feature_string:literal
	 	// $($feature_rest:ident $feature_string_rest:literal)*
	 	) => {
		mod $feature
		{
			#[cfg(feature = $feature_string)]
			const IS_DEFAULT: bool = true;
			#[cfg(not(feature = $feature_string))]
			const IS_DEFAULT: bool = false;

			#[test]
			pub fn test_is_default()
			{
				assert!(IS_DEFAULT, "Feature is not enabled by default.");
			}
		}
	};
}

default_features!(pretty_errors "pretty_errors");
