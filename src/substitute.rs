use crate::{parse_utils::*, SubstitutionGroup};
use proc_macro::{Delimiter, Group, Ident, TokenStream, TokenTree};

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
	pub fn new(arguments: &Vec<String>, stream: impl Iterator<Item = TokenTree>)
		-> Result<Self, ()>
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
	pub fn apply_simple(&self) -> Result<TokenStream, ()>
	{
		self.apply(&Vec::new())
	}

	/// Apply the substitution to the given arguments.
	///
	/// The number of arguments must match the exact number accepted by the
	/// substitution.
	pub fn apply(&self, arguments: &Vec<TokenStream>) -> Result<TokenStream, ()>
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
								subst.apply(arguments)?,
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
			Err(())
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
}

/// Duplicates the given token stream, substituting any identifiers found.
pub(crate) fn substitute(item: TokenStream, groups: Vec<SubstitutionGroup>) -> TokenStream
{
	let mut result = TokenStream::new();

	for substitutions in groups
	{
		let mut item_iter = item.clone().into_iter();
		while let Some(stream) = substitute_next_token(&mut item_iter, &substitutions)
		{
			result.extend(stream);
		}
	}

	result
}

/// Recursively checks the given token for any use of the given substitution
/// identifiers and substitutes them, returning the resulting token stream.
fn substitute_next_token(
	tree: &mut impl Iterator<Item = TokenTree>,
	substitutions: &SubstitutionGroup,
) -> Option<TokenStream>
{
	let mut result = None;
	match tree.next()
	{
		Some(TokenTree::Ident(ident)) =>
		{
			if let Some(subst) = substitutions.substitution_of(&ident.to_string())
			{
				let stream = if subst.arg_count > 0
				{
					if let Ok(group) = parse_group(tree, ident.span(), "")
					{
						let mut group_stream_iter = group.stream().into_iter().peekable();
						let mut args = Vec::new();
						while let Ok(group) = parse_group(&mut group_stream_iter, ident.span(), "")
						{
							args.push(group.stream());
							match group_stream_iter.peek()
							{
								Some(TokenTree::Punct(punct)) if punct_is_char(punct, ',') =>
								{
									let _ = group_stream_iter.next();
								},
								Some(_) => panic!("Expected ','."),
								_ => (),
							}
						}
						subst
							.apply(&args)
							.expect("Error substituting identifier with arguments")
					}
					else
					{
						panic!(format!(
							"Substitution identifier '{}' takes {} arguments but was supplied \
							 with none.",
							ident.to_string(),
							subst.arg_count
						))
					}
				}
				else
				{
					subst
						.apply_simple()
						.expect("Error substituting identifier without arguments")
				};
				result
					.get_or_insert_with(|| TokenStream::new())
					.extend(stream.into_iter());
			}
			else
			{
				result
					.get_or_insert_with(|| TokenStream::new())
					.extend(TokenStream::from(TokenTree::Ident(ident)).into_iter());
			}
		},
		Some(TokenTree::Group(group)) =>
		{
			let mut substituted = TokenStream::new();
			let mut group_iter = group.stream().into_iter();
			while let Some(stream) = substitute_next_token(&mut group_iter, substitutions)
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
	result
}
