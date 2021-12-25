// Test single-token argument
#[duplicate::duplicate_item(
	[
		fn_name		[fn_const_1]
		refs(type)	[&type]
	]
	[
		fn_name		[fn_mut_1]
		refs(type)	[&mut type]
	]
)]//duplicate_end
fn fn_name(arg: refs([i32])){}
//item_end

// Test multi-token argument
#[duplicate::duplicate_item(
	[
		fn_name		[fn_const_2]
		refs(type)	[&type]
	]
	[
		fn_name		[fn_mut_2]
		refs(type)	[&mut type]
	]
)]//duplicate_end
fn fn_name<'a>(arg: refs([Vec<i32>])){}
//item_end

// Test multi-argument identifier
#[duplicate::duplicate_item(
	[
		fn_name					[fn_const_3]
		refs(lifetime, type)	[& 'lifetime type]
	]
	[
		fn_name					[fn_mut_3]
		refs(lifetime, type)	[& 'lifetime mut type]
	]
)]//duplicate_end
fn fn_name<'a>(arg: refs([a],[i32])){}
//item_end

// Test multi-argument identifier and multi-token arguments
#[duplicate::duplicate_item(
	[
		fn_name					[fn_const_4]
		refs(lifetime, type)	[& 'lifetime type]
	]
	[
		fn_name					[fn_mut_4]
		refs(lifetime, type)	[& 'lifetime mut type]
	]
)]//duplicate_end
fn fn_name<'a>(arg: refs([a],[Result<i32,u8>],)){}
//item_end

// Test multiple invocations of identifiers with arguments
#[duplicate::duplicate_item(
	[
		fn_name					[fn_const_5]
		refs(lifetime, type)	[& 'lifetime type]
	]
	[
		fn_name					[fn_mut_5]
		refs(lifetime, type)	[& 'lifetime mut type]
	]
)]//duplicate_end
fn fn_name<'a>(arg: refs([a], [Result<i32,u8>],))  -> refs([a], [i32]) {}
//item_end

// Test the same identifier argument can have different names in different substitution groups.
#[duplicate::duplicate_item(
	[
		fn_name			[fn_const_6]
		refs(type_1)	[&type_1]
	]
	[
		fn_name			[fn_mut_6]
		refs(type_2)	[&mut type_2]
	]
)]//duplicate_end
fn fn_name(arg: refs([i32])){}
//item_end

// Test that identifiers with arguments don't have to come in the same order in different
// substitution groups.
#[duplicate::duplicate_item(
	[
		fn_name		[fn_const_7]
		refs(type)	[&type]
	]
	[
		refs(type)	[&mut type]
		fn_name		[fn_mut_7]
	]
)]//duplicate_end
fn fn_name(arg: refs([i32])){}
//item_end

// Test multiple identifiers with arguments
#[duplicate::duplicate_item(
	[
		fn_name		[fn_const_8]
		refs(type)	[&type]
		arg_2(type)	[&mut type]
	]
	[
		refs(type)	[&mut type]
		fn_name		[fn_mut_8]
		arg_2(type)	[&type]
	]
)]//duplicate_end
fn fn_name(arg: refs([i32]), second_arg: arg_2([i64])){}
//item_end

// Test identifier with argument called inside itself
#[duplicate::duplicate_item(
	[
		fn_name		[fn_const_9]
		refs(type)	[&type]
	]
	[
		refs(type)	[&mut type]
		fn_name		[fn_mut_9]
	]
)]//duplicate_end
fn fn_name(arg: refs([refs([i32])])){}
//item_end