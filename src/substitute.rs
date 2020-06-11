use proc_macro::{Group, TokenStream, TokenTree};
use std::collections::HashMap;

/// Duplicates the given token stream, substituting any identifiers found.
pub fn substitute(item: TokenStream, groups: Vec<HashMap<String, TokenStream>>) -> TokenStream
{
	let mut result = TokenStream::new();

	for substitutions in groups
	{
		for token in item.clone().into_iter()
		{
			result.extend(substitute_token_tree(token, &substitutions))
		}
	}

	result
}

/// Recursively checks the given token for any use of the given substitution
/// identifiers and substitutes them, returning the resulting token stream.
fn substitute_token_tree(
	tree: TokenTree,
	subtitutions: &HashMap<String, TokenStream>,
) -> TokenStream
{
	let mut result = TokenStream::new();
	match tree
	{
		TokenTree::Ident(ident) =>
		{
			if let Some(stream) = subtitutions.get(&ident.to_string())
			{
				result.extend(stream.clone().into_iter());
			}
			else
			{
				result.extend(TokenStream::from(TokenTree::Ident(ident)).into_iter());
			}
		},
		TokenTree::Group(group) =>
		{
			let mut substituted = TokenStream::new();
			for token in group.stream().into_iter()
			{
				substituted.extend(substitute_token_tree(token, subtitutions))
			}
			result.extend(
				TokenStream::from(TokenTree::Group(Group::new(group.delimiter(), substituted)))
					.into_iter(),
			);
		},
		_ => result.extend(TokenStream::from(tree).into_iter()),
	}
	result
}
