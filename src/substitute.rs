#[cfg(feature = "module_disambiguation")]
use crate::module_disambiguation::try_substitute_mod;
use crate::{disambiguate_module, parse_utils::*, Result, SubstitutionGroup};
use proc_macro::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use std::iter::Peekable;

/// The types of sub-substitutions composing a single substitution.
#[derive(Debug)]
pub enum SubType
{
	/// A simple substitution with the TokenStream
	Token(TokenStream),
	/// Substitute with the TokenStream in the argument of given index.
	Argument(usize),
	/// Substitution with a group with the specified delimiter and the contents
	/// being what is produced by the nested substitution.
	Group(Delimiter, Substitution),
}

/// A substitution for an identifier.
///
/// A substitution takes a specific number of arguments as TokenStreams and
/// produces a TokenStream that is to be substituted for the identifier.
///
/// To create a substitution for an identifier, `new` is given the list of
/// arguments and the tokens to be used as a substitution. The `apply` method
/// can then be given a set of substitution arguments as TokenStreams (the
/// number of arguments must match the number given to `new`,) wich will yield
/// the final TokenStream that should be substituted for the identifer ( +
/// arguments).
#[derive(Debug)]
pub struct Substitution
{
	/// The number of arguments to the substitution
	arg_count: usize,
	/// The substitution. The list is ordered, with the result of an application
	/// being the concatenation of each sub-substitution.
	sub: Vec<SubType>,
}

impl Substitution
{
	/// Create a new substitution that takes no arguments.
	pub fn new_simple(substitution: TokenStream) -> Self
	{
		Self {
			arg_count: 0,
			sub: vec![SubType::Token(substitution)],
		}
	}

	/// Create a new substitution.
	///
	/// The given argument list is assumed to be ordered and its length is the
	/// number of arguments to the substitution.
	/// The tokens produced by the iterator will be the basis for applying a
	/// substitution, where each instance of an argument identifier being
	/// replaced by the arguments passed to the a substitution identifier.
	pub fn new(arguments: &Vec<String>, stream: impl Iterator<Item = TokenTree>) -> Result<Self>
	{
		let mut substitutions = Vec::new();
		// Group tokens that aren't substitution identifiers or groups
		let mut saved_tokens = None;

		let find_argument =
			|ident: &Ident| arguments.iter().position(|arg| *arg == ident.to_string());

		for token in stream
		{
			match token
			{
				TokenTree::Ident(ref ident) if find_argument(&ident).is_some() =>
				{
					if let Some(sub_stream) = saved_tokens.take()
					{
						substitutions.push(SubType::Token(sub_stream));
					}
					substitutions.push(SubType::Argument(find_argument(&ident).unwrap()));
				},
				TokenTree::Group(group) =>
				{
					if let Some(sub_stream) = saved_tokens.take()
					{
						substitutions.push(SubType::Token(sub_stream));
					}
					substitutions.push(SubType::Group(
						group.delimiter(),
						Substitution::new(arguments, group.stream().into_iter())?,
					));
				},
				token =>
				{
					saved_tokens
						.get_or_insert_with(|| TokenStream::new())
						.extend(Some(token).into_iter())
				},
			}
		}
		if let Some(sub_stream) = saved_tokens
		{
			substitutions.push(SubType::Token(sub_stream));
		}
		let substitution = Self {
			arg_count: arguments.len(),
			sub: substitutions,
		};
		Ok(substitution)
	}

	/// Apply the substitution, assuming it takes no arguments.
	pub fn apply_simple(&self, err_span: Span) -> Result<TokenStream>
	{
		self.apply(&Vec::new(), err_span)
	}

	/// Apply the substitution to the given arguments.
	///
	/// The number of arguments must match the exact number accepted by the
	/// substitution.
	pub fn apply(&self, arguments: &Vec<TokenStream>, err_span: Span) -> Result<TokenStream>
	{
		if arguments.len() == self.arg_count
		{
			let mut result = TokenStream::new();
			for sub in self.sub.iter()
			{
				result.extend(
					match sub
					{
						SubType::Token(stream) => stream.clone(),
						SubType::Argument(idx) => arguments[*idx].clone(),
						SubType::Group(delimiter, subst) =>
						{
							TokenStream::from(TokenTree::Group(Group::new(
								delimiter.clone(),
								subst.apply(arguments, err_span)?,
							)))
						},
					}
					.into_iter(),
				)
			}
			Ok(result)
		}
		else
		{
			Err((
				err_span,
				format!(
					"Expected {} substitution arguments but got {}",
					self.arg_count,
					arguments.len()
				),
			))
		}
	}

