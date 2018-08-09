//! Macro for static assert that types implement a trait or not.
//!
//! # Example
//!
//! Assuming you have the following definitions:
//! ```
//! struct Nadeshiko;
//! struct Rin;
//! struct Chiaki;
//! struct Aoi;
//! struct Ena;
//!
//! trait Yakuru {}
//! impl Yakuru for Nadeshiko {}
//! impl Yakuru for Chiaki {}
//! impl Yakuru for Aoi {}
//! ```
//!
//! This should build:
//! ```
//! # #[macro_use] extern crate assert_impl;
//! # struct Nadeshiko;
//! # struct Rin;
//! # struct Chiaki;
//! # struct Aoi;
//! # struct Ena;
//! # trait Yakuru {}
//! # impl Yakuru for Nadeshiko {}
//! # impl Yakuru for Chiaki {}
//! # impl Yakuru for Aoi {}
//! assert_impl!(Yakuru: Nadeshiko, Chiaki, Aoi);
//! assert_impl!(!Yakuru: Rin, Ena);
//! ```
//!
//! But these should fail to build:
//! ```compile_fail
//! # #[macro_use] extern crate assert_impl;
//! # struct Nadeshiko;
//! # struct Rin;
//! # struct Chiaki;
//! # struct Aoi;
//! # struct Ena;
//! # trait Yakuru {}
//! # impl Yakuru for Nadeshiko {}
//! # impl Yakuru for Chiaki {}
//! # impl Yakuru for Aoi {}
//! assert_impl!(Yakuru: Rin);
//! ```
//!
//! ```compile_fail
//! # #[macro_use] extern crate assert_impl;
//! # struct Nadeshiko;
//! # struct Rin;
//! # struct Chiaki;
//! # struct Aoi;
//! # struct Ena;
//! # trait Yakuru {}
//! # impl Yakuru for Nadeshiko {}
//! # impl Yakuru for Chiaki {}
//! # impl Yakuru for Aoi {}
//! assert_impl!(!Yakuru: Nadeshiko);
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
