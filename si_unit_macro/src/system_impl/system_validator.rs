use std::collections::HashMap;

use syn::Error;

use crate::System;

impl System {
    pub(crate) fn validate(&self) -> syn::Result<()> {
        self.validate_no_duplicate_type()?;

        Ok(())
    }

    fn validate_no_duplicate_type(&self) -> syn::Result<()> {
        let unit_types: Box<[_]> = self
            .units
            .iter()
            .map(|c| &c.unit)
            .chain(self.prefixes.iter().map(|c| &c.prefix))
            .collect();

        let mut seen_ident = HashMap::new();

        for unit_type in unit_types {
            let new = seen_ident.insert(&unit_type.raw, unit_type);
            if new.is_some() {
                let err_1 = Error::new(
                    unit_type.span,
                    format!("Found duplicate type declaration `{}`", unit_type.raw),
                );
                return Err(err_1);
            }
        }
        Ok(())
    }
}
