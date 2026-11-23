use std::collections::HashMap;

use proc_macro2::Span;
use syn::Error;

use crate::data::Id;

#[derive(Clone, Debug, Default)]
pub struct UnitRegistry {
    to_id: HashMap<String, Id>,
    id_counter: u64,
}

impl UnitRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_id(&mut self, s: impl Into<String>) -> Id {
        *self.to_id.entry(s.into()).or_insert_with(|| {
            let id = Id(self.id_counter);
            self.id_counter += 1;
            id
        })
    }
    pub fn add_alias(&mut self, id: Id, s: impl Into<String>) {
        self.to_id.insert(s.into(), id);
    }
    pub fn try_get(&self, s: &str, span: Span) -> syn::Result<Id> {
        self.to_id
            .get(s)
            .copied()
            .ok_or_else(|| Error::new(span, format!("Undefined unit: {s}")))
    }
}
