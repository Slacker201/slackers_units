use std::collections::HashMap;

use proc_macro2::Span;
use syn::Error;

use crate::data::{
    Decimal, Id,
    parser_data::{PrefixDefinition, UnitDefinition},
    unit_context::prefix_trie::PrefixTrie,
};

pub mod prefix_trie;
pub mod serialization;

#[derive(Debug, PartialEq, Clone)]
pub struct UnitContext {
    pub unit_trie: PrefixTrie<Id>,
    pub prefix_trie: PrefixTrie<Id>,

    pub prefix_id_to_multiplier: HashMap<Id, Decimal>,
    pub unit_id_to_vector: HashMap<Id, (Decimal, Box<[i32]>)>,

    pub prefix_data: Box<[(String, Id)]>,
    pub unit_data: Box<[(String, Id)]>,
}

impl UnitContext {
    pub fn new(units: &[UnitDefinition<String>], prefixes: &[PrefixDefinition]) -> Self {
        let mut unit_trie = PrefixTrie::default();
        let mut prefix_trie = PrefixTrie::default();
        let mut prefix_id_to_multiplier = HashMap::new();
        let unit_id_to_vector = HashMap::new();

        for unit in units {
            for alias in &unit.aliases.aliases {
                unit_trie.add_item(alias.to_string().as_bytes(), unit.id);
            }
        }

        let unit_data = units
            .iter()
            .flat_map(|unit_def| {
                unit_def
                    .aliases
                    .aliases
                    .iter()
                    .map(|alia| (alia.to_string(), unit_def.id))
            })
            .collect();

        for prefix in prefixes {
            for alias in &prefix.aliases.aliases {
                prefix_trie.add_item(alias.to_string().as_bytes(), prefix.id);
            }
            prefix_id_to_multiplier.insert(prefix.id, prefix.multiplier);
        }

        let prefix_data = prefixes
            .iter()
            .flat_map(|prefix_def| {
                prefix_def
                    .aliases
                    .aliases
                    .iter()
                    .map(|alia| (alia.to_string(), prefix_def.id))
            })
            .collect();

        Self {
            unit_trie,
            prefix_trie,
            prefix_id_to_multiplier,
            unit_id_to_vector,
            prefix_data,
            unit_data,
        }
    }

    pub fn parse_suffix(&self, suffix: &str, span: Span) -> syn::Result<(Decimal, Id)> {
        if suffix.is_empty() {
            return Err(Error::new(span, "Found no suffix"));
        }
        if let Some((unit, remainder)) = self.unit_trie.find_longest(suffix.as_bytes())
            && remainder.is_empty()
        {
            return Ok((Decimal::ONE, *unit));
        }
        if let Some((prefix, prefix_remainder)) = self.prefix_trie.find_longest(suffix.as_bytes()) {
            if let Some((unit, unit_remainder)) = self.unit_trie.find_longest(prefix_remainder) {
                if unit_remainder.is_empty() {
                    Ok((
                        *self
                            .prefix_id_to_multiplier
                            .get(prefix)
                            .expect("Safe by construction"),
                        *unit,
                    ))
                } else {
                    Err(Error::new(
                        span,
                        format!(
                            "Found trailing characters '{}'",
                            str::from_utf8(unit_remainder).expect("Safe by construction")
                        ),
                    ))
                }
            } else {
                Err(Error::new(
                    span,
                    format!(
                        "Could not parse unit '{}'",
                        str::from_utf8(prefix_remainder).expect("Safe by construction")
                    ),
                ))
            }
        } else {
            Err(Error::new(span, format!("Could not parse '{suffix}'")))
        }
    }
}
