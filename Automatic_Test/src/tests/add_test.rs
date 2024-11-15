use add::add;

#[test]
fn it_works() {
    let a = add(90, 10);
    assert_ne!(a, 101);
}

