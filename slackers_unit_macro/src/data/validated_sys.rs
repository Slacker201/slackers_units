use std::collections::HashMap;

use syn::Ident;

use crate::data::{Id, registries::NameRegistry, unit_context::UnitContext};

type Operand1 = Box<[Ident]>;
type Operand2 = Box<[Ident]>;
type Result = Box<[Ident]>;

#[derive(Debug)]
pub struct ValidatedSystem {
    #[allow(unused)]
    pub alias_to_id: HashMap<String, Id>,

    // System metadata
    pub sys_hash: Box<[u8]>,

    // Unit definition
    pub unit_generic_names: Box<[Ident]>,
    pub unit_phantom_data_names: Box<[Ident]>,
    pub unit_arith_generic_names: (Operand1, Operand2, Result),
    pub unit_context: UnitContext,
}

#[derive(Default)]
pub struct ValidationContext<'a> {
    pub unit_to_id: Option<&'a NameRegistry>,
}
