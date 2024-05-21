// Static mut variables transformed by `local_static` should have behavior similar to regular static mut variables.
#![deny(warnings)]

use local_static::local_static;

#[local_static]
fn main() {
    static mut X: u32 = 0;
    *X += 1;
    static mut X: u32 = 0; // ERROR
    *X += 1;
}
