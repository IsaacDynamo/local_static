// static mut variables transformed by local_static should have behavior similar to regular static mut variables.

#![deny(warnings)]

use local_static::local_static;

#[test]
fn regular_static_shadow() {
    static mut X: u32 = 0;
    unsafe { X += 1; }
    todo!();
    //static mut X: u32 = 0;
    //unsafe { X += 1; }
}

#[test]
#[local_static]
fn local_static_shadow() {
    static mut X: u32 = 0;
    *X += 1;
    static mut X: u32 = 0; // should be an error
    *X += 1;
}
