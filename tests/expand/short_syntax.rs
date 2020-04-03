use duplicate::duplicate;

#[duplicate(
	name	[SomeName]
)]
pub struct name();

#[duplicate(
	name	[SomeName]
	member	[SomeMember]
)]
pub struct name(member);

#[duplicate(
	name	[SomeName]		[SomeName2]
	member	[SomeMember]	[SomeMember2]
)]
pub struct name(member);

#[duplicate(
	module [ mod1 ]  [ mod2 ]
)]
mod module {
	use super::*;
	
	#[duplicate(
		name	[SomeName]		[SomeName2]
		member	[SomeMember]	[SomeMember2]
	)]
	pub struct name(member);
}

