use syn::{Error, parse::Parse};

use crate::data::{
    parser_data::{System, UnitDefinition, unit},
    registries::UnitRegistry,
};

impl Parse for System {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut units = vec![];
        let mut registry = UnitRegistry::new();
        loop {
            if input.peek(unit) {
                units.push(UnitDefinition::<String>::parse(input, &mut registry)?);
                continue;
            }

            if input.is_empty() {
                break;
            } else {
                return Err(Error::new(input.span(), "Expected eof"));
            }
        }

        let units = units.into_iter().map(|unit| unit.to_ided(&registry)).collect::<Result<_, _>>()?;

        Ok(Self { units: units })
    }
}
