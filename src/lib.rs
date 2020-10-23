#![feature(extend_one)]

use proc_macro2::{Group, Literal, Punct, Spacing, TokenStream, TokenTree};
use syn::{parse2, LitByteStr};

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
                output.extend_one(TokenTree::Group(Group::new(
                    t.delimiter(),
                    _expand(t.stream()),
                )));
            }
            Some(TokenTree::Punct(t)) if t.as_char() == '@' => {
                let t = input.next().unwrap().into();
                let mut xs = parse2::<LitByteStr>(t).unwrap().value().into_iter();

                match xs.next() {
                    None => panic!("bytestring needs to be longer"),
                    Some(x) => output.extend_one(TokenTree::Literal(Literal::u8_suffixed(x))),
                }

                for x in xs {
                    output.extend_one(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
                    output.extend_one(TokenTree::Literal(Literal::u8_suffixed(x)));
                }
            }
            Some(t) => {
                output.extend_one(t);
            }
            None => break,
        }
    }

    output
}
