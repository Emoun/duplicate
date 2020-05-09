use duplicate::duplicate;

#[duplicate(
	name;
	[SomeName];
)]
pub struct name();

#[duplicate(
	name		member;
	[SomeName]	[SomeMember]
)]
pub struct name(member);

#[duplicate(
	name		member;
	[SomeName]	[SomeMember];
	[SomeName2]	[SomeMember2];
)]
pub struct name(member);

#[duplicate(
	module ;
	[ mod1 ];
	[ mod2 ]
)]
mod module {
	use super::*;
	
	#[duplicate(
		name		member;
		[SomeName]	[SomeMember];
		[SomeName2]	[SomeMember2];
	)]
	pub struct name(member);
}

