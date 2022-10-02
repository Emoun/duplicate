use duplicate::*;
pub struct SomeName1();
pub struct SomeName2(SomeMember2);
pub struct SomeName3(SomeMember3);
pub struct SomeName4(SomeMember4);
mod mod1 {
    use super::*;
    pub struct SomeName5(SomeMember5);
    pub struct SomeName6(SomeMember6);
    pub struct SomeName7(SomeMember7);
    pub struct SomeName8(SomeMember8);
}
mod mod2 {
    use super::*;
    pub struct SomeName5(SomeMember5);
    pub struct SomeName6(SomeMember6);
    pub struct SomeName7(SomeMember7);
    pub struct SomeName8(SomeMember8);
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