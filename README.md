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

trait StaticTyped {}
impl StaticTyped for C {}
impl StaticTyped for Java {}
impl StaticTyped for Rust {}
```

This should build:
```rust
assert_impl!(StaticTyped: C, Java, Rust);
assert_impl!(!StaticTyped: JavaScript, Python);
```

But this should fail to build:
```rust
assert_impl!(StaticTyped: JavaScript);
assert_impl!(!StaticTyped: Rust);
```
