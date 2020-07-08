use duplicate::duplicate;
	
#[duplicate(
	name;
	[SomeName1];
	[SomeName2];
	[SomeName3]
)]
mod module {
	pub struct name();
}
