#![doc(html_root_url = "https://docs.rs/maud_macros/0.24.0")]
// TokenStream values are reference counted, and the mental overhead of tracking
// lifetimes outweighs the marginal gains from explicit borrowing
#![allow(clippy::needless_pass_by_value)]

extern crate proc_macro;

mod ast;
mod escape;
mod generate;
mod parse;

use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use proc_macro_error::proc_macro_error;
use quote::quote;

#[proc_macro]
#[proc_macro_error]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand(input.into()).into()
}

fn expand(input: TokenStream) -> TokenStream {
    let output_ident = TokenTree::Ident(Ident::new("__maud_output", Span::mixed_site()));
    let markups = parse::parse(input);
    let stmts = generate::generate(markups, output_ident.clone());
    quote!({
        extern crate alloc;
        let mut #output_ident = submillisecond_live_view::rendered::Rendered::builder();
        #stmts
        #output_ident.build()
    })
}
