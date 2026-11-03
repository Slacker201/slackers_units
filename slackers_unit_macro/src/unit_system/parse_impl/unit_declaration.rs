use syn::{LitBool, Token, braced, parse::Parse};

use crate::unit_system::{Abbreviation, Conversions, TypeIdentifier, UnitDeclaration};

impl Parse for UnitDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_span = input.parse::<super::kw::unit>()?.span;

        let unit = input.parse::<TypeIdentifier>()?;

        let content;
        braced!(content in input);

        let prefixable = content.parse::<Prefixable>()?;
        let _ = content.parse::<Token![;]>()?;

        let abbreviation = content.parse::<Abbreviation>()?;

        let conversions = content.parse::<Conversions>().unwrap_or(Conversions {
            conversions: Box::new([]),
            span: content.span(),
        });

        Ok(Self {
            span: start_span.join(conversions.span).unwrap_or(input.span()),

            unit,
            prefixable: prefixable.prefixable,
            abbreviation,
            conversions,
        })
    }
}

struct Prefixable {
    prefixable: bool,
}

impl Parse for Prefixable {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<super::kw::prefixable>()?;
        let _ = input.parse::<Token![:]>()?;
        let b = input.parse::<LitBool>()?;

        Ok(Self {
            prefixable: b.value(),
        })
    }
}
