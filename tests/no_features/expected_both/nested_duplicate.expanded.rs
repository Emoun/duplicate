use duplicate::*;
pub struct SomeName1();
pub struct SomeName2();
pub struct SomeName3();
pub struct SomeName4();
pub struct SomeName5();
pub struct SomeName6();
pub struct SomeName7();
trait SomeTrait<T1, T2> {}
impl SomeTrait<u8, u32> for () {}
impl SomeTrait<u8, u64> for () {}
impl SomeTrait<u16, u32> for () {}
impl SomeTrait<u16, u64> for () {}
impl SomeTrait<i8, i32> for () {}
impl SomeTrait<i8, i64> for () {}
impl SomeTrait<i16, i32> for () {}
impl SomeTrait<i16, i64> for () {}
fn outer_1() {
    1;
}
fn outer_2() {
    2;
}
