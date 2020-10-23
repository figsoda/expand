# expand

Macro to expand byte string literals


## Usage

```rust
use expand::expand;

assert_eq!(
    &expand!([@b"Hello,", b' ', @b"world", b'!']),
    b"Hello, world!"
);

if let expand!([@b"patt", x, y, b'n', ..]) = b"pattern matching" {
    assert_eq!(x, &b'e');
    assert_eq!(y, &b'r');
} else {
    panic!("pattern matching failed");
}
```