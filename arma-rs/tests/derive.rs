#[test]
fn derive() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/derive/fail_*.rs");
    tests.pass("tests/derive/pass_*.rs");
}
