//! # expand
//!
//! Macro to expand byte string literals
//!
//!
//! ## Usage
//!
//! ```rust
//! use expand::expand;
//!
//! assert_eq!(
//!     &expand!([@b"Hello,", b' ', @b"world", b'!']),
//!     b"Hello, world!"
//! );
//!
//! if let expand!([@b"patt", x, y, b'n', ..]) = b"pattern matching" {
//!     assert_eq!(x, &b'e');
//!     assert_eq!(y, &b'r');
//! } else {
//!     panic!("pattern matching failed");
//! }
//! ```

#![no_std]
#![deny(unsafe_code)]

use proc_macro2::{Group, Literal, Punct, Spacing, TokenStream, TokenTree};
use syn::{parse2, LitByteStr};

/// Expand byte string literals
///
/// Prefix byte strings with `@` to expand them
///
/// ## Examples
///
/// ```rust
/// # use expand::expand;
/// assert_eq!(
///     &expand!([@b"Hello,", b' ', @b"world", b'!']),
///     b"Hello, world!"
/// );
/// ```
///
/// ```rust
/// # use expand::expand;
/// if let expand!([@b"patt", x, y, b'n', ..]) = b"pattern matching" {
///     assert_eq!(x, &b'e');
///     assert_eq!(y, &b'r');
/// } else {
///     panic!("pattern matching failed");
/// }
/// ```
#[proc_macro]
pub fn expand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    _expand(input.into()).into()
}

fn _expand(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();
    let mut output = TokenStream::new();

    loop {
        match input.next() {
            Some(TokenTree::Group(t)) => {
                output.extend(Some(TokenTree::Group(Group::new(
                    t.delimiter(),
                    _expand(t.stream()),
                ))));
            }
            Some(TokenTree::Punct(t)) if t.as_char() == '@' => {
                let t = input
                    .next()
                    .expect("expected a byte string literal after '@'")
                    .into();
                let mut xs = parse2::<LitByteStr>(t)
                    .expect("expected a byte string literal after '@'")
                    .value()
                    .into_iter();

                match xs.next() {
                    None => panic!("cannot expand an empty byte string"),
                    Some(x) => output.extend(Some(TokenTree::Literal(Literal::u8_suffixed(x)))),
                }

                for x in xs {
                    output.extend(Some(TokenTree::Punct(Punct::new(',', Spacing::Alone))));
                    output.extend(Some(TokenTree::Literal(Literal::u8_suffixed(x))));
                }
            }
            Some(t) => {
                output.extend(Some(t));
            }
            None => break,
        }
    }

    output
}
