Macro for static assert that types implement a trait or not.

# Example

Assuming you have the following definitions:
```rust
struct Nadeshiko;
struct Rin;
struct Chiaki;
struct Aoi;
struct Ena;

trait Yakuru {}
impl Yakuru for Nadeshiko {}
impl Yakuru for Chiaki {}
impl Yakuru for Aoi {}
```

This should build:
```rust
assert_impl!(Yakuru: Nadeshiko, Chiaki, Aoi);
assert_impl!(!Yakuru: Rin, Ena);
```

But this should fail to build:
```rust
assert_impl!(Yakuru: Rin);
assert_impl!(!Yakuru: Nadeshiko);
```
