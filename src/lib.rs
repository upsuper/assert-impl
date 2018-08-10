//! Macro for static assert that types implement a trait or not.
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
//! trait StaticTyped {}
//! impl StaticTyped for C {}
//! impl StaticTyped for Java {}
//! impl StaticTyped for Rust {}
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
//! # trait StaticTyped {}
//! # impl StaticTyped for C {}
//! # impl StaticTyped for Java {}
//! # impl StaticTyped for Rust {}
//! assert_impl!(StaticTyped: C, Java, Rust);
//! assert_impl!(!StaticTyped: JavaScript, Python);
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
//! # trait StaticTyped {}
//! # impl StaticTyped for C {}
//! # impl StaticTyped for Java {}
//! # impl StaticTyped for Rust {}
//! assert_impl!(StaticTyped: JavaScript);
//! ```
//!
//! ```compile_fail
//! # #[macro_use] extern crate assert_impl;
//! # struct C;
//! # struct Java;
//! # struct JavaScript;
//! # struct Python;
//! # struct Rust;
//! # trait StaticTyped {}
//! # impl StaticTyped for C {}
//! # impl StaticTyped for Java {}
//! # impl StaticTyped for Rust {}
//! assert_impl!(!StaticTyped: Rust);
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
}
