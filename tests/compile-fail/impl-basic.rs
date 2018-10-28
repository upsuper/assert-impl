include!("../common/basic.rs");

fn _assert() {
    assert_impl!(StaticTyping: JavaScript);
    //~^ ERROR no function or associated item named `assert` found
}
