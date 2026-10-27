use syn::{Token, braced, parse::Parse};

use crate::unit_system::{Conversion, Conversions, Expression};

impl Parse for Conversion {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let from = input.parse::<Expression>()?;
        let _ = input.parse::<Token![:]>();
        let to = input.parse::<Expression>()?;
        let _ = input.parse::<Token![;]>()?;

        Ok(Self { from, to })
    }
}

impl Parse for Conversions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<super::kw::conversions>()?;
        let mut conversions = vec![];

        let content;
        braced!(content in input);

        while let Ok(conversion) = content.parse::<Conversion>() {
            conversions.push(conversion);
        }

        Ok(Self {
            conversions: conversions.into_boxed_slice(),
        })
    }
}
