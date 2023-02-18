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
		member	[u8]
	]
)]//duplicate_end
pub struct name(member);
//item_end

#[duplicate_item(
	[
		name	[SomeName3]
		member	[u8]
	]
	[
		name	[SomeName4]
		member	[u16]
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
			member	[u8]
		]
		[
			name	[SomeName6]
			member	[u16]
		]
	)]
	pub struct name(member);
	
	duplicate!{
		[
			[
				name	[SomeName7]
				member	[u32]
			]
			[
				name	[SomeName8]
				member	[u64]
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
		var		[ std::io::empty() ]
	]
	[
		fn_name [ fn_name_2 ]
		var		[ [4; 0] ]
	]
	[
		fn_name [ fn_name_3 ]
		var		[ {} ]
	]
)]//duplicate_end
fn fn_name() {
	let _ = var;
}
//item_end
