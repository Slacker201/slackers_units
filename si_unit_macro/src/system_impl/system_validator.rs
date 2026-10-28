use std::collections::{HashMap, HashSet};

use syn::Error;

use crate::System;

impl System {
    pub(crate) fn validate(&self) -> syn::Result<()> {
        self.validate_no_duplicate_type()?;
        self.validate_no_duplicate_abbreviations()?;
        self.validate_sufficient_base_types()?;
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

    fn validate_no_duplicate_abbreviations(&self) -> syn::Result<()> {
        let mut seen_unit_abbrev = HashSet::new();
        for abbrev in self.units.iter().map(|unit| &unit.abbreviation) {
            if !seen_unit_abbrev.insert(abbrev) {
                return Err(Error::new(
                    abbrev.span,
                    format!("Found duplicate unit abbreviation {}", abbrev.raw),
                ));
            }
        }

        let mut seen_prefix_abbrev = HashSet::new();
        for abbrev in self.prefixes.iter().map(|prefix| &prefix.abbreviation) {
            if !seen_prefix_abbrev.insert(abbrev) {
                return Err(Error::new(
                    abbrev.span,
                    format!("Found duplicate prefix abbreviation {}", abbrev.raw),
                ));
            }
        }

        Ok(())
    }

    fn validate_sufficient_base_types(&self) -> syn::Result<()> {
        if self.base_types.len() < 1 {
            return Err(
                Error::new(self.span, 
                    "Found no base types, please add at least 1"
                )
            );
        }
        Ok(())
    }
}
