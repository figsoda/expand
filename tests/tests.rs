use expand::expand;

#[test]
fn hello_world() {
    assert_eq!(
        &expand!([@b"Hello,", b' ', @b"world", b'!']),
        b"Hello, world!"
    );
}
