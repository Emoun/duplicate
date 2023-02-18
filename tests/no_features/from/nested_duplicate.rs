use duplicate::*;
struct Example{one: u8, two: u8}
// Tests nesting in substitutions
#[duplicate_item(
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
#[duplicate_item(
	name [StructName1];
	duplicate!{
		[
			typ [u8]
		]
		typ1 [typ];
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
			typ [u8]
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
