#[macro_use]
extern crate assert_impl;

fn main() {}

struct C;
struct Java;
struct JavaScript;
struct Python;
struct Rust;

trait StaticTyping {}
impl StaticTyping for C {}
impl StaticTyping for Java {}
impl StaticTyping for Rust {}
