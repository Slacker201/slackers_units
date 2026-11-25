use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::compile_parser_impl::parsed_unit::ParsedUnit;

mod parsed_unit;

pub fn compile_parser_impl(ts: TokenStream) -> TokenStream {
    let pu = match syn::parse2::<ParsedUnit>(ts) {
        Ok(pu) => pu,
        Err(e) => {
            return e.to_compile_error().into_token_stream();
        }
    };
    quote! {
        #pu
    }
}
