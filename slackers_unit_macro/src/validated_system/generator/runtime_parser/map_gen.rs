use proc_macro2::TokenStream;
use quote::quote;

use crate::{prefix_trie::PrefixTrie, validated_system::ValidatedSystem};

macro_rules! add_abbrevs {
    ($s: ident, $trie: ident, $abbrev_src: ident, $enum_names: ident) => {
        for (abbrev, enum_name) in $s
            .$abbrev_src
            .iter()
            .map(|prefix| &prefix.abbreviation.raw as &str)
            .zip($s.$enum_names.iter())
        {
            $trie.add_word(abbrev, &enum_name.to_string());
        }
    };
}

impl ValidatedSystem {
    pub(super) fn construct_const_trie_enum() -> TokenStream {
        quote! {
            #[derive(core::fmt::Debug)]
            pub(crate) enum ConstTrie<T: 'static + core::fmt::Debug> {
                Leaf {
                    value: T,
                },
                Node {
                    next: ::slackers_units::Map<u8, ConstTrie<T>>,
                    terminal: Option<T>,
                },
            }
        }
    }

    pub(super) fn construct_parser_maps(&self) -> TokenStream {
        let mut prefix_trie = PrefixTrie::new();
        let mut unit_trie = PrefixTrie::new();

        add_abbrevs!(self, prefix_trie, prefixes, prefix_enum_names);
        add_abbrevs!(self, unit_trie, units, unit_enum_names);

        let pmap_toks = prefix_trie.to_const_map("Prefix");
        let umap_toks = unit_trie.to_const_map("UnitType");

        quote! {
            const PREFIX_MAP: ::slackers_units::Map<u8, ConstTrie<Prefix>> = #pmap_toks;
            const UNIT_MAP: ::slackers_units::Map<u8, ConstTrie<UnitType>> = #umap_toks;
        }
    }

    pub(super) fn construct_trait() -> TokenStream {
        quote! {
            pub(crate) trait FindLongest<T> {
                fn find_longest<'a>(&self, byte: &'a [u8]) -> Option<(T, &'a [u8])>;
            }
        }
    }

    pub(super) fn construct_find_longest_impl() -> TokenStream {
        quote! {
            macro_rules! find_longest_impl {
                ($t: ty) => {
                    impl FindLongest<$t> for ::slackers_units::Map<u8, ConstTrie<$t>> {
                        fn find_longest<'a>(&self, bytes: &'a [u8]) -> Option<($t, &'a [u8])> {
                            let first_byte = bytes.first()?;

                            let node = self.get(first_byte)?;

                            match node {
                                ConstTrie::Leaf { value } => return Some((*value, &bytes[1..])),
                                ConstTrie::Node { next, terminal } => {
                                    if bytes[1..].is_empty() {
                                        terminal.map(|prefix| (prefix, &bytes[1..]))
                                    } else {
                                        next.find_longest(&bytes[1..])
                                            .or_else(|| {
                                                terminal.map(|prefix| (prefix, &bytes[1..]))
                                            })
                                    }
                                }
                            }
                        }
                    }
                }
            }
            find_longest_impl!(Prefix);
            find_longest_impl!(UnitType);
        }
    }
}