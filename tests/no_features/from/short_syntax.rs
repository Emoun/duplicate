use duplicate::*;
#[duplicate_item(
	name;
	[SomeName1];
)]//duplicate_end
pub struct name();
//item_end

#[duplicate_item(
	name		member;
	[SomeName2]	[u8]
)]//duplicate_end
pub struct name(member);
//item_end

#[duplicate_item(
	name		member;
	[SomeName3]	[u8];
	[SomeName4]	[u16];
)]//duplicate_end
pub struct name(member);
//item_end

#[duplicate_item(
	module ;
	[ mod1 ];
	[ mod2 ]
)]//duplicate_end
mod module {
	use super::*;
	
	// We add a space so that the test setup doesn't
	// recognize it and try to change it to a `duplicate` call
	#[ duplicate_item(
		name		member;
		[SomeName5]	[u8];
		[SomeName6]	[u16];
	)]
	pub struct name(member);
	
	duplicate!{
		[
			name		member;
			[SomeName7]	[u32];
			[SomeName8]	[u64];
		]
		pub struct name(member);
	}
}
//item_end

// Test substitution that includes braces
#[duplicate_item(
	fn_name 		var;
	[ fn_name_1 ]	[ std::io::empty() ];
	[ fn_name_2 ]	[ [4;0] ];
	[ fn_name_3 ]	[ {} ];
)]//duplicate_end
fn fn_name() {
	let _ = var;
}
//item_end