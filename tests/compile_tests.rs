#[test]
fn system_parsing() {
    let tester = trybuild::TestCases::new();
    tester.compile_fail("./tests/sys_parse_fail/*.rs");
}

#[test]
fn sys_validate() {
    let tester = trybuild::TestCases::new();
    tester.compile_fail("./tests/sys_validate_fail/*.rs");
}

#[test]
fn unit_parse() {
    let tester = trybuild::TestCases::new();
    tester.compile_fail("./tests/unit_parse_fail/*.rs");
}
