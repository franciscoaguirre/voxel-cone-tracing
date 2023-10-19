#![allow(dead_code)]

use macros::my_macro;

my_macro! {
    struct MyStruct {
        field1: usize,
        field2: bool
    }
}

#[test]
fn test() {
    let _ = RenamedStruct {
        field1: 3,
        field2: false,
    };
}
