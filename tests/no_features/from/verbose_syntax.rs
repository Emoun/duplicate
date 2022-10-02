use duplicate::*;
#[duplicate_item(
	[
		name	[SomeName1]
	]
)]//duplicate_end
pub struct name();
//item_end

#[duplicate_item(
	[
		name	[SomeName2]
		member	[SomeMember2]
	]
)]//duplicate_end
pub struct name(member);
//item_end

#[duplicate_item(
	[
		name	[SomeName3]
		member	[SomeMember3]
	]
	[
		name	[SomeName4]
		member	[SomeMember4]
	]
)]//duplicate_end
pub struct name(member);
//item_end

#[duplicate_item(
	[
		module [ mod1 ]
	]
	[
		module [ mod2 ]
	]
)]//duplicate_end
mod module {
	use super::*;
	
	// We add a space so that the test setup doesn't
	// recognize it and try to change it to a `duplicate` call
	#[ duplicate_item(
		[
			name	[SomeName5]
			member	[SomeMember5]
		]
		[
			name	[SomeName6]
			member	[SomeMember6]
		]
	)]
	pub struct name(member);
	
	duplicate!{
		[
			[
				name	[SomeName7]
				member	[SomeMember7]
			]
			[
				name	[SomeName8]
				member	[SomeMember8]
			]
		]
		pub struct name(member);
	}
}
//item_end

// Test substitution that includes braces
#[duplicate_item(
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
)]//duplicate_end
fn fn_name() {
	let _ = var;
}
//item_end
