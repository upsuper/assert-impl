Macro for static assert that types implement a trait or not.

Note: this macro can only be used inside function body due to
restriction of Rust.

# Example

Assuming you have the following definitions:
```rust
struct C;
struct Java;
struct JavaScript;
struct Python;
struct Rust;

trait StaticTyping {}
impl StaticTyping for C {}
impl StaticTyping for Java {}
impl StaticTyping for Rust {}
```

This should build:
```rust
assert_impl!(StaticTyping: C, Java, Rust);
assert_impl!(!StaticTyping: JavaScript, Python);
```

But this should fail to build:
```rust
assert_impl!(StaticTyping: JavaScript);
assert_impl!(!StaticTyping: Rust);
```
