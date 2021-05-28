use crate::{
	parse_utils::*,
	substitute::{duplicate_and_substitute, Substitution},
	DuplicationDefinition, SubstitutionGroup,
};
use proc_macro::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use std::{collections::HashSet, iter::Peekable};

/// Parses the invocation of duplicate, returning all the substitutions that
/// should be made to the item.
///
/// If parsing succeeds returns first a substitution group that indicates global
/// substitutions that should be applied to all duplicates but don't on their
/// own indicate a duplicate. Then comes a list of substitution groups, each of
/// which indicates on duplicate.
pub(crate) fn parse_invocation(attr: TokenStream) -> Result<DuplicationDefinition, (Span, String)>
{
	let mut iter = attr.into_iter().peekable();

	let (global_substitutions, extra) = validate_global_substitutions(&mut iter)?;
	match extra
	{
		None =>
		{
			validate_verbose_invocation(iter, global_substitutions.substitutions.is_empty()).map(
				|dups| {
					DuplicationDefinition {
						global_substitutions,
						duplications: dups,
					}
				},
			)
		},
		Some((ident, group)) =>
		{
			let substitutions = validate_short_attr(
				Some(TokenTree::Ident(ident))
					.into_iter()
					.chain(group.map(|g| TokenTree::Group(g)).into_iter())
					.chain(iter),
			)?;
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
			Ok(DuplicationDefinition {
				global_substitutions,
				duplications: reorder,
			})
		},
	}
}

/// Validates global substitutions and returns a substitution group with them.
///
/// When it fails to validate a global substitution, it might return the next
/// identifier that the iterator produced optionally followed by the next group
/// too. The two must therefore be assumed to precede any tokentrees returned by
/// the iterator and calling this function.
/// This may happen if the global substitutions are followed by short-syntax,
/// which starts the same way as a global substitution.
fn validate_global_substitutions(
	iter: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<(SubstitutionGroup, Option<(Ident, Option<Group>)>), (Span, String)>
{
	let mut sub_group = SubstitutionGroup::new();
	loop
	{
		match extract_inline_substitution(iter)
		{
			Ok((ident, Ok(sub))) => sub_group.add_substitution(ident, sub)?,
			Ok((ident, Err(group))) => return Ok((sub_group, Some((ident, group)))),
			_ => break,
		}

		match iter.peek()
		{
			Some(TokenTree::Punct(p)) if is_semicolon(p) =>
			{
				let _ = iter.next();
			},
			Some(t) => return Err((t.span(), "Expected ';'.".into())),
			_ => break,
		}
	}
	Ok((sub_group, None))
}

/// Validates that a duplicate invocation uses the verbose syntax, and returns
/// all the substitutions that should be made.
fn validate_verbose_invocation(
	iter: impl Iterator<Item = TokenTree>,
	err_on_no_subs: bool,
) -> Result<Vec<SubstitutionGroup>, (Span, String)>
{
	let mut iter = iter.peekable();
	if err_on_no_subs && iter.peek().is_none()
	{
		return Err((Span::call_site(), "No substitutions found.".into()));
	}

	let mut sub_groups = Vec::new();

	let mut substitution_ids = None;
	let mut err_span = Span::call_site();
	loop
	{
		if let Ok(tree) = next_token(&mut iter, err_span, "Substitution group")
		{
			err_span = tree.span();
			match &tree
			{
				TokenTree::Punct(p) if is_nested_invocation(p) =>
				{
					let nested_duplicated = invoke_nested(&mut iter, p.span())?;
					let subs = validate_verbose_invocation(nested_duplicated.into_iter(), true)?;
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

/// Extracts an inline substitution, i.e. a substitution identifier followed by
/// an optional parameter list, followed by a substitution.
///
/// If a substitution identifier is encountered but not the rest of the
/// substitution, the identifier is returned on its own.
fn extract_inline_substitution(
	stream: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<(Ident, Result<Substitution, Option<Group>>), (Span, String)>
{
	let token = peek_next_token(stream, Span::call_site(), "Substitution identifier")?;

	if let TokenTree::Ident(ident) = token
	{
		let _ = stream.next();
		let group_1 = peek_parse_group(stream, Delimiter::Parenthesis, ident.span(), "");

		if let Ok(params) = group_1
		{
			let _ = stream.next();
			parse_group(
				stream,
				Delimiter::Bracket,
				ident.span(),
				"Hint: A substitution identifier should be followed by a group containing the \
				 code to be inserted instead of any occurrence of the identifier.",
			)
			.and_then(|sub| {
				extract_argument_list(&params)
					.map(|args| Ok(Substitution::new(&args, sub.stream().into_iter()).unwrap()))
					.or_else(|err| Err(err))
			})
			.or_else(|_| Ok(Err(Some(params))))
		}
		else
		{
			parse_group(
				stream,
				Delimiter::Bracket,
				ident.span(),
				"Hint: A substitution identifier should be followed by a group containing the \
				 code to be inserted instead of any occurrence of the identifier.",
			)
			.map(|sub| Ok(Substitution::new_simple(sub.stream())))
			.or_else(|_| Ok(Err(None)))
		}
		.map(|result| (ident, result))
	}
	else
	{
		Err((token.span(), "Expected substitution identifier.".into()))
	}
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
		Delimiter::Bracket,
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
		if let Ok((ident, Ok(substitution))) = extract_inline_substitution(&mut stream)
		{
			substitutions.add_substitution(ident, substitution)?;
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

/// Validates a duplicate invocation using the short syntax and returns the
/// substitution that should be made.
fn validate_short_attr(
	iter: impl Iterator<Item = TokenTree>,
) -> Result<Vec<(String, Vec<String>, Vec<TokenStream>)>, (Span, String)>
{
	let mut iter = iter.peekable();

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
	iter: &mut impl Iterator<Item = TokenTree>,
	mut span: Span,
) -> Result<(Vec<(String, Vec<String>)>, Span), (Span, String)>
{
	let mut iter = iter.peekable();
	let mut result = Vec::new();
	loop
	{
		let next_token = next_token(&mut iter, span, "Substitution identifier or ';'")?;
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
			if check_delimiter(group, Delimiter::Parenthesis).is_ok()
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
/// the elements returned by the given group's iterator.
fn validate_short_get_substitutions<'a>(
	iter: &mut Peekable<impl Iterator<Item = TokenTree>>,
	mut span: Span,
	mut groups: impl Iterator<Item = &'a mut TokenStream>,
) -> Result<Span, (Span, String)>
{
	if let Some(token) = iter.next()
	{
		let group = check_group(token, Delimiter::Bracket, "")?;
		span = group.span();
		*groups.next().unwrap() = group.stream();

		for stream in groups
		{
			let group = parse_group(iter, Delimiter::Bracket, span, "")?;
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
	iter: &mut Peekable<impl Iterator<Item = TokenTree>>,
	span: Span,
) -> Result<TokenStream, (Span, String)>
{
	let hints = "Hint: '#' is a nested invocation of the macro and must therefore be followed by \
	             a group containing the invocation.\nExample:\n#[\n\tidentifier [ substitute1 ] [ \
	             substitute2 ]\n][\n\tCode to be substituted whenever 'identifier' occurs \n]";
	let nested_attr = parse_group(iter, Delimiter::Bracket, span, hints)?;
	let nested_dup_def = parse_invocation(nested_attr.stream())?;

	let nested_item = parse_group(iter, Delimiter::Bracket, nested_attr.span(), hints)?;
	duplicate_and_substitute(
		nested_item.stream(),
		&nested_dup_def.global_substitutions,
		nested_dup_def.duplications.iter(),
	)
}
