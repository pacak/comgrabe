use comgrabe::*;

#[test]
fn it_works() {
    assert_eq!(Some("Bar\nBar2"), Foo::<()>::comgrabe("x"));
    assert_eq!(Some("t"), Foo::<()>::comgrabe("ttt"));
    assert_eq!(None, Foo::<()>::comgrabe("nosuch"));
}

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
