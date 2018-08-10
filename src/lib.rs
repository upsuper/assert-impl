//! Macro for static assert that types implement a trait or not.
//!
//! Note: this macro can only be used inside function body due to
//! restriction of Rust.
//!
//! # Example
//!
//! Assuming you have the following definitions:
//! ```
//! struct C;
//! struct Java;
//! struct JavaScript;
//! struct Python;
//! struct Rust;
//!
//! trait StaticTyping {}
//! impl StaticTyping for C {}
//! impl StaticTyping for Java {}
//! impl StaticTyping for Rust {}
//! ```
//!
//! This should build:
//! ```
//! # #[macro_use] extern crate assert_impl;
//! # struct C;
//! # struct Java;
//! # struct JavaScript;
//! # struct Python;
//! # struct Rust;
//! # trait StaticTyping {}
//! # impl StaticTyping for C {}
//! # impl StaticTyping for Java {}
//! # impl StaticTyping for Rust {}
//! assert_impl!(StaticTyping: C, Java, Rust);
//! assert_impl!(StaticTyping: C, Java, Rust, );
//! assert_impl!(!StaticTyping: JavaScript, Python);
//! assert_impl!(!StaticTyping: JavaScript, Python, );
//! ```
//!
//! But these should fail to build:
//! ```compile_fail
//! # #[macro_use] extern crate assert_impl;
//! # struct C;
//! # struct Java;
//! # struct JavaScript;
//! # struct Python;
//! # struct Rust;
//! # trait StaticTyping {}
//! # impl StaticTyping for C {}
//! # impl StaticTyping for Java {}
//! # impl StaticTyping for Rust {}
//! assert_impl!(StaticTyping: JavaScript);
//! ```
//!
//! ```compile_fail
//! # #[macro_use] extern crate assert_impl;
//! # struct C;
//! # struct Java;
//! # struct JavaScript;
//! # struct Python;
//! # struct Rust;
//! # trait StaticTyping {}
//! # impl StaticTyping for C {}
//! # impl StaticTyping for Java {}
//! # impl StaticTyping for Rust {}
//! assert_impl!(!StaticTyping: Rust);
//! ```

#[macro_export]
macro_rules! assert_impl {
    ($trait:path: $($ty:ty),+) => {{
        struct Helper<T>(T);
        trait AssertImpl { fn assert() {} }
        impl<T: $trait> AssertImpl for Helper<T> {}
        $(
            Helper::<$ty>::assert();
         )+
    }};
    (!$trait:path: $($ty:ty),+) => {{
        struct Helper<T>(T);
        trait AssertImpl { fn assert() {} }
        impl<T: $trait> AssertImpl for Helper<T> {}
        trait AssertNotImpl { fn assert() {} }
        $(
            impl AssertNotImpl for Helper<$ty> {}
            Helper::<$ty>::assert();
         )+
    }};
    ($trait:path: $($ty:ty,)+) => (assert_impl!($trait: $($ty),+));
    (!$trait:path: $($ty:ty,)+) => (assert_impl!(!$trait: $($ty),+));
}
