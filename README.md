## this is macro try_enum!
## e.g.:
=======================
```rust
#[derive(Eq,PartialEq)]
enum A<'a> {
    Bar(i32),
    Foo(&'a str),
}
let a=A::Bar(12);
assert_eq!(try_enum!(a,A::Bar),Some(12));
assert_eq!(try_enum!(a,A::Foo),None);
let a_foo=A::Foo("123");
assert_eq!(try_enum!(a_foo,A::Foo),Some("123"));
#[derive(Eq,PartialEq)]
enum B<'a> {
    Bar{a:A<'a>,b:i32},
    Foo(&'a str),
}
let b=B::Bar{a:A::Bar(12),b:14};
assert_eq!(try_enum!(&b,B::Bar[a],A::Bar),Some(&12));
assert_eq!(try_enum!(b,B::Foo),None);
let b_foo=B::Foo("123");
assert_eq!(try_enum!(b_foo,B::Foo),Some("123"));
```