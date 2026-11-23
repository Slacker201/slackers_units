use std::collections::HashMap;

use crate::data::{Decimal, Id, registries::UnitRegistry};

#[derive(Debug)]
pub struct ValidatedSystem {
    pub registry: UnitRegistry,
    pub alias_to_id: HashMap<String, Id>,
    pub id_to_vector_and_mult: HashMap<Id, (Decimal, Box<[i32]>)>,
}

#[derive(Default)]
pub struct ValidationContext<'a> {
    pub unit_to_id: Option<&'a UnitRegistry>,
}
