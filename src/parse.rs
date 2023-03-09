use crate::{
	error::Error,
	pretty_errors::{
		GLOBAL_SUB_SEMICOLON, NO_INVOCATION, SHORT_SYNTAX_NO_GROUPS,
		VERBOSE_SYNTAX_SUBSTITUTION_IDENTIFIERS, VERBOSE_SYNTAX_SUBSTITUTION_IDENTIFIERS_ARGS,
	},
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

	if let (Ok(None), false) = (iter.peek(), global_substitutions.substitutions.is_empty())
	{
		// Accept global substitutions on their own
		Ok(DuplicationDefinition {
			global_substitutions,
			duplications: Vec::new(),
		})
	}
	else if let Some(dups) = validate_verbose_invocation(&mut iter)?
	{
		Ok(DuplicationDefinition {
			global_substitutions,
			duplications: dups,
		})
	}
	else
	{
		// Otherwise, try short syntax
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
			iter.expect_semicolon()
				.map_err(|err| err.hint(GLOBAL_SUB_SEMICOLON))?;
		}
	}
	Ok(sub_group)
}

/// Validates that a duplicate invocation uses the verbose syntax, and returns
/// all the substitutions that should be made.
///
/// Returns 'Some' if the tokens given definitely represent the use of verbose
/// syntax, even though it might still contain errors.
/// Returns 'None' if an error occurred before verbose syntax was recognized
fn validate_verbose_invocation<'a, T: SubGroupIter<'a>>(
	iter: &mut TokenIter<'a, T>,
) -> Result<Option<Vec<SubstitutionGroup>>>
{
	if let Ok(Some(Token::Group(Delimiter::Bracket, _, _))) = iter.peek()
	{
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
		Ok(Some(sub_groups))
	}
	else
	{
		Ok(None)
	}
}

/// Extracts a substitution identifier followed by
/// an optional parameter list, followed by a substitution.
fn extract_inline_substitution<'a, T: SubGroupIter<'a>>(
	stream: &mut TokenIter<'a, T>,
) -> Result<(Ident, Substitution)>
{
	let ident = stream.extract_identifier(Some("a substitution identifier"))?;
	let param_group = stream.next_group(Some(Delimiter::Parenthesis));
	let substitution = stream.next_group(Some(Delimiter::Bracket));

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
		substitution
			.map(|(sub, _)| Substitution::new_simple(sub.process_all()))
			.map_err(|old_err| Error::new("Expected '(' or '['.").span(old_err.extract().0))
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

	// Map idents to string reference so we can use HashSet::difference
	let expected_idents: HashSet<_> = existing.as_ref().map_or(HashSet::new(), |idents| {
		idents
			.iter()
			.map(|(ident, count)| (ident, count.clone()))
			.collect()
	});

	let mut substitutions = SubstitutionGroup::new();
	let mut stream = iter;

	while stream.has_next()?
	{
		#[allow(unused_mut)]
		let mut hint: Option<&str> = None;

		#[cfg(feature = "pretty_errors")]
		{
			if stream.has_next_semicolon()?
			{
				hint = Some(crate::pretty_errors::VERBOSE_SEMICOLON);
			}
		}

		let (ident, substitution) = extract_inline_substitution(&mut stream)
			.map_err(|err| hint.into_iter().fold(err, |err, hint| err.hint(hint)))?;
		if !expected_idents.is_empty()
			&& !expected_idents.contains(&(&ident.to_string(), substitution.argument_count()))
		{
			let (msg, _hint) = if expected_idents
				.iter()
				.find(|(i, _)| **i == ident.to_string())
				.is_some()
			{
				(
					"Wrong argument count for substitution identifier.",
					VERBOSE_SYNTAX_SUBSTITUTION_IDENTIFIERS_ARGS,
				)
			}
			else
			{
				(
					"Unexpected substitution identifier.",
					VERBOSE_SYNTAX_SUBSTITUTION_IDENTIFIERS,
				)
			};
			return Err(Error::new(msg).span(ident.span()).hint(_hint));
		}
		substitutions.add_substitution(ident, substitution)?;
	}

	// Check no substitution idents are missing
	let found_idents: HashSet<_> = substitutions.identifiers_with_args().collect();
	let missing: Vec<_> = expected_idents.difference(&found_idents).collect();

	if missing.len() > 0
	{
		let mut hint = String::new();
		#[cfg(feature = "pretty_errors")]
		{
			hint += "Missing";

			hint += " substitution for:";
			for ident in missing
			{
				hint += " '";
				hint += &ident.0.to_string();
				hint += "'";
			}
			hint += "\n";
		}
		hint += VERBOSE_SYNTAX_SUBSTITUTION_IDENTIFIERS;

		return Err(Error::new("Incomplete substitution group.")
			.span(iter_span)
			.hint(hint));
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

	if result[0].2.is_empty()
	{
		Err(Error::new("No substitution groups.").hint(SHORT_SYNTAX_NO_GROUPS))
	}
	else
	{
		Ok(result)
	}
}

/// Assuming use of the short syntax, gets the initial list of substitution
/// identifiers.
fn validate_short_get_identifiers<'a, T: SubGroupIter<'a>>(
	mut iter: &mut TokenIter<'a, T>,
) -> Result<Vec<(String, Vec<String>)>>
{
	let mut result = Vec::new();
	while let Some(ident) = iter.extract_simple(
		|t| is_ident(t, None) || (is_semicolon(t) && !result.is_empty()),
		|t| get_ident(t),
		Some(
			if result.is_empty()
			{
				NO_INVOCATION
			}
			else
			{
				"substitution_identifier or ';'"
			},
		),
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
			#[allow(unused_mut)]
			let mut error = crate::pretty_errors::SHORT_SYNTAX_MISSING_SUB_BRACKET;
			#[cfg(feature = "pretty_errors")]
			{
				if iter.has_next_semicolon()?
				{
					error = crate::pretty_errors::SHORT_SYNTAX_SUBSTITUTION_COUNT;
				}
			}

			let (group, _) = iter
				.next_group(Some(Delimiter::Bracket))
				.map_err(|err| err.hint(error))?;
			streams.push(group.to_token_stream());
		}

		if iter.has_next()?
		{
			#[cfg(feature = "pretty_errors")]
			{
				if let Ok((_, span)) = iter.next_group(Some(Delimiter::Bracket))
				{
					return Err(Error::new("Unexpected delimiter.")
						.span(span)
						.hint(crate::pretty_errors::SHORT_SYNTAX_SUBSTITUTION_COUNT));
				}
			}
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
