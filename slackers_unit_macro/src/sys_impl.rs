use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{data::parser_data::System, validation::unit_graph::graph_context::GraphContext};

pub(crate) fn system_impl(ts: TokenStream) -> TokenStream {
    let system = match syn::parse2::<System>(ts) {
        Ok(a) => a,
        Err(e) => {
            return e.to_compile_error().to_token_stream();
        }
    };

    let validated_system = match system.validate() {
        Ok(s) => s,
        Err(e) => {
            return e.to_compile_error().to_token_stream();
        }
    };
    quote! {
        #validated_system
    }
}
