use proc_macro::{Delimiter, Group, Span, TokenStream, TokenTree};
use proc_macro_error::{proc_macro::Spacing, *};
use std::collections::{HashMap, HashSet};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn duplicate(attr: TokenStream, item: TokenStream) -> TokenStream
{
	match duplicate_impl(attr, item, false)
	{
		Ok(result) => result,
		Err(err) => abort!(err.0, err.1),
	}
}

/// Implements the macro.
///
/// `allow_short`: If true, accepts short syntax
fn duplicate_impl(
	attr: TokenStream,
	item: TokenStream,
	disallow_short: bool,
) -> Result<TokenStream, (Span, String)>
{
	let subs = parse_attr(attr, disallow_short)?;
	let result = substitute(item, subs);
	Ok(result)
}

fn parse_attr(
	attr: TokenStream,
	disallow_short: bool,
) -> Result<Vec<HashMap<String, TokenStream>>, (Span, String)>
{
	if identify_syntax(attr.clone(), disallow_short)?
	{
		validate_verbose_attr(attr)
	}
	else
	{
		let valid = validate_short_attr(attr)?;
		let mut reorder = Vec::new();
		let substitutions = valid;

		for _ in 0..substitutions[0].1.len()
		{
			reorder.push(HashMap::new());
		}

		for (ident, subs) in substitutions
		{
			for (idx, sub) in subs.into_iter().enumerate()
			{
				reorder[idx].insert(ident.clone(), sub);
			}
		}

		Ok(reorder)
	}
}

/// True is verbose, false is short
fn identify_syntax(attr: TokenStream, disallow_short: bool) -> Result<bool, (Span, String)>
{
	if let Some(token) = attr.into_iter().next()
	{
		match token
		{
			TokenTree::Group(group) =>
			{
				if Delimiter::None == group.delimiter()
				{
					Err((
						Span::call_site(),
						"Expected group in delimiters, got group without.".into(),
					))
				}
				else
				{
					Ok(true)
				}
			},
			TokenTree::Ident(_) if !disallow_short => Ok(false),
			TokenTree::Punct(p) if p.as_char() == '#' && p.spacing() == Spacing::Alone => Ok(true),
			_ if disallow_short =>
			{
				Err((
					token.span(),
					"Expected substitution group (Short syntax is disallowed at this level). \
					 Received neither."
						.into(),
				))
			},
			_ =>
			{
				Err((
					token.span(),
					"Expected substitution identifier or group. Received neither.".into(),
				))
			},
		}
	}
	else
	{
		Err((Span::call_site(), "No substitutions found.".into()))
	}
}

fn validate_verbose_attr(
	attr: TokenStream,
) -> Result<Vec<HashMap<String, TokenStream>>, (Span, String)>
{
	if attr.is_empty()
	{
		return Err((Span::call_site(), "No substitutions found.".into()));
	}

	let mut sub_groups = Vec::new();
	let mut iter = attr.into_iter();

	let mut substitution_ids = None;
	loop
	{
		if let Some(tree) = iter.next()
		{
			match tree
			{
				TokenTree::Punct(p) if p.as_char() == '#' && p.spacing() == Spacing::Alone =>
				{
					if let Some(tree) = iter.next()
					{
						if let TokenTree::Group(group) = tree
						{
							if group.delimiter() == Delimiter::None
							{
								return Err((
									group.span(),
									"Expected group within delimiters. Got group without \
									 delimiters."
										.into(),
								));
							}

							let nested_subs = parse_attr(group.stream(), false)?;

							if let Some(tree) = iter.next()
							{
								if let TokenTree::Group(group) = tree
								{
									if group.delimiter() == Delimiter::None
									{
										return Err((
											group.span(),
											"Expected group within delimiters. Got group without \
											 delimiters."
												.into(),
										));
									}

									let nested_duplicated = substitute(group.stream(), nested_subs);
									let subs = validate_verbose_attr(nested_duplicated)?;
									sub_groups.extend(subs.into_iter());
								}
								else
								{
									return Err((
										group.span(),
										"Nested macro invocation must be followed by group to \
										 duplicate. Did not receive a group."
											.into(),
									));
								}
							}
							else
							{
								return Err((
									group.span(),
									"Expected nested macro invocation to be followed by group to \
									 duplicate."
										.into(),
								));
							}
						}
						else
						{
							return Err((
								tree.span(),
								"Expected nested macro invocation in group. Did not get a group."
									.into(),
							));
						}
					}
					else
					{
						return Err((
							p.span(),
							"'#' must be followed by a nested macro invocation.".into(),
						));
					}
				},
				_ =>
				{
					sub_groups.push(extract_verbose_substitutions(tree, &substitution_ids)?);
					if None == substitution_ids
					{
						substitution_ids = Some(sub_groups[0].keys().cloned().collect())
					}
				},
			}
		}
		else
		{
			break;
		}
	}

	Ok(sub_groups)
}

