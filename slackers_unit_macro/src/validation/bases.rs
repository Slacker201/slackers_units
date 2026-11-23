use std::collections::HashMap;

use syn::Error;

use crate::{
    data::{parser_data::System, validated_sys::ValidationContext},
    validation::unit_graph::graph_context::GraphContext,
};

impl System {
    pub(crate) fn validate_matched_bases(
        &self,
        vc: &mut ValidationContext,
    ) -> syn::Result<GraphContext> {
        let leaf_count = self.bases.len();
        let mut base_to_vector_mapping = HashMap::new();
        for (idx, base) in self.bases.iter().map(|b| &b.unit).enumerate() {
            let Ok(id) = vc
                .unit_to_id
                .unwrap()
                .try_get(&base.to_string(), base.span())
            else {
                return Err(Error::new(
                    base.span(),
                    format!("Please define {} to use it as a base unit", base),
                ));
            };

            if base_to_vector_mapping
                .insert(
                    id,
                    (0..leaf_count)
                        .map(|v| if v == idx { 1 } else { 0 })
                        .collect(),
                )
                .is_some()
            {
                return Err(Error::new(
                    base.span(),
                    format!("Found duplicate declarations of base {base}"),
                ));
            }
        }

        Ok(GraphContext {
            base_to_vector_mapping,
        })
    }
}
