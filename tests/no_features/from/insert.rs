#[duplicate::duplicate(
	name [SomeName]
)]//duplicate_end
pub struct name();
//item_end
#[duplicate::duplicate(
	name	[SomeName2];
	ty 		[i32]
)]//duplicate_end
pub struct name(ty);
//item_end
#[duplicate::duplicate(
	name	[SomeName3];
	rf(ty)	[&ty];
)]//duplicate_end
pub struct name(rf([i32]));
//item_end

#[duplicate::duplicate(
	ty 		[i16];
	name; [SomeName41]; [SomeName42]
)]//duplicate_end
pub struct name(ty);
//item_end
