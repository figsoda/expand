//! Macro to expand byte string and string literals
//!
//!
//! ## Usage
//!
//! ```rust
//! use expand::expand;
//!
//! // expanding a byte string
//! assert_eq!(
//!     &expand!([@b"Hello,", b' ', @b"world", b'!']),
//!     b"Hello, world!",
//! );
//!
//! // expanding a string
//! assert_eq!(
//!     expand!(vec![@"Hello,", ' ', @"world", '!']),
//!     "Hello, world!".chars().collect::<Vec<char>>(),
//! );
//!
//! // pattern matching
//! if let expand!([@b"patt", x, y, b'n', ..]) = b"pattern matching" {
//!     assert_eq!(x, &b'e');
//!     assert_eq!(y, &b'r');
//! } else {
//!     panic!("pattern matching failed");
//! }
//!
//! // more pattern matching
//! if let expand!([@b"msg = \"", xs @ .., b'"']) = br#"msg = "Hello, world!""# {
//!     assert_eq!(xs, b"Hello, world!");
//! } else {
//!     panic!("pattern matching failed");
//! }
//! ```
//!
//!
//! ## Changelog
//!
//! See [CHANGELOG.md](https://github.com/figsoda/expand/blob/main/CHANGELOG.md)

#![no_std]
#![forbid(unsafe_code)]

use proc_macro2::{Group, Literal, Punct, Spacing, TokenStream, TokenTree};
use quote::quote_spanned;
use syn::{parse2, LitByteStr, LitStr};

/// Expand byte string literals
///
/// Prefix a byte string or a string literal with a `@` to expand it
///
/// ## Examples
///
/// ### expanding a byte string
/// ```rust
/// # use expand::expand;
/// assert_eq!(
///     &expand!([@b"Hello,", b' ', @b"world", b'!']),
///     b"Hello, world!",
/// );
/// ```
///
/// ### expanding a string
/// ```rust
/// # use expand::expand;
/// assert_eq!(
///     expand!(vec![@"Hello,", ' ', @"world", '!']),
///     "Hello, world!".chars().collect::<Vec<char>>(),
/// );
/// ```
///
/// ### pattern matching
/// ```rust
/// # use expand::expand;
/// if let expand!([@b"patt", x, y, b'n', ..]) = b"pattern matching" {
///     assert_eq!(x, &b'e');
///     assert_eq!(y, &b'r');
/// } else {
///     panic!("pattern matching failed");
/// }
/// ```
///
/// ### more pattern matching
/// ``` rust
/// # use expand::expand;
/// if let expand!([@b"msg = \"", xs @ .., b'"']) = br#"msg = "Hello, world!""# {
///     assert_eq!(xs, b"Hello, world!");
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
                let tt = if let Some(tt) = input.next() {
                    tt
                } else {
                    output.extend(Some(TokenTree::Punct(t)));
                    break;
                };

                if let Ok(t) = parse2::<LitByteStr>(tt.clone().into()) {
                    let mut xs = t
                        .value()
                        .into_iter()
                        .map(|x| TokenTree::Literal(Literal::u8_suffixed(x)));
                    if let Some(x) = xs.next() {
                        output.extend(Some(x));
                    } else {
                        output.extend(quote_spanned! { tt.span() =>
                            compile_error!("can't expand an empty byte string")
                        });
                        break;
                    }

                    for x in xs {
                        output.extend(Some(TokenTree::Punct(Punct::new(',', Spacing::Alone))));
                        output.extend(Some(x));
                    }
                } else if let Ok(t) = parse2::<LitStr>(tt.clone().into()) {
                    let xs = t.value();
                    let mut xs = xs
                        .chars()
                        .map(|c| TokenTree::Literal(Literal::character(c)));
                    if let Some(x) = xs.next() {
                        output.extend(Some(x));
                    } else {
                        output.extend(quote_spanned! { tt.span() =>
                            compile_error!("can't expand an empty string")
                        });
                        break;
                    }

                    for x in xs {
                        output.extend(Some(TokenTree::Punct(Punct::new(',', Spacing::Alone))));
                        output.extend(Some(x));
                    }
                } else {
                    output.extend(Some(TokenTree::Punct(t)));
                    output.extend(Some(tt));
                    continue;
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
