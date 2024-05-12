#![deny(unsafe_code)]
#![deny(warnings)]

use local_static::local_static;

#[local_static]
fn take_x() -> &'static mut u32 {
    static mut X: u32 = 0;
    X
}

#[test]
#[should_panic]
fn no_alias() {
    let _x1 = take_x();
    let _x2 = take_x();
}