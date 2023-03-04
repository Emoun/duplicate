use duplicate::*;

#[duplicate_item(
	refs(T); [& T]; [T]
)]//duplicate_end
fn from(x: refs(Bits<1, false>)) -> bool {
	x.value == 1
}
//item_end
