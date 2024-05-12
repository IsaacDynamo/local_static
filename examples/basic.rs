#![deny(unsafe_code)]
#![deny(warnings)]

use local_static::local_static;

#[local_static]
fn main() {
    static mut COUNT_A: u32 = 0;
    *COUNT_A += 1;

    static mut COUNT_B: u32 = 0;
    *COUNT_B += 1;
}
