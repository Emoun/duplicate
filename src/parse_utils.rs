use proc_macro::{Delimiter, Group, Punct, Spacing, Span, TokenTree};
use std::iter::Peekable;

/// Creates an error for checking/parsing groups.
///
/// Always returns Err(..) with an error message composed of the prefix,
/// expected delimiter span, and given hints.
pub fn group_err<T>(
	span: Span,
	prefix: &str,
	expected: Delimiter,
	hints: &str,
) -> Result<T, (Span, String)>
{
	Err((
		span,
		format!(
			"{}\nExpected '{}'.\n{}",
			prefix,
			match expected
			{
				Delimiter::Brace => '{',
				Delimiter::Bracket => '[',
				Delimiter::Parenthesis => '(',
				_ => unreachable!("Shouldn't expect None delimiters"),
			},
			hints
		),
	))
}

/// Tries to parse a valid group with the given delimiter from the given token
/// stream iterator, returning the group if successful.
///
/// If the next token is not a valid group, issues an error, that indicates to
/// the given span and adding the given string to the end of the message.
///
/// Always consumes a token from the iterator
pub fn parse_group(
	iter: &mut Peekable<impl Iterator<Item = TokenTree>>,
	del: Delimiter,
	parent_span: Span,
	hints: &str,
) -> Result<Group, (Span, String)>
{
	let result = peek_parse_group(iter, del, parent_span, hints);

	if result.is_ok()
	{
		let _ = iter.next();
	}
	result
}

pub fn peek_parse_group(
	iter: &mut Peekable<impl Iterator<Item = TokenTree>>,
	del: Delimiter,
	parent_span: Span,
	hints: &str,
) -> Result<Group, (Span, String)>
{
	if let Some(tree) = iter.peek()
	{
		if let TokenTree::Group(group) = tree
		{
			check_delimiter(&group, del)?;
			Ok(group.clone())
		}
		else
		{
			group_err(tree.span(), "Not a group.", del, hints)
		}
	}
	else
	{
		group_err(
			parent_span,
			"Unexpected end of macro invocation.",
			del,
			hints,
		)
	}
}

/// Ensures the given token is a valid group with the given delimiter and if so,
/// returns it.
///
/// If not, issues an error, adding the given hints to the error message.
pub fn check_group(tree: TokenTree, del: Delimiter, hints: &str) -> Result<Group, (Span, String)>
{
	if let TokenTree::Group(group) = tree
	{
		check_delimiter(&group, del)?;
		Ok(group)
	}
	else
	{
		group_err(tree.span(), "Not a group.", del, hints)
	}
}

/// Checks that the given group's delimiter is the given one.
///
/// If not, returns an error.
pub fn check_delimiter(group: &Group, del: Delimiter) -> Result<(), (Span, String)>
{
	if group.delimiter() != del
	{
		group_err(group.span(), "Unexpected delimiter for group.", del, "")
	}
	else
	{
		Ok(())
	}
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
/// See `peek_next_token` for how the next token is found.
///
/// Upon success, a token is consumed from the iterator.
/// If an error is returned, no token is consumed.
pub fn next_token(
	iter: &mut Peekable<impl Iterator<Item = TokenTree>>,
	parent_span: Span,
	expected: &str,
) -> Result<TokenTree, (Span, String)>
{
	let result = peek_next_token(iter, parent_span, expected);
	if result.is_ok()
	{
		let _ = iter.next();
	}
	result
}

/// Gets the next token tree from the iterator without consuming anything.
///
/// The first argument is the input, the second argument is the span to use in
/// case of an error, and the third argument is a description of what parsing
/// expected to find at the point of error.
///
/// If the token is a group without delimiters, the token inside the groups is
/// returned. If the group has more than one token, an error is returned.
pub fn peek_next_token(
	iter: &mut Peekable<impl Iterator<Item = TokenTree>>,
	parent_span: Span,
	expected: &str,
) -> Result<TokenTree, (Span, String)>
{
	let make_err = |span, msg| Err((span, format!("{}\nExpected: {}", msg, expected)));
	if let Some(token) = iter.peek()
	{
		match token
		{
			TokenTree::Group(group) if group.delimiter() == Delimiter::None =>
			{
				let mut in_group = group.stream().into_iter();
				if let Some(result) = in_group.next()
				{
					match (in_group.next(), in_group.next())
					{
						(None, _) => Ok(result),
						// If ends with ';' and nothing else, was a statement including
						// only 1 token, so allow.
						(Some(TokenTree::Punct(ref p)), None) if is_semicolon(&p) => Ok(result),
						_ =>
						{
							make_err(
								token.span(),
								"Encountered none-delimited group with multiple tokens. This is \
								 an internal error. Please file a bug report.",
							)
						},
					}
				}
				else
				{
					make_err(
						token.span(),
						"Encountered none-delimited group with no tokens. This is an internal \
						 error. Please file a bug report.",
					)
				}
			},
			token => Ok(token.clone()),
		}
	}
	else
	{
		make_err(parent_span, "Unexpected end of macro invocation.")
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
