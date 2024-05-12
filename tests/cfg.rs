#![deny(warnings)]

use local_static::local_static;

#[test]
#[local_static]
fn cfg() {
    #[cfg(target_os = "linux")]
    static mut X: u32 = 0;

    #[cfg(not(target_os = "linux"))]
    static mut X: i32 = 0;

    *X += 1;
}