use duplicate::*;
// Tests that if the globals are followed by short syntax, the hint refers to 'duplicate'
#[substitute_item(
	name 	[sub1];
	ty 		[u32];
	dup_sub;
	[i8];
	[i16];
)]//duplicate_end
pub struct name(ty);
//item_end
