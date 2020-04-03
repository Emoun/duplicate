use duplicate::duplicate;
pub struct SomeName1();
pub struct SomeName2();
pub struct SomeName3();
pub struct SomeName1();
pub struct SomeName2();
pub struct SomeName3();
pub struct SomeName4();
/// Test 2 substitution groups in nested invocation.
/// Output should be the same as the next test.
pub struct SomeName1(SomeMember1);
/// Test 2 substitution groups in nested invocation.
/// Output should be the same as the next test.
pub struct SomeName1(SomeMember2);
/// Test 2 substitution groups in nested invocation.
/// Output should be the same as the next test.
pub struct SomeName2(SomeMember1);
/// Test 2 substitution groups in nested invocation.
/// Output should be the same as the next test.
pub struct SomeName2(SomeMember2);
/// Test nesting depth of 2.
/// Output should be the same as the previous test
pub struct SomeName1(SomeMember1);
/// Test nesting depth of 2.
/// Output should be the same as the previous test
pub struct SomeName1(SomeMember2);
/// Test nesting depth of 2.
/// Output should be the same as the previous test
pub struct SomeName2(SomeMember1);
/// Test nesting depth of 2.
/// Output should be the same as the previous test
pub struct SomeName2(SomeMember2);