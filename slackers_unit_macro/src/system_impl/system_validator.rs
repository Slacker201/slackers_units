use std::collections::{HashMap, HashSet};

use quote::format_ident;
use syn::Error;

use crate::{System, validated_system::ValidatedSystem};

pub(crate) mod validator_utils;

macro_rules! validate_convesion_subset {
    ($v: expr) => {
        if let Some(s) = $v.contains_power() {
            return Err(Error::new(
                s,
                "Exponent conversions are not supported at this time",
            ));
        }
        if $v.contains_two_units().is_two() {
            return Err(Error::new(
                $v.span(),
                "Exponent conversions are not supported at this time",
            ));
        }
    };
}

impl System {
    pub(crate) fn validate(self) -> syn::Result<ValidatedSystem> {
        self.validate_no_duplicate_type()?;
        self.validate_no_duplicate_abbreviations()?;
        self.validate_sufficient_base_types()?;
        self.validate_no_none_prefix()?;
        self.validate_no_conversions()?;
        self.validate_conversion_expr_subset()?;
        let base_type_mapping = self.validate_all_bases_match_units()?;

        let unit_idents = self
            .units
            .iter()
            .map(|c| format_ident!("{}", c.unit.raw))
            .collect();

        let prefix_idents = self
            .prefixes
            .iter()
            .map(|c| format_ident!("{}", c.prefix.raw))
            .collect();

        let (dimensional_vector_names, dimensional_phantom_data_names): (Vec<_>, Vec<_>) = (0
            ..self.base_types.len())
            .map(|v| {
                (
                    format_ident!("V{}", v),
                    format_ident!("__phantom_data_v{}__", v),
                )
            })
            .unzip();

        let base_types_raw_vectors = (0..self.base_types.len())
            .map(|vector_index| {
                (0..self.base_types.len())
                    .map(|index| i64::from(index == vector_index))
                    .collect()
            })
            .collect();

        Ok(ValidatedSystem {
            units: self.units,
            base_types: self.base_types,
            prefixes: self.prefixes,
            unit_enum_names: unit_idents,
            prefix_enum_names: prefix_idents,
            base_type_mapping,
            dimensional_vector_names: dimensional_vector_names.into_boxed_slice(),
            dimensional_phantom_data_names: dimensional_phantom_data_names.into_boxed_slice(),
            base_types_raw_vectors,
        })
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
        if self.base_types.is_empty() {
            return Err(Error::new(
                self.span,
                "Found no base types, please add at least 1",
            ));
        }
        Ok(())
    }

    fn validate_no_none_prefix(&self) -> syn::Result<()> {
        if let Some(v) = self.prefixes.iter().find(|c| c.prefix.raw == "None") {
            Err(Error::new(v.span, "Cannot have a type named `None`"))
        } else {
            Ok(())
        }
    }

    // TODO remove once a = b^2 is supported
    fn validate_conversion_expr_subset(&self) -> syn::Result<()> {
        for conversion in self.units.iter().flat_map(|c| &c.conversions.conversions) {
            validate_convesion_subset!(conversion.from.expr);
            validate_convesion_subset!(conversion.to.expr);
        }

        Ok(())
    }

    fn validate_no_conversions(&self) -> syn::Result<()> {
        if let Some(v) = self
            .units
            .iter()
            .flat_map(|c| &c.conversions.conversions)
            .next()
        {
            return Err(Error::new(
                v.span,
                "Conversions are not supported at this time",
            ));
        }

        Ok(())
    }

    fn validate_all_bases_match_units(&self) -> syn::Result<Box<[usize]>> {
        let mut bases = HashSet::with_capacity(self.base_types.len());
        let mut indices = Vec::with_capacity(self.base_types.len());

        for base in &self.base_types {
            if let Some((idx, _)) = self
                .units
                .iter()
                .enumerate()
                .find(|(_, unit_decl)| unit_decl.unit.raw == base.ty.raw)
            {
                bases.insert(&base.ty.raw);
                indices.push(idx);
            } else {
                return Err(Error::new(
                    base.ty.span,
                    format!(
                        "Please declare the type `{}` to use it as a base type",
                        base.ty.raw
                    ),
                ));
            }
        }
        Ok(indices.into_boxed_slice())
    }
}
