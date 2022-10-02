use duplicate::*;
// Test single-token argument
#[duplicate_item(
	fn_name 		refs(type);
	[fn_const_1]	[&type];
	[fn_mut_1]		[&mut type];
)]//duplicate_end
fn fn_name(arg: refs([i32])){}
//item_end

// Test multi-token argument
#[duplicate_item(
	fn_name 		refs(type);
	[fn_const_2]	[& type];
	[fn_mut_2]		[& mut type];
)]//duplicate_end
fn fn_name<'a>(arg: refs([Vec<i32>])){}
//item_end

// Test multi-argument identifier
#[duplicate_item(
	fn_name 		refs(lifetime, type);
	[fn_const_3]	[& 'lifetime type];
	[fn_mut_3]		[& 'lifetime mut type];
)]//duplicate_end
fn fn_name<'a>(arg: refs([a],[i32])){}
//item_end

// Test multi-argument identifier and multi-token arguments
#[duplicate_item(
	fn_name 		refs(lifetime, type);
	[fn_const_4]	[& 'lifetime type];
	[fn_mut_4]		[& 'lifetime mut type];
)]//duplicate_end
fn fn_name<'a>(arg: refs([a],[Result<i32,u8>],)){}
//item_end

// Test multiple invocations of identifiers with arguments
#[duplicate_item(
	fn_name 		refs(lifetime, type);
	[fn_const_5]	[& 'lifetime type];
	[fn_mut_5]		[& 'lifetime mut type];
)]//duplicate_end
fn fn_name<'a>(arg: refs([a], [Result<i32,u8>],))  -> refs([a], [i32]) {}
//item_end

// Test identifier with argument declaration can be followed by another identifier.
#[duplicate_item(
	refs(type)	fn_name;
	[&type] 	[fn_const_6];
	[&mut type]	[fn_mut_6];
)]//duplicate_end
fn fn_name(arg: refs([i32])){}
//item_end

// Test identifier with argument called inside itself
#[duplicate_item(
	refs(type)	fn_name;
	[&type] 	[fn_const_7];
	[&mut type] [fn_mut_7];
)]//duplicate_end
fn fn_name(arg: refs([refs([i32])])){}
//item_end
