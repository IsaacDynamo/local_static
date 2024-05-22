#![deny(warnings)]

use local_static::local_static;

#[local_static]
fn main() {
    static mut snake_case: u32 = 0;
    *snake_case += 1;
}
