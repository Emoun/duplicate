use duplicate::*;
impl Example {
    fn inline_new() -> Self {
        Example { one: 0, two: 0 }
    }
    fn attr_new() -> Self {
        Example { one: 0, two: 0 }
    }
}
struct StructName1(TypeName11, TypeName12);
struct TypeName21(TypeName22, TypeName23);
impl SomeType<33, 31> for () {}
impl SomeType<34, 31> for () {}
impl SomeType<33, 32> for () {}
impl SomeType<34, 32> for () {}
impl SomeType<45, 43, 41> for () {}
impl SomeType<46, 43, 41> for () {}
impl SomeType<45, 44, 41> for () {}
impl SomeType<46, 44, 41> for () {}
impl SomeType<45, 43, 42> for () {}
impl SomeType<46, 43, 42> for () {}
impl SomeType<45, 44, 42> for () {}
impl SomeType<46, 44, 42> for () {}