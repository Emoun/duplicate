use duplicate::*;
struct Example{one: u8, two: u8}
// Tests nesting in substitutions
#[substitute_item(
	members [
		duplicate!{
			[
				mem; [one]; [two]
			]
			mem: 0,
		}
	];
)]//duplicate_end
impl Example {
	fn inline_new() -> Self {
		Example {members}
	}
	fn attr_new() -> Self {
		Example {members}
	}
}
//item_end

// Tests nesting between global substitutions
#[substitute_item(
	name [StructName1];
	duplicate!{
		[
			nam typ;
			[typ1 ] [u8];
		]
		nam [typ];
	}
	typ2 [u16];
)]//duplicate_end
struct name(typ1,typ2);
//item_end

// Tests nesting between individual substitutions
#[duplicate_item(
	name typ1 typ2;
	[TypeName21]
	duplicate!{
		[
			typ; [u8]
		]
		[typ]
	}
	[u16];
)]//duplicate_end
struct name(typ1, typ2);
//item_end

// Tests sequential nesting
#[duplicate_item(
	v1;	[u8]; [u16];
)]//duplicate_end
#[duplicate_item(
	v2;	[u32, v1]; [u64, v1];
)]//duplicate_end
impl std::error::Error<v2> for (){}
//item_end
//item_end

trait SomeType<T1,T2,T3>{}
// Tests sequential nesting (3-deep)
#[duplicate_item(
	v1;	[u8]; [u16];
)]//duplicate_end
#[duplicate_item(
	v2;	[u32, v1]; [u64, v1];
)]//duplicate_end
#[duplicate_item(
	v3;	[i8, v2]; [i16, v2];
)]//duplicate_end
impl SomeType<v3> for (){}
//item_end
//item_end
//item_end

struct Example2{one: u8}
// Tests nesting substitute! in substitutions
#[substitute_item(
	member [
		substitute!{
			[
				mem [one];
			]
			mem: 0,
		}
	];
)]//duplicate_end
const SOME_STRUCT1: Example2 = Example2{member};
//item_end
// Tests nesting substitute! in duplicate
#[duplicate_item(
	name member;
	[SOME_STRUCT2] [
		substitute!{
			[
				mem [one];
			]
			mem: 1,
		}
	];
	[SOME_STRUCT3] [
		substitute!{
			[
				val [2];
			]
			one: val,
		}
	];
)]//duplicate_end
const name: Example2 = Example2{member};
//item_end