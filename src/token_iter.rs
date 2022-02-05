use crate::{invoke_nested, Result};
use proc_macro::{
	token_stream::IntoIter, Delimiter, Group, Ident, Spacing, Span, TokenStream, TokenTree,
};
use std::{
	collections::VecDeque,
	fmt::{Debug, Formatter},
	iter::FromIterator,
};

/// Designates the type of a token
#[derive(Debug, Clone)]
pub enum Token
{
	/// A simple token (i.e. not a group)
	Simple(TokenTree),

	/// A group with the given delimiter, body, and original span
	Group(Delimiter, TokenIter, Span),
}
impl Token
{
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

	/// Whether the token is a semicolon punctuation
	pub fn is_semicolon(t: &TokenTree) -> bool
	{
		Self::is_punct(t, ';')
	}

	/// Whether the token is an identifier
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

	/// If the given token is an identifiers, gets it.
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
}
impl From<Token> for TokenTree
{
	fn from(t: Token) -> Self
	{
		match t
		{
			Token::Simple(t) => t,
			Token::Group(d, iter, _) => TokenTree::Group(Group::new(d, iter.to_token_stream())),
		}
	}
}

/// Used to iterate through tokens from a TokenStream.
///
/// Will automatically expand any nested `duplicate` calls, ensuring only final
/// tokens are produced. Will also automatically extract tokens from any group
/// without delimiters instead of producing the group itself. Therefore, any
/// group produced is guaranteed to no use the None delimiter.
///
/// Most methods return a Result because the processing happens lazily, meaning
/// a processing error (e.g. if nested invocations fail) can happen at any time.
/// If a method returns an error, no tokens are consumed.
#[derive(Clone)]
pub struct TokenIter
{
	/// Tokens that have yet to be processed
	raw_tokens: IntoIter,

	/// Tokens that have yet to be produced
	///
	/// If a token is a None-delimited group, its tokens are in the process of
	/// being produced.
	unconsumed: VecDeque<Token>,

	/// The span of the last token to be produced.
	last_span: Span,
}
impl TokenIter
{
	/// Gets at least 1 token from the raw stream and puts it in the unconsumed,
	/// expanding any nested invocation if encountered
	///
	/// Returns whether at least 1 token was added to the unconsumed queue.
	fn fetch(&mut self) -> Result<bool>
	{
		if let Some(t) = self.raw_tokens.next()
		{
			match t
			{
				TokenTree::Group(g) =>
				{
					self.unconsumed.push_back(Token::Group(
						g.delimiter(),
						TokenIter::from(g.stream()),
						g.span(),
					))
				},
				TokenTree::Ident(id) if id.to_string() == "duplicate" =>
				{
					if let Some(TokenTree::Punct(p)) = self.raw_tokens.next()
					{
						if Token::is_punct(&TokenTree::Punct(p.clone()), '!')
						{
							// Nested Invocation
							let stream = invoke_nested(
								&mut TokenStream::from_iter(self.raw_tokens.next().into_iter())
									.into(),
							)?;
							self.unconsumed.push_back(Token::Group(
								Delimiter::None,
								TokenIter::from(stream),
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
	/// If the next token is a None-delimited group, attempts to get it's next
	/// token instead. If such a group is empty, removed it and tries again.
	fn next_unconsumed(&mut self) -> Result<Option<Token>>
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
	pub fn next_fallible(&mut self) -> Result<Option<Token>>
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
			let mut msg: String = error.into();
			if let Some(expected_string) = expected
			{
				msg.push_str(". Expected ");
				msg.push_str(expected_string);
				msg.push_str(" instead.");
			}
			else
			{
				msg.push('.');
			}
			msg
		};
		match self.peek()?
		{
			Some(Token::Simple(t)) if p(&t) =>
			{
				self.last_span = t.span();
				Ok(f(self.next_fallible().unwrap().unwrap().into()))
			},
			Some(Token::Simple(t)) => Err((t.span(), create_error("Unexpected token"))),
			Some(Token::Group(_, _, span)) =>
			{
				Err((span.clone(), create_error("Unexpected delimiter")))
			},
			None => Err((Span::call_site(), create_error("Unexpected end of code"))),
		}
	}

	/// Extracts the next identifier token.
	///
	/// Returns an error if the next token is not an identifier.
	pub fn extract_identifier(&mut self, expected: Option<&str>) -> Result<Ident>
	{
		self.extract_simple(
			|t| Token::is_ident(t, None),
			|t| Token::get_ident(t).unwrap(),
			expected,
		)
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
		self.expect_simple(|t| Token::is_punct(t, ','), Some(","))
	}

	/// Ensures the next token is a semicolon.
	///
	/// Otherwise returns an error.
	pub fn expect_semicolon(&mut self) -> Result<()>
	{
		self.expect_simple(Token::is_semicolon, Some(";"))
	}

	/// Gets the body and span of the next group.
	///
	/// Returns an error if:
	/// * the group is non-delimited
	/// * no more tokens are available
	/// * the next group doesn't use the expected delimiter
	pub fn next_group(
		&mut self,
		expected: Option<Delimiter>,
		hint: &str,
	) -> Result<(TokenIter, Span)>
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
		let error = || format!("Expected {}.\n{}", left_delimiter(expected), hint);

		match self.peek()?
		{
			Some(Token::Group(del, _, span)) if *del != Delimiter::None =>
			{
				if let Some(exp_del) = expected
				{
					if exp_del != *del
					{
						return Err((span.clone(), error()));
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
			_ => Err((self.last_span, error())),
		}
	}

	/// Converts to an Iterator if TokenTrees
	///
	/// The resulting iterator will panic if processing encounters an error.
	pub fn to_token_tree_iter(self) -> impl Iterator<Item = TokenTree>
	{
		self.map(|t| t.into())
	}

	/// Converts to a TokenStream
	///
	/// Immediately processes the whole iterator, panicking if an error is
	/// encountered.
	pub fn to_token_stream(self) -> TokenStream
	{
		TokenStream::from_iter(self.to_token_tree_iter())
	}

	/// Whether there are more tokens to produced
	pub fn has_next(&mut self) -> Result<bool>
	{
		self.peek().map_or_else(|e| Err(e), |t| Ok(t.is_some()))
	}

	/// Peek at the next token to be produced without consuming it
	pub fn peek(&mut self) -> Result<Option<&Token>>
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
		new_front.map_or((), |t| self.unconsumed.push_front(t));
		Ok(self.unconsumed.front())
	}

	/// Returns the given token to the front, such that it is the next to be
	/// produced
	pub fn push_front(&mut self, token: Token)
	{
		self.unconsumed.push_front(token)
	}
}
impl Debug for TokenIter
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		f.write_str("TokenIter{")?;
		self.raw_tokens.clone().collect::<Vec<_>>().fmt(f)?;
		f.write_str(", ")?;
		self.unconsumed.fmt(f)?;
		f.write_str(",...}")
	}
}
impl From<TokenStream> for TokenIter
{
	fn from(stream: TokenStream) -> Self
	{
		Self {
			raw_tokens: stream.into_iter(),
			unconsumed: VecDeque::new(),
			last_span: Span::call_site(),
		}
	}
}
impl Iterator for TokenIter
{
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item>
	{
		self.next_fallible().unwrap()
	}
}
