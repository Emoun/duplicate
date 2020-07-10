use crate::{substitute::Substitution, SubstitutionGroup};
use convert_case::{Case, Casing};
use proc_macro::{Ident, TokenStream, TokenTree};

/// If the given item is a module declaration and the substitutions don't
/// reassign the module identifier for each substitution, this function
/// will try to do so.
pub(crate) fn unambiguate_module(module: Ident, substitutions: &mut Vec<SubstitutionGroup>)
{
	let ident = find_simple(substitutions).unwrap();

	for group in substitutions.iter_mut()
	{
		let postfix = group
			.substitution_of(&ident)
			.unwrap()
			.substitutes_identifier()
			.unwrap()
			.to_string()
			.to_case(Case::Snake);
		let replacement_name = module.to_string() + "_" + &postfix;
		let replacement = Ident::new(&replacement_name, module.span());
		group
			.add_substitution(
				module.clone(),
				Substitution::new_simple(TokenStream::from(TokenTree::Ident(replacement))),
			)
			.unwrap();
	}
}

fn find_simple(substitutions: &mut Vec<SubstitutionGroup>) -> Option<String>
{
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
