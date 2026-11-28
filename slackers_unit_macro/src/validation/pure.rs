use proc_macro2::Span;
use syn::Error;

use crate::data::{parser_data::System, validated_sys::ValidationContext};

impl System {
    pub fn no_colliding_aliases(&self) -> syn::Result<()> {
        let mut unit_aliases: Box<[_]> = self
            .units
            .iter()
            .flat_map(|unit_def| &unit_def.aliases.aliases)
            .collect();
        unit_aliases.sort();
        if let Some(duplicate) = unit_aliases
            .windows(2)
            .find_map(|w| if w[0] == w[1] { w.first() } else { None })
        {
            return Err(Error::new(
                duplicate.span(),
                format!("Found duplicate abbreviation {duplicate}"),
            ));
        }
        Ok(())
    }
    pub fn minimum_one_base(&self) -> syn::Result<()> {
        if self.bases.is_empty() {
            let example_unit = self
                .units
                .iter()
                .find_map(|unit_definition| {
                    if unit_definition.equality.is_none() {
                        Some(unit_definition.name.to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or("ExampleUnit".to_string());
            return Err(Error::new(
                Span::call_site(),
                format!("Declare at minimum 1 base type\nFor example: `base = {example_unit}`"),
            ));
        }

        Ok(())
    }

    pub fn no_bases_are_derived(&self, vc: &ValidationContext) -> syn::Result<()> {
        for base in &self.bases {
            let unit_id = vc
                .unit_to_id
                .unwrap()
                .try_get(&base.unit.to_string(), base.unit.span())
                .map_err(|_| {
                    Error::new(
                        base.unit.span(),
                        format!("Please define {} to use it as a base unit", base.unit),
                    )
                })?;
            let unit = self
                .units
                .iter()
                .find(|unit| unit.id == unit_id)
                .expect("Safe by construction");

            if let Some(eq) = &unit.equality {
                return Err(Error::new(eq.span, "Base units cannot be relative"));
            }
        }
        Ok(())
    }
}
