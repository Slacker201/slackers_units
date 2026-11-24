use std::collections::HashMap;

use syn::Ident;

use crate::data::{Decimal, Id, registries::NameRegistry};

#[derive(Debug)]
pub struct ValidatedSystem {
    pub registry: NameRegistry,
    pub alias_to_id: HashMap<String, Id>,
    pub id_to_vector_and_mult: HashMap<Id, (Decimal, Box<[i32]>)>,

    // System metadata

    // Unit definition
    pub unit_generic_names: Box<[Ident]>,
    pub unit_phantom_data_names: Box<[Ident]>,
    pub unit_arith_generic_names: (Box<[Ident]>, Box<[Ident]>, Box<[Ident]>),
}

#[derive(Default)]
pub struct ValidationContext<'a> {
    pub unit_to_id: Option<&'a NameRegistry>,
}
