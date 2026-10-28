use syn::{LitStr, Token, braced, parse::Parse};

use crate::unit_system::{Abbreviation, Number, PrefixDeclaration, TypeIdentifier};

impl Parse for PrefixDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_span = input.parse::<super::kw::prefix>()?.span;

        let prefix = input.parse::<TypeIdentifier>()?;

        let content;
        braced!(content in input);

        let abbreviation = content.parse::<Abbreviation>()?;

        let _ = content.parse::<super::kw::multiplier>()?;
        let _ = content.parse::<Token![:]>()?;
        let multiplier = content.parse::<Number>()?;
        let end_span = content.parse::<Token![;]>()?.span;

        Ok(Self {
            prefix,
            abbreviation,
            multiplier,

            span: start_span.join(end_span).unwrap_or(input.span()),
        })
    }
}

impl Parse for Abbreviation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<super::kw::abbrev>()?;
        let _ = input.parse::<Token![:]>()?;
        let abbreviation = input.parse::<LitStr>()?;
        let _ = input.parse::<Token![;]>()?;

        Ok(Self {
            raw: abbreviation.value(),
            span: abbreviation.span(),
        })
    }
}
