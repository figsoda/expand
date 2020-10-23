use expand::expand;

#[test]
fn tests() {
    assert_eq!(&expand!([@b"Hello, ", @b"world!"]), b"Hello, world!");
}
