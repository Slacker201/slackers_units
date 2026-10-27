use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{System};

pub(crate) fn system_impl(ts: TokenStream) -> TokenStream {
    let _sys = match syn::parse2::<System>(ts) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().to_token_stream();
        }
    };
    
    quote! {}
}
