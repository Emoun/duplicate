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
