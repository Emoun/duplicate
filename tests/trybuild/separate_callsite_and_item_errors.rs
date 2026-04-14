use duplicate::*;

#[duplicate_item(
    ResultType;
    [i8];
)]
fn main() {
	let _: () = ResultType::try_from(0i32);
}
