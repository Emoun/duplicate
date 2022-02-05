#[duplicate::duplicate_item(
	name;
	duplicate!{[ some_name; [SomeName1]; [SomeName2] ]
		[some_name];
	}
	[SomeName3]
)]//duplicate_end
pub struct name();
//item_end

// Test more than one nesting
#[duplicate::duplicate_item(
	name;
	duplicate!{[ some_name; [SomeName4]; [SomeName5] ]
		[some_name];
	}
	duplicate!{
		[ // Test verbose syntax in nested call
			[ some_name	[SomeName6] ]
			[ some_name	[SomeName7] ]
		]
		[some_name];
	}
)]//duplicate_end
pub struct name();
//item_end

// Test 2 substitution groups in nested invocation.
// Output should be the same as the next test.
#[duplicate::duplicate_item(
	name member;
	duplicate!{ [ some_name; [SomeName8]; [SomeName9] ]
		[some_name] [SomeMember10];
		[some_name] [SomeMember11];
	}
)]//duplicate_end
pub struct name(member);
//item_end

// Test nesting depth of 2.
// Output should be the same as the previous test
#[duplicate::duplicate_item(
	name member;
	duplicate!{[ some_name; [SomeName12]; [SomeName13]]
		duplicate!{[ some_member; [SomeMember14]; [SomeMember15] ]
				[some_name] [some_member];
		}
	}
)]//duplicate_end
pub struct name(member);
//item_end