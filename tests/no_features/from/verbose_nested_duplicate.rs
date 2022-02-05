#[duplicate::duplicate_item(
	duplicate!{[ some_name; [SomeName1]; [SomeName2] ]
		[ name	[some_name] ]
	}
	[
		name	[SomeName3]
	]
)]//duplicate_end
pub struct name();
//item_end

// Test more than one nesting
#[duplicate::duplicate_item(
	duplicate!{[ some_name; [SomeName4]; [SomeName5] ]
		[ name	[some_name] ]
	}
	duplicate!{
		[ // Test verbose syntax in nested call
			[ some_name	[SomeName6] ]
			[ some_name	[SomeName7] ]
		]
		[ name	[some_name] ]
	}
)]//duplicate_end
pub struct name();
//item_end

// Test 2 substitution groups in nested invocation.
// Output should be the same as the next test.
#[duplicate::duplicate_item(
	duplicate!{[ some_name; [SomeName8]; [SomeName9] ]
		[
			name	[some_name]
			member	[SomeMember10]
		]
		[
			name	[some_name]
			member	[SomeMember11]
		]
	}
)]//duplicate_end
pub struct name(member);
//item_end

// Test nesting depth of 2.
// Output should be the same as the previous test
#[duplicate::duplicate_item(
	duplicate!{[ some_name; [SomeName12]; [SomeName13] ]
		duplicate!{[ some_member; [SomeMember14]; [SomeMember15] ]
			[
				name	[some_name]
				member	[some_member]
			]
		}
	}
)]//duplicate_end
pub struct name(member);
//item_end

