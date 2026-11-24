use std::collections::HashMap;

use syn::Ident;

use crate::data::{Decimal, Id, registries::NameRegistry};

type Operand1 = Box<[Ident]>;
type Operand2 = Box<[Ident]>;
type Result = Box<[Ident]>;

#[derive(Debug)]
pub struct ValidatedSystem {
    #[allow(unused)]
    pub registry: NameRegistry,
    #[allow(unused)]
    pub alias_to_id: HashMap<String, Id>,
    #[allow(unused)]
    pub id_to_vector_and_mult: HashMap<Id, (Decimal, Box<[i32]>)>,

    // System metadata

    // Unit definition
    pub unit_generic_names: Box<[Ident]>,
    pub unit_phantom_data_names: Box<[Ident]>,
    pub unit_arith_generic_names: (Operand1, Operand2, Result),
}

#[derive(Default)]
pub struct ValidationContext<'a> {
    pub unit_to_id: Option<&'a NameRegistry>,
}
