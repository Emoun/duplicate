use duplicate::duplicate;

#[duplicate(
	[
		boolean [ true ]
	]
)]
#[test]
fn dublicate_single(){
	assert!(boolean);
}

#[duplicate(
	[
		name [ function_name ]
		boolean [ true ]
	]
)]
#[test]
fn name(){
	assert!(boolean);
}

#[duplicate(
	[
		name [ first_duplicate ]
		boolean [ true ]
	]
	[
		name [ second_duplicate ]
		boolean [ true ]
	]
)]
#[test]
fn name(){
	assert!(boolean);
}

#[duplicate(
	[
		module [ mod1 ]
	]
	[
		module [ mod2 ]
	]
)]
mod module {
	use super::*;
	
	#[duplicate(
		[
			name [ first_duplicate ]
			boolean [ true ]
		]
		[
			name [ second_duplicate ]
			boolean [ true ]
		]
	)]
	#[test]
	fn name() {
		assert!(boolean);
	}
}
