use syn::{Ident, Token, bracketed, parse::Parse, punctuated::Punctuated};

use crate::data::parser_data::{PrefixGrouping, prefix_classes};

impl Parse for PrefixGrouping {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_span = input.parse::<prefix_classes>()?.span;
        let _ = input.parse::<Token![=]>()?;

        let bracketed;
        let _ = bracketed!(bracketed in input);
        let end_span = input.parse::<Token![,]>()?.span;

        let allowed_groups: Box<[_]> =
            Punctuated::<Ident, Token![,]>::parse_terminated(&bracketed)?
                .into_iter()
                .collect();

        Ok(Self {
            allowed_groups,
            span: start_span.join(end_span).unwrap_or(start_span),
        })
    }
}
