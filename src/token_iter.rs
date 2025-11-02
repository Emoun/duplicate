use crate::{error::Error, invoke_nested, new_group, Result, SubstitutionGroup};
use proc_macro::{token_stream::IntoIter, Delimiter, Ident, Spacing, Span, TokenStream, TokenTree};
use std::{
	collections::VecDeque,
	fmt::{Debug, Formatter},
	iter::{once, FromIterator},
};

/// Trait alias
pub(crate) trait SubGroupIter<'a>: Iterator<Item = &'a SubstitutionGroup> + Clone {}
impl<'a, T: Iterator<Item = &'a SubstitutionGroup> + Clone> SubGroupIter<'a> for T {}

/// Designates the type of a token
#[derive(Debug, Clone)]
pub(crate) enum Token<'a, T: SubGroupIter<'a>>
{
	/// A simple token (i.e. not a group)
	Simple(TokenTree),

	/// A group with the given delimiter, body, and original span
	Group(Delimiter, TokenIter<'a, T>, Span),
}
impl<'a, T: SubGroupIter<'a>> Token<'a, T>
{
	/// Returns the span of the enclosed token(s)
	pub(crate) fn span(&self) -> Span
	{
		match self
		{
			Token::Simple(t) => t.span(),
			Token::Group(_, _, span) => span.clone(),
		}
	}
}

impl<'a, T: SubGroupIter<'a>> From<Token<'a, T>> for TokenTree
{
	fn from(t: Token<'a, T>) -> Self
	{
		match t
		{
			Token::Simple(t) => t,
			Token::Group(d, iter, span) =>
			{
				TokenTree::Group(new_group(d, iter.to_token_stream(), span))
			},
		}
	}
}

/// Whether the token tree is a punctuation
fn is_punct(t: &TokenTree, c: char) -> bool
{
	if let TokenTree::Punct(p) = t
	{
		p.as_char() == c && p.spacing() == Spacing::Alone
	}
	else
	{
		false
	}
}

/// Whether the token tree is a semicolon punctuation
pub fn is_semicolon(t: &TokenTree) -> bool
{
	is_punct(t, ';')
}

/// Whether the token tree is an identifier, and if so, whether it is equal to
/// the given string (if given)
pub fn is_ident(t: &TokenTree, comp: Option<&str>) -> bool
{
	if let TokenTree::Ident(id) = t
	{
		comp.map_or(true, |comp| comp == id.to_string())
	}
	else
	{
		false
	}
}

/// If the given token tree is an identifier, gets it.
pub fn get_ident(t: TokenTree) -> Option<Ident>
{
	if let TokenTree::Ident(id) = t
	{
		Some(id)
	}
	else
	{
		None
	}
}

/// Used to iterate through tokens from a TokenStream.
///
/// Will automatically expand any nested `duplicate` calls, ensuring only final
/// tokens are produced. Before doing the expansion, will duplicate/substitute
/// the nested invocation according to the given rules. This is needed e.g. when
/// the outer invocation affects the inner invocation's invocation and not
/// only the body.
///
/// Will also automatically extract tokens from any group without delimiters
/// instead of producing the group itself. Therefore, any group produced is
/// guaranteed to no use the None delimiter.
///
/// Most methods return a Result because the processing happens lazily, meaning
/// a processing error (e.g. if nested invocations fail) can happen at any time.
/// If a method returns an error, no tokens are consumed.
#[derive(Clone)]
pub(crate) struct TokenIter<'a, T: SubGroupIter<'a>>
{
	/// Tokens that have yet to be processed
	raw_tokens: IntoIter,

	/// Tokens that have yet to be produced
	///
	/// If a token is a None-delimited group, its tokens are in the process of
	/// being produced.
	unconsumed: VecDeque<Token<'a, T>>,

	/// While processing, nested invocations are first substituted with these
	/// global substitutions
	global_subs: &'a SubstitutionGroup,

	/// While processing, nested invocations are first duplicated with these
	/// substitution groups.
	sub_groups: T,

