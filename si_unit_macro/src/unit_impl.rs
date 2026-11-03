use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::Parse;

use crate::unit_impl::unit_context::UnitContext;

pub(crate) mod unit_context;

pub(crate) fn unit_impl(tokens: TokenStream) -> TokenStream {
    #[allow(unused)]
    let u = match syn::parse2::<CUnitParserContext>(tokens) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into_token_stream(),
    };

    quote! {
        {
            0
        }
    }
}

#[derive(Debug)]
struct CUnitParserContext {
    #[allow(unused)]
    uc: UnitContext,
}

impl Parse for CUnitParserContext {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            uc: input.parse::<UnitContext>()?,
        })
    }
}
