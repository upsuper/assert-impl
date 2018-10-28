include!("../common/basic.rs");

fn _assert_not_one1() {
    assert_impl!(!StaticTyping: Rust);
    //~^ ERROR multiple applicable items in scope
}

fn _assert_not_one2() {
    assert_impl!(!StaticTyping: Rust,);
    //~^ ERROR multiple applicable items in scope
}

fn _assert_not_multi1() {
    assert_impl!(!StaticTyping: C, Java);
    //~^ ERROR multiple applicable items in scope
    //~^^ ERROR multiple applicable items in scope
}

fn _assert_not_multi2() {
    assert_impl!(!StaticTyping: C, Java,);
    //~^ ERROR multiple applicable items in scope
    //~^^ ERROR multiple applicable items in scope
}

fn _assert_not_multi_some1() {
    assert_impl!(!StaticTyping: C, Python);
    //~^ ERROR multiple applicable items in scope
}

fn _assert_not_multi_some2() {
    assert_impl!(!StaticTyping: C, Python,);
    //~^ ERROR multiple applicable items in scope
}

fn _assert_not_multi_some3() {
    assert_impl!(!StaticTyping: Python, C);
    //~^ ERROR multiple applicable items in scope
}

fn _assert_not_multi_some4() {
    assert_impl!(!StaticTyping: Python, C,);
    //~^ ERROR multiple applicable items in scope
}
