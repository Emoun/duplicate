// The following tests all ensure that if multiple substitution identifiers
// can be used to postfix the module, then the first is chosen.
// We have many test here to ensure that if the choice of identifier to use
// is pseudo-random, most likely at least one of them will fail.
use duplicate::*;
// Test 1
#[duplicate_item(
	name			member_type;
	[SomeName11]	[u8];
	[SomeName12]	[u32];
	[SomeName13]	[u64]
)]//duplicate_end
mod module {
	pub struct name(member_type);
}
//item_end

// Test 1, reversed
#[duplicate_item(
	member_type	name;
	[u8]		[SomeName21];
	[u32]		[SomeName22];
	[u64]		[SomeName23]
)]//duplicate_end
mod module {
	pub struct name(member_type);
}
//item_end

// Test 2, the names have been changed from test 1 to have reverse alphabetical order
#[duplicate_item(
	a_name			b_member_type;
	[SomeName31]	[u8];
	[SomeName32]	[u32];
	[SomeName33]	[u64]
)]//duplicate_end
mod module {
	pub struct a_name(b_member_type);
}
//item_end

// Test 2, reversed
#[duplicate_item(
	b_member_type	a_name;
	[u8]			[SomeName41];
	[u32]			[SomeName42];
	[u64]			[SomeName43]
)]//duplicate_end
mod module {
	pub struct a_name(b_member_type);
}
//item_end

// Test 3, 3 valid identifers
#[duplicate_item(
	name			member_type	last_identifier;
	[SomeName51]	[u8]		[OtherIdent];
	[SomeName52]	[u32]		[AnotherIdent];
	[SomeName53]	[u64]		[ZeroIsDefinatelyNotTheLengthOfThisIdent]
)]//duplicate_end
mod module {
	pub struct name(member_type);
}
//item_end

// Test 3, permutation 2
#[duplicate_item(
	member_type	last_identifier 							name;
	[u8]		[OtherIdent]								[SomeName61];
	[u32]		[AnotherIdent]								[SomeName62];
	[u64]		[ZeroIsDefinatelyNotTheLengthOfThisIdent]	[SomeName63]
)]//duplicate_end
mod module {
	pub struct name(member_type);
}
//item_end

// Test 3, permutation 3
#[duplicate_item(
	last_identifier 							name			member_type;
	[OtherIdent]								[SomeName71]	[u8];
	[AnotherIdent]								[SomeName72]	[u32];
	[ZeroIsDefinatelyNotTheLengthOfThisIdent]	[SomeName73]	[u64]
)]//duplicate_end
mod module {
	pub struct name(member_type);
}
//item_end