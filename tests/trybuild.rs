#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}

#[test]
fn examples() {
    let t = trybuild::TestCases::new();
    t.pass("examples/*.rs");
}
