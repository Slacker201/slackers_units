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

    let mut unit_prefix_trie = PrefixTrie::new();
    let mut prefix_prefix_trie = PrefixTrie::new();

    for abbrev in sys.units.iter().map(|unit| &unit.abbreviation) {
        unit_prefix_trie.add_word(&abbrev.raw);
    }

    for abbrev in sys.prefixes.iter().map(|prefix| &prefix.abbreviation) {
        prefix_prefix_trie.add_word(&abbrev.raw);
    }

    let unit_prefix_trie_map = unit_prefix_trie.to_const_map();
    let prefix_prefix_trie_map = prefix_prefix_trie.to_const_map();

    let trie_decl = quote! {
        const __UNIT_PREFIX_TRIE__: ::slackers_si_units::Map<u8, ::slackers_si_units::ConstTrie> = #unit_prefix_trie_map;
        const __PREFIX_PREFIX_TRIE__: ::slackers_si_units::Map<u8, ::slackers_si_units::ConstTrie> = #prefix_prefix_trie_map;
    };

    quote! {
        #trie_decl
    }
}
