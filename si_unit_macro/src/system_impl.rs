use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{System, prefix_trie::PrefixTrie};

mod system_validator;

pub(crate) fn system_impl(ts: TokenStream) -> TokenStream {
    let sys = match syn::parse2::<System>(ts) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into_token_stream();
        }
    };

    match sys.validate() {
        Ok(_) => {}
        Err(e) => {
            return e.to_compile_error().to_token_stream();
        }
    }

    let mut prefix_trie = PrefixTrie::new();
    prefix_trie.add_word("str");

    let _ = prefix_trie.to_const_map();

    quote! {}
}
