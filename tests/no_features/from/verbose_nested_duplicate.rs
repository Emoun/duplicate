use duplicate::*;
#[duplicate_item(
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
#[duplicate_item(
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

trait SomeTrait<T1, T2> {}
// Test 2 substitution groups in nested invocation.
// Output should be the same as the next test.
#[duplicate_item(
	duplicate!{[ typ; [u8]; [u16] ]
		[
			member1	[typ]
			member2	[u32]
		]
		[
			member1	[typ]
			member2	[u64]
		]
	}
)]//duplicate_end
impl SomeTrait<member1, member2> for (){}
//item_end

// Test nesting depth of 2.
// Output should be the same as the previous test
#[duplicate_item(
	duplicate!{[ typ; [i8]; [i16] ]
		duplicate!{[ typ2; [i32]; [i64] ]
			[
				member1	[typ]
				member2	[typ2]
			]
		}
	}
)]//duplicate_end
impl SomeTrait<member1, member2> for (){}
//item_end

#[duplicate_item(
	[
		name		[outer_1]
		some_int	[1]
	]
	[
		name		[outer_2]
		some_int	[2]
	]
)]//duplicate_end
fn name()
{
	substitute! ( [
		sub [	some_int	]
	]
		sub;
	)
}
//item_end