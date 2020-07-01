// These tests ensure that the verbose syntax works if it
// was produced from the expansion of a macro_rules macro.
//
// Each test consists of a macro_rules declaration which uses
// some specific macro variable type to no_features to the duplicate invocation.
// Then the created macro is invoked.
use duplicate::duplicate;

macro_rules! test_ident_from_macro_variable{
	{ $name:ident } => {
		#[duplicate(
			[
				$name	[SomeName1]
			]
		)]
		pub struct $name();
	}
}
test_ident_from_macro_variable!(name);

macro_rules! test_2_idents_from_macro_variable{
	{ $($idents:ident)*,  $($tts:tt)*} => {
		#[duplicate(
			[
				$($idents[$tts])*
			]
		)]
		pub struct name(member);
	}
}
test_2_idents_from_macro_variable!(name member, SomeName2 SomeMember2);

macro_rules! test_ident_from_macro_path_variable{
	{ $name:path } => {
		#[duplicate(
			[
				$name	[SomeMember3]
			]
		)]
		pub struct SomeName3($name);
	}
}
test_ident_from_macro_path_variable!(name);

macro_rules! test_ident_from_macro_expr_variable{
	{ $name:expr } => {
		#[duplicate(
			[
				$name	[SomeValue4]
			]
		)]
		const SomeName4: () = $name;
	}
}
test_ident_from_macro_expr_variable!(name);

macro_rules! test_ident_from_macro_type_variable{
	{ $name:ty } => {
		#[duplicate(
			[
				$name	[SomeMember5]
			]
		)]
		pub struct SomeName5($name);
	}
}
test_ident_from_macro_type_variable!(name);

macro_rules! test_ident_from_macro_pattern_variable{
	{ $name:pat } => {
		#[duplicate(
			[
				$name	[SomeName6]
			]
		)]
		fn some_fn6(){
			let $name;
		}
	}
}
test_ident_from_macro_pattern_variable!(name);

macro_rules! test_ident_from_macro_statement_variable{
	{ $name:stmt } => {
		#[duplicate(
			[
				$name	[SomeName7]
			]
		)]
		fn some_fn7(){
			$name
		}
	}
}
test_ident_from_macro_statement_variable!(name);

macro_rules! test_ident_from_macro_token_tree_variable{
	{ $name:tt } => {
		#[duplicate(
			[
				$name	[SomeName8]
			]
		)]
		pub struct $name();
	}
}
test_ident_from_macro_token_tree_variable!(name);