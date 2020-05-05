//! This crate provides the `duplicate` attribute macro for
//! code duplication with substitution.
//!
//! # Usage
//!
//! Say you have a trait with a method `is_max` that should return `true` if the
//! value of the object is the maximum allowed and `false` otherwise:
//! ```
//! trait IsMax {
//!   fn is_max(&self) -> bool;
//! }
//! ```
//! You would like to implement this trait for the three integer types `u8`,
//! `u16`, and `u32`:
//!
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! impl IsMax for u8 {
//!   fn is_max(&self) -> bool {
//!     *self == 255
//!   }
//! }
//! impl IsMax for u16 {
//!   fn is_max(&self) -> bool {
//!     *self == 65_535
//!   }
//! }
//! impl IsMax for u32 {
//!   fn is_max(&self) -> bool {
//!     *self == 4_294_967_295
//!   }
//! }
//! ```
//! This is a lot of repetition. Only the type and the maximum value are
//! actually different between the three implementations. This might not be much
//! in our case, but imagine doing this for all the integer types (10, as of the
//! last count.) We can use the `duplicate` attribute to avoid repeating
//! ourselves:
//!
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! use duplicate::duplicate;
//! #[duplicate(
//!   int_type  max_value;
//!   [ u8 ]    [ 255 ];
//!   [ u16 ]   [ 65_535 ];
//!   [ u32 ]   [ 4_294_967_295 ];
//! )]
//! impl IsMax for int_type {
//!   fn is_max(&self) -> bool {
//!     *self == max_value
//!   }
//! }
//!
//! assert!(!42u8.is_max());
//! assert!(!42u16.is_max());
//! assert!(!42u32.is_max());
//! ```
//! The above code will expand to the three implementations before it.
//! The attribute invocation specifies that the following item should be
//! substituted by three duplicates of itself. Additionally, each occurrence of
//! the identifier `int_type` in the first duplicate should be replaced by `u8`,
//! in the second duplicate by `u16`, and in the last by `u32`. Likewise, each
//! occurrence of `max_value` should be replaced by `255`, `65_535`, and
//! `4_294_967_295` in the first, second, and third duplicates respectively.
//!
//! `int_type` and `max_value` are called _substitution identifiers_, while `[
//! u8 ]`, `[ u16 ]`, and `[ u32 ]` are each _substitutions_ for `int_type`. The
//! number of duplicates made is equal to the number of substitutions the
//! substitution identifiers have---all identifiers must have the same number of
//! substitutions. Substitution identifiers must be valid Rust identifiers.
//!
//! The code inside substitutions can be arbitrary, as long as the expanded code
//! is valid. Additionally, any "bracket" type is valid; we could have used `()`
//! or `{}` anywhere `[]` is used in these examples.
//!
//! ## Verbose Syntax
//!
//! The syntax used in the previous examples is the _short syntax_.
//! `duplicate` also accepts a _verbose syntax_ that is less concise, but more
//! powerful. Using the verbose syntax, the above usage looks like this:
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! use duplicate::duplicate;
//! #[duplicate(
//!   [
//!     int_type  [ u8 ]
//!     max_value [ 255 ]
//!   ]
//!   [
//!     int_type  [ u16 ]
//!     max_value [ 65_535 ]
//!   ]
//!   [
//!     int_type  [ u32 ]
//!     max_value [ 4_294_967_295 ]
//!   ]
//! )]
//! impl IsMax for int_type {
//!   fn is_max(&self) -> bool {
//!     *self == max_value
//!   }
//! }
//!
//! # assert!(!42u8.is_max());
//! # assert!(!42u16.is_max());
//! # assert!(!42u32.is_max());
//! ```
//! The verbose syntax is centered around the _substitution group_, which then
//! includes a set of identifier and substitution pairs. Here is an annotated
//! version of the same code:
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! # use duplicate::duplicate;
//! #[duplicate(
//!   [                               //-+
//!     int_type  [ u8 ]              // | Substitution group 1
//!     max_value [ 255 ]             // |
//! //  ^^^^^^^^^ ^^^^^^^ substitution   |
//! //  |                                |
//! //  substitution identifier          |
//!   ]                               //-+
//!   [                               //-+
//!     int_type  [ u16 ]             // | Substitution group 2
//!     max_value [ 65_535 ]          // |
//!   ]                               //-+
//!   [                               //-+
//!     max_value [ 4_294_967_295 ]   // | Substitution group 3
//!     int_type  [ u32 ]             // |
//!   ]                               //-+
//! )]
//! # impl IsMax for int_type {
//! #  fn is_max(&self) -> bool {
//! #     *self == max_value
//! #    }
//! #  }
//! #
//! # assert!(!42u8.is_max());
//! # assert!(!42u16.is_max());
//! # assert!(!42u32.is_max());
//! ```
//! Note that in each substitution group every identifier must have exactly one
//! substitution. Any number of groups can be given with each translating to one
//! duplicate. All the groups must have the exact same identifiers, though the
//! order in which they arrive in each group is not important. For example, in
//! the annotated example, the third group has the `max_value` identifier before
//! `int_type` without having any effect on the expanded code.
//!
//! The short syntax's substitution grouping is based on the order of the
//! substitutions for each identifier. We can annotate the short version of our
//! example to highlight this:
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! # use duplicate::duplicate;
//! #[duplicate(
//!   int_type  max_value;
//!   [ u8 ]    [ 255 ];          // Group 1
//!   [ u16 ]   [ 65_535 ];       // Group 2
//!   [ u32 ]   [ 4_294_967_295 ];// Group 3
//! )]
//! # impl IsMax for int_type {
//! #   fn is_max(&self) -> bool {
//! #     *self == max_value
//! #   }
//! # }
//! #
//! # assert!(!42u8.is_max());
//! # assert!(!42u16.is_max());
//! # assert!(!42u32.is_max());
//! ```
//! The verbose syntax is not very concise but it some advantages over
//! the shorter syntax:
//!
//! - Using many identifiers and long substitutions can quickly become unwieldy
//!   in the short
//! syntax. The verbose syntax deals better with both as it will grow
//! horizontally instead of vertically.
//! - It offers something the short syntax doesn't: nested invocation.
//!
//! ### Nested Invocation
//!
//! Imagine we have the following trait with the method `is_negative` that
//! should return `true` if the value of the object is negative and `false`
//! otherwise:
//! ```
//! trait IsNegative {
//!   fn is_negative(&self) -> bool;
//! }
//! ```
//! We want to implement this for the six integer types `u8`, `u16`, `u32`,
//! `i8`, `i16`, and `i32`. For the first three types, which are all unsigned,
//! the implementation of this trait should trivially return `false` as they
//! can't be negative. However, for the remaining, signed types their
//! implementations is identical (checking whether they are less than `0`), but,
//! of course, different from the first three:
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! impl IsNegative for u8 {
//!   fn is_negative(&self) -> bool {
//!     false
//!   }
//! }
//! impl IsNegative for u16 {
//!   fn is_negative(&self) -> bool {
//!     false
//!   }
//! }
//! impl IsNegative for u32 {
//!   fn is_negative(&self) -> bool {
//!     false
//!   }
//! }
//! impl IsNegative for i8 {
//!   fn is_negative(&self) -> bool {
//!     *self < 0
//!   }
//! }
//! impl IsNegative for i16 {
//!   fn is_negative(&self) -> bool {
//!     *self < 0
//!   }
//! }
//! impl IsNegative for i32 {
//!   fn is_negative(&self) -> bool {
//!     *self <  0
//!   }
//! }
//! # assert!(!42u8.is_negative());
//! # assert!(!42u16.is_negative());
//! # assert!(!42u32.is_negative());
//! # assert!(!42i8.is_negative());
//! # assert!(!42i16.is_negative());
//! # assert!(!42i32.is_negative());
//! ```
//! Notice how the code repetition is split over 2 axes: 1) They all implement
//! the same trait 2) the method implementations of the first 3 are identical to
//! each other but different to the next 3, which are also mutually identical.
//! To implement this using only the syntax we have already seen, we could do
//! something like this:
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! # use duplicate::duplicate;
//! #[duplicate(
//!   [
//!     int_type [ u8 ]
//!     implementation [ false ]
//!   ]
//!   [
//!     int_type [ u16 ]
//!     implementation [ false ]
//!   ]
//!   [
//!     int_type [ u32 ]
//!     implementation [ false ]
//!   ]
//!   [
//!     int_type [ i8 ]
//!     implementation [ *self < 0 ]
//!   ]
//!   [
//!     int_type [ i16 ]
//!     implementation [ *self < 0 ]
//!   ]
//!   [
//!     int_type [ i32 ]
//!     implementation [ *self < 0 ]
//!   ]
//! )]
//! impl IsNegative for int_type {
//!   fn is_negative(&self) -> bool {
//!     implementation
//!   }
//! }
//!
//! assert!(!42u8.is_negative());
//! assert!(!42u16.is_negative());
//! assert!(!42u32.is_negative());
//! assert!(!42i8.is_negative());
//! assert!(!42i16.is_negative());
//! assert!(!42i32.is_negative());
//! ```
//! However ironically, we here had to repeat ourselves in the macro invocation
//! instead of the code: we needed to repeat the implementations `[ false ]` and
//! `[ *self < 0 ]` three times each. Using verbose syntax we can utilize
//! _nested invocation_ to remove the last bit of repetition:
//!
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! # use duplicate::duplicate;
//! #[duplicate(
//!   #[
//!     int_type_nested; [u8]; [u16]; [u32]
//!   ][
//!     [
//!       int_type [ int_type_nested ]
//!       implementation [ false ]
//!     ]
//!   ]
//!   #[
//!     int_type_nested; [i8]; [i16]; [i32]
//!   ][
//!     [
//!       int_type [ int_type_nested ]
//!       implementation [ *self < 0 ]
//!     ]
//!   ]
//! )]
//! impl IsNegative for int_type {
//!   fn is_negative(&self) -> bool {
//!     implementation
//!   }
//! }
//!
//! assert!(!42u8.is_negative());
//! assert!(!42u16.is_negative());
//! assert!(!42u32.is_negative());
//! assert!(!42i8.is_negative());
//! assert!(!42i16.is_negative());
//! assert!(!42i32.is_negative());
//! ```
//!
//! We use `#` to invoke the macro inside itself, producing duplicates
//! of the code inside the following `[]`, `{}`, or `()`.
//! In our example, we have 2 invocations that each produce 3 groups, inserting
//! the correct `implementation` for their signed or unsigned types.
//! The above nested invocation is equivalent to the previous, non-nested
//! invocation, and actually expands to it as an intermediate step before
//! expanding the outer-most invocation.
//!
//! It's important to notice that the nested invocation doesn't know it
//! isn't the outer-most invocation and therefore doesn't discriminate between
//! identifiers. We had to use a different identifier in the nested invocations
//! (`int_type_nested`) than in the code (`int_type`), because otherwise the
//! nested invocation would substitute the substitution identifier, too, instead
//! of only substituting in the nested invocation's substitute.
//!
//! Nested invocation is only possible when using verbose syntax.
//! Additionally, the nested invocations must produce verbose syntax of their
//! parent invocation. However, each nested invocation's private syntax is free
//! to use the short version. Notice in our above example, the nested
//! invocations use short syntax but produce verbose syntax for the outer-most
//! invocation.
//!
//! There is no limit on the depth of nesting, however, as might be clear from
//! our example, it can get complicated to read. Additionally, the syntax used
//! in any invocation that includes a nested invocation must be verbose.
//!
//! Lastly, we should note that we can have nested invocations interleaved with
//! normal substution groups. For example, say we want to implement `IsNegative`
//! for `i8`, but don't want the same for `i16` and `i32`. We could do the
//! following:
//!
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! # use duplicate::duplicate;
//! #[duplicate(
//!   #[                                     // -+
//!     int_type_nested; [u8]; [u16]; [u32]  //  |
//!   ][                                     //  |
//!     [                                    //  | Nested invocation producing 3
//!       int_type [ int_type_nested ]       //  | substitution groups
//!       implementation [ false ]           //  |
//!     ]                                    //  |
//!   ]                                      // -+
//!   [                                      // -+
//!     int_type [ i8 ]                      //  | Substitution group 4
//!     implementation [ *self < 0 ]         //  |
//!   ]                                      // -+
//! )]
//! impl IsNegative for int_type {
//!   fn is_negative(&self) -> bool {
//!     implementation
//!   }
//! }
//!
//! # assert!(!42u8.is_negative());
//! # assert!(!42u16.is_negative());
//! # assert!(!42u32.is_negative());
//! # assert!(!42i8.is_negative());
//! ```
//!
//! # Disclaimer
//!
//! This crate does not try to justify or condone the usage of code duplication
//! instead of proper abstractions.
//! This macro should only be used where it is not possible to reduce code
//! duplication through other means, or where it simply is not worth it.
//!
//! As an example, libraries that have two or more structs/traits with similar
//! APIs might use this macro to test them without having to copy-paste test
//! cases and manually make the needed edits.
use proc_macro::{token_stream::IntoIter, Delimiter, Group, Span, TokenStream, TokenTree};
use proc_macro_error::{
	proc_macro::{Punct, Spacing},
	*,
};
use std::collections::{HashMap, HashSet};

// Tests the crate readme file's Rust examples.
mod crate_readme_test;

/// Duplicates and substitutes given identifiers for different code in each
/// duplicate.
///
/// _Substitution identifiers_ can be inserted into the code. They will be
/// substituted with the different substitution code in each duplicate version
/// of the original code.
///
/// # Short Syntax
/// ```
/// use duplicate::duplicate;
/// trait IsMax {
///   fn is_max(&self) -> bool;
/// }
///
/// #[duplicate(
///   int_type  max_value;
///   [ u8 ]    [ 255 ];
///   [ u16 ]   [ 65_535 ];
///   [ u32 ]   [ 4_294_967_295 ];
/// )]
/// impl IsMax for int_type {
///   fn is_max(&self) -> bool {
///     *self == max_value
///   }
/// }
///
/// assert!(!42u8.is_max());
/// assert!(!42u16.is_max());
/// assert!(!42u32.is_max());
/// ```
/// The implementation of `IsMax` is duplicated 3 times:
///
/// 1. For the type `u8` and the its maximum value `255`.
/// 2. For the type `u16` and the its maximum value `65_535 `.
/// 3. For the type `u32` and the its maximum value `4_294_967_295 `.
///
/// This syntax must start with a list of all identifiers followed by `;`.
/// Then a `;` seperated list of substitution groups must be given (at least 1
/// group). Every group is a list of substitutions, one for each substitution
/// identifier given in the first line.
/// The substitutions must be enclosed in `[]`, `{}`, or `()`, but are otherwise
/// free.
///
/// # Verbose Syntax
///
/// ```
/// use duplicate::duplicate;
/// trait IsMax {
///   fn is_max(&self) -> bool;
/// }
///
/// #[duplicate(
///   [
///     int_type  [ u8 ]
///     max_value [ 255 ]
///   ]
///   [
///     int_type  [ u16 ]
///     max_value [ 65_535 ]
///   ]
///   [
///     max_value [ 4_294_967_295 ]
///     int_type  [ u32 ]
///   ]
/// )]
/// impl IsMax for int_type {
///   fn is_max(&self) -> bool {
///     *self == max_value
///   }
/// }
///
/// assert!(!42u8.is_max());
/// assert!(!42u16.is_max());
/// assert!(!42u32.is_max());
/// ```
/// Has the same functionality as the previous short-syntax example.
///
/// For each duplicate needed, a _substitution group_ must be given enclosed in
/// `[]`, `{}`, or `()`. A substitution group is a set of identifiers and
/// substitution pairs, like in the short syntax, but there can only be one
/// substitution per identifier. All substitution groups must have the same
/// identifiers, however their order is unimportant, as can be seen from the
/// last substitution group above, where `max_value` comes before `int_type`.
///
/// # Nested Invocation
/// ```
/// use duplicate::duplicate;
/// trait IsNegative {
///   fn is_negative(&self) -> bool;
/// }
///
/// #[duplicate(
///   #[                                  // -+
///     int_type_nested;[u8];[u16];[u32]  //  |
///   ][                                  //  |
///     [                                 //  | Nested invocation producing 3
///       int_type [ int_type_nested ]    //  | substitution groups
///       implementation [ false ]        //  |
///     ]                                 //  |
///   ]                                   // -+
///   [                                   // -+
///     int_type [ i8 ]                   //  | Substitution group 4
///     implementation [ *self < 0 ]      //  |
///   ]                                   // -+
/// )]
/// impl IsNegative for int_type {
///   fn is_negative(&self) -> bool {
///     implementation
///   }
/// }
///
/// assert!(!42u8.is_negative());
/// assert!(!42u16.is_negative());
/// assert!(!42u32.is_negative());
/// assert!(!42i8.is_negative());
/// ```
///
/// This implements `IsNegative` 4 times:
///
/// 1. For the type `u8` with the implementation of the method simply returning
/// `false`. 2. For the type `u16` the same way as `u8`.
/// 3. For the type `u32` the same way as `u8` and `u16`.
/// 4. For `i8` with the implementation of the method checking whether it's less
/// than `0`.
///
/// We used `#` to start a _nested invocation_ of the macro. In it, we use the
/// identifier `int_type_nested` to substitute the 3 unsigned integer types into
/// the body of the nested invocation, which is a substitution group for the
/// outer macro invocation. This therefore produces the three substitution
/// groups that makes the outer macro make the duplicates for the unsigned
/// integers.
///
/// This code is identical to the following, which doesn't use nested
/// invocation:
///
/// ```
/// # use duplicate::duplicate;
/// # trait IsNegative {
/// #   fn is_negative(&self) -> bool;
/// # }
/// #[duplicate(
///   [
///     int_type [ u8 ]
///     implementation [ false ]
///   ]
///   [
///     int_type [ u16 ]
///     implementation [ false ]
///   ]
///   [
///     int_type [ u32 ]
///     implementation [ false ]
///   ]
///   [
///     int_type [ i8 ]
///     implementation [ *self < 0 ]
///   ]
/// )]
/// impl IsNegative for int_type {
///   fn is_negative(&self) -> bool {
///     implementation
///   }
/// }
/// # assert!(!42u8.is_negative());
/// # assert!(!42u16.is_negative());
/// # assert!(!42u32.is_negative());
/// # assert!(!42i8.is_negative());
/// ```
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
	let subs = parse_attr(attr, Span::call_site(), disallow_short)?;
	let result = substitute(item, subs);
	Ok(result)
}

fn parse_attr(
	attr: TokenStream,
	stream_span: Span,
	disallow_short: bool,
) -> Result<Vec<HashMap<String, TokenStream>>, (Span, String)>
{
	if identify_syntax(attr.clone(), stream_span, disallow_short)?
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
fn identify_syntax(
	attr: TokenStream,
	stream_span: Span,
	disallow_short: bool,
) -> Result<bool, (Span, String)>
{
	if let Some(token) = next_token(&mut attr.into_iter(), "Could not identify syntax type.")?
	{
		match token
		{
			TokenTree::Group(_) => Ok(true),
			TokenTree::Ident(_) if !disallow_short => Ok(false),
			TokenTree::Punct(p) if is_nested_invocation(&p) => Ok(true),
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
		Err((stream_span, "No substitutions found.".into()))
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
		if let Some(tree) = next_token(&mut iter, "Expected substitution group.")?
		{
			match tree
			{
				TokenTree::Punct(p) if is_nested_invocation(&p) =>
				{
					let hints = "Hint: '#' is a nested invocation of the macro and must therefore \
					             be followed by a group containing the \
					             invocation.\nExample:\n#[\n\tidentifier [ substitute1 ] [ \
					             substitute2 ]\n][\n\tCode to be substituted whenever \
					             'identifier' occurs \n]";
					let nested_attr = parse_group(&mut iter, p.span(), hints)?;
					let nested_subs = parse_attr(nested_attr.stream(), nested_attr.span(), false)?;

					let nested_item = parse_group(&mut iter, nested_attr.span(), hints)?;
					let nested_duplicated = substitute(nested_item.stream(), nested_subs);
					let subs = validate_verbose_attr(nested_duplicated)?;
					sub_groups.extend(subs.into_iter());
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

	let mut substitutions = HashMap::new();
	let mut stream = group.stream().into_iter();

	loop
	{
		if let Some(ident) = next_token(&mut stream, "Epected substitution identifier.")?
		{
			if let TokenTree::Ident(ident) = ident
			{
				let sub = parse_group(
					&mut stream,
					ident.span(),
					"Hint: A substitution identifier should be followed by a group containing the \
					 code to be inserted instead of any occurrence of the identifier.",
				)?;

				let ident_string = ident.to_string();

				// Check have found the same as existing
				if let Some(idents) = existing
				{
					if !idents.contains(&ident_string)
					{
						return Err((
							ident.span(),
							"Unfamiliar substitution identifier. '{}' is not present in previous \
							 substitution groups."
								.into(),
						));
					}
				}
				substitutions.insert(ident_string, sub.stream());
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
					let mut msg: String = "Missing substitutions. Previous substitutions groups \
					                       had the following identifiers not present in this \
					                       group: "
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

fn validate_short_attr(attr: TokenStream)
	-> Result<Vec<(String, Vec<TokenStream>)>, (Span, String)>
{
	if attr.is_empty()
	{
		return Err((Span::call_site(), "No substitutions found.".into()));
	}

	let mut iter = attr.into_iter();
	let (mut result, mut span) = validate_short_get_identifiers(&mut iter, Span::call_site())?;

	loop
	{
		validate_short_get_substitutions(
			&mut iter,
			span,
			result.iter_mut().map(|(_, vec)| {
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

	Ok(result)
}

fn validate_short_get_identifiers(
	iter: &mut IntoIter,
	mut span: Span,
) -> Result<(Vec<(String, Vec<TokenStream>)>, Span), (Span, String)>
{
	let mut result = Vec::new();
	loop
	{
		if let Some(next_token) = next_token(iter, "Expected substitution identifier or ';'.")?
		{
			span = next_token.span();
			match next_token
			{
				TokenTree::Ident(ident) => result.push((ident.to_string(), Vec::new())),
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

fn validate_short_get_substitutions<'a>(
	iter: &mut IntoIter,
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

/// Duplicates the given token stream, substituting any identifiers found.
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

/// Recursively checks the given token for any use of the given substitution
/// identifiers and substitutes them, returning the resulting token stream.
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

/// Tries to parse a valid group from the given token stream iterator, returning
/// the group if successfull.
///
/// If the next token is not a valid group, issues an error, that indicates to
/// the given span and adding the given string to the end of the message.
fn parse_group(iter: &mut IntoIter, parent_span: Span, hints: &str)
	-> Result<Group, (Span, String)>
{
	if let Some(tree) = iter.next()
	{
		check_group(tree, hints)
	}
	else
	{
		return Err((
			parent_span,
			"Unexpected end of macro invocation. Expected '[', '{', or '('.\n".to_string() + hints,
		));
	}
}

/// Ensures the given token is a valid group and if so, returns it.
///
/// If not, issues an error, adding the given hints to the error message.
fn check_group(tree: TokenTree, hints: &str) -> Result<Group, (Span, String)>
{
	if let TokenTree::Group(group) = tree
	{
		check_delimiter(group)
	}
	else
	{
		return Err((
			tree.span(),
			"Unexpected token. Expected '[', '{', or '('.\n".to_string() + hints,
		));
	}
}

/// Checks that the given group's delimiter is a bracket ('[]','{}', or '()').
///
/// If so, returns the same group, otherwise issues an error.
fn check_delimiter(group: Group) -> Result<Group, (Span, String)>
{
	if group.delimiter() == Delimiter::None
	{
		return Err((
			group.span(),
			"Unexpected delimiter for group. Expected '[]','{}', or '()' but received non.".into(),
		));
	}
	Ok(group)
}

/// Checks whether the given punctuation is exactly equal to the given
/// character.
fn punct_is_char(p: &Punct, c: char) -> bool
{
	p.as_char() == c && p.spacing() == Spacing::Alone
}

/// Check whether teh given punctuation is ';'.
fn is_semicolon(p: &Punct) -> bool
{
	punct_is_char(p, ';')
}

/// Checks whether the given punctuation is '#'.
fn is_nested_invocation(p: &Punct) -> bool
{
	punct_is_char(p, '#')
}

/// Gets the next token tree from the iterator.
///
/// If the token is a group without delimiters, the token inside the groups is
/// returned. If the group has more than one token, an error is returned.
fn next_token(iter: &mut IntoIter, err_msg: &str) -> Result<Option<TokenTree>, (Span, String)>
{
	match iter.next()
	{
		Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::None =>
		{
			let mut in_group = group.stream().into_iter();
			let result = in_group.next();
			match in_group.next()
			{
				None => Ok(result),
				// If ends with ';' and nothing else, was a statement including
				// only 1 token, so allow.
				Some(TokenTree::Punct(p)) if is_semicolon(&p) && in_group.next().is_none() =>
				{
					Ok(result)
				},
				_ => Err((group.span(), err_msg.into())),
			}
		},
		token => Ok(token),
	}
}
