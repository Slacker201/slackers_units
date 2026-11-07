use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::System;
pub(crate) mod system_validator;

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

    let runtime_parser = vs.generate_runtime_parser();
    let unit_prefix_enums = vs.generate_unit_prefix_enums();
    let unit_impl = vs.generate_units();
    let unit_macro_redef = vs.generate_unit_macro();

    quote! {
        #unit_prefix_enums
        #unit_impl
        #unit_macro_redef
        #runtime_parser
    }
}
