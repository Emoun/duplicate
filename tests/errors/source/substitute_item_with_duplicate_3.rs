use duplicate::*;
// Tests that if the globals are followed by verbose syntax, the hint refers to 'duplicate'
#[substitute_item(
	name 	[sub1];
	ty 		[u32];
	[
		name2 [sub2]
	]
	[
		name2 [sub3]
	]
)]//duplicate_end
pub struct name(ty);
//item_end
