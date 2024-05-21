# Local Static
Ergonomic mutable references to your local static variables.

# Limitation
Functions with `#[local_static]` attribute can only be called once.

# Goal
- Safe `&'static mut` references to `static mut` variables.
- Support for `#[link_section]` attribute.
- Similar limitation and errors as regular `static mut`.

# Features
- [x] `static mut` to `&'static mut` conversion
- [ ] No panic variant
- [ ] Unchecked variant
- [ ] Dynamic init of statics

# Acknowledgements
This crate is a generalization of a similar transformation done by the [`#[entry]`](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/attr.entry.html) macro in the `cortex_m_rt` crate.

# Related work
- [static-cell](https://crates.io/crates/static_cell)