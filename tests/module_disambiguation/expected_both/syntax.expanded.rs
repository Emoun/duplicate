use duplicate::*;
mod module_some_name11 {
    pub struct SomeName11();
}
mod module_some_name12 {
    pub struct SomeName12();
}
mod module_some_name13 {
    pub struct SomeName13();
}
mod module_some_name21 {
    pub struct SomeName21(Vec<()>);
}
mod module_some_name22 {
    pub struct SomeName22(u32);
}
mod module_some_name23 {
    pub struct SomeName23(u64);
}
mod module_some_name31 {
    pub struct SomeName31(u8);
}
mod module_some_name32 {
    pub struct SomeName32(<SomeType as Trait>::SocType);
}
mod module_some_name33 {
    pub struct SomeName33(u64);
}
mod module_some_name41 {
    pub struct module();
}
mod module_some_name42 {
    pub struct module();
}