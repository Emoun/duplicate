use duplicate::duplicate;
pub struct SomeName();
pub struct SomeName(SomeMember);
pub struct SomeName(SomeMember);
pub struct SomeName2(SomeMember2);
mod mod1 {
    use super::*;
    pub struct SomeName(SomeMember);
    pub struct SomeName2(SomeMember2);
}
mod mod2 {
    use super::*;
    pub struct SomeName(SomeMember);
    pub struct SomeName2(SomeMember2);
}
fn fn_name_1() {
    let _ = Struct();
}
fn fn_name_2() {
    let _ = array[4];
}
fn fn_name_3() {
    let _ = Struct {};
}