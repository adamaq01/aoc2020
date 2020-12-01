use proc_macro::TokenStream;
use proc_macro2::{Group, Punct};
use syn::{parse::Parse, parse_macro_input, DeriveInput, Ident, LitInt};

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

struct PuzzleParams {
    day: LitInt,
    _token: Punct,
    parse: Ident,
    _token2: Punct,
    run: Ident,
}

impl Parse for PuzzleParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(PuzzleParams {
            day: input.parse()?,
            _token: input.parse()?,
            parse: input.parse()?,
            _token2: input.parse()?,
            run: input.parse()?,
        })
    }
}

#[proc_macro_derive(Puzzle, attributes(puzzle))]
pub fn puzzle(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let attributes: TokenStream = input.attrs.first().unwrap().tokens.clone().into();
    let params = parse_macro_input!(attributes as Group);
    let params: TokenStream = params.stream().into();
    let params = parse_macro_input!(params as PuzzleParams);
    let ident = input.ident.clone();

    let day = params.day;
    let parse = params.parse;
    let run = params.run;

    let non = quote! {
        impl Puzzle for #ident {
            fn day() -> u8 {
                #day
            }

            fn run(inputs: &[&str]) -> Result<()> {
                let inputs = Self::#parse(inputs)?;
                Self::#run(inputs)
            }
        }
    };

    non.into()
}
