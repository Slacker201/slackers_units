use syn::{Error, parse::Parse};

use crate::data::{
    parser_data::{BaseDefinition, PrefixDefinition, System, UnitDefinition, base, prefix, unit},
    registries::NameRegistry,
    unit_context::UnitContext,
};

impl Parse for System {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut units = vec![];
        let mut prefixes = vec![];
        let mut bases = vec![];
        let mut unit_registry = NameRegistry::new();
        let mut prefix_registry = NameRegistry::new();
        loop {
            if input.peek(unit) {
                units.push(UnitDefinition::<String>::parse(input, &mut unit_registry)?);
                continue;
            }
            if input.peek(prefix) {
                prefixes.push(PrefixDefinition::parse(input, &mut prefix_registry)?);
                continue;
            }
            if input.peek(base) {
                bases.push(BaseDefinition::parse(input)?);
                continue;
            }

            if input.is_empty() {
                break;
            }
            return Err(Error::new(input.span(), "Expected eof"));
        }
        let null_id = unit_registry.get_id("");
        let unit_context = UnitContext::new(&units, &prefixes, null_id);

        let bytes = unit_context.serialize();
        let other = UnitContext::deserialize(&bytes);
        assert_eq!(unit_context, other);

        let units: Box<[UnitDefinition<crate::data::Id>]> = units
            .into_iter()
            .map(|unit| unit.resolve_suffixes(&unit_context))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            unit_context,
            units,
            prefixes: prefixes.into_boxed_slice(),
            bases: bases.into_boxed_slice(),
            unit_registry,
        })
    }
}
