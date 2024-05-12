#![deny(warnings)]

use local_static::local_static;

#[local_static]
fn main() {
    #[link_section = ".my-section"]
    static mut COUNT: u32 = 0;
    *COUNT += 1;
}
