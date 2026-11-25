use crate::data::{
    parser_data::System,
    validated_sys::{ValidatedSystem, ValidationContext},
};

pub mod bases;
pub mod derivation;
pub mod pure;
pub mod unit_graph;

impl System {
    pub fn validate(self) -> syn::Result<ValidatedSystem> {
        self.no_colliding_aliases()?;

        let mut vc = ValidationContext { unit_to_id: Some(&self.unit_registry) };
        let graph_context = self.validate_matched_bases(&mut vc)?;
        let (alias_to_id, id_to_vector_and_mult) = self.reduce_unit_equations(&graph_context)?;

        let unit_generic_names = self.get_unit_generic_names();
        let unit_phantom_data_names = self.get_unit_phantom_data_names();
        let unit_arith_generic_names = self.get_unit_arith_generic_names();

        Ok(ValidatedSystem {
            unit_context: self.unit_context,
            alias_to_id,
            id_to_vector_and_mult,
            unit_generic_names,
            unit_phantom_data_names,
            unit_arith_generic_names,
        })
    }
}
