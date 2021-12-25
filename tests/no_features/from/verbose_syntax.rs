#[duplicate::duplicate_item(
	[
		name	[SomeName]
	]
)]//duplicate_end
pub struct name();
//item_end

#[duplicate::duplicate_item(
	[
		name	[SomeName]
		member	[SomeMember]
	]
)]//duplicate_end
pub struct name(member);
//item_end

#[duplicate::duplicate_item(
	[
		name	[SomeName]
		member	[SomeMember]
	]
	[
		name	[SomeName2]
		member	[SomeMember2]
	]
)]//duplicate_end
pub struct name(member);
//item_end

#[duplicate::duplicate_item(
	[
		module [ mod1 ]
	]
	[
		module [ mod2 ]
	]
)]//duplicate_end
mod module {
	use super::*;
	
	#[duplicate::duplicate_item(
		[
			name	[SomeName]
			member	[SomeMember]
		]
		[
			name	[SomeName2]
			member	[SomeMember2]
		]
	)]//duplicate_end
	pub struct name(member);
	//item_end
}
//item_end

// Test substitution that includes braces
#[duplicate::duplicate_item(
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
