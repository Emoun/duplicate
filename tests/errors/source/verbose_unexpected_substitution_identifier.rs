use duplicate::*;
#[duplicate_item(
	[
		name 	[sub1]
	]
	[
		name 	[sub1]
		ty 		[sub2]
	]
)]//duplicate_end
pub struct name(i32);
//item_end
