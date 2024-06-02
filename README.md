# Local Static
One-time mutable references to your local static variables.

```rust
#[local_static]
fn main() {
    static mut FOO: u32 = 0;
    *FOO = 42;
}
```

# Limitation
Functions with `#[local_static]` attribute can only be called once.

# Goals
- Safe `&'static mut` references to local `static mut` variables.
- Support for `#[link_section]` attribute.
- Similar limitation and errors as regular `static mut`.

# Acknowledgements
This crate is a generalization of a similar transformation done by the [`#[entry]`](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/attr.entry.html) macro in the `cortex_m_rt` crate.

# Related work
- [static-cell](https://crates.io/crates/static_cell) provides a wrapper type that gives safe access to a `&'static mut T`.