use duplicate::*;
#[duplicate_item(
	[
		name(arg1) 	[sub1(arg1)]
	]
	[
		name 	[sub1]
	]
)]//duplicate_end
pub struct name([i32]);
//item_end
