use crate::substitute::Substitution;
use convert_case::{Case, Casing};
use proc_macro::{Ident, TokenStream, TokenTree};
use std::collections::HashMap;

/// If the given item is a module declaration and the substitutions don't
/// reassign the module identifier for each substitution, this function
/// will try to do so.
pub fn unambiguate_module(module: Ident, substitutions: &mut Vec<HashMap<String, Substitution>>)
{
	let ident = find_simple(substitutions).unwrap();
	// All match
	for group in substitutions.iter_mut()
	{
		let postfix = group[&ident]
			.substitutes_identifier()
			.unwrap()
			.to_string()
			.to_case(Case::Snake);
		let replacement_name = module.to_string() + "_" + &postfix;
		let replacement = Ident::new(&replacement_name, module.span());
		group.insert(
			module.to_string(),
			Substitution::new_simple(TokenStream::from(TokenTree::Ident(replacement))),
		);
	}
}

fn find_simple(substitutions: &mut Vec<HashMap<String, Substitution>>) -> Option<String>
{
	'outer: for ident in substitutions[0].keys()
	{
		for group in substitutions.iter()
		{
			if group[ident].substitutes_identifier().is_none()
			{
				continue 'outer;
			}
		}
		return Some(ident.clone());
	}
	None
}
