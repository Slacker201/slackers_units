use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{prefix_trie::PrefixTrie, validated_system::ValidatedSystem};


impl ValidatedSystem {
    pub(crate) fn generate_suffix_parser(&self) -> TokenStream {
        let mut unit_prefix_trie = PrefixTrie::new();
        let mut prefix_prefix_trie = PrefixTrie::new();

        let mut unit_root = false;

        for abbrev in self.units.iter().map(|unit| &unit.abbreviation) {
            if abbrev.raw.len() == 1 {
                unit_root = true;
            }
            unit_prefix_trie.add_word(&abbrev.raw);
        }

        let mut prefix_root = false;

        for abbrev in self.prefixes.iter().map(|prefix| &prefix.abbreviation) {
            if abbrev.raw.len() == 1 {
                prefix_root = true;
            }
            prefix_prefix_trie.add_word(&abbrev.raw);
        }

        let unit_prefix_trie_map = unit_prefix_trie.to_const_map();
        let prefix_prefix_trie_map = prefix_prefix_trie.to_const_map();

        let unit_abbrev = self
            .units
            .iter()
            .map(|unit| &unit.abbreviation.raw as &str)
            .collect();

        let unit_ident = &self.unit_enum_names;

        let prefix_abbrev = self
            .prefixes
            .iter()
            .map(|prefix| &prefix.abbreviation.raw as &str)
            .collect();

        let prefix_ident = &self.prefix_enum_names;

        let prefix_type_parsers =
            Self::generate_type_parsers(unit_abbrev, unit_ident, prefix_abbrev, prefix_ident);

        quote! {
            const __UNIT_PREFIX_TRIE__: ::slackers_si_units::Map<u8, ::slackers_si_units::ConstTrie> =
                #unit_prefix_trie_map;
            const __PREFIX_PREFIX_TRIE__: ::slackers_si_units::Map<u8, ::slackers_si_units::ConstTrie> =
                #prefix_prefix_trie_map;

            pub struct Parser;

            pub(crate) mod __parser__ {
                impl crate::Parser {
                    pub fn parse_prefix(
                        prefix: &str,
                    ) -> Result<(crate::Prefix, crate::UnitType), ::slackers_si_units::ParseError> {
                        let bytes = prefix.as_bytes();
                        match ::slackers_si_units::FindFirst::find_longest(
                            &crate::__PREFIX_PREFIX_TRIE__,
                            bytes,
                            #prefix_root,
                        ) {
                            Ok((prefix, remainder)) => {
                                match ::slackers_si_units::FindFirst::find_longest(
                                    &crate::__UNIT_PREFIX_TRIE__,
                                    remainder,
                                    #unit_root,
                                ) {
                                    Ok((unit, remainder)) => {
                                        if !remainder.is_empty() {
                                            Err(::slackers_si_units::ParseError::TrailingJunk(
                                                remainder.into(),
                                            ))
                                        } else {
                                            Ok((crate::Prefix::parse(prefix)?, crate::UnitType::parse(unit)?))
                                        }
                                    }
                                    Err(e) => {
                                        let (unit_vec, remainder) =
                                            ::slackers_si_units::FindFirst::find_longest(
                                                &crate::__UNIT_PREFIX_TRIE__,
                                                bytes,
                                                #unit_root,
                                            )?;
                                        if !remainder.is_empty() {
                                            Err(::slackers_si_units::ParseError::TrailingJunk(
                                                remainder.into(),
                                            ))
                                        } else {
                                            Ok((crate::Prefix::None, crate::UnitType::parse(unit_vec)?))
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                let (unit_vec, remainder) = ::slackers_si_units::FindFirst::find_longest(
                                    &crate::__UNIT_PREFIX_TRIE__,
                                    bytes,
                                    #unit_root,
                                )?;

                                if !remainder.is_empty() {
                                    return Err(::slackers_si_units::ParseError::TrailingJunk(
                                        remainder.into(),
                                    ));
                                } else {
                                    Ok((crate::Prefix::None, crate::UnitType::parse(unit_vec)?))
                                }
                            }
                        }
                    }
                }

                #prefix_type_parsers
            }
        }
    }

    fn generate_type_parsers(
        #[allow(clippy::boxed_local)] unit_abbrev: Box<[&str]>,
        unit_ident: &[Ident],
        #[allow(clippy::boxed_local)] prefix_abbrev: Box<[&str]>,
        prefix_ident: &[Ident],
    ) -> TokenStream {

        let unit_type_count = unit_ident.len();
        let prefix_count = prefix_ident.len();

        quote! {
            impl crate::UnitType {
                fn parse(mut input: Vec<u8>) -> Result<Self, ::slackers_si_units::ParseError> {
                    const UNIT_TYPES: [(&'static str, crate::UnitType); #unit_type_count] =
                        [
                            #(
                                (
                                    #unit_abbrev,
                                    crate::UnitType::#unit_ident
                                ),
                            )*
                        ];

                    input.reverse();

                    for (bytes, unit) in UNIT_TYPES.iter().map(|(s, unit)| (s.as_bytes(), unit)) {
                        if bytes == &input {
                            return Ok(*unit);
                        }
                    }

                    Err(::slackers_si_units::ParseError::UnknownUnit(input.into()))
                }
            }

            impl crate::Prefix {
                fn parse(mut input: Vec<u8>) -> Result<Self, ::slackers_si_units::ParseError> {
                    const PREFIX_TYPES: [(&'static str, crate::Prefix); #prefix_count] =
                        [
                            #(
                                (
                                    #prefix_abbrev,
                                    crate::Prefix::#prefix_ident
                                ),
                            )*
                        ];

                    input.reverse();

                    for (bytes, prefix) in PREFIX_TYPES
                        .iter()
                        .map(|(s, prefix)| (s.as_bytes(), prefix))
                    {
                        if bytes == &input {
                            return Ok(*prefix);
                        }
                    }

                    Err(::slackers_si_units::ParseError::UnknownUnit(input.into()))
                }
            }
        }
    }
}