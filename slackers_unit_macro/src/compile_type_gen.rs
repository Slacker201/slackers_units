use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::compile_parser_impl::parsed_unit::{ParsedUnit, UnitVectorWrapper};

pub fn compile_type_gen(ts: TokenStream) -> TokenStream {
    let pu = match syn::parse2::<ParsedUnit<false>>(ts) {
        Ok(pu) => pu,
        Err(e) => {
            return e.to_compile_error().into_token_stream();
        }
    };
    let vec = pu.to_uv();
    quote! {
        Unit::<#vec f64>
    }
}

impl<const ALLOW_NUMBER: bool> ParsedUnit<ALLOW_NUMBER> {
    fn to_uv(&self) -> UnitVectorWrapper<'_> {
        UnitVectorWrapper(&self.unit_vector)
    }
}
