use duplicate::*;
#[duplicate_item(
	[
		name 	[sub1] *
		ty 		[u32]
	]
)]//duplicate_end
pub struct name(ty);
//item_end
