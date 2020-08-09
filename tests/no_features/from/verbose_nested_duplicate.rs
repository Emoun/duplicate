#[duplicate::duplicate(
	#[
		some_name;	[SomeName1];	[SomeName2]
	][
		[
			name	[some_name]
		]
	]
	[
		name	[SomeName3]
	]
)]//duplicate_end
pub struct name();
//item_end

// Test more than one nesting
#[duplicate::duplicate(
	#[
		some_name;	[SomeName1];	[SomeName2]
	][
		[
			name	[some_name]
		]
	]
	#[ // Test verbose syntax in nested call
		[
			some_name	[SomeName3]
		]
		[
			some_name	[SomeName4]
		]
	][
		[
			name	[some_name]
		]
	]
)]//duplicate_end
pub struct name();
//item_end

// Test 2 substitution groups in nested invocation.
// Output should be the same as the next test.
#[duplicate::duplicate(
	#[
		some_name;	[SomeName1];	[SomeName2]
	][
		[
			name	[some_name]
			member	[SomeMember1]
		]
		[
			name	[some_name]
			member	[SomeMember2]
		]
	]
)]//duplicate_end
pub struct name(member);
//item_end

// Test nesting depth of 2.
// Output should be the same as the previous test
#[duplicate::duplicate(
	#[
		some_name;	[SomeName1];	[SomeName2]
	][
		#[
			some_member;	[SomeMember1];	[SomeMember2]
		][
			[
				name	[some_name]
				member	[some_member]
			]
		]
	]
)]//duplicate_end
pub struct name(member);
//item_end

