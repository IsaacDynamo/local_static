#![deny(warnings)]

use local_static::local_static;

#[local_static]
fn main() {
    static mut FOO: u32 = 0;
    *FOO = 42;
}