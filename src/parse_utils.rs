use proc_macro::{Delimiter, Group, Punct, Spacing, Span, TokenTree};

/// Tries to parse a valid group from the given token stream iterator, returning
/// the group if successful.
///
/// If the next token is not a valid group, issues an error, that indicates to
/// the given span and adding the given string to the end of the message.
pub fn parse_group(
	iter: &mut impl Iterator<Item = TokenTree>,
	parent_span: Span,
	hints: &str,
) -> Result<Group, (Span, String)>
{
	if let Some(tree) = iter.next()
	{
		check_group(tree, hints)
	}
	else
	{
		return Err((
			parent_span,
			"Unexpected end of macro invocation. Expected '[', '{', or '('.\n".to_string() + hints,
		));
	}
}

/// Ensures the given token is a valid group and if so, returns it.
///
/// If not, issues an error, adding the given hints to the error message.
pub fn check_group(tree: TokenTree, hints: &str) -> Result<Group, (Span, String)>
{
	if let TokenTree::Group(group) = tree
	{
		check_delimiter(&group)?;
		Ok(group)
	}
	else
	{
		return Err((
			tree.span(),
			"Unexpected token. Expected '[', '{', or '('.\n".to_string() + hints,
		));
	}
}

/// Checks that the given group's delimiter is a bracket ('[]','{}', or '()').
///
/// If so, returns the same group, otherwise issues an error.
pub fn check_delimiter(group: &Group) -> Result<(), (Span, String)>
{
	if group.delimiter() == Delimiter::None
	{
		return Err((
			group.span(),
			"Unexpected delimiter for group. Expected '[]','{}', or '()' but received none.".into(),
		));
	}
	Ok(())
}

/// Checks whether the given punctuation is exactly equal to the given
/// character.
pub fn punct_is_char(p: &Punct, c: char) -> bool
{
	p.as_char() == c && p.spacing() == Spacing::Alone
}

/// Check whether teh given punctuation is ';'.
pub fn is_semicolon(p: &Punct) -> bool
{
	punct_is_char(p, ';')
}

/// Checks whether the given punctuation is '#'.
pub fn is_nested_invocation(p: &Punct) -> bool
{
	punct_is_char(p, '#')
}

/// Gets the next token tree from the iterator.
///
/// If the token is a group without delimiters, the token inside the groups is
/// returned. If the group has more than one token, an error is returned.
pub fn next_token(
	iter: &mut impl Iterator<Item = TokenTree>,
	err_msg: &str,
) -> Result<Option<TokenTree>, (Span, String)>
{
	match iter.next()
	{
		Some(TokenTree::Group(ref group)) if group.delimiter() == Delimiter::None =>
		{
			let mut in_group = group.stream().into_iter();
			let result = in_group.next();
			match (in_group.next(), in_group.next())
			{
				(None, _) => Ok(result),
				// If ends with ';' and nothing else, was a statement including
				// only 1 token, so allow.
				(Some(TokenTree::Punct(ref p)), None) if is_semicolon(&p) => Ok(result),
				_ => Err((group.span(), err_msg.into())),
			}
		},
		token => Ok(token),
	}
}

/// Extracts a list of arguments from the given group.
/// The list is expected to be of comma-separated identifiers.
pub fn extract_argument_list(group: &Group) -> Result<Vec<String>, (Span, String)>
{
	let mut result = Vec::new();
	let mut arg_iter = group.stream().into_iter();
	while let Some(token) = arg_iter.next()
	{
		if let TokenTree::Ident(ident) = token
		{
			result.push(ident.to_string());
			if let Some(token) = arg_iter.next()
			{
				match &token
				{
					TokenTree::Punct(punct) if punct_is_char(&punct, ',') => (),
					_ => return Err((token.span(), "Expected ','.".into())),
				}
			}
		}
		else
		{
			return Err((
				token.span(),
				"Expected substitution identifier argument as identifier.".into(),
			));
		}
	}
	Ok(result)
}