fn extract_verbose_substitutions(
	tree: TokenTree,
	existing: &Option<HashSet<String>>,
) -> Result<HashMap<String, TokenStream>, (Span, String)>
{
	// Must get span now, before it's corrupted.
	let tree_span = tree.span();
	if let TokenTree::Group(group) = tree
	{
		if Delimiter::None == group.delimiter()
		{
			panic!("Expected group in delimiters, got group without.");
		}

		if group.stream().into_iter().count() == 0
		{
			return Err((tree_span, "No substitution groups found.".into()));
		}

		let mut substitutions = HashMap::new();
		let mut stream = group.stream().into_iter();

		loop
		{
			if let Some(ident) = stream.next()
			{
				let sub = stream.next().ok_or((
					tree_span,
					"Unexpected end of substitution group. Substitution identifier must be \
					 followed by the substitute as a delimited group."
						.into(),
				))?;

				if let TokenTree::Ident(ident) = ident
				{
					if let TokenTree::Group(sub) = sub
					{
						if Delimiter::None == sub.delimiter()
						{
							panic!(
								"Expected substituion group using delimiters, got group without."
							);
						}

						let ident_string = ident.to_string();

						// Check have found the same as existing
						if let Some(idents) = existing
						{
							if !idents.contains(&ident_string)
							{
								return Err((
									ident.span(),
									"Unfamiliar substitution identifier. '{}' is not present in \
									 previous substitution groups."
										.into(),
								));
							}
						}
						substitutions.insert(ident_string, sub.stream());
					}
					else
					{
						return Err((
							sub.span(),
							format!("Expected substitution as a delimited group. E.g. [ .. ]."),
						));
					}
				}
				else
				{
					return Err((
						ident.span(),
						"Expected substitution identifier, got something else.".into(),
					));
				}
			}
			else
			{
				// Check no substitution idents are missing.
				if let Some(idents) = existing
				{
					let sub_idents = substitutions.keys().cloned().collect();
					let diff: Vec<_> = idents.difference(&sub_idents).collect();

					if diff.len() > 0
					{
						let mut msg: String = "Missing substitutions. Previous substitutions \
						                       groups had the following identifiers not present \
						                       in this group: "
							.into();
						for ident in diff
						{
							msg.push_str("'");
							msg.push_str(&ident.to_string());
							msg.push_str("' ");
						}

						return Err((tree_span, msg));
					}
				}
				break;
			}
		}
		Ok(substitutions)
	}
	else
	{
		Err((
			tree_span,
			format!("Expected substitution group, got: {}", tree),
		))
	}
}

fn validate_short_attr(attr: TokenStream)
	-> Result<Vec<(String, Vec<TokenStream>)>, (Span, String)>
{
	if attr.is_empty()
	{
		return Err((Span::call_site(), "No substitutions found.".into()));
	}

	let mut result: Vec<(String, Vec<TokenStream>)> = Vec::new();
	let mut iter = attr.into_iter();
	let mut next_token = iter.next();
	loop
	{
		if let Some(ident) = next_token
		{
			next_token = iter.next();
			if let TokenTree::Ident(ident) = ident
			{
				let mut substitutions = Vec::new();
				loop
				{
					if let Some(TokenTree::Group(group)) = next_token
					{
						next_token = iter.next();

						if Delimiter::None == group.delimiter()
						{
							return Err((
								group.span(),
								"Expected substitution in delimiters, got group without \
								 delimiters."
									.into(),
							));
						}

						substitutions.push(group.stream());
					}
					else
					{
						break;
					}
				}
				if substitutions.len() == 0
				{
					return Err((
						ident.span(),
						"Expected substitution identifier to be followed by at least one \
						 substitution."
							.into(),
					));
				}
				if !result.is_empty() && (result[0].1.len() != substitutions.len())
				{
					return Err((
						ident.span(),
						format!(
							"Unexpected number of substitutions for identifier. Expected {}, was \
							 {}.",
							result[0].1.len(),
							substitutions.len()
						),
					));
				}

				result.push((ident.to_string(), substitutions));
			}
			else
			{
				return Err((ident.span(), "Expected substitution identifier.".into()));
			}
		}
		else
		{
			break;
		}
	}

	Ok(result)
}

fn substitute(item: TokenStream, groups: Vec<HashMap<String, TokenStream>>) -> TokenStream
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
