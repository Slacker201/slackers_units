use syn::{LitBool, LitStr, Token, braced, parse::Parse};

use crate::unit_system::{Conversions, TypeIdentifier, UnitDeclaration};

impl Parse for UnitDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_span = input.parse::<super::kw::unit>()?.span;

        let unit = input.parse::<TypeIdentifier>()?;

        let content;
        braced!(content in input);

        let _ = content.parse::<super::kw::prefixable>()?;
        let _ = content.parse::<Token![:]>()?;
        let prefixable = content.parse::<LitBool>()?.value;
        let _ = content.parse::<Token![;]>()?;

        let _ = content.parse::<super::kw::abbrev>()?;
        let _ = content.parse::<Token![:]>()?;
        let abbreviation = content.parse::<LitStr>()?.value();
        let _ = content.parse::<Token![;]>()?;

        let conversions = content.parse::<Conversions>()?;

        Ok(Self {
            span: start_span.join(conversions.span).unwrap_or(input.span()),

            unit,
            prefixable,
            abbreviation,
            conversions,
        })
    }
}
