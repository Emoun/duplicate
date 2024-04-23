use duplicate::*;
// Test that substitute macros look for inline substitutions
#[substitute_item(
	name 	[sub1];
	ty 		[u32];
	123
)]//duplicate_end
pub struct name(ty);
//item_end
