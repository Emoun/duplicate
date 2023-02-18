use duplicate::*;
pub struct SomeName1();
pub struct SomeName2(u8);
pub struct SomeName3(u8);
pub struct SomeName4(u16);
mod mod1 {
    use super::*;
    pub struct SomeName5(u8);
    pub struct SomeName6(u16);
    pub struct SomeName7(u32);
    pub struct SomeName8(u64);
}
mod mod2 {
    use super::*;
    pub struct SomeName5(u8);
    pub struct SomeName6(u16);
    pub struct SomeName7(u32);
    pub struct SomeName8(u64);
}
fn fn_name_1() {
    let _ = std::io::empty();
}
fn fn_name_2() {
    let _ = [4; 0];
}
fn fn_name_3() {
    let _ = {};
}