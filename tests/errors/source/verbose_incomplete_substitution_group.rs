use duplicate::*;
#[duplicate_item(
	[
		name 	[sub1]
		ty 		[sub2]
	]
	[
		name [sub1]
	]
)]//duplicate_end
pub struct name(ty);
//item_end
