use expand::expand;

#[test]
fn hello_world() {
    assert_eq!(
        &expand!([@b"Hello,", b' ', @b"world", b'!']),
        b"Hello, world!"
    );
}

#[test]
fn pattern_matching() {
    if let expand!([@b"patt", x, y, b'n', ..]) = b"pattern matching" {
        assert_eq!(x, &b'e');
        assert_eq!(y, &b'r');
    } else {
        panic!("pattern matching failed");
    }
}

#[test]
fn pattern_matching2() {
    if let expand!([@b"msg = \"", xs @ .., b'"']) = br#"msg = "Hello, world!""# {
        assert_eq!(xs, b"Hello, world!");
    } else {
        panic!("pattern matching failed");
    }
}
