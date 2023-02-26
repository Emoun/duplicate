use crate::{
	error::Error,
	substitute::{duplicate_and_substitute, Substitution},
	token_iter::{get_ident, is_ident, is_semicolon, SubGroupIter, Token, TokenIter},
	DuplicationDefinition, Result, SubstitutionGroup,
};
use proc_macro::{Delimiter, Ident, Span, TokenStream, TokenTree};
use std::collections::HashSet;

/// Parses the invocation of duplicate, returning all the substitutions that
/// should be made to code.
///
/// If parsing succeeds returns first a substitution group that indicates global
/// substitutions that should be applied to all duplicates but don't on their
/// own indicate a duplicate. Then comes a list of substitution groups, each of
/// which indicates on duplicate.
pub(crate) fn parse_invocation(attr: TokenStream) -> Result<DuplicationDefinition>
{
	let empty_global = SubstitutionGroup::new();
	let mut iter = TokenIter::new(attr, &empty_global, std::iter::empty());
	let global_substitutions = validate_global_substitutions(&mut iter)?;

	// First try verbose syntax
	match validate_verbose_invocation(&mut iter, global_substitutions.substitutions.is_empty())
	{
		// Otherwise, try short syntax
		Err(_) =>
		{
			let substitutions = validate_short_attr(iter)?;
			let mut reorder = Vec::new();

			for _ in 0..substitutions[0].2.len()
			{
				reorder.push(SubstitutionGroup::new());
			}

			for (ident, args, subs) in substitutions
			{
				for (idx, sub) in subs.into_iter().enumerate()
				{
					let substitution = Substitution::new(
						&args,
						TokenIter::new(sub, &SubstitutionGroup::new(), std::iter::empty()),
					);
					if let Ok(substitution) = substitution
					{
						reorder[idx].add_substitution(
							Ident::new(&ident.clone(), Span::call_site()),
							substitution,
						)?;
					}
					else
					{
						return Err(Error::new(
							"Duplicate internal error: Failed at creating substitution",
						));
					}
				}
			}
			Ok(DuplicationDefinition {
				global_substitutions,
				duplications: reorder,
			})
		},
		verbose_result =>
		{
			verbose_result.map(|dups| {
				DuplicationDefinition {
					global_substitutions,
					duplications: dups,
				}
			})
		},
	}
}

/// Validates global substitutions and returns a substitution group with them.
///
/// When it fails to validate a global substitution, it might return the next
/// identifier that the iterator produced optionally followed by the next group
/// too. The two must therefore be assumed to precede any tokentrees returned by
/// the iterator after calling this function.
/// This may happen if the global substitutions are followed by short-syntax,
/// which starts the same way as a global substitution.
fn validate_global_substitutions<'a, T: SubGroupIter<'a>>(
	iter: &mut TokenIter<'a, T>,
) -> Result<SubstitutionGroup>
{
	let mut sub_group = SubstitutionGroup::new();
	while let Ok((ident, sub)) = extract_inline_substitution(iter)
	{
		sub_group.add_substitution(ident, sub)?;

		if iter.has_next()?
		{
			iter.expect_semicolon()?;
		}
	}
	Ok(sub_group)
}

/// Validates that a duplicate invocation uses the verbose syntax, and returns
/// all the substitutions that should be made.
///
/// If `err_on_no_subs` is true, if no substitutions are found, an error is
/// returned. Otherwise, no substitutions will results in Ok.
fn validate_verbose_invocation<'a, T: SubGroupIter<'a>>(
	iter: &mut TokenIter<'a, T>,
	err_on_no_subs: bool,
) -> Result<Vec<SubstitutionGroup>>
{
	if err_on_no_subs && !iter.has_next()?
	{
		return Err(Error::new("No substitutions found."));
	}

	let mut sub_groups = Vec::new();

	let mut substitution_ids = None;
	while iter.has_next()?
	{
		let (body, span) = iter.next_group(Some(Delimiter::Bracket)).map_err(|err| {
			err.hint(
				"Hint: When using verbose syntax, a substitutions must be enclosed in a \
				 group.\nExample:\n..\n[\n\tidentifier1 [ substitution1 ]\n\tidentifier2 [ \
				 substitution2 ]\n]",
			)
		})?;
		sub_groups.push(extract_verbose_substitutions(
			body,
			span,
			&substitution_ids,
		)?);
		if None == substitution_ids
		{
			substitution_ids = Some(
				sub_groups[0]
					.identifiers_with_args()
					.map(|(ident, count)| (ident.clone(), count))
					.collect(),
			)
		}
	}
	Ok(sub_groups)
}

