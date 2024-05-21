// Only static mut's in top level scope of the function are transformed.

#![deny(warnings)]

use local_static::local_static;

#[local_static]
fn main() {
    static mut TOP: u32 = 0;

    // Ergonomic mutation via &mut
    *TOP += 1;

    // Create inner scope
    {
        static mut INNER: u32 = 0;

        // Unsafe mutation of static mut
        unsafe {
            INNER += 1;
        }
    }
}
