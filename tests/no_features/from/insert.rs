use duplicate::*;
#[substitute_item(
	name [SomeName]
)]//duplicate_end
pub struct name();
//item_end
#[substitute_item(
	name	[SomeName2];
	ty 		[i32]
)]//duplicate_end
pub struct name(ty);
//item_end
#[substitute_item(
	name	[SomeName3];
	rf(ty)	[Vec<ty>];
)]//duplicate_end
pub struct name(rf([i32]));
//item_end

#[duplicate_item(
	ty 		[i16];
	name; [SomeName41]; [SomeName42]
)]//duplicate_end
pub struct name(ty);
//item_end
