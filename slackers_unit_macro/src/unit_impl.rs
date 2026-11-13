use std::{collections::HashMap, sync::LazyLock};

use dashmap::DashMap;
use num_rational::Ratio;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{LitByteStr, LitInt, LitStr, parse::Parse};

use crate::{
    data_transfer::ParserData, prefix_trie::PrefixTrie, unit_impl::unit_str_parser::ParsedUnit,
    unit_system::NumberRepr,
};

static PARSER_CONTEXT_CACHE: LazyLock<DashMap<u64, DerivedParseData>> = LazyLock::new(
    || DashMap::new()
);

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

pub(crate) struct DerivedParseData {
    #[allow(unused)]
    pub(crate) pd: ParserData,
    pub(crate) unit_trie: PrefixTrie,
    pub(crate) prefix_trie: PrefixTrie,
    pub(crate) unit_lookup_table: HashMap<String, Box<[i64]>>,
    pub(crate) prefix_lookup_table: HashMap<String, Ratio<NumberRepr>>,
    pub(crate) dimension_count: i64,
    pub(crate) call_path: String,
}

#[derive(Debug)]
struct UnitImpl {
    parsed_unit: ParsedUnit,
}

impl Parse for UnitImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let hash = input.parse::<LitInt>()?.base10_parse::<u64>().unwrap();
        if PARSER_CONTEXT_CACHE.contains_key(&hash) {
            let _ = input.parse::<LitByteStr>()?;
            let _ = input.parse::<LitStr>()?;
        }
        let uc = PARSER_CONTEXT_CACHE.entry(
            hash
        ).or_try_insert_with::<syn::Error>(
            || {
                eprintln!("Cache miss");
                let uc = input.parse::<ParserData>()?;
                let call_path = input.parse::<LitStr>()?.value();
                Ok(uc.into_derived(call_path))
            }
        )?;

        let pu = ParsedUnit::parse(input, uc.into_self())?;

        Ok(Self { parsed_unit: pu })
    }
}

impl ParserData {
    fn into_derived(self, call_path: String) -> DerivedParseData {
        let mut unit_trie = PrefixTrie::new();
        let mut prefix_trie = PrefixTrie::new();

        let mut unit_lookup_table = HashMap::new();
        let mut prefix_lookup_table = HashMap::new();

        for (abbrev, vector) in self.unit_vectors.iter().as_ref() {
            unit_trie.add_word(abbrev, abbrev);
            unit_lookup_table.insert(abbrev.to_string(), vector.clone());
        }

        for (abbrev, multiplier) in self.prefix_multipliers.iter().as_ref() {
            prefix_trie.add_word(abbrev, abbrev);
            prefix_lookup_table.insert(abbrev.to_string(), *multiplier);
        }
        DerivedParseData {
            unit_trie,
            prefix_trie,
            unit_lookup_table,
            prefix_lookup_table,
            dimension_count: self.dimension_count.cast_signed() as i64,
            pd: self,
            call_path,
        }
    }
}

impl DerivedParseData {
    fn into_self(&self) -> &Self {
        &self
    }
}