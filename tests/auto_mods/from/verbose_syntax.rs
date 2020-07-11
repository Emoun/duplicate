use duplicate::duplicate;

// Tests module names are postfixed from substitution identifier
#[duplicate(
	[
		name [SomeName11]
	]
	[
		name [SomeName12]
	]
	[
		name [SomeName13]
	]
)]
mod module {
	pub struct name();
}

// Tests if multiple identifiers are given, the first identifier who's substitutions
// all are simple identifiers (and nothing else) is chosen
#[duplicate(
	[
		member_type	[Vec<()>]
		name 		[SomeName21]
	]
	[
		member_type	[u32]
		name 		[SomeName22]
	]
	[
		member_type	[u64]
		name 		[SomeName23]
	]
)]
mod module {
	pub struct name(member_type);
}

// Tests if multiple identifiers are given, the first identifier who's substitutions
// all are simple identifiers is chosen
#[duplicate(
	[
		member_type		[u8]
		filler_ident	[SomeIdent]
		name 			[SomeName31]
	]
	[
		member_type		[<SomeType as Trait>::SocType]
		filler_ident	[SomeOtherIdent]
		name 			[SomeName32]
	]
	[
		member_type		[u64]
		filler_ident	[Not<An::Identifier>]
		name 			[SomeName33]
	]
)]
mod module {
	pub struct name(member_type);
}