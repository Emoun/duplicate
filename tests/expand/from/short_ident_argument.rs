use duplicate::duplicate;

#[duplicate(
	fn_name 	refs(type);
	[fn_const]	[&type];
	[fn_mut]	[&mut type];
)]
fn fn_name(arg: refs([i32])){}