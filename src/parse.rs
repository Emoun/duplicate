use crate::{
	parse_utils::*,
	substitute::{substitute, Substitution},
	SubstitutionGroup,
};
use proc_macro::{token_stream::IntoIter, Ident, Span, TokenStream, TokenTree};
use std::{collections::HashSet, iter::Peekable};

/// Parses the attribute part of an invocation of duplicate, returning
/// all the substitutions that should be made to the item.
pub(crate) fn parse_attr(
	attr: TokenStream,
	stream_span: Span,
) -> Result<Vec<SubstitutionGroup>, (Span, String)>
{
	if identify_syntax(attr.clone(), stream_span)?
	{
		validate_verbose_attr(attr)
	}
	else
	{
		let substitutions = validate_short_attr(attr)?;
		let mut reorder = Vec::new();

		for _ in 0..substitutions[0].2.len()
		{
			reorder.push(SubstitutionGroup::new());
		}

		for (ident, args, subs) in substitutions
		{
			for (idx, sub) in subs.into_iter().enumerate()
			{
				let substitution = Substitution::new(&args, sub.into_iter());
				if let Ok(substitution) = substitution
				{
					reorder[idx].add_substitution(
						Ident::new(&ident.clone(), Span::call_site()),
						substitution,
					)?;
				}
				else
				{
					return Err((Span::call_site(), "Failed creating substitution".into()));
				}
			}
		}
		Ok(reorder)
	}
}

