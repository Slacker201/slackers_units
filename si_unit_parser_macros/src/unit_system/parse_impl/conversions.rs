use syn::{Token, braced, parse::Parse};

use crate::unit_system::{Conversion, Conversions, Expression};



impl Parse for Conversion {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let from = Expression::parse(input)?;
        let _ = input.parse::<Token![:]>();
        let to = Expression::parse(input)?;

        Ok(
            Self { from, to }
        )
    }
}

impl Parse for Conversions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut conversions = vec![];

        let content;
        braced!(content in input);
        
        while let Ok(c) = content.parse::<Conversion>() {
            conversions.push(c);
        }

        Ok(
            Self { conversions: conversions.into_boxed_slice() }
        )
    }
}