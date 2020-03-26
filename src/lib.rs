use proc_macro::{TokenStream, TokenTree, Delimiter, Span, Group};
use proc_macro_error::*;
use std::collections::{HashMap, HashSet};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn duplicate(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let valid = validate_attr(attr);
    if let Err(err) = valid {
        abort!(err.0, err.1);
    }
    let result = substitute(item, valid.unwrap());
    result
}

fn validate_attr(attr: TokenStream) -> Result<Vec<HashMap<String, TokenStream>>, (Span, String)>
{
    if attr.is_empty() {
        return Err((Span::call_site(), "No substitutions found.".into()));
    }
    
    let mut sub_groups = Vec::new();
    let mut iter = attr.into_iter();
    
    let mut substitution_ids = None;
    loop {
        if let Some(tree) = iter.next() {
            sub_groups.push(extract_substitutions(tree, &substitution_ids)?);
            if None == substitution_ids {
                substitution_ids = Some(sub_groups[0].keys().cloned().collect())
            }
        } else {
            break;
        }
    }
    
    
    
    Ok(sub_groups)
}

fn extract_substitutions(tree: TokenTree, existing: &Option<HashSet<String>>) -> Result<HashMap<String, TokenStream>, (Span, String)>
{
    // Must get span now, before it's corrupted.
    let tree_span = tree.span();
    if let TokenTree::Group(group) = tree {
        if Delimiter::None == group.delimiter() {
            panic!("Expected group in delimiters, got group without.");
        }
        
        if group.stream().into_iter().count() == 0 {
            return Err((tree_span, "No substitution groups found.".into()));
        }
        
        let mut substitutions = HashMap::new();
        let mut stream = group.stream().into_iter();
        
        loop {
            if let Some(ident) = stream.next(){
                let sub = stream.next().ok_or((tree_span, "Unexpected end of substitution group. Substitution identifier must be followed by the substitute as a delimited group.".into()))?;
    
                if let TokenTree::Ident(ident) = ident {
                    if let TokenTree::Group(sub) = sub {
                        if Delimiter::None == sub.delimiter() {
                            panic!("Expected substituion group using delimiters, got group without.");
                        }
    
                        let ident_string = ident.to_string();
                        
                        // Check have found the same as existing
                        if let Some(idents) = existing {
                            if !idents.contains(&ident_string) {
                                return Err((ident.span(),
                                "Unfamiliar substitution identifier. '{}' is not present in previous substitution groups.".into()))
                            }
                        }
                        substitutions.insert(ident_string, sub.stream());
                    } else {
                        return Err(( sub.span(), format!("Expected substitution as a delimited group. E.g. [ .. ].") ));
                    }
                } else {
                    return Err((ident.span(), "Expected substitution identifier, got something else.".into()))
                }
            } else {
                // Check no substitution idents are missing.
                if let Some(idents) = existing {
                    let sub_idents = substitutions.keys().cloned().collect();
                    let diff: Vec<_> = idents.difference(&sub_idents).collect();
                    
                    if diff.len() > 0 {
                        let mut msg: String = "Missing substitutions. Previous substitutions groups had the following identifiers not present in this group: ".into();
                        for ident in diff {
                            msg.push_str("'");
                            msg.push_str(&ident.to_string());
                            msg.push_str("' ");
                        }
                        
                        return Err( (tree_span, msg) )
                    }
                }
                break;
            }
        }
        Ok(substitutions)
    } else {
        Err(( tree_span, format!("Expected substitution group, got: {}", tree) ))
    }
}

fn substitute(item: TokenStream, groups: Vec<HashMap<String, TokenStream>>) -> TokenStream{

    let mut result = TokenStream::new();
    
    for substitutions in groups {
        for token in item.clone().into_iter() {
            result.extend(substitute_token_tree(token, &substitutions))
        }
    }
    
    result
}

fn substitute_token_tree(tree: TokenTree, subtitutions: &HashMap<String, TokenStream>) -> TokenStream
{
    let mut result = TokenStream::new();
    match tree {
        TokenTree::Ident(ident) => {
            if let Some(stream) = subtitutions.get(&ident.to_string()){
                result.extend(stream.clone().into_iter());
            } else {
                result.extend(TokenStream::from(TokenTree::Ident(ident)).into_iter());
            }
        },
        TokenTree::Group(group) => {
            let mut substituted = TokenStream::new();
            for token in group.stream().into_iter() {
                substituted.extend(substitute_token_tree(token, subtitutions))
            }
            result.extend(TokenStream::from(TokenTree::Group(Group::new(group.delimiter(), substituted))).into_iter());
        },
        _  => result.extend(TokenStream::from(tree).into_iter())
    }
    result
}