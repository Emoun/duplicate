use duplicate::*;
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
			typ [TypeName11]
		]
		typ1 [typ];
	}
	typ2 [TypeName12];
)]//duplicate_end
struct name(typ1,typ2);
//item_end

// Tests nesting between individual substitutions
#[duplicate_item(
	name typ1 typ2;
	[TypeName21]
	duplicate!{
		[
			typ [TypeName22]
		]
		[typ]
	}
	[TypeName23];
)]//duplicate_end
struct name(typ1, typ2);
//item_end

// Tests sequential nesting
#[duplicate_item(
	v1;	[31]; [32];
)]//duplicate_end
#[duplicate_item(
	v2;	[33, v1]; [34, v1];
)]//duplicate_end
impl SomeType<v2> for (){}
//item_end
//item_end

// Tests sequential nesting (3-deep
#[duplicate_item(
	v1;	[41]; [42];
)]//duplicate_end
#[duplicate_item(
	v2;	[43, v1]; [44, v1];
)]//duplicate_end
#[duplicate_item(
	v3;	[45, v2]; [46, v2];
)]//duplicate_end
impl SomeType<v3> for (){}
//item_end
//item_end
//item_end
