use crate::{substitute::Substitution, DuplicationDefinition};
use heck::SnakeCase;
use proc_macro::{Ident, Span, TokenStream, TokenTree};

/// If the given item is a module declaration and the substitutions don't
/// reassign the module identifier for each substitution, this function
/// will try to do so.
pub(crate) fn disambiguate_module(
	module: Ident,
	dup_def: &mut DuplicationDefinition,
) -> Result<(), (Span, String)>
{
	if let Some(ident) = find_simple(dup_def)
	{
		for group in dup_def.duplications.iter_mut()
		{
			let postfix = group
				.substitution_of(&ident)
				.unwrap()
				.substitutes_identifier()
				.unwrap()
				.to_string()
				.to_snake_case();
			let replacement_name = module.to_string() + "_" + &postfix;
			let replacement = Ident::new(&replacement_name, module.span());
			group
				.add_substitution(
					module.clone(),
					Substitution::new_simple(TokenStream::from(TokenTree::Ident(replacement))),
				)
				.unwrap();
		}
		Ok(())
	}
	else
	{
		Err((
			module.span(),
			"Was unable to find a suitable substitution identifier to postfix on the module's \
			 name.\nHint: If a substitution identifier's substitutions all consist of a single \
			 identifier and nothing, they will automatically be postfixed on the module name to \
			 make them unique."
				.into(),
		))
	}
}

fn find_simple(dup_def: &mut DuplicationDefinition) -> Option<String>
{
	let substitutions = &mut dup_def.duplications;
	'outer: for ident in substitutions[0].identifiers_ordered()
	{
		for group in substitutions.iter()
		{
			if group
				.substitution_of(ident)
				.unwrap()
				.substitutes_identifier()
				.is_none()
			{
				continue 'outer;
			}
		}
		return Some(ident.clone());
	}
	None
}
