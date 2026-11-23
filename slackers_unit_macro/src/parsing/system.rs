use syn::{Error, parse::Parse};

use crate::data::{
    parser_data::{BaseDefinition, PrefixDefinition, System, UnitDefinition, base, prefix, unit},
    registries::UnitRegistry,
};

impl Parse for System {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut units = vec![];
        let mut prefixes = vec![];
        let mut bases = vec![];
        let mut registry = UnitRegistry::new();
        loop {
            if input.peek(unit) {
                units.push(UnitDefinition::<String>::parse(input, &mut registry)?);
                continue;
            }
            if input.peek(prefix) {
                prefixes.push(PrefixDefinition::parse(input)?);
                continue;
            }
            if input.peek(base) {
                bases.push(BaseDefinition::parse(input)?);
                continue;
            }

            if input.is_empty() {
                break;
            } else {
                return Err(Error::new(input.span(), "Expected eof"));
            }
        }

        let units: Box<[UnitDefinition<crate::data::Id>]> = units
            .into_iter()
            .map(|unit| unit.to_ided(&registry))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            units,
            prefixes: prefixes.into_boxed_slice(),
            bases: bases.into_boxed_slice(),
            registry,
        })
    }
}
