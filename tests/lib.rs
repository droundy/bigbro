extern crate bigbro;

#[test]
fn stupid_test() {
    assert!(std::mem::size_of::<bigbro::Command>() > 0);
}
