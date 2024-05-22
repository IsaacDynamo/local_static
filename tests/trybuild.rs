#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}

#[test]
fn compile_pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile_pass/*.rs");
}

#[test]
fn examples() {
    let t = trybuild::TestCases::new();
    t.pass("examples/*.rs");
}
