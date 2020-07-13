use duplicate::duplicate;

// Tests module names are postfixed from substitution identifier
#[duplicate(
	name;
	[SomeName11];
	[SomeName12];
	[SomeName13]
)]
mod module {
	pub struct name();
}

// Tests if multiple identifiers are given, the first identifier who's substitutions
// all are simple identifiers (and nothing else) is chosen
#[duplicate(
	member_type	name;
	[Vec<()>]	[SomeName21];
	[u32]		[SomeName22];
	[u64]		[SomeName23]
)]
mod module {
	pub struct name(member_type);
}

// Tests if multiple identifiers are given, the first identifier who's substitutions
// all are simple identifiers is chosen
#[duplicate(
	member_type						filler_ident			name;
	[u8]							[SomeIdent]				[SomeName31];
	[<SomeType as Trait>::SocType]	[SomeOtherIdent]		[SomeName32];
	[u64]							[Not<An::Identifier>]	[SomeName33]
)]
mod module {
	pub struct name(member_type);
}