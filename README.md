# comgrabe ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![comgrabe on crates.io](https://img.shields.io/crates/v/comgrabe)](https://crates.io/crates/comgrabe) [![comgrabe on docs.rs](https://docs.rs/comgrabe/badge.svg)](https://docs.rs/comgrabe) [![Source Code Repository](https://img.shields.io/badge/Code-On%20github.com-blue)](https://github.com/pacak/comgrabe) [![comgrabe on deps.rs](https://deps.rs/repo/github/pacak/comgrabe/status.svg)](https://deps.rs/repo/github/pacak/comgrabe)

For structs with named fields generates an impl that would return a doc comment for a field, if one is available:


```rust
#[derive(Debug, Comgrabe)]
/// outer
#[allow(dead_code)]
struct Foo<'a, T> {
   /// Bar
   /// Bar2
   x: bool,
   /// B
   b: &'a str,
   /// t
   ttt: T,
}


assert_eq!(Some("Bar\nBar2"), Foo::<()>::comgrabe("x"));
assert_eq!(Some("t"), Foo::<()>::comgrabe("ttt"));
assert_eq!(None, Foo::<()>::comgrabe("nosuch"));
```

