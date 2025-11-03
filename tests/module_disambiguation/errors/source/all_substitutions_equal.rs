use duplicate::*;

#[duplicate_item(
		sub_typ 		sub_fn;
		[ SomeType ] 	[ some_fn ];
		[ SomeType  ]	[ some_fn ];
)]//duplicate_end
mod __
{}
//item_end
