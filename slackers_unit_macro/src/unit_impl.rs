use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::Parse;

use crate::unit_impl::{unit_context::UnitContext, unit_str_parser::ParsedUnit};

pub(crate) mod unit_context;
pub(crate) mod unit_str_parser;

pub(crate) fn unit_impl(tokens: TokenStream) -> TokenStream {
    #[allow(unused)]
    let u = match syn::parse2::<UnitImpl>(tokens) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into_token_stream(),
    };

    let pu = u.parsed_unit;

    quote! {
        {
            #pu
        }
    }
}

#[derive(Debug)]
struct UnitImpl {
    parsed_unit: ParsedUnit,
}

impl Parse for UnitImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let uc = input.parse::<UnitContext>()?;

        let pu = ParsedUnit::parse(input, &uc)?;

        Ok(Self { parsed_unit: pu })
    }
}
