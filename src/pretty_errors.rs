/// For when substitution parameters aren't enclosed in brackets
pub(crate) const BRACKET_SUB_PARAM: &'static str = r#"
Substitution parameters should be enclosed in '[]' each.
Example:
    sub_ident( [ parameter1 ] , [ paramter2 ] )
              ^^^          ^^^ ^^^         ^^^
"#;
