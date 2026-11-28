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
        self.minimum_one_base()?;
        self.no_colliding_aliases()?;

        let mut vc = ValidationContext {
            unit_to_id: Some(&self.unit_registry),
        };
        self.no_bases_are_derived(&vc)?;
        let graph_context = self.validate_matched_bases(&mut vc)?;
        let (alias_to_id, unit_id_to_vector) = self.reduce_unit_equations(&graph_context)?;
        let sys_hash = self.get_hash();
        let unit_generic_names = self.get_unit_generic_names();
        let unit_phantom_data_names = self.get_unit_phantom_data_names();
        let unit_arith_generic_names = self.get_unit_arith_generic_names();

        let mut unit_context = self.unit_context;

        unit_context.unit_id_to_vector = unit_id_to_vector;

        Ok(ValidatedSystem {
            sys_hash: Box::new(sys_hash.to_be_bytes()),
            alias_to_id,
            unit_generic_names,
            unit_phantom_data_names,
            unit_arith_generic_names,
            unit_context,
        })
    }
}
