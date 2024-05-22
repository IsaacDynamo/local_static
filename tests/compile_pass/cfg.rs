#![deny(warnings)]

use local_static::local_static;

#[local_static]
fn main() {
    #[cfg(target_os = "Never equal")]
    static mut UNUSED: u32 = 0;

    // If #[cfg] attributes works on local static variable, UNUSED will not exist and there will not be an unused variable warning

    #[cfg(not(target_os = "Never equal"))]
    static mut USED: i32 = 0;

    *USED += 1;
}
