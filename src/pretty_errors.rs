#![cfg_attr(not(feature = "pretty_errors"), allow(dead_code))]

/// For when substitution parameters aren't enclosed in brackets
pub(crate) const BRACKET_SUB_PARAM: &'static str = r#"Hint: Substitution parameters should be enclosed in '[]' each.
Example:
    sub_ident( [ parameter1 ] , [ paramter2 ] )
              ^^^          ^^^ ^^^         ^^^
"#;

/// For when neither syntaxes get any invocation input
pub(crate) const NO_INVOCATION: &'static str =
	"substitution_identifier (short syntax) or substitution group (verbose syntax)";

/// For when neither syntaxes get any invocation input
pub(crate) const SHORT_SYNTAX_NO_GROUPS: &'static str = r#"Hint: Add a substitution group after the substitution identifiers.
Example:
	name;
	[SomeSubstitution];
	^^^^^^^^^^^^^^^^^^^
"#;

/// For when short syntax substitutions aren't enclosed in brackets
pub(crate) const SHORT_SYNTAX_MISSING_SUB_BRACKET: &'static str = r#"Hint: Each substitution should be enclosed in '[]'.
Example:
    ident1 ident2;
    [ sub1 ] [ sub2 ] ;
   ^^^    ^^^^^    ^^^
"#;

/// For when short syntax substitution group has too few or too many
/// substitutions
pub(crate) const SHORT_SYNTAX_SUBSTITUTION_COUNT: &'static str = r#"Hint: Number of substitutions must match the number of substitutions identifiers.
Example:
    ident1 ident2;
   1^^^^^^ ^^^^^^2
    [sub1] [sub2];
   1^^^^^^ ^^^^^^2
"#;
