use duplicate::duplicate;

// Test single-token argument
#[duplicate(
	fn_name 		refs(type);
	[fn_const_1]	[&type];
	[fn_mut_1]		[&mut type];
)]
fn fn_name(arg: refs([i32])){}

// Test multi-token argument
#[duplicate(
	fn_name 		refs(type);
	[fn_const_2]	[& type];
	[fn_mut_2]		[& mut type];
)]
fn fn_name<'a>(arg: refs([Vec<i32>])){}

// Test multi-argument identifier
#[duplicate(
	fn_name 		refs(lifetime type);
	[fn_const_3]	[& lifetime type];
	[fn_mut_3]		[& lifetime mut type];
)]
fn fn_name<'a>(arg: refs(['a],[i32])){}

// Test multi-argument identifier and multi-token arguments
#[duplicate(
	fn_name 		refs(lifetime type);
	[fn_const_4]	[& lifetime type];
	[fn_mut_4]		[& lifetime mut type];
)]
fn fn_name<'a>(arg: refs(['a],[Result<i32,u8>],)){}

// Test multiple invocations of identifiers with arguments
#[duplicate(
	fn_name 		refs(lifetime type);
	[fn_const_5]	[& lifetime type];
	[fn_mut_5]		[& lifetime mut type];
)]
fn fn_name<'a>(arg: refs(['a], [Result<i32,u8>],))  -> refs(['a], [i32]) {}

// Test identifier with argument declaration can be followed by another identifier.
#[duplicate(
	refs(type)	fn_name;
	[&type] 	[fn_const_6];
	[&mut type] [fn_mut_6];
)]
fn fn_name(arg: refs([i32])){}