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

#[substitute_item(
	ty 		[u16];
)]//duplicate_end
pub struct SomeStruct4(ty);
//item_end

#[substitute_item(
	Name 	[SomeStruct5];
	ty 		[u32];
)]//duplicate_end
pub struct Name(ty);
//item_end

#[substitute_item(
	Name 			[SomeStruct6];
	ty(extra) 		[extra];
)]//duplicate_end
pub struct Name(ty([&'static u64]));
//item_end