	/// The span of the last token to be produced.
	last_span: Span,
}
impl<'a, T: SubGroupIter<'a>> TokenIter<'a, T>
{
	/// Gets at least 1 token from the raw stream and puts it in the unconsumed,
	/// expanding any nested invocation if encountered
	///
	/// Returns whether at least 1 token was added to the unconsumed queue.
	fn fetch(&mut self) -> Result<bool>
	{
		if let Some(t) = self.raw_tokens.next()
		{
			/// The string identifying a nested `duplicate!` invocation
			const NESTED_DUPLICATE_NAME: &'static str = "duplicate";
			/// The string identifying a nested `substitute!` invocation
			const NESTED_SUBSTITUTE_NAME: &'static str = "substitute";
			match t
			{
				TokenTree::Group(g) =>
				{
					self.unconsumed.push_back(Token::Group(
						g.delimiter(),
						TokenIter::new_like(g.stream(), self),
						g.span(),
					))
				},
				TokenTree::Ident(id)
					if id.to_string() == NESTED_DUPLICATE_NAME
						|| id.to_string() == NESTED_SUBSTITUTE_NAME =>
				{
					if let Some(TokenTree::Punct(p)) = self.raw_tokens.next()
					{
						if is_punct(&TokenTree::Punct(p.clone()), '!')
						{
							let stream = invoke_nested(
								&mut TokenIter::new_like(
									TokenStream::from_iter(self.raw_tokens.next().into_iter()),
									self,
								),
								id.to_string() == NESTED_DUPLICATE_NAME,
							)?;
							self.unconsumed.push_back(Token::Group(
								Delimiter::None,
								TokenIter::new_like(stream, self),
								p.span(),
							));
						}
						else
						{
							// Not nested invocation
							self.unconsumed
								.push_back(Token::Simple(TokenTree::Ident(id)));
							self.unconsumed
								.push_back(Token::Simple(TokenTree::Punct(p)));
						}
					}
					else
					{
						// Not nested invocation
						self.unconsumed
							.push_back(Token::Simple(TokenTree::Ident(id)));
					}
				},
				_ => self.unconsumed.push_back(Token::Simple(t)),
			}
			Ok(true)
		}
		else
		{
			Ok(false)
		}
	}

	/// Attempts to get the next unconsumed token.
	///
	/// If the next token is a None-delimited group, attempts to get its next
	/// token instead. If such a group is empty, removed it and tries again.
	fn next_unconsumed(&mut self) -> Result<Option<Token<'a, T>>>
	{
		self.unconsumed.pop_front().map_or(Ok(None), |t| {
			match t
			{
				Token::Group(del, mut iter, span) if del == Delimiter::None =>
				{
					match iter.next_fallible()
					{
						Ok(Some(t)) =>
						{
							self.unconsumed.push_front(Token::Group(del, iter, span));
							Ok(Some(t))
						},
						Ok(None) => self.next_fallible(),
						err => err,
					}
				},
				t => Ok(Some(t)),
			}
		})
	}

	/// Gets the next fully processed token
	pub fn next_fallible(&mut self) -> Result<Option<Token<'a, T>>>
	{
		self.fetch()?;
		self.next_unconsumed()
	}

	/// Extracts a value from the next token.
	///
	/// An error is returned if:
	/// * the next token is a delimited group
	/// * no token is left
	/// * `p` returns false for the next token
	///
	/// If `p` returns true, the token is given to `f` whose result is returned.
	/// If an error is returned, if given, `expected` should describe what input
	/// was expected
	pub fn extract_simple<R, P: FnOnce(&TokenTree) -> bool, F: FnOnce(TokenTree) -> R>(
		&mut self,
		p: P,
		f: F,
		expected: Option<&str>,
	) -> Result<R>
	{
		let create_error = |error: &str| {
			let mut err = Error::new(error);
			if let Some(expected_string) = expected
			{
				err = err.hint("Expected ".to_string() + expected_string + ".");
			}
			err
		};
		match self.peek()?
		{
			Some(Token::Simple(t)) if p(&t) =>
			{
				self.last_span = t.span();
				Ok(f(self.next_fallible().unwrap().unwrap().into()))
			},
			Some(Token::Simple(t)) => Err(create_error("Unexpected token.").span(t.span())),
			Some(Token::Group(_, _, span)) =>
			{
				Err(create_error("Unexpected delimiter.").span(span.clone()))
			},
			None => Err(create_error("Unexpected end of code.")),
		}
	}

	/// Extracts the next identifier token.
	///
	/// Returns an error if the next token is not an identifier.
	pub fn extract_identifier(&mut self, expected: Option<&str>) -> Result<Ident>
	{
		self.extract_simple(|t| is_ident(t, None), |t| get_ident(t).unwrap(), expected)
	}

	/// Ensures the next token is a simple token.
	///
	/// Returns an error if:
	/// * the next token is a delimited group
	/// * no token is left
	/// * `p` returns false for the next token
	///
	/// If an error is returned, if given, the expected string is used in the
	/// error message
	pub fn expect_simple<P: FnOnce(&TokenTree) -> bool>(
		&mut self,
		p: P,
		expected: Option<&str>,
	) -> Result<()>
	{
		self.extract_simple(p, |_| (), expected)
	}

	/// Ensures the next token is a comma.
	///
	/// Otherwise returns an error.
	pub fn expect_comma(&mut self) -> Result<()>
	{
		self.expect_simple(|t| is_punct(t, ','), Some(","))
	}

	/// Ensures the next token is a semicolon.
	///
	/// Otherwise returns an error.
	pub fn expect_semicolon(&mut self) -> Result<()>
	{
		self.expect_simple(is_semicolon, Some("';'"))
	}

