#![deny(warnings)]

use local_static::local_static;

#[local_static]
fn main() {
    #[allow(non_upper_case_globals)]
    static mut snake_case: u32 = 0;
    *snake_case += 1;
}
