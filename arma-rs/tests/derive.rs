#[test]
fn derive() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/derive/*fail*.rs");
    tests.pass("tests/derive/*pass*.rs");
}
