use trybuild::TestCases;

#[test]
fn tests() {
    let t = TestCases::new();
    t.pass("tests/sub_menus.rs");
}
