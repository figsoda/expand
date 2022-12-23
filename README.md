# expand

[![version](https://img.shields.io/crates/v/expand?logo=rust&style=flat-square)](https://crates.io/crates/expand)
[![deps](https://deps.rs/repo/github/figsoda/expand/status.svg?style=flat-square&compact=true)](https://deps.rs/repo/github/figsoda/expand)
[![license](https://img.shields.io/badge/license-MPL--2.0-blue?style=flat-square)](https://www.mozilla.org/en-US/MPL/2.0)
[![ci](https://img.shields.io/github/actions/workflow/status/figsoda/expand/ci.yml?label=ci&logo=github-actions&style=flat-square)](https://github.com/figsoda/expand/actions?query=workflow:ci)

Macro to expand byte string and string literals

[Documentation](https://docs.rs/expand)


## Usage

```rust
use expand::expand;

// expanding a byte string
assert_eq!(
    &expand!([@b"Hello,", b' ', @b"world", b'!']),
    b"Hello, world!"
);

// expanding a string
assert_eq!(
    expand!(vec![@"Hello,", ' ', @"world", '!']),
    "Hello, world!".chars().collect::<Vec<char>>(),
);

// pattern matching
if let expand!([@b"patt", x, y, b'n', ..]) = b"pattern matching" {
    assert_eq!(x, &b'e');
    assert_eq!(y, &b'r');
} else {
    panic!("pattern matching failed");
}

// more pattern matching
if let expand!([@b"msg = \"", xs @ .., b'"']) = br#"msg = "Hello, world!""# {
    assert_eq!(xs, b"Hello, world!");
} else {
    panic!("pattern matching failed");
}
```


## Changelog

See [CHANGELOG.md](CHANGELOG.md)
