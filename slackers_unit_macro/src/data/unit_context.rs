use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use proc_macro2::Span;
use syn::Error;

use crate::data::{
    Decimal, Id,
    parser_data::{PrefixClasses, PrefixDefinition, UnitDefinition},
    unit_context::prefix_trie::PrefixTrie,
};

pub mod prefix_trie;
pub mod serialization;

#[derive(Debug, PartialEq, Clone)]
pub struct UnitContext {
    pub unit_trie: PrefixTrie<Id>,
    pub prefix_trie: PrefixTrie<Id>,

    pub prefix_id_to_multiplier: HashMap<Id, Decimal>,
    pub prefix_id_to_grouping: HashMap<Id, PrefixClasses>,
    pub unit_id_to_vector: HashMap<Id, (Decimal, Box<[i32]>)>,
    pub unit_id_to_grouping: HashMap<Id, PrefixClasses>,

    pub prefix_data: Box<[(String, Id)]>,
    pub unit_data: Box<[(String, Id)]>,
    pub null_id: Id,
}

impl UnitContext {
    pub fn new(
        units: &[UnitDefinition<String>],
        prefixes: &[PrefixDefinition],
        null_id: Id,
    ) -> Self {
        let mut unit_trie = PrefixTrie::default();
        let mut prefix_trie = PrefixTrie::default();
        let mut prefix_id_to_multiplier = HashMap::new();
        let unit_id_to_vector = HashMap::new();
        let mut prefix_id_to_grouping = HashMap::new();
        let mut unit_id_to_grouping = HashMap::new();

        for unit in units {
            for alias in &unit.aliases.aliases {
                unit_trie.add_item(alias.to_string().as_bytes(), unit.id);
            }
            unit_id_to_grouping.insert(unit.id, unit.prefix_groups.clone());
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
            prefix_id_to_grouping.insert(prefix.id, prefix.groups.clone());
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
            prefix_id_to_grouping,
            unit_id_to_vector,
            unit_id_to_grouping,
            prefix_data,
            unit_data,
            null_id,
        }
    }

    pub fn parse_suffix(&self, suffix: &str, span: Span) -> syn::Result<(Decimal, Id)> {
        if suffix.is_empty() {
            return Ok((Decimal::ONE, self.null_id));
        }
        if let Some((unit, remainder)) = self.unit_trie.find_longest(suffix.as_bytes())
            && remainder.is_empty()
        {
            return Ok((Decimal::ONE, *unit));
        }
        if let Some((prefix, prefix_remainder)) = self.prefix_trie.find_longest(suffix.as_bytes()) {
            if let Some((unit, unit_remainder)) = self.unit_trie.find_longest(prefix_remainder) {
                let prefix_name = str::from_utf8(
                    suffix
                        .as_bytes()
                        .strip_suffix(prefix_remainder)
                        .unwrap_or(suffix.as_bytes()),
                )
                .expect("Safe by construction");
                let unit_name = str::from_utf8(prefix_remainder).expect("Safe by construction");
                let prefix_grouping = self
                    .prefix_id_to_grouping
                    .get(prefix)
                    .expect("Safe by construction");
                let unit_grouping = self
                    .unit_id_to_grouping
                    .get(unit)
                    .expect("Safe by construction");
                if !prefix_grouping.contains(unit_grouping) {
                    return Err(Error::new(
                        span,
                        format!(
                            "Unit {unit_name} does not support the prefix {prefix_name}\n{unit_name} supports {unit_grouping}\nThe prefix {prefix_name} is part of {prefix_grouping}",
                        ),
                    ));
                }

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

impl PartialEq for PrefixClasses {
    fn eq(&self, other: &Self) -> bool {
        self.allowed_groups == other.allowed_groups
    }
}

impl PrefixClasses {
    fn contains(&self, other: &PrefixClasses) -> bool {
        let self_set: HashSet<_> = self.allowed_groups.iter().collect();
        other.allowed_groups.iter().any(|v| self_set.contains(v))
    }
}

impl Display for PrefixClasses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.allowed_groups.is_empty() {
            write!(f, "no prefix classes")
        } else {
            write!(f, "`{}`", self.allowed_groups.join("`, `"))
        }
    }
}
