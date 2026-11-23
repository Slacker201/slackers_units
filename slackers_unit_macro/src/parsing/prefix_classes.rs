use proc_macro2::Span;
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

impl PrefixGrouping {
    pub fn new_with_all() -> Self {
        Self {
            allowed_groups: Box::from([Ident::new("All", Span::call_site())]),
            span: Span::call_site(),
        }
    }
}
