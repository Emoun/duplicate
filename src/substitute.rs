#[cfg(feature = "module_disambiguation")]
use crate::module_disambiguation::try_substitute_mod;
use crate::{
	disambiguate_module, error::Error, new_group, token_iter::SubGroupIter, Result,
	SubstitutionGroup, Token, TokenIter,
};
use proc_macro::{Delimiter, Ident, Span, TokenStream, TokenTree};

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
/// number of arguments must match the number given to `new`,) which will yield
/// the final TokenStream that should be substituted for the identifier ( +
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
	pub(crate) fn new<'a, T: SubGroupIter<'a>>(
		arguments: &Vec<String>,
		mut stream: TokenIter<'a, T>,
	) -> Result<Self>
	{
		let mut substitutions = Vec::new();
		// Group tokens that aren't substitution identifiers or groups
		let mut saved_tokens = None;

		let find_argument =
			|ident: &Ident| arguments.iter().position(|arg| *arg == ident.to_string());

		while let Some(token) = stream.next_fallible()?
		{
			match token
			{
				Token::Simple(TokenTree::Ident(ident)) if find_argument(&ident).is_some() =>
				{
					if let Some(sub_stream) = saved_tokens.take()
					{
						substitutions.push(SubType::Token(sub_stream));
					}
					substitutions.push(SubType::Argument(find_argument(&ident).unwrap()));
				},
				Token::Group(del, iter, _) =>
				{
					if let Some(sub_stream) = saved_tokens.take()
					{
						substitutions.push(SubType::Token(sub_stream));
					}
					substitutions.push(SubType::Group(del, Substitution::new(arguments, iter)?));
				},
				token =>
				{
					saved_tokens
						.get_or_insert_with(|| TokenStream::new())
						.extend(Some(TokenTree::from(token)).into_iter())
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
							TokenStream::from(TokenTree::Group(new_group(
								delimiter.clone(),
								subst.apply(arguments, err_span)?,
								Span::call_site(),
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
			Err(Error::new(format!(
				"Expected {} substitution arguments but got {}",
				self.arg_count,
				arguments.len()
			))
			.span(err_span))
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
	global_subs: &'a SubstitutionGroup,
	mut sub_groups: impl Iterator<Item = &'a SubstitutionGroup> + Clone,
) -> Result<TokenStream>
{
	let mut result = TokenStream::new();
	#[allow(unused_variables)]
	let mod_and_postfix_sub = disambiguate_module(&item, sub_groups.clone())?;

	let sub_groups_clone = sub_groups.clone();
	let mut duplicate_and_substitute_one = |substitutions: &SubstitutionGroup| -> Result<()> {
		let mut item_iter = TokenIter::new(item.clone(), global_subs, sub_groups_clone.clone());

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
fn substitute_next_token<'a, T: SubGroupIter<'a>>(
	tree: &mut TokenIter<'a, T>,
	global_subs: &SubstitutionGroup,
	substitutions: &SubstitutionGroup,
) -> Result<Option<TokenStream>>
{
	let mut result = None;
	match tree.next_fallible()?
	{
		Some(Token::Simple(TokenTree::Ident(ident))) =>
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
						let (mut group_iter, span) =
							tree.next_group(Some(Delimiter::Parenthesis))?;
						let mut args = Vec::new();
						loop
						{
							match group_iter.next_group(Some(Delimiter::Bracket))
							{
								Ok((group, _)) =>
								{
									args.push(duplicate_and_substitute(
										group.to_token_stream(),
										global_subs,
										Some(substitutions).into_iter(),
									)?);
									if group_iter.has_next()?
									{
										group_iter.expect_comma()?;
									}
								},
								Err(err) =>
								{
									if group_iter.has_next()?
									{
										return Err(
											err.hint(crate::pretty_errors::BRACKET_SUB_PARAM)
										);
									}
									else
									{
										break;
									}
								},
							}
						}
						subst.apply(&args, span)?
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
				_ =>
				{
					return Err(
						Error::new("Multiple substitutions for identifier").span(ident.span())
					)
				},
			}
		},
		Some(Token::Group(del, mut group_iter, span)) =>
		{
			let mut substituted = TokenStream::new();
			while let Some(stream) =
				substitute_next_token(&mut group_iter, global_subs, substitutions)?
			{
				substituted.extend(stream)
			}
			result.get_or_insert_with(|| TokenStream::new()).extend(
				TokenStream::from(TokenTree::Group(new_group(del, substituted, span))).into_iter(),
			);
		},
		Some(token) =>
		{
			result
				.get_or_insert_with(|| TokenStream::new())
				.extend(Some(TokenTree::from(token)).into_iter())
		},
		_ => (),
	}
	Ok(result)
}
