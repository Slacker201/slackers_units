use syn::{Token, braced, parse::Parse};

use crate::unit_system::{Conversion, Conversions, Expression};

impl Parse for Conversion {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let from = input.parse::<Expression>()?;
        let _ = input.parse::<Token![:]>();
        let to = input.parse::<Expression>()?;
        let end_span = input.parse::<Token![;]>()?.span;

        Ok(Self {
            span: from.span.join(end_span).unwrap_or(input.span()),
            from,
            to,
        })
    }
}

impl Parse for Conversions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_span = input.parse::<super::kw::conversions>()?.span;
        let mut conversions = vec![];

        let content;
        braced!(content in input);

        let mut final_span = content.span();

        while let Ok(conversion) = content.parse::<Conversion>() {
            conversions.push(conversion);
            final_span = content.span();
        }

        Ok(Self {
            conversions: conversions.into_boxed_slice(),
            span: start_span.join(final_span).unwrap_or(input.span()),
        })
    }
}