	#[cfg(feature = "module_disambiguation")]
	/// If this substitution simply produces an identifier and nothing else,
	/// then that identifier is returned, otherwise None
	pub fn substitutes_identifier(&self) -> Option<Ident>
	{
		if self.sub.len() == 1
		{
			if let SubType::Token(token) = &self.sub[0]
			{
				let mut iter = token.clone().into_iter();
				if let TokenTree::Ident(ident) = iter.next()?
				{
					// Ensure there are no more tokens, since we only allow 1 identifier.
					if iter.next().is_none()
					{
						return Some(ident);
					}
				}
			}
		}
		None
	}

	pub fn argument_count(&self) -> usize
	{
		self.arg_count
	}
}

/// Duplicates the given token stream, substituting any identifiers found.
pub(crate) fn duplicate_and_substitute<'a>(
	item: TokenStream,
	global_subs: &SubstitutionGroup,
	mut sub_groups: impl Iterator<Item = &'a SubstitutionGroup> + Clone,
) -> Result<TokenStream>
{
	let mut result = TokenStream::new();
	#[allow(unused_variables)]
	let mod_and_postfix_sub = disambiguate_module(&item, sub_groups.clone())?;

	let mut duplicate_and_substitute_one = |substitutions: &SubstitutionGroup| -> Result<()> {
		let mut item_iter = item.clone().into_iter().peekable();

		#[cfg(feature = "module_disambiguation")]
		let mut substituted_mod = false;
		loop
		{
			#[cfg(feature = "module_disambiguation")]
			{
				if !substituted_mod
				{
					let stream =
						try_substitute_mod(&mod_and_postfix_sub, substitutions, &mut item_iter);
					substituted_mod = !stream.is_empty();
					result.extend(stream);
				}
			}

			if let Some(stream) = substitute_next_token(&mut item_iter, global_subs, substitutions)?
			{
				result.extend(stream);
			}
			else
			{
				break;
			}
		}
		Ok(())
	};

	// We always want at least 1 duplicate.
	// If no groups are given, we just want to run the global substitutions
	let empty_sub = SubstitutionGroup::new();
	duplicate_and_substitute_one(sub_groups.next().unwrap_or(&empty_sub))?;

	for substitutions in sub_groups
	{
		duplicate_and_substitute_one(&substitutions)?;
	}

	Ok(result)
}

/// Recursively checks the given token for any use of the given substitution
/// identifiers and substitutes them, returning the resulting token stream.
fn substitute_next_token(
	tree: &mut Peekable<impl Iterator<Item = TokenTree>>,
	global_subs: &SubstitutionGroup,
	substitutions: &SubstitutionGroup,
) -> Result<Option<TokenStream>>
{
	let mut result = None;
	match tree.next()
	{
		Some(TokenTree::Ident(ident)) =>
		{
			match (
				substitutions.substitution_of(&ident.to_string()),
				global_subs.substitution_of(&ident.to_string()),
			)
			{
				(Some(subst), None) | (None, Some(subst)) =>
				{
					let stream = if subst.arg_count > 0
					{
						let group = parse_group(tree, Delimiter::Parenthesis, ident.span(), "")?;
						let mut group_stream_iter = group.stream().into_iter().peekable();
						let mut args = Vec::new();
						loop
						{
							match parse_group(
								&mut group_stream_iter,
								Delimiter::Bracket,
								ident.span(),
								"",
							)
							{
								Ok(group) =>
								{
									args.push(duplicate_and_substitute(
										group.stream(),
										global_subs,
										Some(substitutions).into_iter(),
									)?);
									match group_stream_iter.peek()
									{
										Some(TokenTree::Punct(punct))
											if punct_is_char(punct, ',') =>
										{
											let _ = group_stream_iter.next();
										},
										Some(t) => return Err((t.span(), "Expected ','".into())),
										_ => (),
									}
								},
								Err(err) =>
								{
									if group_stream_iter.peek().is_some()
									{
										return Err(err);
									}
									else
									{
										break;
									}
								},
							}
						}
						subst.apply(&args, group.span())?
					}
					else
					{
						subst.apply_simple(ident.span())?
					};
					result
						.get_or_insert_with(|| TokenStream::new())
						.extend(stream.into_iter());
				},
				(None, None) =>
				{
					result
						.get_or_insert_with(|| TokenStream::new())
						.extend(TokenStream::from(TokenTree::Ident(ident)).into_iter());
				},
				_ => return Err((ident.span(), "Multiple substitutions for identifier".into())),
			}
		},
		Some(TokenTree::Group(group)) =>
		{
			let mut substituted = TokenStream::new();
			let mut group_iter = group.stream().into_iter().peekable();
			while let Some(stream) =
				substitute_next_token(&mut group_iter, global_subs, substitutions)?
			{
				substituted.extend(stream)
			}
			result.get_or_insert_with(|| TokenStream::new()).extend(
				TokenStream::from(TokenTree::Group(Group::new(group.delimiter(), substituted)))
					.into_iter(),
			);
		},
		Some(token) =>
		{
			result
				.get_or_insert_with(|| TokenStream::new())
				.extend(Some(token).into_iter())
		},
		_ => (),
	}
	Ok(result)
}
