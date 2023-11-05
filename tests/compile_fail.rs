#[test]
fn test() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/compile_fail/enum.rs");
    t.compile_fail("tests/compile_fail/generics.rs");
}
