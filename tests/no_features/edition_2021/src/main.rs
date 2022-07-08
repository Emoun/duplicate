/// Used to test that duplicate doesn't force its edition on the expanded code.
/// The following code acceptable in edition 2021 but not the earlier.
/// So if duplicate uses an earlier edition, it shouldn't result in this code being
/// rejected.
#[deny(non_fmt_panics)]
fn main(){
	let a = 42;
	// This allowed in edition 2021 but not <=2018
	// Above deny ensured build fails if this code is treated as edition <=2018
	duplicate::duplicate! { [foo; [];]
		panic!("{a}");
    }
}