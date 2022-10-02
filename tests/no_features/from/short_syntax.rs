use duplicate::*;
#[duplicate_item(
	name;
	[SomeName1];
)]//duplicate_end
pub struct name();
//item_end

#[duplicate_item(
	name		member;
	[SomeName2]	[SomeMember2]
)]//duplicate_end
pub struct name(member);
//item_end

#[duplicate_item(
	name		member;
	[SomeName3]	[SomeMember3];
	[SomeName4]	[SomeMember4];
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
		[SomeName5]	[SomeMember5];
		[SomeName6]	[SomeMember6];
	)]
	pub struct name(member);
	
	duplicate!{
		[
			name		member;
			[SomeName7]	[SomeMember7];
			[SomeName8]	[SomeMember8];
		]
		pub struct name(member);
	}
}
//item_end

// Test substitution that includes braces
#[duplicate_item(
	fn_name 		var;
	[ fn_name_1 ]	[ Struct() ];
	[ fn_name_2 ]	[ array[4] ];
	[ fn_name_3 ]	[ Struct{} ];
)]//duplicate_end
fn fn_name() {
	let _ = var;
}
//item_end