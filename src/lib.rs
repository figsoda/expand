//! Macro to expand byte string and string literals
//!
//!
//! ## Usage
//!
//! ```
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

#![forbid(unsafe_code)]
#![no_std]

use proc_macro::{Group, Literal, Punct, Spacing, TokenStream, TokenTree};
use quote::quote_spanned;
use syn::{parse, LitByteStr, LitStr};

/// Expand byte string literals
///
/// Prefix a byte string or a string literal with a `@` to expand it
///
/// ## Examples
///
/// ### expanding a byte string
/// ```
/// # use expand::expand;
/// assert_eq!(
///     &expand!([@b"Hello,", b' ', @b"world", b'!']),
///     b"Hello, world!",
/// );
/// ```
///
/// ### expanding a string
/// ```
/// # use expand::expand;
/// assert_eq!(
///     expand!(vec![@"Hello,", ' ', @"world", '!']),
///     "Hello, world!".chars().collect::<Vec<char>>(),
/// );
/// ```
///
/// ### pattern matching
/// ```
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
pub fn expand(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();
    let mut output = TokenStream::new();

    loop {
        match input.next() {
            Some(TokenTree::Group(t)) => {
                output.extend(Some(TokenTree::Group(Group::new(
                    t.delimiter(),
                    expand(t.stream()),
                ))));
            }

            Some(TokenTree::Punct(t)) if t == '@' => {
                let tt = if let Some(tt) = input.next() {
                    tt
                } else {
                    output.extend(Some(TokenTree::Punct(t)));
                    break;
                };

                if let Ok(t) = parse::<LitByteStr>(tt.clone().into()) {
                    let mut xs = t
                        .value()
                        .into_iter()
                        .map(Literal::u8_suffixed)
                        .map(TokenTree::Literal);

                    if let Some(x) = xs.next() {
                        output.extend(Some(x));
                    } else {
                        output.extend::<TokenStream>(
                            quote_spanned! { tt.span().into() =>
                                compile_error!("can't expand an empty byte string")
                            }
                            .into(),
                        );
                        break;
                    }

                    for x in xs {
                        output.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone)), x]);
                    }
                } else if let Ok(t) = parse::<LitStr>(tt.clone().into()) {
                    let xs = t.value();
                    let mut xs = xs.chars().map(Literal::character).map(TokenTree::Literal);

                    if let Some(x) = xs.next() {
                        output.extend(Some(x));
                    } else {
                        output.extend::<TokenStream>(
                            quote_spanned! { tt.span().into() =>
                                compile_error!("can't expand an empty string")
                            }
                            .into(),
                        );
                        break;
                    }

                    for x in xs {
                        output.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone)), x]);
                    }
                } else {
                    output.extend([TokenTree::Punct(t), tt]);
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
