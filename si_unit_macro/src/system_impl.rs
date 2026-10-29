use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::System;
mod system_validator;

pub(crate) fn system_impl(ts: TokenStream) -> TokenStream {
    let sys = match syn::parse2::<System>(ts) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into_token_stream();
        }
    };

    let vs = match sys.validate() {
        Ok(validated_system) => validated_system,
        Err(e) => {
            return e.to_compile_error().to_token_stream();
        }
    };

    let suffix_parser = vs.generate_suffix_parser();

    quote! {
        #suffix_parser
    }
}
