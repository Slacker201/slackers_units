#[test]
fn system_parsing() {
    let tester = trybuild::TestCases::new();
    tester.compile_fail("./tests/sys_parse/*.rs");
}

#[test]
fn sys_validate() {
    let tester = trybuild::TestCases::new();
    tester.compile_fail("./tests/sys_validate/*.rs");
}

#[test]
fn unit_parse() {
    let tester = trybuild::TestCases::new();
    tester.compile_fail("./tests/unit_parse/*.rs");
}