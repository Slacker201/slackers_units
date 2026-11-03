use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::Parse;

use crate::{prefix_trie::PrefixTrie, unit_impl::{unit_context::UnitContext, unit_str_parser::ParsedUnit}};

pub(crate) mod unit_context;
mod unit_str_parser;

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
struct UnitImpl{
    parsed_unit: ParsedUnit,
}

impl Parse for UnitImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let uc = input.parse::<UnitContext>()?;

        let mut unit_trie = PrefixTrie::new();
        let mut prefix_trie = PrefixTrie::new();

        let mut unit_lookup_table = HashMap::new();
        let mut prefix_lookup_table = HashMap::new();

        let mut len = 0;

        for (abbrev, vector) in uc.units.iter().map(|piece| (&piece.abbrev as &str, &piece.vector)) {
            if len == 0 {
                len = vector.len();
            }
            unit_trie.add_word(abbrev);
            unit_lookup_table.insert(abbrev, vector);
        }

        for (abbrev, multiplier) in uc.prefixes.iter().map(|piece| (&piece.abbrev as &str, piece.multiplier)) {
            prefix_trie.add_word(abbrev);
            prefix_lookup_table.insert(abbrev, multiplier);
        }

        let pu = ParsedUnit::parse(input, &unit_trie, &prefix_trie, &unit_lookup_table, &prefix_lookup_table, len as i64)?;

        eprintln!("{:?}", pu);
        
        Ok(
            Self {
                parsed_unit: pu
            }
        )
    }
}
