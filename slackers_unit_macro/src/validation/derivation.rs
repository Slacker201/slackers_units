use std::hash::{DefaultHasher, Hash, Hasher};

use quote::format_ident;
use syn::Ident;

use crate::data::{
    Id,
    parser_data::{
        Aliases, BaseDefinition, EqOpt, EqualityExpression, PrefixDefinition, PrefixGrouping,
        System, UnitDefinition,
    },
    registries::NameRegistry,
    unit_context::UnitContext,
};
type Operand1 = Box<[Ident]>;
type Operand2 = Box<[Ident]>;
type Result = Box<[Ident]>;
impl System {
    pub fn get_unit_generic_names(&self) -> Box<[Ident]> {
        (0..self.bases.len())
            .map(|idx| format_ident!("V{idx}"))
            .collect()
    }
    pub fn get_unit_phantom_data_names(&self) -> Box<[Ident]> {
        (0..self.bases.len())
            .map(|idx| format_ident!("_pd_marker_{idx}"))
            .collect()
    }
    pub fn get_unit_arith_generic_names(&self) -> (Operand1, Operand2, Result) {
        (
            (0..self.bases.len())
                .map(|idx| format_ident!("Operand1{idx}"))
                .collect(),
            (0..self.bases.len())
                .map(|idx| format_ident!("Operand2{idx}"))
                .collect(),
            (0..self.bases.len())
                .map(|idx| format_ident!("Result{idx}"))
                .collect(),
        )
    }
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Hash for System {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.units.hash(state);
        self.prefixes.hash(state);
        self.bases.hash(state);
        self.unit_registry.hash(state);
        self.unit_context.hash(state);
    }
}

impl Hash for UnitDefinition<Id> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.id.hash(state);
        self.aliases.hash(state);
        self.prefix_groups.hash(state);
        self.equality.hash(state);
    }
}

impl Hash for Aliases {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.aliases.hash(state);
    }
}

impl Hash for PrefixGrouping {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.allowed_groups.hash(state);
    }
}

impl Hash for EqualityExpression<Id> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}
impl Hash for EqOpt<Id> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Hash for PrefixDefinition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.id.hash(state);
        self.multiplier.hash(state);
        self.aliases.hash(state);
        self.groups.hash(state);
    }
}

impl Hash for BaseDefinition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unit.hash(state);
    }
}

impl Hash for NameRegistry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id_counter.hash(state);
    }
}

impl Hash for UnitContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unit_trie.hash(state);
        self.prefix_trie.hash(state);
        let mut kv_refs: Box<[_]> = self.prefix_id_to_multiplier.iter().collect();
        kv_refs.sort_by_key(|(byte, _)| **byte);
        kv_refs.hash(state);
        let mut kv_refs: Box<[_]> = self.unit_id_to_vector.iter().collect();
        kv_refs.sort_by_key(|(byte, _)| **byte);
        kv_refs.hash(state);
        self.prefix_data.hash(state);
        self.unit_data.hash(state);
    }
}
