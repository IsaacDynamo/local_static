// Static mut variables transformed by `local_static` should have behavior similar to regular static mut variables.
#![deny(warnings)]

fn main() {
    static mut X: u32 = 0;
    unsafe { X += 1; }
    static mut X: u32 = 0; // ERROR
    unsafe { X += 1; }
}
