use syn::{LitStr, Token, braced, parse::Parse};

use crate::unit_system::{Number, PrefixDeclaration, TypeIdentifier};

impl Parse for PrefixDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<super::kw::prefix>()?;

        let prefix = input.parse::<TypeIdentifier>()?;

        let content;
        braced!(content in input);

        let _ = content.parse::<super::kw::abbrev>()?;
        let _ = content.parse::<Token![:]>()?;
        let abbreviation = content.parse::<LitStr>()?.value();
        let _ = content.parse::<Token![;]>()?;

        let _ = content.parse::<super::kw::multiplier>()?;
        let _ = content.parse::<Token![:]>()?;
        let multiplier = content.parse::<Number>()?;
        let _ = content.parse::<Token![;]>()?;

        Ok(Self {
            prefix,
            abbreviation,
            multiplier,
        })
    }
}
