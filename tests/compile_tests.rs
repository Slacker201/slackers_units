#[test]
fn must_fail() {
    let tester = trybuild::TestCases::new();

    tester.compile_fail("./tests/c_fail/*.rs");
}
