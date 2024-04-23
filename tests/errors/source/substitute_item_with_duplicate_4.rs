use duplicate::*;
// Test that substitute macros require at least one global substitution
#[substitute_item(
)]//duplicate_end
pub struct name(ty);
//item_end