/// Extracts a substitution identifier followed by
/// an optional parameter list, followed by a substitution.
fn extract_inline_substitution<'a, T: SubGroupIter<'a>>(
	stream: &mut TokenIter<'a, T>,
) -> Result<(Ident, Substitution)>
{
	let ident = stream.extract_identifier(Some("substitution identifier"))?;
	let param_group = stream.next_group(Some(Delimiter::Parenthesis));
	let substitution = stream.next_group(Some(Delimiter::Bracket)).map_err(|err| {
		err.hint(
			"Hint: A substitution identifier should be followed by a group containing the code to \
			 be inserted instead of any occurrence of the identifier.",
		)
	});

	if let Ok((params, span)) = param_group
	{
		// Found parameters, now get substitution
		substitution
			.and_then(|(sub, _)| {
				extract_argument_list(params.clone())
					.map(|args| Substitution::new(&args, sub).unwrap())
					.or_else(|err| Err(err))
			})
			.or_else(|err| {
				stream.push_front(Token::Group(Delimiter::Parenthesis, params, span));
				Err(err)
			})
	}
	else
	{
		// No parameters, get substitution
		substitution.map(|(sub, _)| Substitution::new_simple(sub.process_all()))
	}
	.or_else(|err| {
		stream.push_front(Token::Simple(TokenTree::Ident(ident.clone())));
		Err(err)
	})
	.map(|result| (ident, result))
}

/// Extracts a substitution group in the verbose syntax.
fn extract_verbose_substitutions<'a, T: SubGroupIter<'a>>(
	mut iter: TokenIter<'a, T>,
	iter_span: Span,
	existing: &Option<HashSet<(String, usize)>>,
) -> Result<SubstitutionGroup>
{
	if !iter.has_next()?
	{
		return Err(Error::new("No substitution groups found.").span(iter_span));
	}

	let mut substitutions = SubstitutionGroup::new();
	let mut stream = iter;

	while let Ok((ident, substitution)) = extract_inline_substitution(&mut stream)
	{
		substitutions.add_substitution(ident, substitution)?;
	}
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
			let mut msg: String = "Invalid substitutions.\nThe following identifiers were not \
			                       found in previous substitution groups or had different \
			                       arguments:\n"
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
			return Err(Error::new(msg).span(iter_span));
		}
	}
	Ok(substitutions)
}

/// Validates a duplicate invocation using the short syntax and returns the
/// substitution that should be made.
fn validate_short_attr<'a, T: SubGroupIter<'a>>(
	mut iter: TokenIter<'a, T>,
) -> Result<Vec<(String, Vec<String>, Vec<TokenStream>)>>
{
	let idents = validate_short_get_identifiers(&mut iter)?;
	let mut result: Vec<_> = idents
		.into_iter()
		.map(|(ident, args)| (ident, args, Vec::new()))
		.collect();
	validate_short_get_all_substitution_goups(iter, &mut result)?;

	Ok(result)
}

/// Assuming use of the short syntax, gets the initial list of substitution
/// identifiers.
fn validate_short_get_identifiers<'a, T: SubGroupIter<'a>>(
	mut iter: &mut TokenIter<'a, T>,
) -> Result<Vec<(String, Vec<String>)>>
{
	let mut result = Vec::new();
	while let Some(ident) = iter.extract_simple(
		|t| is_ident(t, None) || is_semicolon(t),
		|t| get_ident(t),
		Some("substitution identifier or ';'"),
	)?
	{
		result.push((
			ident.to_string(),
			validate_short_get_identifier_arguments(&mut iter)?,
		));
	}
	Ok(result)
}

/// Assuming use of the short syntax, gets the list of identifier arguments.
fn validate_short_get_identifier_arguments<'a, T: SubGroupIter<'a>>(
	iter: &mut TokenIter<'a, T>,
) -> Result<Vec<String>>
{
	if let Ok((group, _)) = iter.next_group(Some(Delimiter::Parenthesis))
	{
		let result = extract_argument_list(group)?;
		return Ok(result);
	}
	Ok(Vec::new())
}

/// Gets all substitution groups in the short syntax and inserts
/// them into the given vec.
fn validate_short_get_all_substitution_goups<'a, T: SubGroupIter<'a>>(
	mut iter: TokenIter<'a, T>,
	result: &mut Vec<(String, Vec<String>, Vec<TokenStream>)>,
) -> Result<()>
{
	while iter.has_next()?
	{
		for (_, _, streams) in result.iter_mut()
		{
			let (group, _) = iter.next_group(Some(Delimiter::Bracket))?;
			streams.push(group.to_token_stream());
		}

		if iter.has_next()?
		{
			iter.expect_semicolon()?;
		}
	}
	Ok(())
}

/// Invokes a nested invocation of duplicate, assuming the
/// next group is the body of call to `duplicate`
pub(crate) fn invoke_nested<'a, T: SubGroupIter<'a>>(
	iter: &mut TokenIter<'a, T>,
) -> Result<TokenStream>
{
	let (mut nested_body_iter, _) = iter.next_group(None)?;

	let (nested_invocation, _) = nested_body_iter.next_group(Some(Delimiter::Bracket))?;
	let nested_dup_def = parse_invocation(nested_invocation.to_token_stream())?;

	duplicate_and_substitute(
		nested_body_iter.to_token_stream(),
		&nested_dup_def.global_substitutions,
		nested_dup_def.duplications.iter(),
	)
}

/// Extracts a list of arguments from.
/// The list is expected to be of comma-separated identifiers.
pub(crate) fn extract_argument_list<'a, T: SubGroupIter<'a>>(
	mut args: TokenIter<'a, T>,
) -> Result<Vec<String>>
{
	let mut result = Vec::new();
	while args.has_next()?
	{
		let ident =
			args.extract_identifier(Some("substitution identifier argument as identifier"))?;
		result.push(ident.to_string());

		if args.has_next()?
		{
			args.expect_comma()?;
		}
	}
	Ok(result)
}
