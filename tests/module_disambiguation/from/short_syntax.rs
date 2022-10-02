use duplicate::*;
// Tests module names are postfixed from substitution identifier
#[duplicate_item(
	name;
	[SomeName11];
	[SomeName12];
	[SomeName13]
)]//duplicate_end
mod module {
	pub struct name();
}
//item_end

// Tests if multiple identifiers are given, the first identifier who's substitutions
// all are simple identifiers (and nothing else) is chosen
#[duplicate_item(
	member_type	name;
	[Vec<()>]	[SomeName21];
	[u32]		[SomeName22];
	[u64]		[SomeName23]
)]//duplicate_end
mod module {
	pub struct name(member_type);
}
//item_end

// Tests if multiple identifiers are given, the first identifier who's substitutions
// all are simple identifiers is chosen
#[duplicate_item(
	member_type						filler_ident			name;
	[u8]							[SomeIdent]				[SomeName31];
	[<SomeType as Trait>::SocType]	[SomeOtherIdent]		[SomeName32];
	[u64]							[Not<An::Identifier>]	[SomeName33]
)]//duplicate_end
mod module {
	pub struct name(member_type);
}
//item_end

// Tests only the module name is disambiguated and not any identifiers used inside it.
#[duplicate_item(
	name;
	[SomeName41];
	[SomeName42];
)]//duplicate_end
mod module {
	pub struct module();
}
//item_end