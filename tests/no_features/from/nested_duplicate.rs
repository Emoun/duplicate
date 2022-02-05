// Tests nesting in substitutions
#[duplicate::duplicate_item(
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
#[duplicate::duplicate_item(
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
#[duplicate::duplicate_item(
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
