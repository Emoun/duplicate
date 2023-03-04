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

/// For when verbose syntax substitution group has too few or too many
/// substitutions
pub(crate) const VERBOSE_SYNTAX_SUBSTITUTION_IDENTIFIERS: &'static str = r#"Hint: All substitution groups must define the same substitution identifiers.
Example:
    [
        ident1  [sub1]
        ident2  [sub2]
    ]
    [
        ident1  [sub3]
        ident2  [sub4]
    ]
"#;

/// For when verbose syntax substitution identifier has too few or too many
/// arguments
pub(crate) const VERBOSE_SYNTAX_SUBSTITUTION_IDENTIFIERS_ARGS: &'static str = r#"Hint: The same substitution identifier must take the same number of argument across all substitution groups.
Example:
    [
        ident1(arg1, arg2)  [sub1 arg1 arg2]
               ^^^^^^^^^^
    ]
    [
        ident1(arg1, arg2)  [arg1 arg2 sub2]
               ^^^^^^^^^^
    ]
"#;
