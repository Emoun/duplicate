// Tests that a missing type originating in a substitution triggers error highlight pointing to
// the substitution code
#[duplicate::duplicate_item(
    SomeType(t);
    [Type::<t>];
)]
fn main()
{
	let v: SomeType([i8]);
}