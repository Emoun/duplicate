use duplicate::duplicate;

#[duplicate(
	[
		name	[SomeName]
	]
)]
pub struct name();

#[duplicate(
	[
		name	[SomeName]
		member	[SomeMember]
	]
)]
pub struct name(member);

#[duplicate(
	[
		name	[SomeName]
		member	[SomeMember]
	]
	[
		name	[SomeName2]
		member	[SomeMember2]
	]
)]
pub struct name(member);

#[duplicate(
	[
		module [ mod1 ]
	]
	[
		module [ mod2 ]
	]
)]
mod module {
	use super::*;
	
	#[duplicate(
		[
			name	[SomeName]
			member	[SomeMember]
		]
		[
			name	[SomeName2]
			member	[SomeMember2]
		]
	)]
	pub struct name(member);
}

// Test substitution that includes braces
#[duplicate(
	[
		fn_name [ fn_name_1 ]
		var		[ Struct() ]
	]
	[
		fn_name [ fn_name_2 ]
		var		[ array[4] ]
	]
	[
		fn_name [ fn_name_3 ]
		var		[ Struct{} ]
	]
)]
fn fn_name() {
	let _ = var;
}
