use duplicate::*;

#[duplicate_item(
	ty 		[i16];
)]//duplicate_end
pub struct SomeStruct(ty);

//item_end
#[duplicate_item(
Name 	[SomeStruct2];
ty 		[i32];
)]//duplicate_end
pub struct Name(ty);
//item_end

#[duplicate_item(
	Name 			[SomeStruct3];
	ty(extra) 		[extra];
)]//duplicate_end
pub struct Name(ty([&'static i64]));
//item_end