	/// Gets the body and span of the next group.
	///
	/// Returns an error if:
	/// * the group is non-delimited
	/// * no more tokens are available
	/// * the next group doesn't use the expected delimiter
	pub fn next_group(&mut self, expected: Option<Delimiter>) -> Result<(Self, Span)>
	{
		assert_ne!(
			Some(Delimiter::None),
			expected,
			"should only be used with non-None delimiters"
		);

		let left_delimiter = |d| {
			match d
			{
				Some(Delimiter::Bracket) => "'['",
				Some(Delimiter::Brace) => "'{'",
				Some(Delimiter::Parenthesis) => "'('",
				None => "'{', '[', or '('",
				_ => unreachable!(),
			}
		};
		let error = || format!("Expected {}.", left_delimiter(expected));

		match self.peek()?
		{
			Some(Token::Group(del, _, span)) if *del != Delimiter::None =>
			{
				if let Some(exp_del) = expected
				{
					if exp_del != *del
					{
						return Err(Error::new(error()).span(span.clone()));
					}
				}
				if let Token::Group(_, iter, span) = self.next_fallible()?.unwrap()
				{
					self.last_span = span;
					Ok((iter, span))
				}
				else
				{
					unreachable!()
				}
			},
			Some(token) => Err(Error::new(error()).span(token.span())),
			_ => Err(Error::new(error()).span(self.last_span)),
		}
	}

	/// Converts to a TokenStream immediately processing the whole iterator,
	/// panicking if an error is encountered.
	pub fn process_all(mut self) -> TokenStream
	{
		let mut result = TokenStream::new();
		while let Some(t) = self.next_fallible().unwrap()
		{
			result.extend(once(TokenTree::from(t)));
		}
		result
	}

	/// Convert to TokenStream __without any processing__.
	pub fn to_token_stream(self) -> TokenStream
	{
		TokenStream::from_iter(
			self.unconsumed
				.into_iter()
				.map(|tok| TokenTree::from(tok))
				.chain(self.raw_tokens),
		)
	}

	/// Whether there are more tokens to produced
	pub fn has_next(&mut self) -> Result<bool>
	{
		self.peek().map_or_else(|e| Err(e), |t| Ok(t.is_some()))
	}

	/// Whether there is a next token and it is a ';'
	#[cfg_attr(not(feature = "pretty_errors"), allow(dead_code))]
	pub fn has_next_semicolon(&mut self) -> Result<bool>
	{
		self.peek().map_or_else(
			|e| Err(e),
			|t| {
				Ok(match t
				{
					Some(Token::Simple(t)) if is_semicolon(t) => true,
					_ => false,
				})
			},
		)
	}

	/// Peek at the next token to be produced without consuming it
	pub fn peek(&mut self) -> Result<Option<&Token<'a, T>>>
	{
		let (pop_front, should_fetch, new_front) = match self.unconsumed.front_mut()
		{
			Some(Token::Group(del, iter, _)) if *del == Delimiter::None =>
			{
				if let Some(t) = iter.next_fallible()?
				{
					(false, false, Some(t))
				}
				else
				{
					(true, true, None)
				}
			},
			None => (false, true, None),
			_ => (false, false, None),
		};
		if pop_front
		{
			self.unconsumed.pop_front();
		}
		if should_fetch
		{
			if self.fetch()?
			{
				return self.peek();
			}
		}
		if let Some(t) = new_front
		{
			self.unconsumed.push_front(t);
		}
		Ok(self.unconsumed.front())
	}

	/// Returns the given token to the front, such that it is the next to be
	/// produced
	pub fn push_front(&mut self, token: Token<'a, T>)
	{
		self.unconsumed.push_front(token)
	}

	/// Construct new token iterator from the given stream.
	///
	/// The given global substitutions and substitution groups will be used
	/// to substitute/duplicate nested invocations before they are expanded.
	pub(crate) fn new(
		stream: TokenStream,
		global_subs: &'a SubstitutionGroup,
		sub_groups: T,
	) -> Self
	{
		Self {
			raw_tokens: stream.into_iter(),
			unconsumed: VecDeque::new(),
			last_span: Span::call_site(),
			global_subs,
			sub_groups,
		}
	}

	/// Construct new token iterator from the given stream.
	///
	/// Substitution/duplication of nested invocations is taken from 'like'
	pub fn new_like(stream: TokenStream, like: &Self) -> Self
	{
		Self::new(stream, like.global_subs, like.sub_groups.clone())
	}
}
impl<'a, T: SubGroupIter<'a> + Debug> Debug for TokenIter<'a, T>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		f.write_str("TokenIter{")?;
		self.raw_tokens.clone().collect::<Vec<_>>().fmt(f)?;
		f.write_str(", ")?;
		self.unconsumed.fmt(f)?;
		f.write_str(", ")?;
		self.global_subs.fmt(f)?;
		f.write_str(", ")?;
		self.sub_groups.fmt(f)?;
		f.write_str(",...}")
	}
}
