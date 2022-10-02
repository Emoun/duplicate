use crate::{
	token_iter::{is_ident, SubGroupIter},
	Result, SubstitutionGroup, TokenIter,
};
use heck::ToSnakeCase;
use proc_macro::{Ident, Span, TokenStream, TokenTree};

/// Finds a substitution identifier whose substitutions only contain one
/// identifier and nothing else for all duplicates.
pub(crate) fn find_simple<'a>(
	substitutions: impl Iterator<Item = &'a SubstitutionGroup> + Clone,
	mod_span: Span,
) -> Result<String>
{
	let mut substitutions = substitutions.peekable();
	if substitutions.peek().is_none()
	{
		// No duplications are made, so either the module doesn't need disambiguation
		// (as even with global substitutions only 1 duplicate will be made)
		// or the invocation will fails somewhere else (from the lack of substitution
		// groups)
		return Ok("".into());
	}
	'outer: for ident in substitutions.peek().unwrap().identifiers_ordered()
	{
		for group in substitutions.clone()
		{
			let substitution = group.substitution_of(ident).unwrap();
			if substitution.substitutes_identifier().is_none()
			{
				continue 'outer;
			}
		}
		return Ok(ident.clone());
	}
	Err((
		mod_span,
		"Was unable to find a suitable substitution identifier to postfix on the module's \
		 name.\nHint: If a substitution identifier's substitutions all consist of a single \
		 identifier and nothing, they will automatically be postfixed on the module name to make \
		 them unique."
			.into(),
	))
}

/// If the next token is the 'mod' keyword, substitutes the following module
/// name with its disambiguation, returning 'mod' plus the disambiguation.
pub(crate) fn try_substitute_mod<'a, T: SubGroupIter<'a>>(
	// If Some(), then tries to disambiguate, otherwise doesn't.
	//
	// First is the module name to disambiguate, then the substitution identifier to use
	// for disambiguation.
	mod_and_postfix_sub: &Option<(Ident, String)>,
	substitutions: &SubstitutionGroup,
	// The item being substituted. Will consume 'mod' and the following name if successful
	item_iter: &mut TokenIter<'a, T>,
) -> TokenStream
{
	let mut result = TokenStream::new();
	if let Some((mod_name, mod_sub_ident)) = mod_and_postfix_sub
	{
		item_iter
			.extract_simple(|t| is_ident(t, Some("mod")), |t| t, None)
			.map_or((), |mod_keyword| {
				result.extend(Some(mod_keyword).into_iter());

				// Consume mod name (since we will replace it)
				let mod_name_t = item_iter.next_fallible().unwrap().unwrap();

				let postfix = substitutions
					.substitution_of(&mod_sub_ident)
					.unwrap()
					.substitutes_identifier()
					.unwrap()
					.to_string()
					.to_snake_case();
				let replacement_name = mod_name.to_string() + "_" + &postfix;
				let replacement = Ident::new(&replacement_name, TokenTree::from(mod_name_t).span());
				result.extend(Some(TokenTree::Ident(replacement)).into_iter());
			});
	}
	result
}
