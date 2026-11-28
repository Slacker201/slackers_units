#[test]
fn unit_system() {
    let try_build = trybuild::TestCases::new();

    try_build.compile_fail("./unit_system_try_build/fail/*.rs");
    try_build.pass("./unit_system_try_build/succeed/*.rs");
}