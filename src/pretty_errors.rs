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
