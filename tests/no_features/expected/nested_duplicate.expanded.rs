use duplicate::*;
struct Example {
    one: u8,
    two: u8,
}
impl Example {
    fn inline_new() -> Self {
        Example { one: 0, two: 0 }
    }
    fn attr_new() -> Self {
        Example { one: 0, two: 0 }
    }
}
struct StructName1(u8, u16);
struct TypeName21(u8, u16);
impl std::error::Error<u32, u8> for () {}
impl std::error::Error<u64, u8> for () {}
impl std::error::Error<u32, u16> for () {}
impl std::error::Error<u64, u16> for () {}
trait SomeType<T1, T2, T3> {}
impl SomeType<i8, u32, u8> for () {}
impl SomeType<i16, u32, u8> for () {}
impl SomeType<i8, u64, u8> for () {}
impl SomeType<i16, u64, u8> for () {}
impl SomeType<i8, u32, u16> for () {}
impl SomeType<i16, u32, u16> for () {}
impl SomeType<i8, u64, u16> for () {}
impl SomeType<i16, u64, u16> for () {}