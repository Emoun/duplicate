use duplicate::duplicate;

#[duplicate(
	boolean [true]
)]
#[test]
fn duplicate_single(){
	assert!(boolean);
}

#[duplicate(
	name 	[function_name]
	boolean [true]
)]
#[test]
fn name(){
	assert!(boolean);
}

#[duplicate(
	name 	[first_duplicate]	[second_duplicate]
	boolean	[true] 				[true]
)]
#[test]
fn name(){
	assert!(boolean);
}

#[duplicate(
	module [ mod1 ]  [ mod2 ]
)]
mod module {
	use super::*;
	
	#[duplicate(
		name 	[ first_duplicate ]	[ second_duplicate ]
		boolean	[ true ]			[ true ]
	)]
	#[test]
	fn name() {
		assert!(boolean);
	}
}