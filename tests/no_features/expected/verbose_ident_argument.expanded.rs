fn fn_const_1(arg: &i32) {}
fn fn_mut_1(arg: &mut i32) {}
fn fn_const_2<'a>(arg: &Vec<i32>) {}
fn fn_mut_2<'a>(arg: &mut Vec<i32>) {}
fn fn_const_3<'a>(arg: &'a i32) {}
fn fn_mut_3<'a>(arg: &'a mut i32) {}
fn fn_const_4<'a>(arg: &'a Result<i32, u8>) {}
fn fn_mut_4<'a>(arg: &'a mut Result<i32, u8>) {}
fn fn_const_5<'a>(arg: &'a Result<i32, u8>) -> &'a i32 {}
fn fn_mut_5<'a>(arg: &'a mut Result<i32, u8>) -> &'a mut i32 {}
fn fn_const_6(arg: &i32) {}
fn fn_mut_6(arg: &mut i32) {}
fn fn_const_7(arg: &i32) {}
fn fn_mut_7(arg: &mut i32) {}
fn fn_const_8(arg: &i32, second_arg: &mut i64) {}
fn fn_mut_8(arg: &mut i32, second_arg: &i64) {}
fn fn_const_9(arg: &&i32) {}
fn fn_mut_9(arg: &mut &mut i32) {}