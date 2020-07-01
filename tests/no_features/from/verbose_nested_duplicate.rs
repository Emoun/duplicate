use duplicate::duplicate;

#[duplicate(
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
)]
pub struct name();

// Test more than one nesting
#[duplicate(
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
)]
pub struct name();

// Test 2 substitution groups in nested invocation.
// Output should be the same as the next test.
#[duplicate(
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
)]
pub struct name(member);

// Test nesting depth of 2.
// Output should be the same as the previous test
#[duplicate(
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
)]
pub struct name(member);