/// True is verbose, false is short
fn identify_syntax(attr: TokenStream, stream_span: Span) -> Result<bool, (Span, String)>
{
	if let Some(token) = next_token(&mut attr.into_iter(), "Could not identify syntax type.")?
	{
		match &token
		{
			TokenTree::Group(_) => Ok(true),
			TokenTree::Ident(_) => Ok(false),
			TokenTree::Punct(p) if is_nested_invocation(p) => Ok(true),
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
		Err((stream_span, "No substitutions found.".into()))
	}
}

/// Validates that the attribute part of a duplicate invocation uses
/// the verbose syntax, and returns all the substitutions that should be made.
fn validate_verbose_attr(attr: TokenStream) -> Result<Vec<SubstitutionGroup>, (Span, String)>
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
		if let Some(tree) = next_token(&mut iter, "Expected substitution group.")?
		{
			match &tree
			{
				TokenTree::Punct(p) if is_nested_invocation(p) =>
				{
					let nested_duplicated = invoke_nested(&mut iter, p.span())?;
					let subs = validate_verbose_attr(nested_duplicated)?;
					sub_groups.extend(subs.into_iter());
				},
				_ =>
				{
					sub_groups.push(extract_verbose_substitutions(tree, &substitution_ids)?);
					if None == substitution_ids
					{
						substitution_ids = Some(
							sub_groups[0]
								.identifiers_with_args()
								.map(|(ident, count)| (ident.clone(), count))
								.collect(),
						)
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

/// Extracts a substitution group in the verbose syntax.
fn extract_verbose_substitutions(
	tree: TokenTree,
	existing: &Option<HashSet<(String, usize)>>,
) -> Result<SubstitutionGroup, (Span, String)>
{
	// Must get span now, before it's corrupted.
	let tree_span = tree.span();
	let group = check_group(
		tree,
		"Hint: When using verbose syntax, a substitutions must be enclosed in a \
		 group.\nExample:\n..\n[\n\tidentifier1 [ substitution1 ]\n\tidentifier2 [ substitution2 \
		 ]\n]",
	)?;

	if group.stream().into_iter().count() == 0
	{
		return Err((group.span(), "No substitution groups found.".into()));
	}

	let mut substitutions = SubstitutionGroup::new();
	let mut stream = group.stream().into_iter().peekable();

	loop
	{
		if let Some(ident) = next_token(&mut stream, "Expected substitution identifier.")?
		{
			if let TokenTree::Ident(ident) = ident
			{
				let group_1 = parse_group(
					&mut stream,
					ident.span(),
					"Hint: A substitution identifier should be followed by a group containing the \
					 code to be inserted instead of any occurrence of the identifier.",
				)?;

				let group_2 = if let Some(TokenTree::Group(group)) = stream.peek()
				{
					if check_delimiter(&group).is_ok()
					{
						stream.next()
					}
					else
					{
						None
					}
				}
				else
				{
					None
				};

				let substitution = if let Some(TokenTree::Group(sub)) = group_2
				{
					let args = extract_argument_list(&group_1)?;
					Substitution::new(&args, sub.stream().into_iter()).unwrap()
				}
				else
				{
					Substitution::new_simple(group_1.stream())
				};

				substitutions.add_substitution(ident, substitution)?;
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
			// Check no substitution idents are missing or with wrong argument counts.
			if let Some(idents) = existing
			{
				let sub_idents: HashSet<_> = substitutions.identifiers_with_args().collect();
				// Map idents to string reference so we can use HashSet::difference
				let idents = idents
					.iter()
					.map(|(ident, count)| (ident, count.clone()))
					.collect();
				let diff: Vec<_> = sub_idents.difference(&idents).collect();

				if diff.len() > 0
				{
					let mut msg: String = "Invalid substitutions.\nThe following identifiers were \
					                       not found in previous substitution groups or had \
					                       different arguments:\n"
						.into();
					for ident in diff
					{
						msg.push_str(&ident.0.to_string());
						msg.push_str("(");
						if ident.1 > 0
						{
							msg.push_str("_");
						}
						for _ in 1..(ident.1)
						{
							msg.push_str(",_")
						}
						msg.push_str(")");
					}

					return Err((tree_span, msg));
				}
			}
			break;
		}
	}
	Ok(substitutions)
}

/// Validates that the attribute part of a duplicate invocation uses
/// the short syntax and returns the substitution that should be made.
fn validate_short_attr(
	attr: TokenStream,
) -> Result<Vec<(String, Vec<String>, Vec<TokenStream>)>, (Span, String)>
{
	if attr.is_empty()
	{
		return Err((Span::call_site(), "No substitutions found.".into()));
	}

	let mut iter = attr.into_iter();
	let (idents, span) = validate_short_get_identifiers(&mut iter, Span::call_site())?;
	let mut result: Vec<_> = idents
		.into_iter()
		.map(|(ident, args)| (ident, args, Vec::new()))
		.collect();
	validate_short_get_all_substitution_goups(iter, span, &mut result)?;

	Ok(result)
}

/// Assuming use of the short syntax, gets the initial list of substitution
/// identifiers.
fn validate_short_get_identifiers(
	iter: &mut IntoIter,
	mut span: Span,
) -> Result<(Vec<(String, Vec<String>)>, Span), (Span, String)>
{
	let mut iter = iter.peekable();
	let mut result = Vec::new();
	loop
	{
		if let Some(next_token) = next_token(&mut iter, "Expected substitution identifier or ';'.")?
		{
			span = next_token.span();
			match &next_token
			{
				TokenTree::Ident(ident) =>
				{
					result.push((
						ident.to_string(),
						validate_short_get_identifier_arguments(&mut iter)?, // Vec::new()
					))
				},
				TokenTree::Punct(p) if is_semicolon(&p) => break,
				_ => return Err((span, "Expected substitution identifier or ';'.".into())),
			}
		}
		else
		{
			return Err((span, "Expected substitution identifier or ';'.".into()));
		}
	}
	Ok((result, span))
}

/// Assuming use of the short syntax, gets the list of identifier arguments.
fn validate_short_get_identifier_arguments(
	iter: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<Vec<String>, (Span, String)>
{
	if let Some(token) = iter.peek()
	{
		if let TokenTree::Group(group) = token
		{
			if check_delimiter(group).is_ok()
			{
				let result = extract_argument_list(group)?;
				// Make sure to consume the group
				let _ = iter.next();
				return Ok(result);
			}
		}
	}
	Ok(Vec::new())
}

/// Gets all substitution groups in the short syntax and inserts
/// them into the given vec.
fn validate_short_get_all_substitution_goups<'a>(
	iter: impl Iterator<Item = TokenTree>,
	mut span: Span,
	result: &mut Vec<(String, Vec<String>, Vec<TokenStream>)>,
) -> Result<(), (Span, String)>
{
	let mut iter = iter.peekable();
	loop
	{
		if let Some(TokenTree::Punct(p)) = iter.peek()
		{
			if is_nested_invocation(&p)
			{
				let p_span = p.span();
				// consume '#'
				iter.next();

				let nested_duplicated = invoke_nested(&mut iter, p_span)?;
				validate_short_get_all_substitution_goups(
					&mut nested_duplicated.into_iter(),
					span.clone(),
					result,
				)?;
			}
		}
		else
		{
			validate_short_get_substitutions(
				&mut iter,
				span,
				result.iter_mut().map(|(_, _, vec)| {
					vec.push(TokenStream::new());
					vec.last_mut().unwrap()
				}),
			)?;

			if let Some(token) = iter.next()
			{
				span = token.span();
				if let TokenTree::Punct(p) = token
				{
					if is_semicolon(&p)
					{
						continue;
					}
				}
				return Err((span, "Expected ';'.".into()));
			}
			else
			{
				break;
			}
		}
	}
	Ok(())
}

/// Extracts a substitution group in the short syntax and inserts it into
/// the elements returned by the given groups iterator.
fn validate_short_get_substitutions<'a>(
	iter: &mut impl Iterator<Item = TokenTree>,
	mut span: Span,
	mut groups: impl Iterator<Item = &'a mut TokenStream>,
) -> Result<Span, (Span, String)>
{
	if let Some(token) = iter.next()
	{
		let group = check_group(token, "")?;
		span = group.span();
		*groups.next().unwrap() = group.stream();

		for stream in groups
		{
			let group = parse_group(iter, span, "")?;
			span = group.span();
			*stream = group.stream();
		}
	}
	Ok(span)
}

/// Invokes a nested invocation of duplicate, assuming the
/// next group is the attribute part of the invocation and the
/// group after that is the element.
fn invoke_nested(
	iter: &mut impl Iterator<Item = TokenTree>,
	span: Span,
) -> Result<TokenStream, (Span, String)>
{
	let hints = "Hint: '#' is a nested invocation of the macro and must therefore be followed by \
	             a group containing the invocation.\nExample:\n#[\n\tidentifier [ substitute1 ] [ \
	             substitute2 ]\n][\n\tCode to be substituted whenever 'identifier' occurs \n]";
	let nested_attr = parse_group(iter, span, hints)?;
	let nested_subs = parse_attr(nested_attr.stream(), nested_attr.span())?;

	let nested_item = parse_group(iter, nested_attr.span(), hints)?;
	Ok(substitute(nested_item.stream(), nested_subs))
}
