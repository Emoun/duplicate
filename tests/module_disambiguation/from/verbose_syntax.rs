use duplicate::*;
// Tests module names are postfixed from substitution identifier
#[duplicate_item(
	[
		name [SomeName11]
	]
	[
		name [SomeName12]
	]
	[
		name [SomeName13]
	]
)]//duplicate_end
mod module {
	pub struct name();
}
//item_end

// Tests if multiple identifiers are given, the first identifier who's substitutions
// all are simple identifiers (and nothing else) is chosen
#[duplicate_item(
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
)]//duplicate_end
mod module {
	pub struct name(member_type);
}
//item_end

// Tests if multiple identifiers are given, the first identifier who's substitutions
// all are simple identifiers is chosen
#[duplicate_item(
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
)]//duplicate_end
mod module {
	pub struct name(member_type);
}
//item_end

// Tests only the module name is disambiguated and not any identifiers used inside it.
#[duplicate_item(
	[
		name [SomeName41]
	]
	[
		name [SomeName42]
	]
)]//duplicate_end
mod module {
	pub struct module();
}
//item_end