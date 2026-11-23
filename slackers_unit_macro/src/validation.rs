use crate::data::{
    parser_data::System,
    validated_sys::{ValidatedSystem, ValidationContext},
};

pub mod bases;
pub mod pure;
pub mod unit_graph;

impl System {
    pub fn validate(self) -> syn::Result<ValidatedSystem> {
        self.no_colliding_aliases()?;

        let mut vc = ValidationContext::default();
        vc.unit_to_id = Some(&self.registry);
        let graph_context = self.validate_matched_bases(&mut vc)?;
        let (alias_to_id, id_to_vector_and_mult) = self.reduce_unit_equations(&graph_context)?;

        Ok(ValidatedSystem {
            registry: self.registry,
            alias_to_id,
            id_to_vector_and_mult,
        })
    }
}
